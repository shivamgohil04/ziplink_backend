use axum::{Router, routing::get};

pub fn root_router() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
