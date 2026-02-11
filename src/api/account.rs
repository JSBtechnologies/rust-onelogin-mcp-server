use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::account::*;
use std::sync::Arc;
use tracing::instrument;

pub struct AccountApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl AccountApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// Get global OneLogin account settings
    #[instrument(skip(self))]
    pub async fn get_account_settings(&self) -> Result<AccountSettings> {
        self.client.get("/api/2/account").await
    }

    /// Update account settings
    #[instrument(skip(self, request))]
    pub async fn update_account_settings(
        &self,
        request: UpdateAccountSettingsRequest,
    ) -> Result<AccountSettings> {
        self.client.put("/api/2/account", Some(&request)).await
    }

    /// Get list of enabled features for the account
    #[instrument(skip(self))]
    pub async fn get_account_features(&self) -> Result<Vec<AccountFeature>> {
        self.client.get("/api/2/account/features").await
    }

    /// Get account usage statistics
    #[instrument(skip(self))]
    pub async fn get_account_usage(
        &self,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> Result<AccountUsage> {
        let mut path = "/api/2/account/usage".to_string();
        let mut params = vec![];

        if let Some(start) = start_date {
            params.push(format!("start_date={}", start));
        }
        if let Some(end) = end_date {
            params.push(format!("end_date={}", end));
        }

        if !params.is_empty() {
            path.push_str(&format!("?{}", params.join("&")));
        }

        self.client.get(&path).await
    }
}
