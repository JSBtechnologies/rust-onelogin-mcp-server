use serde::{Deserialize, Serialize};

/// Global OneLogin account settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    /// Account ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Account subdomain
    #[serde(default)]
    pub subdomain: Option<String>,
    /// Account name
    #[serde(default)]
    pub name: Option<String>,
    /// Plan type (enterprise, business, etc.)
    #[serde(default)]
    pub plan: Option<String>,
    /// Region (us, eu)
    #[serde(default)]
    pub region: Option<String>,
    /// Default locale
    #[serde(default)]
    pub default_locale: Option<String>,
    /// Default timezone
    #[serde(default)]
    pub default_timezone: Option<String>,
    /// Idle session timeout in minutes
    #[serde(default)]
    pub session_timeout: Option<i32>,
    /// Maximum session duration in minutes
    #[serde(default)]
    pub absolute_session_timeout: Option<i32>,
    /// Default password policy ID
    #[serde(default)]
    pub password_policy_id: Option<i64>,
    /// Whether MFA is required globally
    #[serde(default)]
    pub mfa_required: Option<bool>,
    /// IP whitelist for admin access (CIDR notation)
    #[serde(default)]
    pub allowed_ip_ranges: Option<Vec<String>>,
    /// Country whitelist (ISO codes)
    #[serde(default)]
    pub allowed_countries: Option<Vec<String>>,
    /// Security mode (strict, moderate, relaxed)
    #[serde(default)]
    pub security_mode: Option<String>,
}

/// Request to update account settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateAccountSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_session_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ip_ranges: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_countries: Option<Vec<String>>,
}

/// Account feature information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountFeature {
    /// Feature name
    #[serde(default)]
    pub name: Option<String>,
    /// Whether feature is enabled
    #[serde(default)]
    pub enabled: Option<bool>,
    /// Plan requirement for this feature
    #[serde(default)]
    pub plan_requirement: Option<String>,
    /// Usage count
    #[serde(default)]
    pub usage_count: Option<i32>,
}

/// Account usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUsage {
    /// Number of active users
    #[serde(default)]
    pub active_users_count: Option<i32>,
    /// Total number of users
    #[serde(default)]
    pub total_users_count: Option<i32>,
    /// Number of successful authentications
    #[serde(default)]
    pub authentication_count: Option<i32>,
    /// Number of app launches
    #[serde(default)]
    pub app_launch_count: Option<i32>,
    /// Number of MFA verifications
    #[serde(default)]
    pub mfa_verification_count: Option<i32>,
    /// Number of failed logins
    #[serde(default)]
    pub failed_login_count: Option<i32>,
    /// Number of API calls
    #[serde(default)]
    pub api_calls_count: Option<i32>,
    /// Storage used in bytes
    #[serde(default)]
    pub storage_used: Option<i64>,
    /// Billing period
    #[serde(default)]
    pub billing_period: Option<String>,
}

/// Query parameters for account usage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountUsageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
