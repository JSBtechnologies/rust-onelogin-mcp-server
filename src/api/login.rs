use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::login::*;
use std::sync::Arc;
use tracing::instrument;

pub struct LoginApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl LoginApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// Create a session login token (authenticate user)
    /// Note: This uses the v1 API which returns wrapped responses
    #[instrument(skip(self, request))]
    pub async fn create_session_login_token(
        &self,
        request: SessionLoginRequest,
    ) -> Result<SessionLoginResponse> {
        self.client
            .post("/api/1/login/auth", Some(&request))
            .await
    }

    /// Verify MFA factor during login flow
    #[instrument(skip(self, request))]
    pub async fn verify_factor_login(
        &self,
        request: VerifyFactorLoginRequest,
    ) -> Result<SessionLoginResponse> {
        self.client
            .post("/api/1/login/verify_factor", Some(&request))
            .await
    }

    /// Create a browser session from a session token
    /// Note: This endpoint is on the main domain, not the API domain
    #[instrument(skip(self, request))]
    pub async fn create_session(&self, request: CreateSessionRequest) -> Result<serde_json::Value> {
        self.client
            .post("/session_via_api_token", Some(&request))
            .await
    }
}
