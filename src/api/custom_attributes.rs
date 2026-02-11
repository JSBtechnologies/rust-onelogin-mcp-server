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
        // OneLogin API v2 returns custom attributes as a direct array (no wrapper)
        self.client.get("/api/2/users/custom_attributes").await
    }

    #[instrument(skip(self, request))]
    pub async fn create_custom_attribute(
        &self,
        request: CreateCustomAttributeRequest,
    ) -> Result<CustomAttribute> {
        // OneLogin API v2 returns direct object
        self.client.post("/api/2/users/custom_attributes", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_custom_attribute(
        &self,
        attribute_id: i64,
        request: UpdateCustomAttributeRequest,
    ) -> Result<CustomAttribute> {
        // OneLogin API v2 returns direct object
        self.client
            .put(
                &format!("/api/2/users/custom_attributes/{}", attribute_id),
                Some(&request),
            )
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_custom_attribute(&self, attribute_id: i64) -> Result<()> {
        // OneLogin API v2
        self.client
            .delete(&format!("/api/2/users/custom_attributes/{}", attribute_id))
            .await
    }
}
