use serde::{Deserialize, Serialize};

/// Custom login page configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPage {
    /// Page ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Page name
    #[serde(default)]
    pub name: Option<String>,
    /// Whether page is enabled
    #[serde(default)]
    pub enabled: Option<bool>,
    /// Whether this is the default page for subdomain
    #[serde(default)]
    pub default: Option<bool>,
    /// Associated subdomain
    #[serde(default)]
    pub subdomain: Option<String>,
    /// HTML content
    #[serde(default)]
    pub html_content: Option<String>,
    /// CSS content
    #[serde(default)]
    pub css_content: Option<String>,
    /// JavaScript content
    #[serde(default)]
    pub javascript_content: Option<String>,
    /// Preview URL
    #[serde(default)]
    pub preview_url: Option<String>,
    /// Last modified timestamp
    #[serde(default)]
    pub last_modified: Option<String>,
}

/// Request to create a custom login page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoginPageRequest {
    /// Page name
    pub name: String,
    /// HTML content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// CSS content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_content: Option<String>,
    /// JavaScript content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javascript_content: Option<String>,
    /// Associated subdomain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    /// Whether page is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Request to update a custom login page
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateLoginPageRequest {
    /// New page name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New HTML content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// New CSS content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_content: Option<String>,
    /// New JavaScript content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub javascript_content: Option<String>,
    /// New subdomain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    /// New enabled status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
