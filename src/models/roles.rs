use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub admins: Option<Vec<i64>>,
    pub users: Option<Vec<i64>>,
    pub apps: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
    pub admins: Option<Vec<i64>>,
    pub users: Option<Vec<i64>>,
    pub apps: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
