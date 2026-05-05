use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ─────────────────────────────────────────
// REQUESTS
// ─────────────────────────────────────────

/// POST /api/v1/auth/register
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,

    pub name: Option<String>,

    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}

/// POST /api/v1/auth/login
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "password cannot be empty"))]
    pub password: String,
}

/// POST /api/v1/auth/refresh
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

// ─────────────────────────────────────────
// RESPONSES
// ─────────────────────────────────────────

/// Returned on login and register
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String, // always "Bearer"
    pub expires_in: u64,    // seconds
    pub user: UserInfo,
}

/// Minimal user info embedded in auth response
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub plan_type: String,
    pub created_at: DateTime<Utc>,
}
