use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthorization {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub configuration: ApiAuthConfig,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub onelogin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthConfig {
    pub resource_identifier: String,
    pub audiences: Vec<String>,
    #[serde(alias = "token_lifetime_minutes")]
    pub access_token_expiration_minutes: Option<i32>,
    pub refresh_token_expiration_minutes: Option<i32>,
    pub scopes: Option<Vec<ApiAuthScope>>,
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
