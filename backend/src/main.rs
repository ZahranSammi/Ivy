mod api;
mod db;
mod graph;
mod mcp;
mod recon;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/api/v1", api::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    tracing::info!("ivy-backend listening on :3001");
    axum::serve(listener, app).await?;

    Ok(())
}
