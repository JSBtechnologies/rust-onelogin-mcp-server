use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartHook {
    pub id: String,
    #[serde(rename = "type")]
    pub hook_type: String,
    pub status: String,
    pub function: String,
    pub runtime: String,
    pub packages: Option<HashMap<String, String>>,
    pub env_vars: Option<Vec<String>>,
    pub options: Option<HookOptions>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookOptions {
    pub risk_enabled: Option<bool>,
    pub location_enabled: Option<bool>,
    pub mfa_device_info_enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHookRequest {
    #[serde(rename = "type")]
    pub hook_type: String,
    pub function: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HookOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HookOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookLog {
    pub id: String,
    pub hook_id: String,
    pub timestamp: String,
    pub status: String,
    pub execution_time_ms: i64,
    pub logs: Vec<String>,
    pub error: Option<String>,
}
