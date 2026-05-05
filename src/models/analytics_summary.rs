use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Pre-aggregated analytics data for a short URL.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AnalyticsSummary {
    pub summary_id: Uuid,
    pub url_id: Uuid,
    pub total_clicks: i64,
    pub unique_visitors: i64,
    pub date: NaiveDate,
    pub top_country: Option<String>,
    pub top_referrer: Option<String>,
    pub updated_at: DateTime<Utc>,
}
