mod graph;
mod scan;
mod targets;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(targets::router())
        .merge(scan::router())
        .merge(graph::router())
}
