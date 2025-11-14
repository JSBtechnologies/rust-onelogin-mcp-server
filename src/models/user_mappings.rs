use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMapping {
    pub id: String,
    pub name: String,
    pub match_type: String,
    pub enabled: bool,
    pub position: i32,
    pub rules: Vec<MappingRule>,
    pub actions: Vec<MappingAction>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    pub source_attribute: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingAction {
    pub action_type: String,
    pub target: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMappingRequest {
    pub name: String,
    pub match_type: String,
    pub enabled: Option<bool>,
    pub rules: Vec<MappingRule>,
    pub actions: Vec<MappingAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMappingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<MappingRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<MappingAction>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortMappingsRequest {
    pub mapping_ids: Vec<String>,
}
