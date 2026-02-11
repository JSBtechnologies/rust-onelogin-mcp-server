use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::smart_mfa::*;
use std::sync::Arc;
use tracing::instrument;

pub struct SmartMfaApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl SmartMfaApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self, request))]
    pub async fn validate(
        &self,
        request: SmartMfaValidateRequest,
    ) -> Result<SmartMfaValidateResponse> {
        self.client
            .post("/api/2/smart_mfa/validate", Some(&request))
            .await
    }
}
