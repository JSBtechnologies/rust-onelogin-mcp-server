use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::app_rules::*;
use std::sync::Arc;
use tracing::instrument;

pub struct AppRulesApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl AppRulesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// List all rules for an application
    #[instrument(skip(self))]
    pub async fn list_rules(&self, app_id: i64, params: Option<AppRuleQueryParams>) -> Result<Vec<AppRule>> {
        let mut path = format!("/api/2/apps/{}/rules", app_id);
        if let Some(p) = params {
            if let Ok(query) = serde_qs::to_string(&p) {
                if !query.is_empty() {
                    path.push('?');
                    path.push_str(&query);
                }
            }
        }
        self.client.get(&path).await
    }

    /// Get a specific rule by ID
    #[instrument(skip(self))]
    pub async fn get_rule(&self, app_id: i64, rule_id: i64) -> Result<AppRule> {
        self.client
            .get(&format!("/api/2/apps/{}/rules/{}", app_id, rule_id))
            .await
    }

    /// Create a new rule for an application
    #[instrument(skip(self, request))]
    pub async fn create_rule(&self, app_id: i64, request: CreateAppRuleRequest) -> Result<AppRule> {
        self.client
            .post(&format!("/api/2/apps/{}/rules", app_id), Some(&request))
            .await
    }

    /// Update an existing rule
    #[instrument(skip(self, request))]
    pub async fn update_rule(
        &self,
        app_id: i64,
        rule_id: i64,
        request: UpdateAppRuleRequest,
    ) -> Result<AppRule> {
        self.client
            .put(
                &format!("/api/2/apps/{}/rules/{}", app_id, rule_id),
                Some(&request),
            )
            .await
    }

    /// Delete a rule
    #[instrument(skip(self))]
    pub async fn delete_rule(&self, app_id: i64, rule_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/api/2/apps/{}/rules/{}", app_id, rule_id))
            .await
    }

    /// List available condition types for an application's rules
    #[instrument(skip(self))]
    pub async fn list_conditions(&self, app_id: i64) -> Result<Vec<RuleConditionDef>> {
        self.client
            .get(&format!("/api/2/apps/{}/rules/conditions", app_id))
            .await
    }

    /// List available action types for an application's rules
    #[instrument(skip(self))]
    pub async fn list_actions(&self, app_id: i64) -> Result<Vec<RuleActionDef>> {
        self.client
            .get(&format!("/api/2/apps/{}/rules/actions", app_id))
            .await
    }

    /// List operators for a specific condition type
    #[instrument(skip(self))]
    pub async fn list_condition_operators(
        &self,
        app_id: i64,
        condition_value: &str,
    ) -> Result<Vec<RuleConditionOperator>> {
        self.client
            .get(&format!(
                "/api/2/apps/{}/rules/conditions/{}/operators",
                app_id, condition_value
            ))
            .await
    }

    /// List available values for a specific condition type
    #[instrument(skip(self))]
    pub async fn list_condition_values(
        &self,
        app_id: i64,
        condition_value: &str,
    ) -> Result<Vec<RuleConditionValue>> {
        self.client
            .get(&format!(
                "/api/2/apps/{}/rules/conditions/{}/values",
                app_id, condition_value
            ))
            .await
    }

    /// List available values for a specific action type
    #[instrument(skip(self))]
    pub async fn list_action_values(
        &self,
        app_id: i64,
        action_value: &str,
    ) -> Result<Vec<RuleActionValue>> {
        self.client
            .get(&format!(
                "/api/2/apps/{}/rules/actions/{}/values",
                app_id, action_value
            ))
            .await
    }

    /// Sort/reorder rules for an application
    #[instrument(skip(self, request))]
    pub async fn sort_rules(&self, app_id: i64, request: SortRulesRequest) -> Result<Vec<i64>> {
        self.client
            .put(&format!("/api/2/apps/{}/rules/sort", app_id), Some(&request))
            .await
    }
}
