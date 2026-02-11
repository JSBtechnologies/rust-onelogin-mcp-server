use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::user_mappings::*;
use std::sync::Arc;
use tracing::instrument;

/// Response from create_mapping API - only returns the id
#[derive(Debug, serde::Deserialize)]
struct CreateMappingResponse {
    id: i64,
}

pub struct UserMappingsApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl UserMappingsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_mappings(&self) -> Result<Vec<UserMapping>> {
        // Note: mappings use API v2
        self.client.get("/api/2/mappings").await
    }

    #[instrument(skip(self))]
    pub async fn get_mapping(&self, mapping_id: &str) -> Result<UserMapping> {
        // Note: mappings use API v2
        self.client.get(&format!("/api/2/mappings/{}", mapping_id)).await
    }

    #[instrument(skip(self, request))]
    pub async fn create_mapping(&self, request: CreateMappingRequest) -> Result<UserMapping> {
        // Note: mappings use API v2
        // API returns just {id} on create, so we need to fetch the full mapping
        let response: CreateMappingResponse = self.client.post("/api/2/mappings", Some(&request)).await?;
        // Fetch the full mapping to return
        self.get_mapping(&response.id.to_string()).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_mapping(
        &self,
        mapping_id: &str,
        request: UpdateMappingRequest,
    ) -> Result<UserMapping> {
        // Note: mappings use API v2
        self.client
            .put(&format!("/api/2/mappings/{}", mapping_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_mapping(&self, mapping_id: &str) -> Result<()> {
        // Note: mappings use API v2
        self.client
            .delete(&format!("/api/2/mappings/{}", mapping_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn sort_mapping_order(&self, request: SortMappingsRequest) -> Result<()> {
        // Note: mappings use API v2
        self.client.post("/api/2/mappings/sort", Some(&request)).await
    }

    #[instrument(skip(self))]
    pub async fn list_conditions(&self) -> Result<Vec<AvailableCondition>> {
        // Note: mappings use API v2
        self.client.get("/api/2/mappings/conditions").await
    }
}
