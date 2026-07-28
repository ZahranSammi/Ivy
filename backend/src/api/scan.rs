use axum::{extract::Path, routing::post, Json, Router};
use serde_json::{json, Value};
use crate::recon;

pub fn router() -> Router {
    Router::new()
        .route("/targets/:id/scan/passive", post(start_passive_scan))
        .route("/targets/:id/scan/active", post(start_active_scan))
}

async fn start_passive_scan(Path(id): Path<String>) -> Json<Value> {
    // Kita gunakan domain target dari URL (id)
    let target_domain = &id;

    // Memanggil fungsi perencana recon pasif
    match recon::plan_passive_recon(target_domain).await {
        Ok(plan) => Json(json!({
                          "status": "success",
                          "message": "Passive scan selesai dijalankan",
                          "tools_executed": plan.tool_ids
                      })),
        Err(e) => Json(json!({
            "status": "error",
            "message": format!("Gagal menjalankan passive scan: {}", e)
        })),
    }
}

async fn start_active_scan(Path(id): Path<String>) -> Json<Value> {
    // Kita gunakan target dari URL (id)
    let target_domain = &id;

    // Memanggil fungsi perencana recon aktif
    match recon::plan_active_recon(target_domain).await {
        Ok(plan) => Json(json!({
                          "status": "success",
                          "message": "Active scan selesai dijalankan",
                          "tools_executed": plan.tool_ids
                      })),
        Err(e) => Json(json!({
            "status": "error",
            "message": format!("Gagal menjalankan active scan: {}", e)
        })),
    }
}
