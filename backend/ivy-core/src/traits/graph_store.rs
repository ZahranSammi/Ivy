use async_trait::async_trait;
use crate::domain::graph_node::GraphNode;
use crate::domain::graph_edge::GraphEdge;

#[async_trait]
pub trait GraphStore: Send + Sync {
    async fn insert_node(&self, node: &GraphNode) -> Result<(), anyhow::Error>;
    async fn insert_edge(&self, edge: &GraphEdge) -> Result<(), anyhow::Error>;
    async fn get_subgraph(&self, project_id: &str) -> Result<(Vec<GraphNode>, Vec<GraphEdge>), anyhow::Error>;
    async fn query(&self, cypher: &str) -> Result<serde_json::Value, anyhow::Error>;
}
