//! FR-003: per-tool subprocess wrappers — plain functions that shell out to
//! locally installed binaries. No Docker sandbox in v1 (see docs/Ivy_srs.md FR-003).

pub async fn run_tool(tool_id: &str, target: &str) -> anyhow::Result<String> {
    anyhow::bail!("tool execution not implemented yet: {tool_id} -> {target}")
}
