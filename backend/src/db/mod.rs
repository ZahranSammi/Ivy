//! Postgres client — targets, scan_sessions, tool_executions, llm_configs
//! (schema: docs/Ivy_srs.md §5.1)

pub async fn connect(_database_url: &str) -> anyhow::Result<()> {
    anyhow::bail!("postgres connection not implemented yet")
}
