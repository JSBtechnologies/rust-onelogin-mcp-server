use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::user_mappings::*;
use std::sync::Arc;
use tracing::instrument;

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
        self.client.get("/mappings").await
    }

    #[instrument(skip(self))]
    pub async fn get_mapping(&self, mapping_id: &str) -> Result<UserMapping> {
        self.client
            .get(&format!("/mappings/{}", mapping_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn create_mapping(&self, request: CreateMappingRequest) -> Result<UserMapping> {
        self.client.post("/mappings", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_mapping(
        &self,
        mapping_id: &str,
        request: UpdateMappingRequest,
    ) -> Result<UserMapping> {
        self.client
            .put(&format!("/mappings/{}", mapping_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_mapping(&self, mapping_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/mappings/{}", mapping_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn sort_mapping_order(&self, request: SortMappingsRequest) -> Result<()> {
        self.client
            .post("/mappings/sort", Some(&request))
            .await
    }
}
