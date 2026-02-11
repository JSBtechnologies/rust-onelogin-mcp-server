use serde::{Deserialize, Serialize};

/// Response wrapper for list self-registration profiles endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfRegistrationProfilesResponse {
    #[serde(default)]
    pub self_registration_profiles: Vec<SelfRegistrationProfile>,
}

/// A self-registration profile for user sign-up workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfRegistrationProfile {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_role_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_whitelist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_blacklist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helpdesk_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ProfileField>>,
}

/// A field in a self-registration profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

/// A pending registration in a self-registration profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Request to create a self-registration profile
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSelfRegistrationProfileRequest {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_role_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_whitelist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_blacklist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helpdesk_message: Option<String>,
}

/// Request to update a self-registration profile
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSelfRegistrationProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_role_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_whitelist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_blacklist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helpdesk_message: Option<String>,
}

/// Request to approve or reject a registration
#[derive(Debug, Serialize, Deserialize)]
pub struct ApproveRegistrationRequest {
    /// "approved" or "rejected"
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
}
