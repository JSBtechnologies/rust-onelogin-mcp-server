use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::groups::*;
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
        self.client.get("/groups").await
    }

    #[instrument(skip(self))]
    pub async fn get_group(&self, group_id: i64) -> Result<Group> {
        self.client.get(&format!("/groups/{}", group_id)).await
    }

    #[instrument(skip(self, request))]
    pub async fn create_group(&self, request: CreateGroupRequest) -> Result<Group> {
        self.client.post("/groups", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_group(&self, group_id: i64, request: UpdateGroupRequest) -> Result<Group> {
        self.client
            .put(&format!("/groups/{}", group_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_group(&self, group_id: i64) -> Result<()> {
        self.client.delete(&format!("/groups/{}", group_id)).await
    }
}
