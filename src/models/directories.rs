use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryConnector {
    pub id: String,
    pub name: String,
    pub connector_type: String,
    pub status: String,
    pub configuration: HashMap<String, serde_json::Value>,
    pub last_sync_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDirectoryConnectorRequest {
    pub name: String,
    pub connector_type: String,
    pub configuration: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDirectoryConnectorRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub users_added: i32,
    pub users_updated: i32,
    pub users_deleted: i32,
    pub errors: Vec<String>,
}
