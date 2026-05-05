// DTOs are the request and response shapes for your API. They are separate from models — models map to DB, DTOs map to HTTP.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ─────────────────────────────────────────
// REQUESTS
// ─────────────────────────────────────────

/// POST /api/v1/urls — create a short URL
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUrlRequest {
    #[validate(url(message = "original_url must be a valid URL"))]
    pub original_url: String,

    #[validate(length(
        min = 3,
        max = 50,
        message = "custom_alias must be between 3 and 50 characters"
    ))]
    pub custom_alias: Option<String>,

    pub expires_at: Option<DateTime<Utc>>,
}

/// PATCH /api/v1/urls/:short_code — update a short URL
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUrlRequest {
    #[validate(url(message = "original_url must be a valid URL"))]
    pub original_url: Option<String>,

    pub is_active: Option<bool>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// GET /api/v1/urls — list URLs query params
#[derive(Debug, Deserialize)]
pub struct ListUrlsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<String>, // e.g. "created_at", "clicks"
}

impl ListUrlsQuery {
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(20).min(100) // max 100 per page
    }

    pub fn get_offset(&self) -> u32 {
        (self.get_page() - 1) * self.get_limit()
    }
}

// ─────────────────────────────────────────
// RESPONSES
// ─────────────────────────────────────────

/// Single URL response — returned on create, get, update
#[derive(Debug, Serialize)]
pub struct UrlResponse {
    pub url_id: Uuid,
    pub short_code: String,
    pub short_url: String, // full URL e.g. https://bit.ly/xyz123
    pub original_url: String,
    pub custom_alias: Option<String>,
    pub is_active: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Paginated list of URLs
#[derive(Debug, Serialize)]
pub struct ListUrlsResponse {
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub urls: Vec<UrlResponse>,
}
