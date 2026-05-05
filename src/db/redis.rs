use crate::config::AppConfig;
use redis::{Client, aio::ConnectionManager};
use tracing::info;

pub async fn create_redis_client(config: &AppConfig) -> ConnectionManager {
    info!("connection to Redis...");

    let client = Client::open(config.redis_url.as_str()).expect("Failed to create Redis client");
    let connection_manager = ConnectionManager::new(client)
        .await
        .expect("Failed to create Redis connection manager");

    info!("Redis connection manager created successfully");
    connection_manager
}
