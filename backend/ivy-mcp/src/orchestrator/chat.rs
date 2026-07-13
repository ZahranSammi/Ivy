use std::collections::HashSet;

use chrono::Utc;
use ivy_core::domain::chat::{ChatMessage, ChatRole};
use ivy_core::traits::chat_repository::ChatRepository;
use ivy_core::traits::graph_store::GraphStore;
use ivy_core::traits::tool_executor::ToolExecutor;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::llm::provider::LlmProvider;
use crate::orchestrator::executor::{Executor, StepResult, DEFAULT_TOOL_TIMEOUT};
use crate::orchestrator::planner::{ExecutionPlan, PlannedStep};
use crate::tools::schema::McpToolSchema;

/// Result of handling one chat turn (FR-011).
pub struct ChatOutcome {
    pub reply: String,
    pub tool_results: Vec<StepResult>,
    pub graph_result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct ChatToolStep {
    tool: String,
    #[serde(default)]
    parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ChatAction {
    #[default]
    None,
    RunTools {
        steps: Vec<ChatToolStep>,
    },
    QueryGraph {
        cypher: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
struct ChatLlmResponse {
    reply: String,
    #[serde(default)]
    action: ChatAction,
}

pub struct ChatHandler;

impl ChatHandler {
    /// Handle one user chat message: persist it, ask the LLM (grounded in this
    /// project's chat history) for a reply and an optional action, carry out that
    /// action (running extra tools through `tool_executor`, or a Cypher query through
    /// `graph_store`), then persist the assistant's reply.
    pub async fn handle(
        provider: &dyn LlmProvider,
        chat_repo: &dyn ChatRepository,
        graph_store: &dyn GraphStore,
        tool_executor: &dyn ToolExecutor,
        available_tools: &[McpToolSchema],
        project_id: Uuid,
        user_message: &str,
    ) -> Result<ChatOutcome, anyhow::Error> {
        chat_repo
            .save(&ChatMessage {
                id: Uuid::new_v4(),
                project_id,
                role: ChatRole::User,
                content: user_message.to_string(),
                created_at: Utc::now(),
            })
            .await?;

        let history = chat_repo.list_by_project(project_id).await?;
        let prompt = build_prompt(&history, available_tools);
        let raw = provider.generate(&prompt).await?;
        let parsed = parse_response(&raw)?;

        let mut tool_results = Vec::new();
        let mut graph_result = None;

        match parsed.action {
            ChatAction::RunTools { steps } => {
                let allowed: HashSet<&str> =
                    available_tools.iter().map(|t| t.name.as_str()).collect();
                let plan = ExecutionPlan {
                    steps: steps
                        .into_iter()
                        .filter(|s| allowed.contains(s.tool.as_str()))
                        .enumerate()
                        .map(|(i, s)| PlannedStep {
                            order: i as u32 + 1,
                            tool: s.tool,
                            parameters: s.parameters,
                            depends_on: vec![],
                        })
                        .collect(),
                };
                tool_results = Executor::run(tool_executor, &plan, DEFAULT_TOOL_TIMEOUT).await;
            }
            ChatAction::QueryGraph { cypher } => {
                graph_result = Some(graph_store.query(&cypher).await?);
            }
            ChatAction::None => {}
        }

        chat_repo
            .save(&ChatMessage {
                id: Uuid::new_v4(),
                project_id,
                role: ChatRole::Assistant,
                content: parsed.reply.clone(),
                created_at: Utc::now(),
            })
            .await?;

        Ok(ChatOutcome {
            reply: parsed.reply,
            tool_results,
            graph_result,
        })
    }
}

fn build_prompt(history: &[ChatMessage], available_tools: &[McpToolSchema]) -> String {
    let history_json = serde_json::json!(
        history
            .iter()
            .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
            .collect::<Vec<_>>()
    );
    let tools_json = serde_json::json!(
        available_tools
            .iter()
            .map(|t| serde_json::json!({ "name": t.name, "description": t.description }))
            .collect::<Vec<_>>()
    );

    format!(
        "You are Ivy's OSINT assistant, chatting with a user about their recon project. \
         Use the conversation history to answer questions about scan results directly in \
         \"reply\". Reply with JSON only (no prose, no markdown fences), matching one of \
         these shapes exactly:\n\
         {{\"reply\":\"<text to show the user>\",\"action\":{{\"type\":\"none\"}}}}\n\
         {{\"reply\":\"...\",\"action\":{{\"type\":\"run_tools\",\"steps\":[{{\"tool\":\"<tool name>\",\"parameters\":{{}}}}]}}}}\n\
         {{\"reply\":\"...\",\"action\":{{\"type\":\"query_graph\",\"cypher\":\"<cypher query>\"}}}}\n\n\
         Use \"run_tools\" only when the user explicitly asks to scan or run something \
         additional, and only with tools from the available tools list. Use \
         \"query_graph\" when the user asks about entities/relationships in the recon \
         graph (e.g. subdomains, services) in natural language, translating their \
         question into a read-only Cypher query.\n\n\
         Conversation history: {history_json}\n\
         Available tools: {tools_json}\n"
    )
}

fn parse_response(raw: &str) -> Result<ChatLlmResponse, anyhow::Error> {
    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    serde_json::from_str(cleaned)
        .map_err(|e| anyhow::anyhow!("chat: LLM did not return a valid response JSON: {e}"))
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
    struct FakeChatRepo {
        messages: Mutex<Vec<ChatMessage>>,
    }

    #[async_trait::async_trait]
    impl ChatRepository for FakeChatRepo {
        async fn save(&self, message: &ChatMessage) -> Result<(), anyhow::Error> {
            self.messages.lock().unwrap().push(message.clone());
            Ok(())
        }

        async fn list_by_project(&self, project_id: Uuid) -> Result<Vec<ChatMessage>, anyhow::Error> {
            Ok(self
                .messages
                .lock()
                .unwrap()
                .iter()
                .filter(|m| m.project_id == project_id)
                .cloned()
                .collect())
        }
    }

    struct FakeGraphStore;

    #[async_trait::async_trait]
    impl GraphStore for FakeGraphStore {
        async fn insert_node(&self, _node: &ivy_core::domain::graph_node::GraphNode) -> Result<(), anyhow::Error> {
            Ok(())
        }

        async fn insert_edge(&self, _edge: &ivy_core::domain::graph_edge::GraphEdge) -> Result<(), anyhow::Error> {
            Ok(())
        }

        async fn get_subgraph(
            &self,
            _project_id: &str,
        ) -> Result<(Vec<ivy_core::domain::graph_node::GraphNode>, Vec<ivy_core::domain::graph_edge::GraphEdge>), anyhow::Error>
        {
            Ok((vec![], vec![]))
        }

        async fn query(&self, cypher: &str) -> Result<serde_json::Value, anyhow::Error> {
            Ok(serde_json::json!({ "cypher": cypher, "rows": [] }))
        }
    }

    struct FakeToolExecutor;

    #[async_trait::async_trait]
    impl ToolExecutor for FakeToolExecutor {
        async fn run_tool(
            &self,
            tool_id: &str,
            _parameters: serde_json::Value,
        ) -> Result<serde_json::Value, anyhow::Error> {
            Ok(serde_json::json!({ "tool": tool_id }))
        }

        async fn stop_tool(&self, _execution_id: &str) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    fn tool(name: &str) -> McpToolSchema {
        McpToolSchema {
            name: name.to_string(),
            description: String::new(),
            input_schema: serde_json::json!({}),
            output_schema: serde_json::json!({}),
        }
    }

    #[tokio::test]
    async fn handle_answers_question_with_reply_only() {
        let provider = FakeProvider(r#"{"reply":"3 subdomains found.","action":{"type":"none"}}"#);
        let chat_repo = FakeChatRepo::default();
        let graph_store = FakeGraphStore;
        let tool_executor = FakeToolExecutor;

        let outcome = ChatHandler::handle(
            &provider,
            &chat_repo,
            &graph_store,
            &tool_executor,
            &[],
            Uuid::nil(),
            "berapa subdomain yang ditemukan?",
        )
        .await
        .unwrap();

        assert_eq!(outcome.reply, "3 subdomains found.");
        assert!(outcome.tool_results.is_empty());
        assert!(outcome.graph_result.is_none());
    }

    #[tokio::test]
    async fn handle_runs_additional_tools_filtered_by_scope() {
        let provider = FakeProvider(
            r#"{"reply":"Scanning port 443.","action":{"type":"run_tools","steps":[
                {"tool":"nmap","parameters":{"port":443}},
                {"tool":"nuclei","parameters":{}}
            ]}}"#,
        );
        let chat_repo = FakeChatRepo::default();
        let graph_store = FakeGraphStore;
        let tool_executor = FakeToolExecutor;

        let outcome = ChatHandler::handle(
            &provider,
            &chat_repo,
            &graph_store,
            &tool_executor,
            &[tool("nmap")],
            Uuid::nil(),
            "scan port 443 pada semua subdomain",
        )
        .await
        .unwrap();

        assert_eq!(outcome.tool_results.len(), 1, "nuclei is outside available_tools and must be dropped");
        assert_eq!(outcome.tool_results[0].tool, "nmap");
        assert_eq!(outcome.tool_results[0].status, StepStatus::Completed);
    }

    #[tokio::test]
    async fn handle_queries_graph_with_natural_language() {
        let provider = FakeProvider(
            r#"{"reply":"Here are the Apache services.","action":{"type":"query_graph","cypher":"MATCH (s:Service {name:'Apache'}) RETURN s"}}"#,
        );
        let chat_repo = FakeChatRepo::default();
        let graph_store = FakeGraphStore;
        let tool_executor = FakeToolExecutor;

        let outcome = ChatHandler::handle(
            &provider,
            &chat_repo,
            &graph_store,
            &tool_executor,
            &[],
            Uuid::nil(),
            "tampilkan semua service Apache",
        )
        .await
        .unwrap();

        let result = outcome.graph_result.unwrap();
        assert_eq!(result["cypher"], "MATCH (s:Service {name:'Apache'}) RETURN s");
    }

    #[tokio::test]
    async fn handle_persists_user_and_assistant_messages_per_project() {
        let provider = FakeProvider(r#"{"reply":"ok","action":{"type":"none"}}"#);
        let chat_repo = FakeChatRepo::default();
        let graph_store = FakeGraphStore;
        let tool_executor = FakeToolExecutor;
        let project_id = Uuid::new_v4();

        ChatHandler::handle(
            &provider,
            &chat_repo,
            &graph_store,
            &tool_executor,
            &[],
            project_id,
            "hello",
        )
        .await
        .unwrap();

        let saved = chat_repo.list_by_project(project_id).await.unwrap();
        assert_eq!(saved.len(), 2);
        assert_eq!(saved[0].role, ChatRole::User);
        assert_eq!(saved[0].content, "hello");
        assert_eq!(saved[1].role, ChatRole::Assistant);
        assert_eq!(saved[1].content, "ok");
    }

    #[tokio::test]
    async fn handle_errors_when_llm_output_is_not_json() {
        let provider = FakeProvider("I refuse to answer");
        let chat_repo = FakeChatRepo::default();
        let graph_store = FakeGraphStore;
        let tool_executor = FakeToolExecutor;

        let result = ChatHandler::handle(
            &provider,
            &chat_repo,
            &graph_store,
            &tool_executor,
            &[],
            Uuid::nil(),
            "hello",
        )
        .await;

        assert!(result.is_err());
    }
}
