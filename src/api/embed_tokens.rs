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
        self.client
            .post("/embed_token", Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn list_embeddable_apps(&self) -> Result<Vec<EmbeddableApp>> {
        self.client.get("/embed/apps").await
    }
}
