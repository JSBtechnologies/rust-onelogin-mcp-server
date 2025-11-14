use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::users::*;
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
        let mut path = "/users".to_string();
        if let Some(p) = params {
            if let Ok(query) = serde_qs::to_string(&p) {
                path.push('?');
                path.push_str(&query);
            }
        }
        self.client.get(&path).await
    }

    #[instrument(skip(self))]
    pub async fn get_user(&self, user_id: i64) -> Result<User> {
        let cache_key = CacheManager::build_key("user", &[&user_id.to_string()]);

        if let Some(user) = self.cache.get(&cache_key).await {
            return Ok(user);
        }

        let user: User = self.client.get(&format!("/users/{}", user_id)).await?;

        self.cache.set(cache_key, &user).await;
        Ok(user)
    }

    #[instrument(skip(self, request))]
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        self.client.post("/users", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_user(&self, user_id: i64, request: UpdateUserRequest) -> Result<User> {
        let cache_key = CacheManager::build_key("user", &[&user_id.to_string()]);
        self.cache.invalidate(&cache_key).await;

        self.client
            .put(&format!("/users/{}", user_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_user(&self, user_id: i64) -> Result<()> {
        let cache_key = CacheManager::build_key("user", &[&user_id.to_string()]);
        self.cache.invalidate(&cache_key).await;

        self.client.delete(&format!("/users/{}", user_id)).await
    }

    #[instrument(skip(self))]
    pub async fn get_user_apps(&self, user_id: i64) -> Result<Vec<crate::models::apps::App>> {
        self.client
            .get(&format!("/users/{}/apps", user_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_user_roles(&self, user_id: i64) -> Result<Vec<crate::models::roles::Role>> {
        self.client
            .get(&format!("/users/{}/roles", user_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn lock_user(&self, user_id: i64, minutes: i32) -> Result<()> {
        self.client
            .put(
                &format!("/users/{}/lock_user", user_id),
                Some(&serde_json::json!({ "locked_until": minutes })),
            )
            .await
    }

    #[instrument(skip(self))]
    pub async fn logout_user(&self, user_id: i64) -> Result<()> {
        self.client
            .put(&format!("/users/{}/logout", user_id), None::<&()>)
            .await
    }
}
