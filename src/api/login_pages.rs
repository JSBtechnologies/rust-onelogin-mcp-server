use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::login_pages::*;
use std::sync::Arc;
use tracing::instrument;

pub struct LoginPagesApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl LoginPagesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// List all custom login pages
    #[instrument(skip(self))]
    pub async fn list_login_pages(&self) -> Result<Vec<LoginPage>> {
        self.client.get("/api/2/login_pages").await
    }

    /// Get a specific login page by ID
    #[instrument(skip(self))]
    pub async fn get_login_page(&self, page_id: i64) -> Result<LoginPage> {
        self.client
            .get(&format!("/api/2/login_pages/{}", page_id))
            .await
    }

    /// Create a new custom login page
    #[instrument(skip(self, request))]
    pub async fn create_login_page(&self, request: CreateLoginPageRequest) -> Result<LoginPage> {
        self.client
            .post("/api/2/login_pages", Some(&request))
            .await
    }

    /// Update a custom login page
    #[instrument(skip(self, request))]
    pub async fn update_login_page(
        &self,
        page_id: i64,
        request: UpdateLoginPageRequest,
    ) -> Result<LoginPage> {
        self.client
            .put(&format!("/api/2/login_pages/{}", page_id), Some(&request))
            .await
    }

    /// Delete a custom login page
    #[instrument(skip(self))]
    pub async fn delete_login_page(&self, page_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/api/2/login_pages/{}", page_id))
            .await
    }
}
