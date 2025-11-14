use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::privileges::*;
use std::sync::Arc;
use tracing::instrument;

pub struct PrivilegesApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl PrivilegesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_privileges(&self) -> Result<Vec<Privilege>> {
        self.client.get("/privileges").await
    }

    #[instrument(skip(self))]
    pub async fn get_privilege(&self, privilege_id: &str) -> Result<Privilege> {
        let cache_key = CacheManager::build_key("privilege", &[privilege_id]);

        if let Some(privilege) = self.cache.get(&cache_key).await {
            return Ok(privilege);
        }

        let privilege: Privilege = self
            .client
            .get(&format!("/privileges/{}", privilege_id))
            .await?;

        self.cache.set(cache_key, &privilege).await;
        Ok(privilege)
    }

    #[instrument(skip(self, request))]
    pub async fn create_privilege(&self, request: CreatePrivilegeRequest) -> Result<Privilege> {
        self.client.post("/privileges", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_privilege(
        &self,
        privilege_id: &str,
        request: UpdatePrivilegeRequest,
    ) -> Result<Privilege> {
        let cache_key = CacheManager::build_key("privilege", &[privilege_id]);
        self.cache.invalidate(&cache_key).await;

        self.client
            .put(&format!("/privileges/{}", privilege_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_privilege(&self, privilege_id: &str) -> Result<()> {
        let cache_key = CacheManager::build_key("privilege", &[privilege_id]);
        self.cache.invalidate(&cache_key).await;

        self.client
            .delete(&format!("/privileges/{}", privilege_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn assign_to_user(&self, privilege_id: &str, user_id: i64) -> Result<()> {
        self.client
            .post(
                &format!("/privileges/{}/users/{}", privilege_id, user_id),
                None::<&()>,
            )
            .await
    }

    #[instrument(skip(self))]
    pub async fn assign_to_role(&self, privilege_id: &str, role_id: i64) -> Result<()> {
        self.client
            .post(
                &format!("/privileges/{}/roles/{}", privilege_id, role_id),
                None::<&()>,
            )
            .await
    }
}
