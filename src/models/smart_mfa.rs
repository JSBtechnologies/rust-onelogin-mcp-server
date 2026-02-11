use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartMfaValidateRequest {
    pub user_id: i64,
    pub app_id: Option<i64>,
    pub ip_address: String,
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartMfaValidateResponse {
    pub mfa_required: bool,
    pub device_id: Option<String>,
    pub state_token: Option<String>,
    pub user: Option<SmartMfaUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartMfaUser {
    pub id: i64,
    pub username: String,
    pub email: String,
}
