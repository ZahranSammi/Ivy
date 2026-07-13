//! FR-004/005/006: passive recon, consent gate, active recon

use axum::{extract::Path, routing::post, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/targets/:id/scan/passive", post(start_passive_scan))
        .route("/targets/:id/scan/active", post(start_active_scan))
}

async fn start_passive_scan(Path(_id): Path<String>) -> Json<Value> {
    Json(json!({ "status": "not_implemented" }))
}

async fn start_active_scan(Path(_id): Path<String>) -> Json<Value> {
    // Caller must have already gone through the consent gate (FR-005) before hitting this.
    Json(json!({ "status": "not_implemented" }))
}
