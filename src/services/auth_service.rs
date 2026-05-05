use redis::AsyncCommands;
use uuid::Uuid;
use validator::Validate;

use crate::dto::{
    AuthResponse, ChangePasswordRequest, LoginRequest, LogoutRequest, MessageResponse,
    RefreshTokenRequest, RegisterRequest, UserInfo,
};
use crate::errors::app_error::{AppError, AppResult};
use crate::models::User;
use crate::repositories::auth_repo;
use crate::state::AppState;
use crate::utils::jwt;
use crate::utils::password;

// ─────────────────────────────────────────
// REGISTER
// ─────────────────────────────────────────

/// Register a new user, hash their password, generate tokens, and store the
/// refresh token in Redis.
pub async fn register(state: &AppState, req: RegisterRequest) -> AppResult<AuthResponse> {
    // 1. Validate input
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // 2. Check if email already taken
    if auth_repo::find_user_by_email(&state.db, &req.email)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict(
            "A user with this email already exists".to_string(),
        ));
    }

    // 3. Hash password
    let hashed = password::hash_password(&req.password)?;

    // 4. Create user
    let user_id = Uuid::new_v4();
    let name = req.name.unwrap_or_else(|| "User".to_string());
    let user = auth_repo::create_user(&state.db, user_id, &req.email, &name, &hashed).await?;

    // 5. Generate tokens
    let (access_token, expires_in) = jwt::generate_access_token(&state.config, &user)?;
    let refresh_token = jwt::generate_refresh_token();

    // 6. Store refresh token in Redis
    store_refresh_token(state, &refresh_token, user.user_id).await?;

    // 7. Build response
    Ok(build_auth_response(
        access_token,
        refresh_token,
        expires_in,
        &user,
    ))
}

// ─────────────────────────────────────────
// LOGIN
// ─────────────────────────────────────────

/// Authenticate a user with email + password, return tokens on success.
pub async fn login(state: &AppState, req: LoginRequest) -> AppResult<AuthResponse> {
    // 1. Validate input
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // 2. Find user by email
    let user = auth_repo::find_user_by_email(&state.db, &req.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    // 3. Verify password
    let valid = password::verify_password(&req.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // 4. Generate tokens
    let (access_token, expires_in) = jwt::generate_access_token(&state.config, &user)?;
    let refresh_token = jwt::generate_refresh_token();

    // 5. Store refresh token in Redis
    store_refresh_token(state, &refresh_token, user.user_id).await?;

    // 6. Build response
    Ok(build_auth_response(
        access_token,
        refresh_token,
        expires_in,
        &user,
    ))
}

// ─────────────────────────────────────────
// REFRESH
// ─────────────────────────────────────────

/// Exchange a valid refresh token for a new token pair.
/// The old refresh token is deleted (rotation).
pub async fn refresh(state: &AppState, req: RefreshTokenRequest) -> AppResult<AuthResponse> {
    // 1. Lookup refresh token in Redis → get user_id
    let redis_key = refresh_token_key(&req.refresh_token);
    let mut redis_conn = state.redis.clone();

    let user_id_str: String = redis_conn
        .get(&redis_key)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid or expired refresh token".to_string()))?;

    let user_id = Uuid::parse_str(&user_id_str).map_err(|_| AppError::InternalServerError)?;

    // 2. Delete old refresh token (one-time use)
    remove_refresh_token(state, &req.refresh_token, user_id).await?;

    // 3. Fetch user from DB
    let user = auth_repo::find_user_by_id(&state.db, user_id).await?;

    // 4. Generate new token pair
    let (access_token, expires_in) = jwt::generate_access_token(&state.config, &user)?;
    let new_refresh_token = jwt::generate_refresh_token();

    // 5. Store new refresh token
    store_refresh_token(state, &new_refresh_token, user.user_id).await?;

    // 6. Build response
    Ok(build_auth_response(
        access_token,
        new_refresh_token,
        expires_in,
        &user,
    ))
}

// ─────────────────────────────────────────
// LOGOUT
// ─────────────────────────────────────────

/// Invalidate a single refresh token (single session logout).
pub async fn logout(state: &AppState, req: LogoutRequest) -> AppResult<MessageResponse> {
    let redis_key = refresh_token_key(&req.refresh_token);
    let mut redis_conn = state.redis.clone();

    // Check if the token exists
    let user_id_str: Option<String> = redis_conn
        .get(&redis_key)
        .await
        .map_err(|e| AppError::CacheError(e.to_string()))?;

    if let Some(uid_str) = user_id_str {
        let user_id =
            Uuid::parse_str(&uid_str).map_err(|_| AppError::InternalServerError)?;
        remove_refresh_token(state, &req.refresh_token, user_id).await?;
    }
    // Silently succeed even if token was already expired/missing (idempotent)

    Ok(MessageResponse {
        message: "Logged out successfully".to_string(),
    })
}

// ─────────────────────────────────────────
// LOGOUT ALL
// ─────────────────────────────────────────

/// Invalidate all refresh tokens for a user (all sessions logout).
pub async fn logout_all(state: &AppState, user_id: Uuid) -> AppResult<MessageResponse> {
    invalidate_all_sessions(state, user_id).await?;

    Ok(MessageResponse {
        message: "All sessions logged out successfully".to_string(),
    })
}

// ─────────────────────────────────────────
// CHANGE PASSWORD
// ─────────────────────────────────────────

/// Change the authenticated user's password, then invalidate all sessions.
pub async fn change_password(
    state: &AppState,
    user_id: Uuid,
    req: ChangePasswordRequest,
) -> AppResult<MessageResponse> {
    // 1. Validate input
    req.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // 2. Fetch user and verify current password
    let user = auth_repo::find_user_by_id(&state.db, user_id).await?;
    let valid = password::verify_password(&req.current_password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized(
            "Current password is incorrect".to_string(),
        ));
    }

    // 3. Hash new password and update DB
    let new_hash = password::hash_password(&req.new_password)?;
    auth_repo::update_password(&state.db, user_id, &new_hash).await?;

    // 4. Invalidate all sessions (force re-login everywhere)
    invalidate_all_sessions(state, user_id).await?;

    Ok(MessageResponse {
        message: "Password changed successfully. All sessions have been logged out.".to_string(),
    })
}

// ─────────────────────────────────────────
// HELPERS
// ─────────────────────────────────────────

/// Redis key for a refresh token.
fn refresh_token_key(token: &str) -> String {
    format!("refresh_token:{}", token)
}

/// Redis key for the set of active refresh tokens per user.
fn user_sessions_key(user_id: Uuid) -> String {
    format!("user_sessions:{}", user_id)
}

/// Store a refresh token in Redis with a configurable TTL,
/// and track it in the user's sessions set.
async fn store_refresh_token(
    state: &AppState,
    refresh_token: &str,
    user_id: Uuid,
) -> AppResult<()> {
    let key = refresh_token_key(refresh_token);
    let sessions_key = user_sessions_key(user_id);
    let ttl_secs = state.config.refresh_token_expiry_days * 24 * 3600;
    let mut redis_conn = state.redis.clone();

    // Store the refresh token → user_id mapping
    redis_conn
        .set_ex::<_, _, ()>(&key, user_id.to_string(), ttl_secs)
        .await
        .map_err(|e| AppError::CacheError(e.to_string()))?;

    // Track this token in the user's sessions set
    redis_conn
        .sadd::<_, _, ()>(&sessions_key, refresh_token)
        .await
        .map_err(|e| AppError::CacheError(e.to_string()))?;

    // Set/refresh the TTL on the sessions set to match the longest token
    redis_conn
        .expire::<_, ()>(&sessions_key, ttl_secs as i64)
        .await
        .map_err(|e| AppError::CacheError(e.to_string()))?;

    Ok(())
}

/// Remove a single refresh token from Redis and the user's sessions set.
async fn remove_refresh_token(
    state: &AppState,
    refresh_token: &str,
    user_id: Uuid,
) -> AppResult<()> {
    let key = refresh_token_key(refresh_token);
    let sessions_key = user_sessions_key(user_id);
    let mut redis_conn = state.redis.clone();

    // Delete the token key
    redis_conn
        .del::<_, ()>(&key)
        .await
        .map_err(|e| AppError::CacheError(e.to_string()))?;

    // Remove from the sessions set
    redis_conn
        .srem::<_, _, ()>(&sessions_key, refresh_token)
        .await
        .map_err(|e| AppError::CacheError(e.to_string()))?;

    Ok(())
}

/// Invalidate all refresh tokens for a user.
async fn invalidate_all_sessions(state: &AppState, user_id: Uuid) -> AppResult<()> {
    let sessions_key = user_sessions_key(user_id);
    let mut redis_conn = state.redis.clone();

    // Get all active refresh tokens for this user
    let tokens: Vec<String> = redis_conn
        .smembers(&sessions_key)
        .await
        .map_err(|e| AppError::CacheError(e.to_string()))?;

    // Delete each refresh token key
    for token in &tokens {
        let key = refresh_token_key(token);
        let _: () = redis_conn
            .del(&key)
            .await
            .map_err(|e| AppError::CacheError(e.to_string()))?;
    }

    // Delete the sessions set itself
    redis_conn
        .del::<_, ()>(&sessions_key)
        .await
        .map_err(|e| AppError::CacheError(e.to_string()))?;

    Ok(())
}

/// Build a standardised `AuthResponse` from tokens + user.
fn build_auth_response(
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    user: &User,
) -> AuthResponse {
    AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in,
        user: UserInfo {
            user_id: user.user_id,
            email: user.email.clone(),
            name: user.name.clone(),
            plan_type: format!("{:?}", user.plan_type),
            created_at: user.created_at,
        },
    }
}
