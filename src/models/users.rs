use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub title: Option<String>,
    pub department: Option<String>,
    pub company: Option<String>,
    pub phone: Option<String>,
    pub status: i32,
    pub state: i32,
    pub created_at: String,
    pub updated_at: String,
    pub locked_until: Option<String>,
    pub invalid_login_attempts: i32,
    pub last_login: Option<String>,
    pub activated_at: Option<String>,
    pub custom_attributes: Option<HashMap<String, serde_json::Value>>,
    pub role_ids: Option<Vec<i64>>,
    pub group_id: Option<i64>,
    pub directory_id: Option<i64>,
    pub trusted_idp_id: Option<i64>,
    pub manager_ad_id: Option<String>,
    pub manager_user_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub title: Option<String>,
    pub department: Option<String>,
    pub company: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub password_confirmation: Option<String>,
    pub custom_attributes: Option<HashMap<String, serde_json::Value>>,
    pub role_ids: Option<Vec<i64>>,
    pub group_id: Option<i64>,
    pub directory_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i64>,
}
