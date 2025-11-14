pub mod users;
pub mod apps;
pub mod roles;
pub mod groups;
pub mod mfa;
pub mod saml;
pub mod smart_hooks;
pub mod vigilance;
pub mod privileges;
pub mod user_mappings;
pub mod policies;
pub mod invitations;
pub mod custom_attributes;
pub mod embed_tokens;
pub mod oauth;
pub mod webhooks;
pub mod scim;
pub mod oidc;
pub mod directories;
pub mod branding;
pub mod events;
pub mod sessions;
pub mod api_auth;

use serde::{Deserialize, Serialize};

// Common response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub error: bool,
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// Pagination
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub before_cursor: Option<String>,
    pub after_cursor: Option<String>,
    pub previous_link: Option<String>,
    pub next_link: Option<String>,
}

// Common filters
#[derive(Debug, Default, Serialize)]
pub struct QueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
}
