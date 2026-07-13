//! FR-004/006: AI-orchestrated passive & active reconnaissance planning

pub mod tools;

pub struct ExecutionPlan {
    pub tool_ids: Vec<String>,
}

pub async fn plan_passive_recon(_target: &str) -> anyhow::Result<ExecutionPlan> {
    anyhow::bail!("AI passive recon planning not implemented yet")
}

pub async fn plan_active_recon(_target: &str) -> anyhow::Result<ExecutionPlan> {
    // Only called after the user has passed the consent gate (FR-005).
    anyhow::bail!("AI active recon planning not implemented yet")
}
