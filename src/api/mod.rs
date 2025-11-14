pub mod users;
pub mod apps;
pub mod roles;
pub mod groups;
pub mod mfa;
pub mod saml;
pub mod smart_hooks;
pub mod vigilance;
pub mod privileges;
pub mod user_mappings;
pub mod policies;
pub mod invitations;
pub mod custom_attributes;
pub mod embed_tokens;
pub mod oauth;
pub mod webhooks;
pub mod scim;
pub mod oidc;
pub mod directories;
pub mod branding;
pub mod events;
pub mod sessions;
pub mod api_auth;

use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use std::sync::Arc;

/// Main API client that aggregates all OneLogin API modules
pub struct OneLoginClient {
    pub users: users::UsersApi,
    pub apps: apps::AppsApi,
    pub roles: roles::RolesApi,
    pub groups: groups::GroupsApi,
    pub mfa: mfa::MfaApi,
    pub saml: saml::SamlApi,
    pub smart_hooks: smart_hooks::SmartHooksApi,
    pub vigilance: vigilance::VigilanceApi,
    pub privileges: privileges::PrivilegesApi,
    pub user_mappings: user_mappings::UserMappingsApi,
    pub policies: policies::PoliciesApi,
    pub invitations: invitations::InvitationsApi,
    pub custom_attributes: custom_attributes::CustomAttributesApi,
    pub embed_tokens: embed_tokens::EmbedTokensApi,
    pub oauth: oauth::OAuthApi,
    pub webhooks: webhooks::WebhooksApi,
    pub scim: scim::ScimApi,
    pub oidc: oidc::OidcApi,
    pub directories: directories::DirectoriesApi,
    pub branding: branding::BrandingApi,
    pub events: events::EventsApi,
    pub sessions: sessions::SessionsApi,
    pub api_auth: api_auth::ApiAuthApi,
}

impl OneLoginClient {
    pub fn new(http_client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self {
            users: users::UsersApi::new(http_client.clone(), cache.clone()),
            apps: apps::AppsApi::new(http_client.clone(), cache.clone()),
            roles: roles::RolesApi::new(http_client.clone(), cache.clone()),
            groups: groups::GroupsApi::new(http_client.clone(), cache.clone()),
            mfa: mfa::MfaApi::new(http_client.clone(), cache.clone()),
            saml: saml::SamlApi::new(http_client.clone(), cache.clone()),
            smart_hooks: smart_hooks::SmartHooksApi::new(http_client.clone(), cache.clone()),
            vigilance: vigilance::VigilanceApi::new(http_client.clone(), cache.clone()),
            privileges: privileges::PrivilegesApi::new(http_client.clone(), cache.clone()),
            user_mappings: user_mappings::UserMappingsApi::new(
                http_client.clone(),
                cache.clone(),
            ),
            policies: policies::PoliciesApi::new(http_client.clone(), cache.clone()),
            invitations: invitations::InvitationsApi::new(http_client.clone(), cache.clone()),
            custom_attributes: custom_attributes::CustomAttributesApi::new(
                http_client.clone(),
                cache.clone(),
            ),
            embed_tokens: embed_tokens::EmbedTokensApi::new(http_client.clone(), cache.clone()),
            oauth: oauth::OAuthApi::new(http_client.clone(), cache.clone()),
            webhooks: webhooks::WebhooksApi::new(http_client.clone(), cache.clone()),
            scim: scim::ScimApi::new(http_client.clone(), cache.clone()),
            oidc: oidc::OidcApi::new(http_client.clone(), cache.clone()),
            directories: directories::DirectoriesApi::new(http_client.clone(), cache.clone()),
            branding: branding::BrandingApi::new(http_client.clone(), cache.clone()),
            events: events::EventsApi::new(http_client.clone(), cache.clone()),
            sessions: sessions::SessionsApi::new(http_client.clone(), cache.clone()),
            api_auth: api_auth::ApiAuthApi::new(http_client.clone(), cache.clone()),
        }
    }
}
