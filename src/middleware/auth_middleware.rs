use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::state::AppState;
use crate::utils::jwt;

/// Authenticated user extracted from the `Authorization: Bearer <token>` header.
///
/// Use this as an extractor in any handler that requires authentication:
/// ```ignore
/// async fn protected(auth: AuthUser) -> impl IntoResponse { ... }
/// ```
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Extract the Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

        // 2. Strip "Bearer " prefix
        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            AppError::Unauthorized("Invalid Authorization header format".to_string())
        })?;

        // 3. Validate JWT and extract claims
        let claims = jwt::validate_access_token(&state.config, token)?;

        // 4. Parse user_id from claims
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid token subject".to_string()))?;

        Ok(AuthUser {
            user_id,
            email: claims.email,
        })
    }
}
