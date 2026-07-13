use std::collections::HashSet;

use ivy_core::domain::project::Target;
use serde::{Deserialize, Serialize};

use crate::llm::provider::LlmProvider;
use crate::tools::schema::McpToolSchema;

/// A single tool invocation within an [`ExecutionPlan`], as decided by the AI planner.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlannedStep {
    pub order: u32,
    pub tool: String,
    pub parameters: serde_json::Value,
    #[serde(default)]
    pub depends_on: Vec<u32>,
}

/// Structured recon plan produced by [`Planner::plan`]. This is what gets stored as
/// `ScanSession::execution_plan` and shown to the user for review before FR-009 executes it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExecutionPlan {
    pub steps: Vec<PlannedStep>,
}

pub struct Planner;

impl Planner {
    /// Ask the LLM to turn `targets` into an ordered tool execution plan.
    ///
    /// `available_tools` must already be scope-filtered by the caller (project scope
    /// config + intensity resolved upstream, per the layered enforcement in
    /// `.agents/rules/overview.md`) — this is the last of those layers: any step the
    /// LLM proposes for a tool outside that set is dropped, along with dependencies
    /// that pointed at a dropped step.
    pub async fn plan(
        provider: &dyn LlmProvider,
        targets: &[Target],
        available_tools: &[McpToolSchema],
        intensity: &str,
        user_context: Option<&str>,
    ) -> Result<ExecutionPlan, anyhow::Error> {
        let prompt = build_prompt(targets, available_tools, intensity, user_context);
        let raw = provider.generate(&prompt).await?;
        let plan = parse_plan(&raw)?;

        let allowed: HashSet<&str> = available_tools.iter().map(|t| t.name.as_str()).collect();
        let mut steps: Vec<PlannedStep> = plan
            .steps
            .into_iter()
            .filter(|s| allowed.contains(s.tool.as_str()))
            .collect();

        let remaining_orders: HashSet<u32> = steps.iter().map(|s| s.order).collect();
        for step in &mut steps {
            step.depends_on.retain(|dep| remaining_orders.contains(dep));
        }

        Ok(ExecutionPlan { steps })
    }
}

fn build_prompt(
    targets: &[Target],
    available_tools: &[McpToolSchema],
    intensity: &str,
    user_context: Option<&str>,
) -> String {
    let targets_json = serde_json::json!(
        targets
            .iter()
            .map(|t| serde_json::json!({ "value": t.value, "type": t.target_type }))
            .collect::<Vec<_>>()
    );
    let tools_json = serde_json::json!(
        available_tools
            .iter()
            .map(|t| serde_json::json!({ "name": t.name, "description": t.description }))
            .collect::<Vec<_>>()
    );

    format!(
        "You are Ivy's OSINT recon planner. Given the targets, intensity, and available \
         tools below, produce an execution plan as JSON only (no prose, no markdown \
         fences), matching this shape exactly:\n\
         {{\"steps\":[{{\"order\":1,\"tool\":\"<tool name>\",\"parameters\":{{}},\"depends_on\":[]}}]}}\n\n\
         Only use tools from the available tools list. Order steps so dependencies \
         (e.g. subdomain enumeration before port scanning) run first, and reference \
         them by their \"order\" number in \"depends_on\".\n\n\
         Targets: {targets_json}\n\
         Intensity: {intensity}\n\
         Available tools: {tools_json}\n\
         User context: {}\n",
        user_context.unwrap_or("none")
    )
}

fn parse_plan(raw: &str) -> Result<ExecutionPlan, anyhow::Error> {
    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    serde_json::from_str(cleaned)
        .map_err(|e| anyhow::anyhow!("planner: LLM did not return a valid ExecutionPlan JSON: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use ivy_core::domain::project::TargetType;
    use uuid::Uuid;

    fn tool(name: &str) -> McpToolSchema {
        McpToolSchema {
            name: name.to_string(),
            description: String::new(),
            input_schema: serde_json::json!({}),
            output_schema: serde_json::json!({}),
        }
    }

    fn target(value: &str) -> Target {
        Target {
            id: Uuid::nil(),
            project_id: Uuid::nil(),
            value: value.to_string(),
            target_type: TargetType::Domain,
            in_scope: true,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn parse_plan_strips_markdown_fence() {
        let raw = "```json\n{\"steps\":[{\"order\":1,\"tool\":\"amass\",\"parameters\":{},\"depends_on\":[]}]}\n```";
        let plan = parse_plan(raw).unwrap();
        assert_eq!(plan.steps.len(), 1);
        assert_eq!(plan.steps[0].tool, "amass");
    }

    #[test]
    fn parse_plan_errors_on_invalid_json() {
        assert!(parse_plan("not json").is_err());
    }

    struct FakeProvider(&'static str);

    #[async_trait::async_trait]
    impl LlmProvider for FakeProvider {
        async fn generate(&self, _prompt: &str) -> Result<String, anyhow::Error> {
            Ok(self.0.to_string())
        }
    }

    #[tokio::test]
    async fn plan_drops_steps_outside_available_tools_and_their_dangling_deps() {
        let provider = FakeProvider(
            r#"{"steps":[
                {"order":1,"tool":"amass","parameters":{},"depends_on":[]},
                {"order":2,"tool":"nuclei","parameters":{},"depends_on":[1]},
                {"order":3,"tool":"subfinder","parameters":{},"depends_on":[2]}
            ]}"#,
        );

        let targets = vec![target("example.com")];
        let available = vec![tool("amass"), tool("subfinder")];

        let plan = Planner::plan(&provider, &targets, &available, "light", None)
            .await
            .unwrap();

        let tools: Vec<&str> = plan.steps.iter().map(|s| s.tool.as_str()).collect();
        assert_eq!(tools, vec!["amass", "subfinder"]);

        let subfinder_step = plan.steps.iter().find(|s| s.tool == "subfinder").unwrap();
        assert!(
            subfinder_step.depends_on.is_empty(),
            "dependency on the dropped nuclei step should be removed"
        );
    }

    #[tokio::test]
    async fn plan_errors_when_llm_output_is_not_json() {
        let provider = FakeProvider("I refuse to answer");
        let targets = vec![target("example.com")];
        let available = vec![tool("amass")];

        let result = Planner::plan(&provider, &targets, &available, "light", None).await;
        assert!(result.is_err());
    }
}
