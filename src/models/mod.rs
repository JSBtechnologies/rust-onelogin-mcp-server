// Allow dead code in model modules - these provide comprehensive API model coverage
// even if not all types are currently used by MCP tools
#[allow(dead_code)]
pub mod account;
#[allow(dead_code)]
pub mod api_auth;
#[allow(dead_code)]
pub mod app_rules;
#[allow(dead_code)]
pub mod apps;
#[allow(dead_code)]
pub mod branding;
#[allow(dead_code)]
pub mod certificates;
#[allow(dead_code)]
pub mod connectors;
#[allow(dead_code)]
pub mod custom_attributes;
#[allow(dead_code)]
pub mod device_trust;
#[allow(dead_code)]
pub mod directories;
#[allow(dead_code)]
pub mod embed_tokens;
#[allow(dead_code)]
pub mod events;
#[allow(dead_code)]
pub mod groups;
#[allow(dead_code)]
pub mod invitations;
#[allow(dead_code)]
pub mod login;
#[allow(dead_code)]
pub mod login_pages;
#[allow(dead_code)]
pub mod mfa;
#[allow(dead_code)]
pub mod oauth;
#[allow(dead_code)]
pub mod oidc;
#[allow(dead_code)]
pub mod password_policies;
#[allow(dead_code)]
pub mod privileges;
#[allow(dead_code)]
pub mod rate_limits;
#[allow(dead_code)]
pub mod reports;
#[allow(dead_code)]
pub mod roles;
#[allow(dead_code)]
pub mod saml;
#[allow(dead_code)]
pub mod self_registration;
#[allow(dead_code)]
pub mod smart_hooks;
#[allow(dead_code)]
pub mod smart_mfa;
#[allow(dead_code)]
pub mod trusted_idps;
#[allow(dead_code)]
pub mod user_mappings;
#[allow(dead_code)]
pub mod users;
#[allow(dead_code)]
pub mod vigilance;
#[allow(dead_code)]
pub mod webhooks;

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
    #[serde(default)]
    pub error: bool,
    #[serde(default)]
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
}

// Status-only response (no data field) - used by some v1 endpoints like send_invite_link
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusOnlyResponse {
    pub status: Status,
}

// Pagination
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub before_cursor: Option<String>,
    pub after_cursor: Option<String>,
    pub previous_link: Option<String>,
    pub next_link: Option<String>,
}

// Common filters
#[allow(dead_code)]
#[derive(Debug, Default, Serialize)]
pub struct QueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
}
