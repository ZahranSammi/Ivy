pub mod auth;
pub mod projects;
pub mod scan;
pub mod graph;
pub mod chat;
pub mod tools;
pub mod plugins;
pub mod reports;
pub mod settings;
pub mod audit;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/api/v1/auth", auth::router())
        .nest("/api/v1/projects", projects::router())
        .nest("/api/v1/scan", scan::router())
        .nest("/api/v1/graph", graph::router())
        .nest("/api/v1/chat", chat::router())
        .nest("/api/v1/tools", tools::router())
        .nest("/api/v1/plugins", plugins::router())
        .nest("/api/v1/reports", reports::router())
        .nest("/api/v1/settings", settings::router())
        .nest("/api/v1/audit", audit::router())
}
