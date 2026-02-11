use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::groups::*;
use crate::models::{ApiResponse, PaginatedResponse};
use std::sync::Arc;
use tracing::instrument;

pub struct GroupsApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl GroupsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_groups(&self) -> Result<Vec<Group>> {
        // OneLogin API v1 returns groups wrapped in a pagination envelope
        let response: PaginatedResponse<Group> = self.client.get("/api/1/groups").await?;
        Ok(response.data)
    }

    #[instrument(skip(self))]
    pub async fn get_group(&self, group_id: i64) -> Result<Group> {
        // OneLogin API v1 returns group wrapped in response envelope with data as array
        let response: ApiResponse<Vec<Group>> = self.client.get(&format!("/api/1/groups/{}", group_id)).await?;
        response.data.into_iter().next().ok_or_else(|| {
            crate::core::error::OneLoginError::NotFound(format!("Group {} not found", group_id))
        })
    }

    #[instrument(skip(self, request))]
    pub async fn create_group(&self, request: CreateGroupRequest) -> Result<Group> {
        // OneLogin API v1 returns group wrapped in response envelope
        let response: ApiResponse<Group> = self.client.post("/api/1/groups", Some(&request)).await?;
        Ok(response.data)
    }

    #[instrument(skip(self, request))]
    pub async fn update_group(&self, group_id: i64, request: UpdateGroupRequest) -> Result<Group> {
        // OneLogin API v1 returns group wrapped in response envelope
        let response: ApiResponse<Group> = self.client
            .put(&format!("/api/1/groups/{}", group_id), Some(&request))
            .await?;
        Ok(response.data)
    }

    #[instrument(skip(self))]
    pub async fn delete_group(&self, group_id: i64) -> Result<()> {
        self.client.delete(&format!("/api/1/groups/{}", group_id)).await
    }
}
