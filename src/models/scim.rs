use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimUser {
    pub id: Option<String>,
    pub schemas: Vec<String>,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub name: Option<ScimName>,
    pub emails: Option<Vec<ScimEmail>>,
    pub active: Option<bool>,
    pub groups: Option<Vec<ScimGroupRef>>,
    pub meta: Option<ScimMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimName {
    pub formatted: Option<String>,
    #[serde(rename = "familyName")]
    pub family_name: Option<String>,
    #[serde(rename = "givenName")]
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimEmail {
    pub value: String,
    #[serde(rename = "type")]
    pub email_type: Option<String>,
    pub primary: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimGroupRef {
    pub value: String,
    #[serde(rename = "$ref")]
    pub ref_url: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimGroup {
    pub id: Option<String>,
    pub schemas: Vec<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub members: Option<Vec<ScimMemberRef>>,
    pub meta: Option<ScimMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimMemberRef {
    pub value: String,
    #[serde(rename = "$ref")]
    pub ref_url: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimMeta {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub created: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimListResponse<T> {
    pub schemas: Vec<String>,
    #[serde(rename = "totalResults")]
    pub total_results: i64,
    #[serde(rename = "Resources")]
    pub resources: Vec<T>,
    #[serde(rename = "startIndex")]
    pub start_index: Option<i64>,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScimPatchOperation {
    pub op: String, // "add", "remove", "replace"
    pub path: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScimPatchRequest {
    pub schemas: Vec<String>,
    #[serde(rename = "Operations")]
    pub operations: Vec<ScimPatchOperation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScimBulkRequest {
    pub schemas: Vec<String>,
    #[serde(rename = "Operations")]
    pub operations: Vec<ScimBulkOperation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScimBulkOperation {
    pub method: String, // "POST", "PUT", "PATCH", "DELETE"
    #[serde(rename = "bulkId")]
    pub bulk_id: Option<String>,
    pub path: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimBulkResponse {
    pub schemas: Vec<String>,
    #[serde(rename = "Operations")]
    pub operations: Vec<ScimBulkOperationResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimBulkOperationResponse {
    #[serde(rename = "bulkId")]
    pub bulk_id: Option<String>,
    pub method: String,
    pub location: Option<String>,
    pub status: String,
    pub response: Option<serde_json::Value>,
}
