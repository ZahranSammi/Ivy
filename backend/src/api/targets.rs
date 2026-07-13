//! FR-001: Target & Scope Input

use axum::{routing::post, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new().route("/targets", post(create_target).get(list_targets))
}

async fn create_target() -> Json<Value> {
    Json(json!({ "status": "not_implemented" }))
}

async fn list_targets() -> Json<Value> {
    Json(json!({ "status": "not_implemented" }))
}
