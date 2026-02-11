use serde::{Deserialize, Serialize};

// Available MFA factor (before enrollment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaFactor {
    pub factor_id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

// Enrolled MFA device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaDevice {
    pub device_id: String,
    pub user_display_name: String,
    pub type_display_name: String,
    pub auth_factor_name: String,
    #[serde(default)]
    pub default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrollMfaRequest {
    pub device_type: String,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaVerification {
    pub state_token: String,
    pub device_id: i64,
    pub otp_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaVerificationResponse {
    pub status: String,
    pub message: Option<String>,
}

// Request to generate a temporary MFA token for helpdesk scenarios
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateMfaTokenRequest {
    /// The time in seconds the token will be valid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
    /// Whether to reuse an existing token if one is still valid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
}

// Generated temporary MFA token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
}

// Request to verify an MFA token
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyMfaTokenRequest {
    pub mfa_token: String,
}

// Response from MFA token verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyMfaTokenResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
