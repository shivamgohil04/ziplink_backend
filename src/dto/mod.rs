pub mod analytics_dto;
pub mod auth_dto;
pub mod url_dto;

pub use url_dto::{
    CreateUrlRequest, ListUrlsQuery, ListUrlsResponse, UpdateUrlRequest, UrlResponse,
};

pub use auth_dto::{AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest, UserInfo};

pub use analytics_dto::{
    AnalyticsSummaryResponse, ClickEventResponse, ClicksListResponse, ClicksQuery, SummaryQuery,
};