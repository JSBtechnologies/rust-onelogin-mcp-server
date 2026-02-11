use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::smart_hooks::*;
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
        // OneLogin API v2 returns direct object, not wrapped
        let hook: SmartHook =
            self.client.post("/api/2/hooks", Some(&request)).await?;
        Ok(hook)
    }

    #[instrument(skip(self, request))]
    pub async fn update_hook(
        &self,
        hook_id: &str,
        request: UpdateHookRequest,
    ) -> Result<SmartHook> {
        let cache_key = CacheManager::build_key("hook", &[hook_id]);
        self.cache.invalidate(&cache_key).await;

        // OneLogin API v2 returns direct object, not wrapped
        let hook: SmartHook = self.client
            .put(&format!("/api/2/hooks/{}", hook_id), Some(&request))
            .await?;
        Ok(hook)
    }

    /// Update hook with full request (all fields required by OneLogin API)
    #[instrument(skip(self, request))]
    pub async fn update_hook_full(
        &self,
        hook_id: &str,
        request: FullUpdateHookRequest,
    ) -> Result<SmartHook> {
        let cache_key = CacheManager::build_key("hook", &[hook_id]);
        self.cache.invalidate(&cache_key).await;

        let hook: SmartHook = self.client
            .put(&format!("/api/2/hooks/{}", hook_id), Some(&request))
            .await?;
        Ok(hook)
    }

    #[instrument(skip(self))]
    pub async fn delete_hook(&self, hook_id: &str) -> Result<()> {
        let cache_key = CacheManager::build_key("hook", &[hook_id]);
        self.cache.invalidate(&cache_key).await;

        self.client.delete(&format!("/api/2/hooks/{}", hook_id)).await
    }

    #[instrument(skip(self))]
    pub async fn get_hook(&self, hook_id: &str) -> Result<SmartHook> {
        let cache_key = CacheManager::build_key("hook", &[hook_id]);

        if let Some(hook) = self.cache.get(&cache_key).await {
            return Ok(hook);
        }

        // OneLogin API v2 returns direct object, not wrapped
        let hook: SmartHook =
            self.client.get(&format!("/api/2/hooks/{}", hook_id)).await?;

        self.cache.set(cache_key, &hook).await;
        Ok(hook)
    }

    #[instrument(skip(self))]
    pub async fn list_hooks(&self) -> Result<Vec<SmartHook>> {
        // OneLogin API v2 returns direct array, not wrapped
        let hooks: Vec<SmartHook> =
            self.client.get("/api/2/hooks").await?;
        Ok(hooks)
    }

    #[instrument(skip(self))]
    pub async fn get_hook_logs(&self, hook_id: &str) -> Result<Vec<HookLog>> {
        // OneLogin API v2 returns direct array, not wrapped
        let logs: Vec<HookLog> =
            self.client.get(&format!("/api/2/hooks/{}/logs", hook_id)).await?;
        Ok(logs)
    }

    // ==================== ENVIRONMENT VARIABLES (Account-Level) ====================
    // Note: Env vars are shared across ALL hooks in the account, not per-hook

    /// List all environment variables in the account
    #[instrument(skip(self))]
    pub async fn list_env_vars(&self) -> Result<Vec<HookEnvVar>> {
        self.client.get("/api/2/hooks/envs").await
    }

    /// Get a specific environment variable by ID
    #[instrument(skip(self))]
    pub async fn get_env_var(&self, env_var_id: &str) -> Result<HookEnvVar> {
        self.client
            .get(&format!("/api/2/hooks/envs/{}", env_var_id))
            .await
    }

    /// Create a new environment variable (account-level, shared by all hooks)
    #[instrument(skip(self, request))]
    pub async fn create_env_var(&self, request: CreateEnvVarRequest) -> Result<HookEnvVar> {
        self.client.post("/api/2/hooks/envs", Some(&request)).await
    }

    /// Update an environment variable's value (name cannot be changed)
    #[instrument(skip(self, request))]
    pub async fn update_env_var(
        &self,
        env_var_id: &str,
        request: UpdateEnvVarRequest,
    ) -> Result<HookEnvVar> {
        self.client
            .put(&format!("/api/2/hooks/envs/{}", env_var_id), Some(&request))
            .await
    }

    /// Delete an environment variable
    #[instrument(skip(self))]
    pub async fn delete_env_var(&self, env_var_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/api/2/hooks/envs/{}", env_var_id))
            .await
    }
}
