use serde::{Deserialize, Serialize};

/// Trusted Identity Provider for federation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedIdp {
    /// IDP ID
    #[serde(default)]
    pub id: Option<i64>,
    /// IDP name
    #[serde(default)]
    pub name: Option<String>,
    /// IDP type (saml, oidc)
    #[serde(default, rename = "type")]
    pub idp_type: Option<String>,
    /// Whether IDP is enabled
    #[serde(default)]
    pub enabled: Option<bool>,
    /// Issuer URL
    #[serde(default)]
    pub issuer: Option<String>,
    /// SAML SSO endpoint URL
    #[serde(default)]
    pub sso_endpoint: Option<String>,
    /// SAML SLO endpoint URL
    #[serde(default)]
    pub slo_endpoint: Option<String>,
    /// X.509 certificate (SAML)
    #[serde(default)]
    pub certificate: Option<String>,
    /// OIDC client ID
    #[serde(default)]
    pub client_id: Option<String>,
    /// OIDC authorization endpoint
    #[serde(default)]
    pub authorization_endpoint: Option<String>,
    /// OIDC token endpoint
    #[serde(default)]
    pub token_endpoint: Option<String>,
    /// Attribute mappings
    #[serde(default)]
    pub attribute_mappings: Option<serde_json::Value>,
    /// Apps that can use this IDP
    #[serde(default)]
    pub apps: Option<Vec<i64>>,
    /// JIT provisioning enabled
    #[serde(default)]
    pub jit_provisioning: Option<bool>,
    /// Login hints enabled
    #[serde(default)]
    pub login_hints: Option<bool>,
}

/// Request to create a trusted IDP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTrustedIdpRequest {
    /// IDP name
    pub name: String,
    /// IDP type (saml or oidc)
    #[serde(rename = "type")]
    pub idp_type: String,
    /// Whether IDP is enabled (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Issuer URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// SAML SSO endpoint URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_endpoint: Option<String>,
    /// SAML SLO endpoint URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo_endpoint: Option<String>,
    /// X.509 certificate (SAML)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// OIDC client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// OIDC client secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// OIDC authorization endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    /// OIDC token endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
}

/// Request to update a trusted IDP
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTrustedIdpRequest {
    /// New IDP name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New enabled status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// New issuer URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// New SAML SSO endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_endpoint: Option<String>,
    /// New SAML SLO endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo_endpoint: Option<String>,
    /// New certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

/// Request to update IDP metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTrustedIdpMetadataRequest {
    /// SAML metadata XML content
    pub metadata: String,
}

/// Trusted IDP issuer response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedIdpIssuer {
    /// Issuer URL
    #[serde(default)]
    pub issuer: Option<String>,
}
