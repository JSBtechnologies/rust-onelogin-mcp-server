use crate::utils::serde_helpers::flexible_string;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    #[serde(default)]
    pub event_type_id: i32,
    #[serde(default, deserialize_with = "flexible_string")]
    pub event_type_name: Option<String>,
    pub account_id: Option<i64>,
    pub user_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub user_name: Option<String>,
    pub app_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub app_name: Option<String>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub ipaddr: Option<String>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub notes: Option<String>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub created_at: Option<String>,
    pub actor_user_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub actor_user_name: Option<String>,
    pub assuming_acting_user_id: Option<i64>,
    pub role_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub role_name: Option<String>,
    pub group_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub group_name: Option<String>,
    pub otp_device_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub otp_device_name: Option<String>,
    pub policy_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub policy_name: Option<String>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub custom_message: Option<String>,
    pub directory_sync_run_id: Option<i64>,
    pub directory_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub resolution: Option<String>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub client_id: Option<String>,
    pub resource_type_id: Option<i64>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub error_description: Option<String>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub proxy_ip: Option<String>,
    pub risk_score: Option<i32>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub risk_reasons: Option<String>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub risk_cookie_id: Option<String>,
    #[serde(default, deserialize_with = "flexible_string")]
    pub browser_fingerprint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub event_type_id: i32,
    pub account_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventType {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    pub description: Option<String>,
}
