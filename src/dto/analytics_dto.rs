use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─────────────────────────────────────────
// REQUESTS
// ─────────────────────────────────────────

/// GET /api/v1/urls/:short_code/clicks — query params
#[derive(Debug, Deserialize)]
pub struct ClicksQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl ClicksQuery {
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(50).min(200)
    }

    pub fn get_offset(&self) -> u32 {
        (self.get_page() - 1) * self.get_limit()
    }
}

/// GET /api/v1/urls/:short_code/clicks/summary — query params
#[derive(Debug, Deserialize)]
pub struct SummaryQuery {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

// ─────────────────────────────────────────
// RESPONSES
// ─────────────────────────────────────────

/// Single click event in the list response
#[derive(Debug, Serialize)]
pub struct ClickEventResponse {
    pub click_id: Uuid,
    pub clicked_at: DateTime<Utc>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub referrer: Option<String>,
}

/// Paginated list of raw click events
#[derive(Debug, Serialize)]
pub struct ClicksListResponse {
    pub short_code: String,
    pub total_clicks: i64,
    pub page: u32,
    pub limit: u32,
    pub clicks: Vec<ClickEventResponse>,
}

/// Aggregated analytics summary response
#[derive(Debug, Serialize)]
pub struct AnalyticsSummaryResponse {
    pub short_code: String,
    pub total_clicks: i64,
    pub unique_visitors: i64,
    pub date_range: DateRange,
    pub by_date: Vec<ByDate>,
    pub by_country: Vec<ByCountry>,
    pub by_device: Vec<ByDevice>,
    pub by_browser: Vec<ByBrowser>,
    pub top_referrers: Vec<ByReferrer>,
}

#[derive(Debug, Serialize)]
pub struct DateRange {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct ByDate {
    pub date: NaiveDate,
    pub clicks: i64,
}

#[derive(Debug, Serialize)]
pub struct ByCountry {
    pub country: String,
    pub clicks: i64,
}

#[derive(Debug, Serialize)]
pub struct ByDevice {
    pub device: String,
    pub clicks: i64,
}

#[derive(Debug, Serialize)]
pub struct ByBrowser {
    pub browser: String,
    pub clicks: i64,
}

#[derive(Debug, Serialize)]
pub struct ByReferrer {
    pub referrer: String,
    pub clicks: i64,
}
