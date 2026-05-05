use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

/// Global application error enum
/// Every service, handler, and repo returns this error type
#[derive(Debug, Error)]
pub enum AppError {
    // ─── 400 Bad Request ───────────────────────────────────────────
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    // ─── 401 Unauthorized ──────────────────────────────────────────
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid or expired token")]
    InvalidToken,

    // ─── 403 Forbidden ─────────────────────────────────────────────
    #[error("Forbidden: {0}")]
    Forbidden(String),

    // ─── 404 Not Found ─────────────────────────────────────────────
    #[error("Not found: {0}")]
    NotFound(String),

    // ─── 409 Conflict ──────────────────────────────────────────────
    #[error("Conflict: {0}")]
    Conflict(String),

    // ─── 410 Gone ──────────────────────────────────────────────────
    #[error("Resource expired or deleted: {0}")]
    Gone(String),

    // ─── 429 Too Many Requests ─────────────────────────────────────
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    // ─── 500 Internal Server Error ─────────────────────────────────
    #[error("Internal server error")]
    InternalServerError,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Kafka error: {0}")]
    KafkaError(String),
}

/// How AppError maps to HTTP responses
/// This is called automatically by Axum when a handler returns Err(AppError)
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            // 400
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.clone())
            }

            // 401
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "INVALID_TOKEN",
                "Invalid or expired token".to_string(),
            ),

            // 403
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),

            // 404
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),

            // 409
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),

            // 410
            AppError::Gone(msg) => (StatusCode::GONE, "GONE", msg.clone()),

            // 429
            AppError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMIT_EXCEEDED",
                "Too many requests, slow down".to_string(),
            ),

            // 500
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                "Something went wrong".to_string(),
            ),
            AppError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                msg.clone(),
            ),
            AppError::CacheError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CACHE_ERROR",
                msg.clone(),
            ),
            AppError::KafkaError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "KAFKA_ERROR",
                msg.clone(),
            ),
        };

        // Standard error response shape for all errors
        let body = Json(json!({
            "error": {
                "code"    : error_code,
                "message" : message,
            }
        }));

        (status, body).into_response()
    }
}

/// Auto-convert sqlx DB errors into AppError::DatabaseError
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Record not found".to_string()),
            _ => AppError::DatabaseError(e.to_string()),
        }
    }
}

/// Auto-convert redis errors into AppError::CacheError
impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        AppError::CacheError(e.to_string())
    }
}

/// Auto-convert serde_json errors into AppError::BadRequest
impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::BadRequest(e.to_string())
    }
}

/// Convenience type alias — use this as return type in all handlers and services
pub type AppResult<T> = Result<T, AppError>;
