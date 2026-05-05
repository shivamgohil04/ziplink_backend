pub mod auth_routes;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn root_router() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/v1/auth", auth_routes::auth_router())
}
