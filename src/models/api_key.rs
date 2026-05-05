use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiKey {
    pub api_key_id: Uuid,
    pub user_id: Uuid,

    #[serde(skip_serializing)]
    pub api_key: String,

    pub name: String,
    pub rate_limit: Option<i32>,
    pub is_active: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
