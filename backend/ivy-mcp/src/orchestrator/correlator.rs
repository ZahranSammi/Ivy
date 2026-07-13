use std::collections::HashSet;

use ivy_core::domain::finding::Finding;
use ivy_core::domain::graph_edge::GraphEdge;
use ivy_core::domain::graph_node::GraphNode;
use ivy_core::traits::graph_store::GraphStore;

use crate::llm::provider::LlmProvider;
use crate::orchestrator::executor::StepResult;

/// FR-010 false-positive filter: findings below this confidence are dropped, not stored.
// ponytail: fixed cutoff, not a per-project config knob — add one if a project
// ever needs a different bar than 0.5.
pub const MIN_CONFIDENCE: f32 = 0.5;

pub struct Correlator;

impl Correlator {
    /// Ask the LLM to correlate completed tool outputs from `results` into deduplicated,
    /// related [`Finding`]s, then persist every finding that clears [`MIN_CONFIDENCE`]
    /// to `graph_store` as nodes and edges.
    pub async fn correlate(
        provider: &dyn LlmProvider,
        graph_store: &dyn GraphStore,
        results: &[StepResult],
    ) -> Result<Vec<Finding>, anyhow::Error> {
        let prompt = build_prompt(results);
        let raw = provider.generate(&prompt).await?;
        let findings = parse_findings(&raw)?;

        let kept: Vec<Finding> = findings
            .into_iter()
            .filter(|f| f.confidence >= MIN_CONFIDENCE)
            .collect();

        let mut seen_nodes = HashSet::new();
        for finding in &kept {
            let node = finding_to_node(finding);
            if seen_nodes.insert(node.id.clone()) {
                graph_store.insert_node(&node).await?;
            }
            for edge in finding_to_edges(finding, &node.id) {
                graph_store.insert_edge(&edge).await?;
            }
        }

        Ok(kept)
    }
}

fn build_prompt(results: &[StepResult]) -> String {
    let outputs = serde_json::json!(
        results
            .iter()
            .filter(|r| r.output.is_some())
            .map(|r| serde_json::json!({ "tool": r.tool, "output": r.output }))
            .collect::<Vec<_>>()
    );

    format!(
        "You are Ivy's OSINT result correlator. Given the raw tool outputs below, \
         identify entities (Subdomain, IPAddress, Port, Service, Vulnerability, etc.) \
         and their relationships, merging duplicate entities reported by more than one \
         tool into a single finding. Reply with JSON only (no prose, no markdown \
         fences), matching this shape exactly:\n\
         {{\"findings\":[{{\"type\":\"Subdomain\",\"data\":{{}},\"confidence\":0.9,\
         \"relationships\":[{{\"type\":\"HAS_SUBDOMAIN\",\"from_type\":\"Domain\",\"from_key\":\"example.com\"}}]}}]}}\n\n\
         Set \"confidence\" between 0.0 and 1.0, lower for speculative or single-source \
         findings you suspect may be false positives.\n\n\
         Tool outputs: {outputs}\n"
    )
}

fn parse_findings(raw: &str) -> Result<Vec<Finding>, anyhow::Error> {
    #[derive(serde::Deserialize)]
    struct FindingsResponse {
        findings: Vec<Finding>,
    }

    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    let response: FindingsResponse = serde_json::from_str(cleaned)
        .map_err(|e| anyhow::anyhow!("correlator: LLM did not return valid findings JSON: {e}"))?;
    Ok(response.findings)
}

/// Derive a stable node id from a finding's type and its identifying data field, so
/// re-correlating the same entity (e.g. a subdomain seen by two tools) maps to the same
/// node instead of creating a duplicate.
fn finding_to_node(finding: &Finding) -> GraphNode {
    let key = finding
        .data
        .get("name")
        .or_else(|| finding.data.get("value"))
        .or_else(|| finding.data.get("address"))
        .or_else(|| finding.data.get("id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| finding.data.to_string());

    GraphNode {
        id: format!("{}:{key}", finding.r#type),
        labels: vec![finding.r#type.clone()],
        properties: finding.data.clone(),
    }
}

fn finding_to_edges(finding: &Finding, node_id: &str) -> Vec<GraphEdge> {
    finding
        .relationships
        .iter()
        .enumerate()
        .map(|(i, rel)| GraphEdge {
            id: format!("{node_id}:{}:{i}", rel.r#type),
            source_id: format!("{}:{}", rel.from_type, rel.from_key),
            target_id: node_id.to_string(),
            r#type: rel.r#type.clone(),
            properties: serde_json::json!({}),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestrator::executor::StepStatus;
    use std::sync::Mutex;

    struct FakeProvider(&'static str);

    #[async_trait::async_trait]
    impl LlmProvider for FakeProvider {
        async fn generate(&self, _prompt: &str) -> Result<String, anyhow::Error> {
            Ok(self.0.to_string())
        }
    }

    #[derive(Default)]
    struct FakeGraphStore {
        nodes: Mutex<Vec<GraphNode>>,
        edges: Mutex<Vec<GraphEdge>>,
    }

    #[async_trait::async_trait]
    impl GraphStore for FakeGraphStore {
        async fn insert_node(&self, node: &GraphNode) -> Result<(), anyhow::Error> {
            self.nodes.lock().unwrap().push(node.clone());
            Ok(())
        }

        async fn insert_edge(&self, edge: &GraphEdge) -> Result<(), anyhow::Error> {
            self.edges.lock().unwrap().push(edge.clone());
            Ok(())
        }

        async fn get_subgraph(
            &self,
            _project_id: &str,
        ) -> Result<(Vec<GraphNode>, Vec<GraphEdge>), anyhow::Error> {
            Ok((self.nodes.lock().unwrap().clone(), self.edges.lock().unwrap().clone()))
        }

        async fn query(&self, _cypher: &str) -> Result<serde_json::Value, anyhow::Error> {
            Ok(serde_json::json!([]))
        }
    }

    fn step_result(tool: &str) -> StepResult {
        StepResult {
            order: 1,
            tool: tool.to_string(),
            status: StepStatus::Completed,
            output: Some(serde_json::json!({ "found": tool })),
            error: None,
        }
    }

    #[tokio::test]
    async fn correlate_stores_findings_above_confidence_threshold() {
        let provider = FakeProvider(
            r#"{"findings":[
                {"type":"Subdomain","data":{"name":"api.example.com"},"confidence":0.9,
                 "relationships":[{"type":"HAS_SUBDOMAIN","from_type":"Domain","from_key":"example.com"}]}
            ]}"#,
        );
        let store = FakeGraphStore::default();
        let results = vec![step_result("amass")];

        let findings = Correlator::correlate(&provider, &store, &results).await.unwrap();

        assert_eq!(findings.len(), 1);
        let nodes = store.nodes.lock().unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id, "Subdomain:api.example.com");
        let edges = store.edges.lock().unwrap();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].source_id, "Domain:example.com");
        assert_eq!(edges[0].target_id, "Subdomain:api.example.com");
    }

    #[tokio::test]
    async fn correlate_drops_low_confidence_findings_as_false_positives() {
        let provider = FakeProvider(
            r#"{"findings":[
                {"type":"Subdomain","data":{"name":"sketchy.example.com"},"confidence":0.2,"relationships":[]}
            ]}"#,
        );
        let store = FakeGraphStore::default();
        let results = vec![step_result("amass")];

        let findings = Correlator::correlate(&provider, &store, &results).await.unwrap();

        assert!(findings.is_empty());
        assert!(store.nodes.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn correlate_dedupes_same_entity_reported_by_two_tools() {
        let provider = FakeProvider(
            r#"{"findings":[
                {"type":"Subdomain","data":{"name":"api.example.com"},"confidence":0.9,"relationships":[]},
                {"type":"Subdomain","data":{"name":"api.example.com"},"confidence":0.8,"relationships":[]}
            ]}"#,
        );
        let store = FakeGraphStore::default();
        let results = vec![step_result("amass"), step_result("subfinder")];

        let findings = Correlator::correlate(&provider, &store, &results).await.unwrap();

        assert_eq!(findings.len(), 2, "both findings are still returned to the caller");
        assert_eq!(
            store.nodes.lock().unwrap().len(),
            1,
            "duplicate entity should only be inserted into the graph once"
        );
    }

    #[tokio::test]
    async fn correlate_errors_when_llm_output_is_not_json() {
        let provider = FakeProvider("I refuse to answer");
        let store = FakeGraphStore::default();
        let results = vec![step_result("amass")];

        let result = Correlator::correlate(&provider, &store, &results).await;
        assert!(result.is_err());
    }
}
