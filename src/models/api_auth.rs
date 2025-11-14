use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthorization {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub configuration: ApiAuthConfig,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthConfig {
    pub resource_identifier: String,
    pub audiences: Vec<String>,
    pub token_lifetime_minutes: i32,
    pub scopes: Vec<ApiAuthScope>,
    pub claims: Option<HashMap<String, ClaimSource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthScope {
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimSource {
    pub attribute: String,
    pub transform: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiAuthRequest {
    pub name: String,
    pub description: Option<String>,
    pub configuration: ApiAuthConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateApiAuthRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ApiAuthConfig>,
}
