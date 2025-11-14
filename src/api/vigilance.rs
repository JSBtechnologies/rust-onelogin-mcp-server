use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::vigilance::*;
use std::sync::Arc;
use tracing::instrument;

pub struct VigilanceApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl VigilanceApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self, context))]
    pub async fn get_risk_score(
        &self,
        user_identifier: &str,
        context: RiskContext,
    ) -> Result<RiskScore> {
        self.client
            .post(
                "/risk/score",
                Some(&serde_json::json!({
                    "user_identifier": user_identifier,
                    "context": context
                })),
            )
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn validate_user(&self, request: UserValidationRequest) -> Result<ValidationResult> {
        self.client.post("/risk/validate", Some(&request)).await
    }

    #[instrument(skip(self))]
    pub async fn list_risk_rules(&self) -> Result<Vec<RiskRule>> {
        self.client.get("/risk/rules").await
    }

    #[instrument(skip(self, request))]
    pub async fn create_risk_rule(&self, request: CreateRiskRuleRequest) -> Result<RiskRule> {
        self.client.post("/risk/rules", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_risk_rule(
        &self,
        rule_id: &str,
        request: CreateRiskRuleRequest,
    ) -> Result<RiskRule> {
        self.client
            .put(&format!("/risk/rules/{}", rule_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_risk_rule(&self, rule_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/risk/rules/{}", rule_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_risk_events(&self, user_id: &str) -> Result<Vec<RiskEvent>> {
        self.client
            .get(&format!("/risk/events?user_id={}", user_id))
            .await
    }

    #[instrument(skip(self, event))]
    pub async fn track_risk_event(&self, event: RiskEvent) -> Result<()> {
        self.client.post("/risk/events", Some(&event)).await
    }
}
