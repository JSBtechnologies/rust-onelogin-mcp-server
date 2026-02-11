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

    #[instrument(skip(self))]
    pub async fn get_risk_score(
        &self,
        user_id: &str,
        ip: &str,
        user_agent: &str,
    ) -> Result<RiskScore> {
        // OneLogin API v2 returns direct object: {score, triggers, messages}
        let risk_score: RiskScore = self.client
            .post(
                "/api/2/risk/verify",
                Some(&serde_json::json!({
                    "ip": ip,
                    "user_agent": user_agent,
                    "user": {
                        "id": user_id
                    }
                })),
            )
            .await?;
        Ok(risk_score)
    }

    #[instrument(skip(self, request))]
    pub async fn validate_user(&self, request: UserValidationRequest) -> Result<ValidationResult> {
        // OneLogin API v2 returns direct object, not wrapped
        let result: ValidationResult = self.client
            .post("/api/2/risk/validate", Some(&request))
            .await?;
        Ok(result)
    }

    #[instrument(skip(self))]
    pub async fn list_risk_rules(&self) -> Result<Vec<RiskRule>> {
        // OneLogin API v2 returns direct array, not wrapped
        let rules: Vec<RiskRule> =
            self.client.get("/api/2/risk/rules").await?;
        Ok(rules)
    }

    #[instrument(skip(self, request))]
    pub async fn create_risk_rule(&self, request: CreateRiskRuleRequest) -> Result<RiskRule> {
        // OneLogin API v2 returns direct object, not wrapped
        let rule: RiskRule = self.client
            .post("/api/2/risk/rules", Some(&request))
            .await?;
        Ok(rule)
    }

    #[instrument(skip(self, request))]
    pub async fn update_risk_rule(
        &self,
        rule_id: &str,
        request: CreateRiskRuleRequest,
    ) -> Result<RiskRule> {
        // OneLogin API v2 returns direct object, not wrapped
        let rule: RiskRule = self.client
            .put(&format!("/api/2/risk/rules/{}", rule_id), Some(&request))
            .await?;
        Ok(rule)
    }

    #[instrument(skip(self))]
    pub async fn delete_risk_rule(&self, rule_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/api/2/risk/rules/{}", rule_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_risk_events(&self, user_id: &str) -> Result<Vec<RiskEvent>> {
        // OneLogin API v2 returns direct array, not wrapped
        let events: Vec<RiskEvent> = self.client
            .get(&format!("/api/2/risk/events?user_id={}", user_id))
            .await?;
        Ok(events)
    }

    #[instrument(skip(self, event))]
    pub async fn track_risk_event(&self, event: RiskEvent) -> Result<()> {
        self.client
            .post("/api/2/risk/events", Some(&event))
            .await
    }
}
