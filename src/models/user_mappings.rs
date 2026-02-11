use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMapping {
    pub id: i64,
    pub name: String,
    #[serde(rename = "match")]
    pub match_type: String,
    pub enabled: bool,
    pub position: Option<i32>,
    pub conditions: Vec<MappingCondition>,
    pub actions: Vec<MappingAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingCondition {
    pub source: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingAction {
    pub action: String,
    pub value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMappingRequest {
    pub name: String,
    #[serde(rename = "match", alias = "match_type")]
    pub match_type: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    pub conditions: Vec<MappingCondition>,
    pub actions: Vec<MappingAction>,
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMappingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "match")]
    pub match_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<MappingCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<MappingAction>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortMappingsRequest {
    pub mapping_ids: Vec<String>,
}

/// Available condition types that can be used in user mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableCondition {
    pub name: String,
    pub value: String,
}
