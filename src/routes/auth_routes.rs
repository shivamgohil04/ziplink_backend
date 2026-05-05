use axum::{Router, routing::{get, post}};

use crate::handlers::auth_handler;
use crate::state::AppState;

/// Auth routes: all mounted under `/api/v1/auth`
pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handler::register_handler))
        .route("/login", post(auth_handler::login_handler))
        .route("/refresh", post(auth_handler::refresh_handler))
        .route("/logout", post(auth_handler::logout_handler))
        .route("/logout-all", post(auth_handler::logout_all_handler))
        .route("/change-password", post(auth_handler::change_password_handler))
        .route("/me", get(auth_handler::me_handler))
}
