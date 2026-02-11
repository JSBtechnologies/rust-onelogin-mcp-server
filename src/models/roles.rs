use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    // Note: name may not be present in create response (only id is returned)
    #[serde(default)]
    pub name: Option<String>,
    pub description: Option<String>,
    pub admins: Option<Vec<i64>>,
    pub users: Option<Vec<i64>>,
    pub apps: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    // Note: OneLogin API v2 does not accept 'description' for role creation
    // Description can only be set via update
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// App assigned to a role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleApp {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub icon_url: Option<String>,
}

/// Person reference (used in added_by fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRef {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}

/// User assigned to a role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleUser {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub added_at: Option<String>,
    #[serde(default)]
    pub added_by: Option<PersonRef>,
    #[serde(default)]
    pub assigned: Option<bool>,
}

/// Admin of a role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAdmin {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub added_at: Option<String>,
    #[serde(default)]
    pub added_by: Option<i64>,
    #[serde(default)]
    pub assigned: Option<bool>,
}

/// Request to set apps for a role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRoleAppsRequest {
    pub app_id_array: Vec<i64>,
}

/// Request to add admins to a role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRoleAdminsRequest {
    pub admin_id_array: Vec<i64>,
}

/// Request to assign/remove roles to/from a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleIdsRequest {
    pub role_id_array: Vec<i64>,
}
