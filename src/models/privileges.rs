use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Privilege {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub actions: Vec<String>,
    pub scope: PrivilegeScope,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeScope {
    pub scope_type: String,
    pub filters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePrivilegeRequest {
    pub name: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub actions: Vec<String>,
    pub scope: PrivilegeScope,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePrivilegeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<PrivilegeScope>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignPrivilegeRequest {
    pub privilege_id: String,
    pub target_id: String,
    pub target_type: String, // "user" or "role"
}
