use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttribute {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    pub name: String,
    pub shortname: String,
    // These fields are optional in list responses but required in create/update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomAttributeRequest {
    pub name: String,
    pub shortname: String,
    pub data_type: String,
    pub required: Option<bool>,
    pub user_visible: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomAttributeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_visible: Option<bool>,
}
