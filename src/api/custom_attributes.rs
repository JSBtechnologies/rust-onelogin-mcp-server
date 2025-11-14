use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::custom_attributes::*;
use std::sync::Arc;
use tracing::instrument;

pub struct CustomAttributesApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl CustomAttributesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_custom_attributes(&self) -> Result<Vec<CustomAttribute>> {
        self.client.get("/custom_attributes").await
    }

    #[instrument(skip(self, request))]
    pub async fn create_custom_attribute(
        &self,
        request: CreateCustomAttributeRequest,
    ) -> Result<CustomAttribute> {
        self.client
            .post("/custom_attributes", Some(&request))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn update_custom_attribute(
        &self,
        attribute_id: i64,
        request: UpdateCustomAttributeRequest,
    ) -> Result<CustomAttribute> {
        self.client
            .put(&format!("/custom_attributes/{}", attribute_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_custom_attribute(&self, attribute_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/custom_attributes/{}", attribute_id))
            .await
    }
}
