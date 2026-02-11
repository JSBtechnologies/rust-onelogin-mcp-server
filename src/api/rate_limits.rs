use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::rate_limits::*;
use std::sync::Arc;
use tracing::instrument;

pub struct RateLimitsApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl RateLimitsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// Get current rate limit status for the API client
    /// Endpoint: /auth/rate_limit (returns X-RateLimit-* values)
    #[instrument(skip(self))]
    pub async fn get_rate_limit_status(&self) -> Result<RateLimitStatus> {
        self.client.get("/auth/rate_limit").await
    }

    /// Get rate limit configuration (alias for status)
    /// Note: OneLogin only provides a single rate limit endpoint
    #[instrument(skip(self))]
    pub async fn get_rate_limits(&self) -> Result<RateLimitStatus> {
        self.client.get("/auth/rate_limit").await
    }
}
