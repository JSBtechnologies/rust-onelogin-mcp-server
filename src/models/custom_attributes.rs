use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttribute {
    pub id: i64,
    pub name: String,
    pub shortname: String,
    pub data_type: String,
    pub required: bool,
    pub user_visible: bool,
    pub created_at: String,
    pub updated_at: String,
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
