use crate::config::AppConfig;
use redis::aio::ConnectionManager;
use sqlx::PgPool;

/// AppState holds the shared state for the application, including database connections and configuration.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub config: AppConfig,
}

impl AppState {
    /// Creates a new AppState instance by initializing the PostgreSQL connection pool and Redis client.
    pub async fn new(config: AppConfig, db: PgPool, redis: ConnectionManager) -> Self {
        Self { db, redis, config }
    }
}
