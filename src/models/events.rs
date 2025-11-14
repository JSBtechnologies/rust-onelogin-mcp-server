use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    pub event_type_id: i32,
    pub event_type_name: String,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub app_id: Option<i64>,
    pub app_name: Option<String>,
    pub ipaddr: Option<String>,
    pub created_at: String,
    pub actor_user_id: Option<i64>,
    pub actor_user_name: Option<String>,
    pub risk_score: Option<i32>,
    pub risk_reasons: Option<Vec<String>>,
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
    pub client_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEventRequest {
    pub event_type_id: i32,
    pub user_id: Option<i64>,
    pub notes: Option<String>,
}
