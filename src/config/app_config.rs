use dotenvy::dotenv;
use std::env;

/// Holds all application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Server
    pub server_host: String,
    pub server_port: u16,

    // PostgreSQL
    pub database_url: String,
    pub db_max_connections: u32,

    // Redis
    pub redis_url: String,

    // Kafka
    pub kafka_brokers: String,
    pub kafka_click_topic: String,

    // JWT
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,

    // App
    pub base_url: String,
    pub rust_log: String,
}

impl AppConfig {
    /// Load config from environment — panics early if required vars are missing
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            // Server
            server_host: get_env("SERVER_HOST", "0.0.0.0"),
            server_port: get_env("SERVER_PORT", "8080")
                .parse()
                .expect("SERVER_PORT must be a valid number"),

            // PostgreSQL
            database_url: require_env("DATABASE_URL"),
            db_max_connections: get_env("DB_MAX_CONNECTIONS", "10")
                .parse()
                .expect("DB_MAX_CONNECTIONS must be a valid number"),

            // Redis
            redis_url: require_env("REDIS_URL"),

            // Kafka
            kafka_brokers: get_env("KAFKA_BROKERS", "localhost:9092"),
            kafka_click_topic: get_env("KAFKA_CLICK_TOPIC", "click-events"),

            // JWT
            jwt_secret: require_env("JWT_SECRET"),
            jwt_expiry_hours: get_env("JWT_EXPIRY_HOURS", "24")
                .parse()
                .expect("JWT_EXPIRY_HOURS must be a valid number"),

            // App
            base_url: get_env("BASE_URL", "http://localhost:8080"),
            rust_log: get_env("RUST_LOG", "debug"),
        }
    }
}

/// Returns env var value or a default fallback
fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Returns env var value or panics with a clear message — for required vars
fn require_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing required env var: {}", key))
}