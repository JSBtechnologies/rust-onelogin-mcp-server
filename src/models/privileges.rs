use serde::{Deserialize, Serialize};

// Updated to match actual OneLogin API response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Privilege {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub privilege: PrivilegeStatement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivilegeStatement {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Statement")]
    pub statement: Vec<StatementItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementItem {
    #[serde(rename = "Effect")]
    pub effect: String,
    #[serde(rename = "Action")]
    pub action: Vec<String>,
    #[serde(rename = "Scope")]
    pub scope: Vec<String>,
}

/// Request to create a privilege
/// Must match the exact OneLogin API format with privilege.Version and privilege.Statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrivilegeRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub privilege: PrivilegeStatement,
}

/// Response from creating a privilege (only returns ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrivilegeResponse {
    pub id: String,
}

/// Request to update a privilege
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePrivilegeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privilege: Option<PrivilegeStatement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignPrivilegeRequest {
    pub privilege_id: String,
    pub target_id: String,
    pub target_type: String, // "user" or "role"
}
