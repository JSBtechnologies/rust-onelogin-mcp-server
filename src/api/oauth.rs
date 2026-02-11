use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::oauth::*;
use std::sync::Arc;
use tracing::instrument;

pub struct OAuthApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl OAuthApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self, request))]
    pub async fn generate_tokens(&self, request: TokenRequest) -> Result<TokenResponse> {
        self.client
            .post("/auth/oauth2/v2/token", Some(&request))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn revoke_token(&self, request: RevokeTokenRequest) -> Result<()> {
        self.client
            .post("/auth/oauth2/revoke", Some(&request))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn introspect_token(
        &self,
        request: IntrospectTokenRequest,
    ) -> Result<crate::models::oidc::TokenIntrospection> {
        self.client
            .post("/auth/oauth2/introspect", Some(&request))
            .await
    }
}
