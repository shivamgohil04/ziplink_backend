use crate::config::AppConfig;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

pub async fn create_pg_pool(config: &AppConfig) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create PostgreSQL connection pool");

    info!("PostgreSQL connection pool created successfully");
    pool
}
