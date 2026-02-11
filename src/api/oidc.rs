use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::oidc::*;
use std::sync::Arc;
use tracing::instrument;

pub struct OidcApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl OidcApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn get_well_known_configuration(&self) -> Result<OidcConfiguration> {
        // OneLogin OIDC well-known is under /oidc/2 path
        self.client.get("/oidc/2/.well-known/openid-configuration").await
    }

    #[instrument(skip(self))]
    pub async fn get_jwks(&self) -> Result<Jwks> {
        self.client.get("/oidc/2/certs").await
    }

    #[instrument(skip(self))]
    pub async fn get_userinfo(&self, access_token: &str) -> Result<UserInfo> {
        // UserInfo requires bearer token
        self.client.get("/oidc/2/me").await
    }
}
