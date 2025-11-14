use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub visible: bool,
    pub connector_id: i64,
    pub auth_method: i32,
    pub policy_id: Option<i64>,
    pub allow_assumed_signin: Option<bool>,
    pub tab_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub configuration: Option<HashMap<String, serde_json::Value>>,
    pub parameters: Option<HashMap<String, AppParameter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppParameter {
    pub id: i64,
    pub label: String,
    pub user_attribute_mappings: Option<String>,
    pub provisioned_entitlements: Option<bool>,
    pub safe_entitlements_enabled: Option<bool>,
    pub include_in_saml_assertion: Option<bool>,
    pub values: Option<String>,
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
