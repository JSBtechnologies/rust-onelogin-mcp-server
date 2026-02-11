// PRIVILEGES API ACCESS REQUIREMENTS
//
// If you're getting 403 Forbidden errors, check these requirements:
//
// 1. API Credential Scope: Your API credentials MUST have "Manage All" scope
//    - Log into OneLogin Admin → Developers → API Credentials
//    - Verify your credentials have "Manage All" (not "Read All" or lower)
//    - If not, create new credentials with "Manage All" scope
//
// 2. OneLogin Subscription: Requires "Delegated Administration" feature
//    - This is a subscription-level feature, not just an API setting
//    - Contact OneLogin support to verify your plan includes this feature
//
// 3. API Endpoint: Privileges API only exists in v1 (not v2)
//    - All endpoints use /api/1/privileges/* paths
//    - Response format is direct arrays/objects (no status/data wrappers)

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
        self.client.get("/api/1/privileges").await
    }

    #[instrument(skip(self))]
    pub async fn get_privilege(&self, privilege_id: &str) -> Result<Privilege> {
        let cache_key = CacheManager::build_key("privilege", &[privilege_id]);

        if let Some(privilege) = self.cache.get(&cache_key).await {
            return Ok(privilege);
        }

        let privilege: Privilege = self
            .client
            .get(&format!("/api/1/privileges/{}", privilege_id))
            .await?;

        self.cache.set(cache_key, &privilege).await;
        Ok(privilege)
    }

    #[instrument(skip(self, request))]
    pub async fn create_privilege(&self, request: CreatePrivilegeRequest) -> Result<Privilege> {
        // Create returns only {"id": "..."}, so we need to fetch the full privilege after
        let response: CreatePrivilegeResponse = self.client.post("/api/1/privileges", Some(&request)).await?;
        // Fetch the full privilege details
        self.get_privilege(&response.id).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_privilege(
        &self,
        privilege_id: &str,
        request: UpdatePrivilegeRequest,
    ) -> Result<Privilege> {
        let cache_key = CacheManager::build_key("privilege", &[privilege_id]);
        self.cache.invalidate(&cache_key).await;

        // Update returns only {"id": "..."}, so we need to fetch the full privilege after
        let _response: CreatePrivilegeResponse = self.client
            .put(&format!("/api/1/privileges/{}", privilege_id), Some(&request))
            .await?;
        // Fetch the full privilege details
        self.get_privilege(privilege_id).await
    }

    #[instrument(skip(self))]
    pub async fn delete_privilege(&self, privilege_id: &str) -> Result<()> {
        let cache_key = CacheManager::build_key("privilege", &[privilege_id]);
        self.cache.invalidate(&cache_key).await;

        self.client
            .delete(&format!("/api/1/privileges/{}", privilege_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn assign_to_user(&self, privilege_id: &str, user_id: i64) -> Result<()> {
        self.client
            .post(
                &format!("/api/1/privileges/{}/users/{}", privilege_id, user_id),
                None::<&()>,
            )
            .await
    }

    #[instrument(skip(self))]
    pub async fn assign_to_role(&self, privilege_id: &str, role_id: i64) -> Result<()> {
        self.client
            .post(
                &format!("/api/1/privileges/{}/roles/{}", privilege_id, role_id),
                None::<&()>,
            )
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_assigned_users(&self, privilege_id: &str) -> Result<Vec<i64>> {
        self.client
            .get(&format!("/api/1/privileges/{}/users", privilege_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_assigned_roles(&self, privilege_id: &str) -> Result<Vec<i64>> {
        self.client
            .get(&format!("/api/1/privileges/{}/roles", privilege_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn remove_user(&self, privilege_id: &str, user_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/api/1/privileges/{}/users/{}", privilege_id, user_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn remove_role(&self, privilege_id: &str, role_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/api/1/privileges/{}/roles/{}", privilege_id, role_id))
            .await
    }
}
