use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::branding::*;
use std::sync::Arc;
use tracing::instrument;

pub struct BrandingApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl BrandingApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn get_branding_settings(&self) -> Result<BrandingSettings> {
        self.client.get("/branding").await
    }

    #[instrument(skip(self, request))]
    pub async fn update_branding_settings(
        &self,
        request: UpdateBrandingRequest,
    ) -> Result<BrandingSettings> {
        self.client.put("/branding", Some(&request)).await
    }
}
