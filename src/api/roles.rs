use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::roles::*;
use std::sync::Arc;
use tracing::instrument;

pub struct RolesApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl RolesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_roles(&self) -> Result<Vec<Role>> {
        // OneLogin API returns roles as a plain array, not wrapped in a pagination envelope
        let response: Vec<Role> = self.client.get("/roles").await?;
        Ok(response)
    }

    #[instrument(skip(self))]
    pub async fn get_role(&self, role_id: i64) -> Result<Role> {
        // OneLogin API returns a plain role object, not wrapped
        let role: Role = self.client.get(&format!("/roles/{}", role_id)).await?;
        Ok(role)
    }

    #[instrument(skip(self, request))]
    pub async fn create_role(&self, request: CreateRoleRequest) -> Result<Role> {
        // OneLogin API returns a plain role object, not wrapped
        let role: Role = self.client.post("/roles", Some(&request)).await?;
        Ok(role)
    }

    #[instrument(skip(self, request))]
    pub async fn update_role(&self, role_id: i64, request: UpdateRoleRequest) -> Result<Role> {
        // OneLogin API returns a plain role object, not wrapped
        let role: Role = self.client
            .put(&format!("/roles/{}", role_id), Some(&request))
            .await?;
        Ok(role)
    }

    #[instrument(skip(self))]
    pub async fn delete_role(&self, role_id: i64) -> Result<()> {
        self.client.delete(&format!("/roles/{}", role_id)).await
    }

    // Sub-resource methods

    /// Get apps assigned to a role
    #[instrument(skip(self))]
    pub async fn get_role_apps(&self, role_id: i64) -> Result<Vec<RoleApp>> {
        self.client
            .get(&format!("/roles/{}/apps", role_id))
            .await
    }

    /// Set apps for a role (replaces existing apps)
    #[instrument(skip(self, request))]
    pub async fn set_role_apps(&self, role_id: i64, request: SetRoleAppsRequest) -> Result<Vec<i64>> {
        self.client
            .put(&format!("/roles/{}/apps", role_id), Some(&request))
            .await
    }

    /// Get users assigned to a role
    #[instrument(skip(self))]
    pub async fn get_role_users(&self, role_id: i64) -> Result<Vec<RoleUser>> {
        self.client
            .get(&format!("/roles/{}/users", role_id))
            .await
    }

    /// Get admins assigned to a role
    #[instrument(skip(self))]
    pub async fn get_role_admins(&self, role_id: i64) -> Result<Vec<RoleAdmin>> {
        self.client
            .get(&format!("/roles/{}/admins", role_id))
            .await
    }

    /// Add admins to a role
    #[instrument(skip(self, request))]
    pub async fn add_role_admins(&self, role_id: i64, request: AddRoleAdminsRequest) -> Result<()> {
        self.client
            .post(&format!("/roles/{}/admins", role_id), Some(&request))
            .await
    }

    /// Remove an admin from a role
    #[instrument(skip(self))]
    pub async fn remove_role_admin(&self, role_id: i64, admin_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/roles/{}/admins/{}", role_id, admin_id))
            .await
    }

    /// Assign roles to a user
    #[instrument(skip(self, request))]
    pub async fn assign_roles_to_user(&self, user_id: i64, request: RoleIdsRequest) -> Result<()> {
        self.client
            .put(&format!("/users/{}/add_roles", user_id), Some(&request))
            .await
    }

    /// Remove roles from a user
    #[instrument(skip(self, request))]
    pub async fn remove_roles_from_user(&self, user_id: i64, request: RoleIdsRequest) -> Result<()> {
        self.client
            .put(&format!("/users/{}/remove_roles", user_id), Some(&request))
            .await
    }
}
