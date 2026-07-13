//! FR-002: MCP setup & LLM provider connection

pub struct LlmConfig {
    pub provider: String,
    pub api_key: String,
    pub endpoint: Option<String>,
}

pub async fn test_connection(_config: &LlmConfig) -> anyhow::Result<()> {
    anyhow::bail!("MCP connection handshake not implemented yet")
}
