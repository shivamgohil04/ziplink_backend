use ziplink_backend::config::AppConfig;
use ziplink_backend::db::{create_pg_pool, create_redis_client};
use ziplink_backend::routes;
use ziplink_backend::state::AppState;
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

    info!("Starting ziplink backend...");

    // connect to DB and Redis

    let db = create_pg_pool(&config).await;
    let redis = create_redis_client(&config).await;

    // Run database migrations
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run database migrations");

    info!("Database migrations applied successfully");

    // Build shared application state

    let state = AppState::new(config.clone(), db, redis).await;

    let app = routes::root_router().with_state(state);

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
