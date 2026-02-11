use serde::{Deserialize, Serialize};

/// Trusted device for device-based authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Device ID
    #[serde(default)]
    pub id: Option<String>,
    /// User-friendly device name
    #[serde(default)]
    pub device_name: Option<String>,
    /// Device type (desktop, mobile, tablet)
    #[serde(default)]
    pub device_type: Option<String>,
    /// User ID who owns this device
    #[serde(default)]
    pub user_id: Option<i64>,
    /// Operating system/platform
    #[serde(default)]
    pub platform: Option<String>,
    /// Browser name
    #[serde(default)]
    pub browser: Option<String>,
    /// Registration date
    #[serde(default)]
    pub registration_date: Option<String>,
    /// Last used timestamp
    #[serde(default)]
    pub last_used_at: Option<String>,
    /// Trust level
    #[serde(default)]
    pub trust_level: Option<String>,
    /// Device fingerprint (unique identifier)
    #[serde(default)]
    pub device_fingerprint: Option<String>,
    /// Security status
    #[serde(default)]
    pub security_status: Option<String>,
}

/// Request to register a new trusted device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterDeviceRequest {
    /// User ID to associate device with
    pub user_id: i64,
    /// User-friendly device name
    pub device_name: String,
    /// Device type (desktop, mobile, tablet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Operating system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// Browser name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
}

/// Request to update a trusted device
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDeviceRequest {
    /// New device name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// New trust level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<String>,
}

/// Query parameters for listing devices
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}
