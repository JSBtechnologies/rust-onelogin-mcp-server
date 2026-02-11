use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub title: Option<String>,
    pub department: Option<String>,
    pub company: Option<String>,
    pub phone: Option<String>,
    #[serde(default)]
    pub status: i32,
    #[serde(default)]
    pub state: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub locked_until: Option<String>,
    #[serde(default)]
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
    // Required fields
    pub email: String,
    pub username: String,
    // Basic information
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
    pub comment: Option<String>,
    // Authentication & Password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_confirmation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_algorithm: Option<String>, // "salt+sha256", "sha256+salt", or "bcrypt"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    // Status & State
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>, // 0=Unapproved, 1=Approved, 2=Rejected, 3=Unlicensed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>, // 0=Unactivated, 1=Active, 2=Suspended, 3=Locked, etc.
    // Directory & Authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_idp_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samaccountname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userprincipalname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinguished_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid_name: Option<String>,
    // Management & Assignments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_ad_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_user_id: Option<i64>,
    // Other
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_login_attempts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<HashMap<String, serde_json::Value>>,
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
    // Note: role_ids is NOT supported by the OneLogin Update User API
    // Use assign_roles or remove_roles endpoints instead
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRolesRequest {
    pub role_id_array: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveRolesRequest {
    pub role_id_array: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCustomAttributesRequest {
    pub custom_attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPasswordClearTextRequest {
    pub password: String,
    pub password_confirmation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_policy: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetPasswordHashRequest {
    pub password: String,
    pub password_confirmation: String,
    pub password_algorithm: String, // "salt+sha256"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockUserRequest {
    pub locked_until: i32, // Minutes to lock (0 delegates to policy)
}

/// Response from lock_user API (v1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockUserResponse {
    pub status: LockUserApiStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockUserApiStatus {
    #[serde(rename = "type")]
    pub status_type: String,
    pub code: i32,
    pub message: String,
    pub error: bool,
}

/// Response from unlock_user API (v2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockUserResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedPrivilege {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
