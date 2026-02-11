// Allow dead code in API modules - these provide comprehensive API coverage
// even if not all methods are currently wired up to MCP tools
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

use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use std::sync::Arc;

/// Main API client that aggregates all OneLogin API modules
#[allow(dead_code)]
pub struct OneLoginClient {
    pub users: users::UsersApi,
    pub apps: apps::AppsApi,
    pub app_rules: app_rules::AppRulesApi,
    pub roles: roles::RolesApi,
    pub groups: groups::GroupsApi,
    pub mfa: mfa::MfaApi,
    pub saml: saml::SamlApi,
    pub self_registration: self_registration::SelfRegistrationApi,
    pub smart_hooks: smart_hooks::SmartHooksApi,
    pub smart_mfa: smart_mfa::SmartMfaApi,
    pub vigilance: vigilance::VigilanceApi,
    pub privileges: privileges::PrivilegesApi,
    pub reports: reports::ReportsApi,
    pub user_mappings: user_mappings::UserMappingsApi,
    pub invitations: invitations::InvitationsApi,
    pub login: login::LoginApi,
    pub custom_attributes: custom_attributes::CustomAttributesApi,
    pub embed_tokens: embed_tokens::EmbedTokensApi,
    pub oauth: oauth::OAuthApi,
    pub webhooks: webhooks::WebhooksApi,
    pub oidc: oidc::OidcApi,
    pub directories: directories::DirectoriesApi,
    pub branding: branding::BrandingApi,
    pub events: events::EventsApi,
    pub api_auth: api_auth::ApiAuthApi,
    pub connectors: connectors::ConnectorsApi,
    // New API modules
    pub rate_limits: rate_limits::RateLimitsApi,
    pub account: account::AccountApi,
    pub password_policies: password_policies::PasswordPoliciesApi,
    pub certificates: certificates::CertificatesApi,
    pub device_trust: device_trust::DeviceTrustApi,
    pub login_pages: login_pages::LoginPagesApi,
    pub trusted_idps: trusted_idps::TrustedIdpsApi,
}

impl OneLoginClient {
    pub fn new(http_client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self {
            users: users::UsersApi::new(http_client.clone(), cache.clone()),
            apps: apps::AppsApi::new(http_client.clone(), cache.clone()),
            app_rules: app_rules::AppRulesApi::new(http_client.clone(), cache.clone()),
            roles: roles::RolesApi::new(http_client.clone(), cache.clone()),
            groups: groups::GroupsApi::new(http_client.clone(), cache.clone()),
            mfa: mfa::MfaApi::new(http_client.clone(), cache.clone()),
            saml: saml::SamlApi::new(http_client.clone(), cache.clone()),
            self_registration: self_registration::SelfRegistrationApi::new(http_client.clone(), cache.clone()),
            smart_hooks: smart_hooks::SmartHooksApi::new(http_client.clone(), cache.clone()),
            smart_mfa: smart_mfa::SmartMfaApi::new(http_client.clone(), cache.clone()),
            vigilance: vigilance::VigilanceApi::new(http_client.clone(), cache.clone()),
            privileges: privileges::PrivilegesApi::new(http_client.clone(), cache.clone()),
            reports: reports::ReportsApi::new(http_client.clone(), cache.clone()),
            user_mappings: user_mappings::UserMappingsApi::new(http_client.clone(), cache.clone()),
            invitations: invitations::InvitationsApi::new(http_client.clone(), cache.clone()),
            login: login::LoginApi::new(http_client.clone(), cache.clone()),
            custom_attributes: custom_attributes::CustomAttributesApi::new(
                http_client.clone(),
                cache.clone(),
            ),
            embed_tokens: embed_tokens::EmbedTokensApi::new(http_client.clone(), cache.clone()),
            oauth: oauth::OAuthApi::new(http_client.clone(), cache.clone()),
            webhooks: webhooks::WebhooksApi::new(),
            oidc: oidc::OidcApi::new(http_client.clone(), cache.clone()),
            directories: directories::DirectoriesApi::new(http_client.clone(), cache.clone()),
            branding: branding::BrandingApi::new(http_client.clone(), cache.clone()),
            events: events::EventsApi::new(http_client.clone(), cache.clone()),
            api_auth: api_auth::ApiAuthApi::new(http_client.clone(), cache.clone()),
            connectors: connectors::ConnectorsApi::new(http_client.clone(), cache.clone()),
            // New API modules
            rate_limits: rate_limits::RateLimitsApi::new(http_client.clone(), cache.clone()),
            account: account::AccountApi::new(http_client.clone(), cache.clone()),
            password_policies: password_policies::PasswordPoliciesApi::new(http_client.clone(), cache.clone()),
            certificates: certificates::CertificatesApi::new(http_client.clone(), cache.clone()),
            device_trust: device_trust::DeviceTrustApi::new(http_client.clone(), cache.clone()),
            login_pages: login_pages::LoginPagesApi::new(http_client.clone(), cache.clone()),
            trusted_idps: trusted_idps::TrustedIdpsApi::new(http_client.clone(), cache.clone()),
        }
    }
}
