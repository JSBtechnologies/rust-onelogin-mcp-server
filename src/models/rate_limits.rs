use serde::{Deserialize, Serialize};

/// Rate limit data from /auth/rate_limit endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitData {
    /// Maximum requests allowed per hour (5000)
    #[serde(rename = "X-RateLimit-Limit", default)]
    pub limit: Option<i32>,
    /// Requests remaining in current window
    #[serde(rename = "X-RateLimit-Remaining", default)]
    pub remaining: Option<i32>,
    /// Seconds until rate limit resets
    #[serde(rename = "X-RateLimit-Reset", default)]
    pub reset: Option<i32>,
}

/// Status wrapper for v1 API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStatus {
    #[serde(default)]
    pub error: Option<bool>,
    #[serde(default)]
    pub code: Option<i32>,
    #[serde(rename = "type", default)]
    pub status_type: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Current rate limit status for the API client
/// Response from /auth/rate_limit endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    #[serde(default)]
    pub status: Option<ApiStatus>,
    #[serde(default)]
    pub data: Option<RateLimitData>,
}

/// Rate limit configuration for API endpoints (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Endpoint URL pattern
    #[serde(default)]
    pub endpoint_pattern: Option<String>,
    /// HTTP methods this limit applies to
    #[serde(default)]
    pub methods: Option<Vec<String>>,
    /// Requests per hour limit
    #[serde(default)]
    pub limit: Option<i32>,
    /// Burst limit (requests per minute)
    #[serde(default)]
    pub burst_limit: Option<i32>,
}

/// Response wrapper for rate limits API (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitsResponse {
    #[serde(default)]
    pub data: Option<Vec<RateLimitConfig>>,
}
