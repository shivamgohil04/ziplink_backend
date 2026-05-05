use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClickEvent {
    pub click_id: Uuid,
    pub url_id: Uuid,
    pub clicked_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub device_type: Option<DeviceType>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub referrer: Option<String>,
}

/// Device type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "device_type", rename_all = "UPPERCASE")]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    Unknown,
}
