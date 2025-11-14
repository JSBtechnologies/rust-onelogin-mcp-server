use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::api_auth::*;
use std::sync::Arc;
use tracing::instrument;

pub struct ApiAuthApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl ApiAuthApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_api_authorizations(&self) -> Result<Vec<ApiAuthorization>> {
        self.client.get("/api_authorizations").await
    }

    #[instrument(skip(self))]
    pub async fn get_api_authorization(&self, auth_id: &str) -> Result<ApiAuthorization> {
        self.client
            .get(&format!("/api_authorizations/{}", auth_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn create_api_authorization(
        &self,
        request: CreateApiAuthRequest,
    ) -> Result<ApiAuthorization> {
        self.client
            .post("/api_authorizations", Some(&request))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn update_api_authorization(
        &self,
        auth_id: &str,
        request: UpdateApiAuthRequest,
    ) -> Result<ApiAuthorization> {
        self.client
            .put(&format!("/api_authorizations/{}", auth_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_api_authorization(&self, auth_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/api_authorizations/{}", auth_id))
            .await
    }
}
