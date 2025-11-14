use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::scim::*;
use std::sync::Arc;
use tracing::instrument;

pub struct ScimApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl ScimApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn get_users(&self, filter: Option<String>) -> Result<ScimListResponse<ScimUser>> {
        let mut path = "/scim/v2/Users".to_string();
        if let Some(f) = filter {
            path.push_str(&format!("?filter={}", urlencoding::encode(&f)));
        }
        self.client.get(&path).await
    }

    #[instrument(skip(self, user))]
    pub async fn create_user(&self, user: ScimUser) -> Result<ScimUser> {
        self.client.post("/scim/v2/Users", Some(&user)).await
    }

    #[instrument(skip(self))]
    pub async fn get_user(&self, user_id: &str) -> Result<ScimUser> {
        self.client
            .get(&format!("/scim/v2/Users/{}", user_id))
            .await
    }

    #[instrument(skip(self, user))]
    pub async fn update_user(&self, user_id: &str, user: ScimUser) -> Result<ScimUser> {
        self.client
            .put(&format!("/scim/v2/Users/{}", user_id), Some(&user))
            .await
    }

    #[instrument(skip(self, patch_request))]
    pub async fn patch_user(
        &self,
        user_id: &str,
        patch_request: ScimPatchRequest,
    ) -> Result<ScimUser> {
        // SCIM PATCH uses a special endpoint
        let response = self
            .client
            .http_client()
            .patch(self.client.config().api_url(&format!("/scim/v2/Users/{}", user_id)))
            .json(&patch_request)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    #[instrument(skip(self))]
    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/scim/v2/Users/{}", user_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_groups(&self, filter: Option<String>) -> Result<ScimListResponse<ScimGroup>> {
        let mut path = "/scim/v2/Groups".to_string();
        if let Some(f) = filter {
            path.push_str(&format!("?filter={}", urlencoding::encode(&f)));
        }
        self.client.get(&path).await
    }

    #[instrument(skip(self, group))]
    pub async fn create_group(&self, group: ScimGroup) -> Result<ScimGroup> {
        self.client.post("/scim/v2/Groups", Some(&group)).await
    }

    #[instrument(skip(self, bulk_request))]
    pub async fn bulk_operations(&self, bulk_request: ScimBulkRequest) -> Result<ScimBulkResponse> {
        self.client
            .post("/scim/v2/Bulk", Some(&bulk_request))
            .await
    }
}
