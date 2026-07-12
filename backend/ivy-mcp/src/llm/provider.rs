use async_trait::async_trait;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, anyhow::Error>;
    async fn analyze_findings(&self, findings: &serde_json::Value) -> Result<serde_json::Value, anyhow::Error>;
}
