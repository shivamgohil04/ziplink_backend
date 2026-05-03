mod config;
use config::AppConfig;
use axum::Router;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load config first — panics immediately if required vars are missing
    let config = AppConfig::from_env();

    // Initialize tracing using log level from config
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.rust_log.as_str().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Bitly backend...");
    info!("Base URL     : {}", config.base_url);
    info!("Database     : {}", config.database_url);
    info!("Redis        : {}", config.redis_url);
    info!("Kafka        : {}", config.kafka_brokers);

    let app = Router::new();

    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port)
        .parse()
        .expect("Invalid server address");

    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}