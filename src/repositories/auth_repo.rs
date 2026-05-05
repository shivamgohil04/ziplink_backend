use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::app_error::{AppError, AppResult};
use crate::models::User;

/// Insert a new user into the database.
///
/// Returns `AppError::Conflict` if the email already exists.
pub async fn create_user(
    pool: &PgPool,
    user_id: Uuid,
    email: &str,
    name: &str,
    password_hash: &str,
) -> AppResult<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (user_id, email, name, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(email)
    .bind(name)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("A user with this email already exists".to_string())
        }
        other => AppError::from(other),
    })?;

    Ok(user)
}

/// Find a user by email address. Returns `None` if not found.
pub async fn find_user_by_email(pool: &PgPool, email: &str) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Find a user by their UUID. Returns `AppError::NotFound` if absent.
pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> AppResult<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Update the password hash for a user.
pub async fn update_password(
    pool: &PgPool,
    user_id: Uuid,
    new_password_hash: &str,
) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE users SET password_hash = $1, updated_at = NOW() WHERE user_id = $2
        "#,
    )
    .bind(new_password_hash)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}
