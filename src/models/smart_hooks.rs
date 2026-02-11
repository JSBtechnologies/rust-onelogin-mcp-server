use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartHook {
    pub id: String,
    #[serde(rename = "type")]
    pub hook_type: String,
    #[serde(default)]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    pub runtime: String,
    #[serde(default)]
    pub disabled: Option<bool>,
    #[serde(default)]
    pub context_version: Option<String>,
    #[serde(default)]
    pub retries: Option<i32>,
    #[serde(default)]
    pub timeout: Option<i32>,
    pub packages: Option<HashMap<String, String>>,
    pub env_vars: Option<Vec<EnvVar>>,
    pub conditions: Option<Vec<serde_json::Value>>,
    pub options: Option<HookOptions>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: Option<String>,
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
    #[serde(default)]  // Defaults to empty string, handler will use default function
    pub function: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
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

/// Full update request with all required fields for OneLogin API
/// The API requires ALL fields to be present on update, not just changed ones
#[derive(Debug, Serialize, Deserialize)]
pub struct FullUpdateHookRequest {
    #[serde(rename = "type")]
    pub hook_type: String,
    pub function: String,
    pub disabled: bool,
    pub runtime: String,
    pub timeout: i32,
    pub retries: i32,
    pub packages: HashMap<String, String>,
    pub env_vars: Vec<String>,
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

/// Account-level environment variable for Smart Hooks
/// Note: Env vars are shared across ALL hooks in the account, not per-hook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEnvVar {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Request to create an account-level environment variable
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEnvVarRequest {
    pub name: String,
    pub value: String,
}

/// Request to update an environment variable (name cannot be changed)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEnvVarRequest {
    pub value: String,
}
