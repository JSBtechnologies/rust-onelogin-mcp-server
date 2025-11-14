use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub score: i32,
    pub risk_level: String,
    pub factors: Vec<RiskFactor>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub value: String,
    pub weight: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskContext {
    pub ip_address: String,
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserValidationRequest {
    pub user_identifier: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub context: RiskContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub validation_id: String,
    pub status: String,
    pub risk_score: RiskScore,
    pub mfa_required: bool,
    pub mfa_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRule {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub conditions: Vec<RiskCondition>,
    pub action: RiskAction,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAction {
    pub action_type: String,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRiskRuleRequest {
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub conditions: Vec<RiskCondition>,
    pub action: RiskAction,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvent {
    pub user_id: String,
    pub event_type: String,
    pub risk_score: i32,
    pub timestamp: String,
    pub details: Option<serde_json::Value>,
}
