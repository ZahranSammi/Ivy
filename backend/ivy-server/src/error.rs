use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

#[allow(dead_code)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.0.to_string()
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
