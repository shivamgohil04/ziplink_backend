use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::AppConfig;
use crate::errors::app_error::{AppError, AppResult};
use crate::models::User;

/// JWT claims embedded in every access token.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — the user's UUID
    pub sub: String,
    /// User's email
    pub email: String,
    /// Expiration timestamp (seconds since epoch)
    pub exp: usize,
    /// Issued-at timestamp
    pub iat: usize,
}

/// Generate an access token (JWT) for the given user.
///
/// Returns `(token_string, expires_in_seconds)`.
pub fn generate_access_token(config: &AppConfig, user: &User) -> AppResult<(String, u64)> {
    let now = Utc::now();
    let expires_in_secs = config.jwt_expiry_hours * 3600;
    let exp = (now.timestamp() as u64 + expires_in_secs) as usize;

    let claims = Claims {
        sub: user.user_id.to_string(),
        email: user.email.clone(),
        exp,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| {
        tracing::error!("JWT encoding error: {}", e);
        AppError::InternalServerError
    })?;

    Ok((token, expires_in_secs))
}

/// Validate an access token and extract its claims.
pub fn validate_access_token(config: &AppConfig, token: &str) -> AppResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        tracing::debug!("JWT validation failed: {}", e);
        AppError::InvalidToken
    })?;

    Ok(token_data.claims)
}

/// Generate an opaque refresh token (UUID v4).
pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}
