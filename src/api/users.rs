use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::users::*;
use serde_json::Value;
use std::sync::Arc;
use tracing::instrument;

pub struct UsersApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl UsersApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_users(&self, params: Option<UserQueryParams>) -> Result<Vec<User>> {
        let mut path = "/api/2/users".to_string();
        if let Some(p) = params {
            if let Ok(query) = serde_qs::to_string(&p) {
                path.push('?');
                path.push_str(&query);
            }
        }
        // OneLogin API v2 returns users as a plain array, not wrapped in a pagination envelope
        // Pagination metadata is available via response headers (Link, X-Total-Count, etc.)
        let response: Vec<User> = self.client.get(&path).await?;
        Ok(response)
    }

    #[instrument(skip(self))]
    pub async fn get_user(&self, user_id: i64) -> Result<User> {
        let cache_key = CacheManager::build_key("user", &[&user_id.to_string()]);

        if let Some(user) = self.cache.get(&cache_key).await {
            return Ok(user);
        }

        // OneLogin API v2 returns a plain user object, not wrapped
        let user: User = self.client.get(&format!("/api/2/users/{}", user_id)).await?;

        self.cache.set(cache_key, &user).await;
        Ok(user)
    }

    #[instrument(skip(self, request))]
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        // OneLogin API v2 returns a plain user object, not wrapped
        let user: User = self.client.post("/api/2/users", Some(&request)).await?;
        Ok(user)
    }

    #[instrument(skip(self, request))]
    pub async fn update_user(&self, user_id: i64, request: UpdateUserRequest) -> Result<User> {
        let cache_key = CacheManager::build_key("user", &[&user_id.to_string()]);
        self.cache.invalidate(&cache_key).await;

        // OneLogin API v2 returns a plain user object, not wrapped
        let user: User = self.client
            .put(&format!("/api/2/users/{}", user_id), Some(&request))
            .await?;
        Ok(user)
    }

    #[instrument(skip(self))]
    pub async fn delete_user(&self, user_id: i64) -> Result<()> {
        let cache_key = CacheManager::build_key("user", &[&user_id.to_string()]);
        self.cache.invalidate(&cache_key).await;

        self.client.delete(&format!("/api/2/users/{}", user_id)).await
    }

    #[instrument(skip(self))]
    pub async fn get_user_apps(&self, user_id: i64) -> Result<Vec<Value>> {
        // API returns direct array, not wrapped in ApiResponse
        self.client.get(&format!("/api/2/users/{}/apps", user_id)).await
    }

    #[instrument(skip(self))]
    pub async fn get_user_roles(&self, user_id: i64) -> Result<Vec<i64>> {
        // Use the fixed get_user method which now properly unwraps the response
        let user = self.get_user(user_id).await?;
        Ok(user.role_ids.unwrap_or_default())
    }

    #[instrument(skip(self))]
    pub async fn unlock_user(&self, user_id: i64) -> Result<()> {
        // API returns JSON response, not empty body
        let _response: UnlockUserResponse = self.client
            .post(&format!("/api/2/users/{}/unlock", user_id), None::<&()>)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn logout_user(&self, user_id: i64) -> Result<()> {
        // Note: logout_user uses API v1 with PUT method
        // API returns JSON response (same structure as lock_user)
        let _response: LockUserResponse = self.client
            .put(&format!("/api/1/users/{}/logout", user_id), None::<&()>)
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn lock_user(&self, user_id: i64, request: LockUserRequest) -> Result<()> {
        // Note: lock_user uses API v1
        // Endpoint is /lock_user not /lock per OneLogin API docs
        // API returns JSON response, not empty body
        let _response: LockUserResponse = self.client
            .put(&format!("/api/1/users/{}/lock_user", user_id), Some(&request))
            .await?;
        Ok(())
    }

    #[instrument(skip(self, request))]
    pub async fn assign_roles(&self, user_id: i64, request: AssignRolesRequest) -> Result<()> {
        // Note: assign_roles uses API v1
        // API returns JSON status response
        let _response: LockUserResponse = self.client
            .put(&format!("/api/1/users/{}/add_roles", user_id), Some(&request))
            .await?;
        Ok(())
    }

    #[instrument(skip(self, request))]
    pub async fn remove_roles(&self, user_id: i64, request: RemoveRolesRequest) -> Result<()> {
        // Note: remove_roles uses API v1
        // API returns JSON status response
        let _response: LockUserResponse = self.client
            .put(&format!("/api/1/users/{}/remove_roles", user_id), Some(&request))
            .await?;
        Ok(())
    }

    #[instrument(skip(self, request))]
    pub async fn set_custom_attributes(
        &self,
        user_id: i64,
        request: SetCustomAttributesRequest,
    ) -> Result<()> {
        let cache_key = CacheManager::build_key("user", &[&user_id.to_string()]);
        self.cache.invalidate(&cache_key).await;

        // Note: set_custom_attributes uses API v1
        // API returns JSON status response
        let _response: LockUserResponse = self.client
            .put(
                &format!("/api/1/users/{}/set_custom_attributes", user_id),
                Some(&request),
            )
            .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_delegated_privileges(
        &self,
        user_id: i64,
    ) -> Result<Vec<DelegatedPrivilege>> {
        self.client
            .get(&format!("/api/2/users/{}/delegated_privileges", user_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn set_password_clear_text(
        &self,
        user_id: i64,
        request: SetPasswordClearTextRequest,
    ) -> Result<()> {
        // Note: set_password uses API v1
        // API returns JSON status response
        let _response: LockUserResponse = self.client
            .put(
                &format!("/api/1/users/set_password_clear_text/{}", user_id),
                Some(&request),
            )
            .await?;
        Ok(())
    }

    #[instrument(skip(self, request))]
    pub async fn set_password_hash(
        &self,
        user_id: i64,
        request: SetPasswordHashRequest,
    ) -> Result<()> {
        // Note: set_password uses API v1
        // API returns JSON status response
        let _response: LockUserResponse = self.client
            .put(
                &format!("/api/1/users/set_password_using_salt/{}", user_id),
                Some(&request),
            )
            .await?;
        Ok(())
    }
}
