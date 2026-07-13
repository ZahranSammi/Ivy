//! FR-008: Graph Visualization (basic)

use axum::{extract::Path, routing::get, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new().route("/targets/:id/graph", get(get_graph))
}

async fn get_graph(Path(_id): Path<String>) -> Json<Value> {
    Json(json!({ "nodes": [], "edges": [] }))
}
