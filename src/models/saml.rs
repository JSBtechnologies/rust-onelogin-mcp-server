use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SamlAssertionRequest {
    pub username_or_email: String,
    pub password: String,
    pub app_id: String,
    pub subdomain: String,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlAssertionResponse {
    pub status: String,
    pub data: Option<String>,
    pub message: Option<String>,
    pub state_token: Option<String>,
    pub mfa_required: Option<bool>,
    pub devices: Option<Vec<MfaDeviceInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaDeviceInfo {
    pub device_id: i64,
    pub device_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySamlFactorRequest {
    pub app_id: String,
    pub device_id: String,
    pub state_token: String,
    pub otp_token: Option<String>,
    pub do_not_notify: Option<bool>,
}
