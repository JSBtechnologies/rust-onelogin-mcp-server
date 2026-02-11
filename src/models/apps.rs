use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub visible: Option<bool>,
    pub connector_id: i64,
    pub auth_method: Option<i32>,
    pub policy_id: Option<i64>,
    pub allow_assumed_signin: Option<bool>,
    pub tab_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub configuration: Option<HashMap<String, serde_json::Value>>,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    pub auth_method_description: Option<String>,
    pub brand_id: Option<i64>,
    pub provisioning: Option<serde_json::Value>,
    /// Catch any additional fields the API might return
    #[serde(flatten)]
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// Strongly-typed app parameter - used when we control the data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppParameter {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub label: String,
    pub user_attribute_mappings: Option<String>,
    #[serde(default)]
    pub provisioned_entitlements: bool,
    #[serde(default)]
    pub safe_entitlements_enabled: bool,
    #[serde(default)]
    pub include_in_saml_assertion: bool,
    pub values: Option<String>,
    pub default_values: Option<String>,
    pub skip_if_blank: Option<bool>,
    pub attributes_transformations: Option<String>,
    pub user_attribute_macros: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRequest {
    pub connector_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub visible: Option<bool>,
    pub configuration: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<HashMap<String, serde_json::Value>>,
}
