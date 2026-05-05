use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub plan_type: PlanType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Plan tier enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "plan_type", rename_all = "UPPERCASE")]
pub enum PlanType {
    Free,
    Pro,
    Enterprise,
}


impl Default for PlanType {
    fn default() -> Self {
        PlanType::Free
    }
}