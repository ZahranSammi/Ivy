//! FR-007/008: result correlation & graph storage (Neo4j)

use serde_json::{json, Value};

pub struct Finding {
    pub entity_type: String,
    pub data: Value,
}

pub async fn store_findings(_target_id: &str, _findings: Vec<Finding>) -> anyhow::Result<()> {
    anyhow::bail!("graph storage not implemented yet")
}

pub async fn get_graph(_target_id: &str) -> anyhow::Result<Value> {
    Ok(json!({ "nodes": [], "edges": [] }))
}
