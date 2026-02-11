use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::saml::*;
use std::sync::Arc;
use tracing::instrument;

pub struct SamlApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl SamlApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self, request))]
    pub async fn get_saml_assertion(
        &self,
        request: SamlAssertionRequest,
    ) -> Result<SamlAssertionResponse> {
        self.client.post("/saml_assertion", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn get_saml_assertion_v2(
        &self,
        request: SamlAssertionRequest,
    ) -> Result<SamlAssertionResponse> {
        self.client
            .post("/api/2/saml_assertion", Some(&request))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn verify_saml_factor(
        &self,
        request: VerifySamlFactorRequest,
    ) -> Result<SamlAssertionResponse> {
        self.client
            .post("/saml_assertion/verify_factor", Some(&request))
            .await
    }
}
