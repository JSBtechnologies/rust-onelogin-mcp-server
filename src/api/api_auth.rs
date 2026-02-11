use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::api_auth::*;
use std::sync::Arc;
use tracing::instrument;

/// Response from create API - only returns the id
#[derive(Debug, serde::Deserialize)]
struct CreateApiAuthResponse {
    id: i64,
}

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
        // Note: api_authorizations use API v2
        self.client.get("/api/2/api_authorizations").await
    }

    #[instrument(skip(self))]
    pub async fn get_api_authorization(&self, auth_id: &str) -> Result<ApiAuthorization> {
        // Note: api_authorizations use API v2
        self.client
            .get(&format!("/api/2/api_authorizations/{}", auth_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn create_api_authorization(
        &self,
        request: CreateApiAuthRequest,
    ) -> Result<ApiAuthorization> {
        // Note: api_authorizations use API v2
        // API returns just {id} on create, so we need to fetch the full record
        let response: CreateApiAuthResponse = self.client
            .post("/api/2/api_authorizations", Some(&request))
            .await?;
        // Fetch the full authorization to return
        self.get_api_authorization(&response.id.to_string()).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_api_authorization(
        &self,
        auth_id: &str,
        request: UpdateApiAuthRequest,
    ) -> Result<ApiAuthorization> {
        // Note: api_authorizations use API v2
        self.client
            .put(&format!("/api/2/api_authorizations/{}", auth_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_api_authorization(&self, auth_id: &str) -> Result<()> {
        // Note: api_authorizations use API v2
        self.client
            .delete(&format!("/api/2/api_authorizations/{}", auth_id))
            .await
    }
}
