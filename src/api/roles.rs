use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::roles::*;
use std::sync::Arc;
use tracing::instrument;

pub struct RolesApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl RolesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_roles(&self) -> Result<Vec<Role>> {
        self.client.get("/roles").await
    }

    #[instrument(skip(self))]
    pub async fn get_role(&self, role_id: i64) -> Result<Role> {
        self.client.get(&format!("/roles/{}", role_id)).await
    }

    #[instrument(skip(self, request))]
    pub async fn create_role(&self, request: CreateRoleRequest) -> Result<Role> {
        self.client.post("/roles", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_role(&self, role_id: i64, request: UpdateRoleRequest) -> Result<Role> {
        self.client
            .put(&format!("/roles/{}", role_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_role(&self, role_id: i64) -> Result<()> {
        self.client.delete(&format!("/roles/{}", role_id)).await
    }
}
