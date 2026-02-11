use serde::{Deserialize, Serialize};

// Account Brand (v2 API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBrand {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_accent_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mastheads: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_instruction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enrollment_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBrandRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_accent_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_instruction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBrandRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_accent_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_instruction: Option<String>,
}

// Legacy BrandingSettings (for backward compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingSettings {
    pub brands: Vec<AccountBrand>,
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

// ==================== EMAIL SETTINGS ====================
// ==================== MESSAGE TEMPLATES ====================

/// A message template for notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageTemplate {
    pub id: i64,
    #[serde(rename = "type")]
    pub template_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

/// Request to create a message template
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageTemplateRequest {
    #[serde(rename = "type")]
    pub template_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

/// Request to update a message template
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageTemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
