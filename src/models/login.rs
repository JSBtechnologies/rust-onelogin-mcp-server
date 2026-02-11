use serde::{Deserialize, Serialize};

/// Request to create a session login token
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionLoginRequest {
    pub username_or_email: String,
    pub password: String,
    pub subdomain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// Response from session login token creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLoginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LoginStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SessionLoginData>>,
}

/// Login status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginStatus {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
}

/// Session login data (returned on successful auth)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLoginData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<LoginUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_to_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<MfaDevice>>,
}

/// Basic user info returned in login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
}

/// MFA device info returned when MFA is required
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
}

/// Request to verify MFA during login
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyFactorLoginRequest {
    pub state_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_notify: Option<bool>,
}

/// Request to create a session from a session token
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub session_token: String,
}
