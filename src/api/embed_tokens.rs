use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::embed_tokens::*;
use std::sync::Arc;
use tracing::instrument;

pub struct EmbedTokensApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl EmbedTokensApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self, request))]
    pub async fn generate_embed_token(
        &self,
        request: GenerateEmbedTokenRequest,
    ) -> Result<EmbedToken> {
        // Note: embed_token uses API v1
        self.client.post("/api/1/embed_token", Some(&request)).await
    }

    #[instrument(skip(self))]
    pub async fn list_embeddable_apps(&self) -> Result<Vec<EmbeddableApp>> {
        // Note: embed/apps uses API v2
        self.client.get("/api/2/embed/apps").await
    }
}
