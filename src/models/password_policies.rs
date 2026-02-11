use serde::{Deserialize, Serialize};

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    /// Policy ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Policy name
    #[serde(default)]
    pub name: Option<String>,
    /// Whether this is the default policy for new users
    #[serde(default)]
    pub default: Option<bool>,
    /// Number of users assigned to this policy
    #[serde(default)]
    pub usage_count: Option<i32>,
    /// Minimum password length (8-256)
    #[serde(default)]
    pub min_length: Option<i32>,
    /// Require uppercase letters
    #[serde(default)]
    pub require_uppercase: Option<bool>,
    /// Require lowercase letters
    #[serde(default)]
    pub require_lowercase: Option<bool>,
    /// Require numbers
    #[serde(default)]
    pub require_numbers: Option<bool>,
    /// Require special characters
    #[serde(default)]
    pub require_special_chars: Option<bool>,
    /// Custom set of allowed special characters
    #[serde(default)]
    pub special_chars_allowed: Option<String>,
    /// Prevent reuse of last N passwords (0-24)
    #[serde(default)]
    pub password_history: Option<i32>,
    /// Password expires after N days (0=never, 1-999)
    #[serde(default)]
    pub expiration_days: Option<i32>,
    /// Minimum age in days before password can be changed
    #[serde(default)]
    pub min_age_days: Option<i32>,
    /// Lockout after N failed attempts (0=disabled, 1-20)
    #[serde(default)]
    pub max_failed_attempts: Option<i32>,
    /// Lockout duration in minutes
    #[serde(default)]
    pub lockout_duration_minutes: Option<i32>,
    /// Show password strength indicator
    #[serde(default)]
    pub password_strength_indicator: Option<bool>,
}

/// Request to create a password policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePasswordPolicyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_special_chars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_history: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_attempts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockout_duration_minutes: Option<i32>,
}

/// Request to update a password policy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePasswordPolicyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_uppercase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_lowercase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_numbers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_special_chars: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_history: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_attempts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockout_duration_minutes: Option<i32>,
}
