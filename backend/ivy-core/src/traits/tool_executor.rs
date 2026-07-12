use async_trait::async_trait;

#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn run_tool(&self, tool_id: &str, parameters: serde_json::Value) -> Result<serde_json::Value, anyhow::Error>;
    async fn stop_tool(&self, execution_id: &str) -> Result<(), anyhow::Error>;
}
