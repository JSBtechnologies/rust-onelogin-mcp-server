use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaDevice {
    pub id: i64,
    pub user_id: i64,
    pub device_type: String,
    pub default: bool,
    pub auth_factor_name: String,
    pub type_display_name: String,
    pub active: bool,
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
