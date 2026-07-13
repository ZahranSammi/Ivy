use async_trait::async_trait;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, anyhow::Error>;

    async fn analyze_findings(&self, findings: &serde_json::Value) -> Result<serde_json::Value, anyhow::Error> {
        let prompt = format!(
            "Analyze these OSINT reconnaissance findings and reply with a JSON object only:\n\n{findings}"
        );
        let raw = self.generate(&prompt).await?;
        Ok(serde_json::from_str(&raw).unwrap_or_else(|_| serde_json::json!({ "summary": raw })))
    }

    // ponytail: default "ping" generate call as connection test, not a
    // dedicated health-check endpoint — swap per-provider if a cheaper
    // native check (e.g. list-models) is needed.
    async fn test_connection(&self) -> Result<(), anyhow::Error> {
        self.generate("ping").await.map(|_| ())
    }
}
