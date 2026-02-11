use serde::{Deserialize, Serialize};

/// X.509 certificate used for SAML signing and encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    /// Certificate ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Certificate name
    #[serde(default)]
    pub name: Option<String>,
    /// PEM-encoded X.509 certificate
    #[serde(default)]
    pub certificate: Option<String>,
    /// Certificate validity start date
    #[serde(default)]
    pub not_before: Option<String>,
    /// Certificate validity end date
    #[serde(default)]
    pub not_after: Option<String>,
    /// Certificate status (active, expiring_soon, expired)
    #[serde(default)]
    pub status: Option<String>,
    /// SHA-256 fingerprint
    #[serde(default)]
    pub fingerprint: Option<String>,
    /// Issuer information
    #[serde(default)]
    pub issuer: Option<String>,
    /// Subject information
    #[serde(default)]
    pub subject: Option<String>,
    /// Serial number
    #[serde(default)]
    pub serial_number: Option<String>,
    /// Certificate usage (saml_signing, saml_encryption)
    #[serde(default)]
    pub usage: Option<String>,
}

/// Request to generate a new certificate
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenerateCertificateRequest {
    /// Certificate name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Validity period in years (1-10, default 3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_years: Option<i32>,
}
