use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::apps::*;
use std::sync::Arc;
use tracing::instrument;

pub struct AppsApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl AppsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_apps(&self) -> Result<Vec<App>> {
        // OneLogin API returns apps as a plain array, not wrapped in a pagination envelope
        let response: Vec<App> = self.client.get("/apps").await?;
        Ok(response)
    }

    #[instrument(skip(self))]
    pub async fn get_app(&self, app_id: i64) -> Result<App> {
        let cache_key = CacheManager::build_key("app", &[&app_id.to_string()]);

        if let Some(app) = self.cache.get(&cache_key).await {
            return Ok(app);
        }

        // OneLogin API returns a plain app object, not wrapped
        let app: App = self.client.get(&format!("/apps/{}", app_id)).await?;

        self.cache.set(cache_key, &app).await;
        Ok(app)
    }

    #[instrument(skip(self, request))]
    pub async fn create_app(&self, request: CreateAppRequest) -> Result<App> {
        // OneLogin API returns a plain app object, not wrapped
        let app: App = self.client.post("/apps", Some(&request)).await?;
        Ok(app)
    }

    #[instrument(skip(self, request))]
    pub async fn update_app(&self, app_id: i64, request: UpdateAppRequest) -> Result<App> {
        let cache_key = CacheManager::build_key("app", &[&app_id.to_string()]);
        self.cache.invalidate(&cache_key).await;

        // OneLogin API returns a plain app object, not wrapped
        let app: App = self.client
            .put(&format!("/apps/{}", app_id), Some(&request))
            .await?;
        Ok(app)
    }

    #[instrument(skip(self))]
    pub async fn delete_app(&self, app_id: i64) -> Result<()> {
        let cache_key = CacheManager::build_key("app", &[&app_id.to_string()]);
        self.cache.invalidate(&cache_key).await;

        self.client.delete(&format!("/apps/{}", app_id)).await
    }

    #[instrument(skip(self))]
    pub async fn delete_parameter(&self, app_id: i64, parameter_id: i64) -> Result<()> {
        let cache_key = CacheManager::build_key("app", &[&app_id.to_string()]);
        self.cache.invalidate(&cache_key).await;

        self.client
            .delete(&format!("/apps/{}/parameters/{}", app_id, parameter_id))
            .await
    }
}
