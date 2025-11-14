use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingSettings {
    pub logo_url: Option<String>,
    pub background_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub custom_css: Option<String>,
    pub login_message: Option<String>,
    pub company_name: Option<String>,
    pub favicon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBrandingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_css: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
}
