use axum::{Json, extract::State};

use crate::dto::{
    AuthResponse, ChangePasswordRequest, LoginRequest, LogoutRequest, MessageResponse,
    RefreshTokenRequest, RegisterRequest, UserInfo,
};
use crate::errors::app_error::AppResult;
use crate::middleware::auth_middleware::AuthUser;
use crate::models::User;
use crate::repositories::auth_repo;
use crate::services::auth_service;
use crate::state::AppState;

/// POST /api/v1/auth/register
pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    let response = auth_service::register(&state, req).await?;
    Ok(Json(response))
}

/// POST /api/v1/auth/login
pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let response = auth_service::login(&state, req).await?;
    Ok(Json(response))
}

/// POST /api/v1/auth/refresh
pub async fn refresh_handler(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> AppResult<Json<AuthResponse>> {
    let response = auth_service::refresh(&state, req).await?;
    Ok(Json(response))
}

/// POST /api/v1/auth/logout
pub async fn logout_handler(
    State(state): State<AppState>,
    Json(req): Json<LogoutRequest>,
) -> AppResult<Json<MessageResponse>> {
    let response = auth_service::logout(&state, req).await?;
    Ok(Json(response))
}

/// POST /api/v1/auth/logout-all
pub async fn logout_all_handler(
    State(state): State<AppState>,
    auth: AuthUser,
) -> AppResult<Json<MessageResponse>> {
    let response = auth_service::logout_all(&state, auth.user_id).await?;
    Ok(Json(response))
}

/// POST /api/v1/auth/change-password
pub async fn change_password_handler(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<ChangePasswordRequest>,
) -> AppResult<Json<MessageResponse>> {
    let response = auth_service::change_password(&state, auth.user_id, req).await?;
    Ok(Json(response))
}

/// GET /api/v1/auth/me
pub async fn me_handler(
    State(state): State<AppState>,
    auth: AuthUser,
) -> AppResult<Json<UserInfo>> {
    let user: User = auth_repo::find_user_by_id(&state.db, auth.user_id).await?;

    Ok(Json(UserInfo {
        user_id: user.user_id,
        email: user.email,
        name: user.name,
        plan_type: format!("{:?}", user.plan_type),
        created_at: user.created_at,
    }))
}
