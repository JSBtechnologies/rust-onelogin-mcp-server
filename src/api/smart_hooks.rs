use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::smart_hooks::*;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;

pub struct SmartHooksApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl SmartHooksApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self, request))]
    pub async fn create_hook(&self, request: CreateHookRequest) -> Result<SmartHook> {
        self.client.post("/hooks", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_hook(&self, hook_id: &str, request: UpdateHookRequest) -> Result<SmartHook> {
        let cache_key = CacheManager::build_key("hook", &[hook_id]);
        self.cache.invalidate(&cache_key).await;

        self.client
            .put(&format!("/hooks/{}", hook_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_hook(&self, hook_id: &str) -> Result<()> {
        let cache_key = CacheManager::build_key("hook", &[hook_id]);
        self.cache.invalidate(&cache_key).await;

        self.client.delete(&format!("/hooks/{}", hook_id)).await
    }

    #[instrument(skip(self))]
    pub async fn get_hook(&self, hook_id: &str) -> Result<SmartHook> {
        let cache_key = CacheManager::build_key("hook", &[hook_id]);

        if let Some(hook) = self.cache.get(&cache_key).await {
            return Ok(hook);
        }

        let hook: SmartHook = self.client.get(&format!("/hooks/{}", hook_id)).await?;

        self.cache.set(cache_key, &hook).await;
        Ok(hook)
    }

    #[instrument(skip(self))]
    pub async fn list_hooks(&self) -> Result<Vec<SmartHook>> {
        self.client.get("/hooks").await
    }

    #[instrument(skip(self))]
    pub async fn get_hook_logs(&self, hook_id: &str) -> Result<Vec<HookLog>> {
        self.client
            .get(&format!("/hooks/{}/logs", hook_id))
            .await
    }

    #[instrument(skip(self, vars))]
    pub async fn update_environment_variables(
        &self,
        hook_id: &str,
        vars: HashMap<String, String>,
    ) -> Result<()> {
        self.client
            .put(
                &format!("/hooks/{}/envs", hook_id),
                Some(&serde_json::json!({ "env_vars": vars })),
            )
            .await
    }
}
