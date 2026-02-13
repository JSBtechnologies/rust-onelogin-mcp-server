use crate::api::OneLoginClient;
use crate::core::error::OneLoginError;
use crate::core::tenant_manager::TenantManager;
use crate::core::tool_config::ToolConfig;
use crate::models::events::EventQueryParams;
use crate::models::roles::CreateRoleRequest;
use crate::models::users::{User, UserQueryParams};
use crate::utils::{base64_encode, base64_decode};
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{info, debug, warn};

/// Extract an i64 from a JSON Value, handling both numeric and string representations.
/// MCP clients often send numbers as strings (e.g., "257299146" instead of 257299146).
fn value_as_i64(v: &Value) -> Option<i64> {
    v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))
}

#[allow(dead_code)]
pub struct ToolRegistry {
    tenant_manager: Arc<TenantManager>,
    tool_config: Arc<ToolConfig>,
}

#[derive(Debug, Default, Deserialize)]
struct ListUsersArgs {
    email: Option<String>,
    username: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    directory_id: Option<i64>,
    role_id: Option<i64>,
    limit: Option<i32>,
    page: Option<i32>,
    auto_paginate: Option<bool>,
    max_pages: Option<i32>,
    max_results: Option<u32>,
}

#[allow(dead_code)]
impl ToolRegistry {
    pub fn new(tenant_manager: Arc<TenantManager>, tool_config: Arc<ToolConfig>) -> Self {
        Self { tenant_manager, tool_config }
    }

    /// Extract the optional "tenant" parameter from tool args and resolve to the correct client.
    fn resolve_client(&self, args: &Value) -> Result<Arc<OneLoginClient>> {
        let tenant = args.get("tenant").and_then(|v| v.as_str());
        self.tenant_manager.resolve(tenant)
    }

    /// Inject the optional "tenant" parameter into a tool's inputSchema when in multi-tenant mode.
    fn with_tenant_param(&self, mut tool: Value) -> Value {
        if !self.tenant_manager.is_multi_tenant() {
            return tool;
        }

        let tenant_names: Vec<String> = self.tenant_manager
            .tenant_info()
            .iter()
            .map(|t| t.name.clone())
            .collect();

        let default_name = self.tenant_manager.default_tenant_name();

        if let Some(schema) = tool.get_mut("inputSchema") {
            if let Some(props) = schema.get_mut("properties") {
                if let Some(obj) = props.as_object_mut() {
                    obj.insert("tenant".to_string(), json!({
                        "type": "string",
                        "enum": tenant_names,
                        "description": format!(
                            "Target tenant for this operation. Available: {}. Default: '{}'",
                            tenant_names.join(", "),
                            default_name
                        )
                    }));
                }
            }
        }
        tool
    }

    /// Returns a reference to the tool config for external access (e.g., hot reload watcher)
    pub fn tool_config(&self) -> &Arc<ToolConfig> {
        &self.tool_config
    }

    pub fn list_tools(&self) -> Vec<Value> {
        let all_tools = vec![
            // Users API
            self.tool_list_users(),
            self.tool_get_user(),
            self.tool_create_user(),
            self.tool_update_user(),
            self.tool_delete_user(),
            self.tool_get_user_apps(),
            self.tool_get_user_roles(),
            self.tool_unlock_user(),
            self.tool_logout_user(),
            self.tool_assign_roles(),
            self.tool_remove_roles(),
            self.tool_lock_user(),
            self.tool_set_password(),
            self.tool_set_custom_attributes(),
            // Apps API
            self.tool_list_apps(),
            self.tool_get_app(),
            self.tool_create_app(),
            self.tool_update_app(),
            self.tool_delete_app(),
            // App Rules API
            self.tool_list_app_rules(),
            self.tool_get_app_rule(),
            self.tool_create_app_rule(),
            self.tool_update_app_rule(),
            self.tool_delete_app_rule(),
            self.tool_list_app_rule_conditions(),
            self.tool_list_app_rule_actions(),
            self.tool_list_condition_operators(),
            self.tool_list_condition_values(),
            self.tool_list_action_values(),
            self.tool_sort_app_rules(),
            // Connectors API
            self.tool_list_connectors(),
            self.tool_get_connector(),
            // Roles API
            self.tool_list_roles(),
            self.tool_get_role(),
            self.tool_create_role(),
            self.tool_update_role(),
            self.tool_delete_role(),
            // Groups API (read-only - groups are managed via directory sync or admin console)
            self.tool_list_groups(),
            self.tool_get_group(),
            // MFA API
            self.tool_list_mfa_factors(),
            self.tool_enroll_mfa_factor(),
            self.tool_remove_mfa_factor(),
            self.tool_verify_mfa_factor(),
            self.tool_enroll_mfa(),
            self.tool_verify_mfa(),
            self.tool_remove_mfa(),
            self.tool_generate_mfa_token(),
            self.tool_verify_mfa_token(),
            // SAML API
            self.tool_get_saml_assertion(),
            self.tool_verify_saml_factor(),
            self.tool_get_saml_assertion_v2(),
            // Smart Hooks API
            self.tool_create_smart_hook(),
            self.tool_update_smart_hook(),
            self.tool_delete_smart_hook(),
            self.tool_get_smart_hook(),
            self.tool_list_smart_hooks(),
            self.tool_get_smart_hook_logs(),
            // Hook Environment Variables (account-level, shared by all hooks)
            self.tool_list_hook_env_vars(),
            self.tool_get_hook_env_var(),
            self.tool_create_hook_env_var(),
            self.tool_update_hook_env_var(),
            self.tool_delete_hook_env_var(),
            // Vigilance/Risk API
            self.tool_get_risk_score(),
            self.tool_validate_user_smart_mfa(),
            self.tool_list_risk_rules(),
            self.tool_create_risk_rule(),
            self.tool_update_risk_rule(),
            self.tool_delete_risk_rule(),
            self.tool_get_risk_events(),
            self.tool_track_risk_event(),
            // Privileges API
            self.tool_list_privileges(),
            self.tool_get_privilege(),
            self.tool_create_privilege(),
            self.tool_update_privilege(),
            self.tool_delete_privilege(),
            self.tool_assign_privilege_to_user(),
            self.tool_assign_privilege_to_role(),
            // User Mappings API
            self.tool_list_user_mappings(),
            self.tool_get_user_mapping(),
            self.tool_create_user_mapping(),
            self.tool_update_user_mapping(),
            self.tool_delete_user_mapping(),
            self.tool_sort_user_mappings(),
            self.tool_sort_mapping_order(),
            self.tool_list_mapping_conditions(),
            // Invitations API
            self.tool_generate_invite_link(),
            self.tool_send_invite_link(),
            // Custom Attributes API
            self.tool_list_custom_attributes(),
            self.tool_create_custom_attribute(),
            self.tool_update_custom_attribute(),
            self.tool_delete_custom_attribute(),
            // Embed Tokens API
            self.tool_generate_embed_token(),
            self.tool_list_embeddable_apps(),
            // OAuth API
            self.tool_generate_oauth_tokens(),
            self.tool_revoke_oauth_token(),
            self.tool_introspect_oauth_token(),
            // OIDC API
            self.tool_oidc_get_well_known_config(),
            self.tool_oidc_get_jwks(),
            self.tool_oidc_get_userinfo(),
            // Directories API
            self.tool_list_directory_connectors(),
            self.tool_get_directory_connector(),
            self.tool_create_directory_connector(),
            self.tool_update_directory_connector(),
            self.tool_delete_directory_connector(),
            self.tool_sync_directory(),
            self.tool_get_sync_status(),
            // Branding API
            self.tool_get_branding_settings(),
            self.tool_update_branding_settings(),
            self.tool_list_message_templates(),
            self.tool_get_message_template(),
            self.tool_get_template_by_type(),
            self.tool_get_template_by_locale(),
            self.tool_create_message_template(),
            self.tool_update_message_template(),
            self.tool_update_template_by_locale(),
            self.tool_delete_message_template(),
            // Self-Registration API
            self.tool_list_self_registration_profiles(),
            self.tool_get_self_registration_profile(),
            self.tool_create_self_registration_profile(),
            self.tool_update_self_registration_profile(),
            self.tool_delete_self_registration_profile(),
            self.tool_list_registrations(),
            self.tool_approve_registration(),
            // Reports API
            self.tool_list_reports(),
            self.tool_get_report(),
            self.tool_run_report(),
            self.tool_get_report_results(),
            // Login/Session API
            self.tool_create_session_login_token(),
            self.tool_verify_factor_login(),
            self.tool_create_session(),
            // Events API
            self.tool_list_events(),
            self.tool_get_event(),
            self.tool_create_event(),
            self.tool_list_event_types(),
            // API Authorization API
            self.tool_list_api_authorizations(),
            self.tool_get_api_authorization(),
            self.tool_create_api_authorization(),
            self.tool_update_api_authorization(),
            self.tool_delete_api_authorization(),
            // Rate Limits API
            self.tool_get_rate_limit_status(),
            self.tool_get_rate_limits(),
            // Account Settings API
            self.tool_get_account_settings(),
            self.tool_update_account_settings(),
            self.tool_get_account_features(),
            self.tool_get_account_usage(),
            // Password Policies API
            self.tool_list_password_policies(),
            self.tool_get_password_policy(),
            self.tool_create_password_policy(),
            self.tool_update_password_policy(),
            // Certificates API
            self.tool_list_certificates(),
            self.tool_get_certificate(),
            self.tool_generate_certificate(),
            self.tool_renew_certificate(),
            // Device Trust API
            self.tool_list_devices(),
            self.tool_get_device(),
            self.tool_register_device(),
            self.tool_update_device(),
            self.tool_delete_device(),
            // Login Pages API
            self.tool_list_login_pages(),
            self.tool_get_login_page(),
            self.tool_create_login_page(),
            self.tool_update_login_page(),
            self.tool_delete_login_page(),
            // Trusted IDPs API
            self.tool_list_trusted_idps(),
            self.tool_get_trusted_idp(),
            self.tool_create_trusted_idp(),
            self.tool_update_trusted_idp(),
            self.tool_delete_trusted_idp(),
            self.tool_get_trusted_idp_metadata(),
            self.tool_update_trusted_idp_metadata(),
            self.tool_get_trusted_idp_issuer(),
            // Expanded Roles API (sub-resources)
            self.tool_get_role_apps(),
            self.tool_set_role_apps(),
            self.tool_get_role_users(),
            self.tool_get_role_admins(),
            self.tool_add_role_admins(),
            self.tool_remove_role_admin(),
            // Note: assign_roles_to_user and remove_roles_from_user omitted - use existing
            // onelogin_assign_roles and onelogin_remove_roles instead (same functionality)
        ];

        // Inject tenant parameter into all tools when in multi-tenant mode
        let mut tools: Vec<Value> = all_tools
            .into_iter()
            .map(|t| self.with_tenant_param(t))
            .collect();

        // Add tenant management tools
        tools.push(self.tool_list_tenants());

        // Filter tools based on configuration
        tools
            .into_iter()
            .filter(|tool| {
                let name = tool["name"].as_str().unwrap_or("");
                self.tool_config.is_tool_enabled(name)
            })
            .collect()
    }

    pub async fn call_tool(&self, params: &super::server::CallToolParams) -> Result<String> {
        // Check if tool is enabled before executing
        if !self.tool_config.is_tool_enabled(&params.name) {
            warn!("Attempted to call disabled tool: {}", params.name);
            let config_location = self.tool_config
                .config_path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "default configuration".to_string());
            return Err(anyhow!(
                "Tool '{}' is not enabled. Check your tool configuration at: {}",
                params.name,
                config_location
            ));
        }

        info!("Calling tool: {}", params.name);

        let result = match params.name.as_str() {
            // Users
            "onelogin_list_users" => self.handle_list_users(&params.arguments).await?,
            "onelogin_get_user" => self.handle_get_user(&params.arguments).await?,
            "onelogin_create_user" => self.handle_create_user(&params.arguments).await?,
            "onelogin_update_user" => self.handle_update_user(&params.arguments).await?,
            "onelogin_delete_user" => self.handle_delete_user(&params.arguments).await?,
            "onelogin_get_user_apps" => self.handle_get_user_apps(&params.arguments).await?,
            "onelogin_get_user_roles" => self.handle_get_user_roles(&params.arguments).await?,
            "onelogin_unlock_user" => self.handle_unlock_user(&params.arguments).await?,
            "onelogin_logout_user" => self.handle_logout_user(&params.arguments).await?,

            // Apps
            "onelogin_list_apps" => self.handle_list_apps(&params.arguments).await?,
            "onelogin_get_app" => self.handle_get_app(&params.arguments).await?,
            "onelogin_create_app" => self.handle_create_app(&params.arguments).await?,
            "onelogin_update_app" => self.handle_update_app(&params.arguments).await?,
            "onelogin_delete_app" => self.handle_delete_app(&params.arguments).await?,

            // App Rules
            "onelogin_list_app_rules" => self.handle_list_app_rules(&params.arguments).await?,
            "onelogin_get_app_rule" => self.handle_get_app_rule(&params.arguments).await?,
            "onelogin_create_app_rule" => self.handle_create_app_rule(&params.arguments).await?,
            "onelogin_update_app_rule" => self.handle_update_app_rule(&params.arguments).await?,
            "onelogin_delete_app_rule" => self.handle_delete_app_rule(&params.arguments).await?,
            "onelogin_list_app_rule_conditions" => self.handle_list_app_rule_conditions(&params.arguments).await?,
            "onelogin_list_app_rule_actions" => self.handle_list_app_rule_actions(&params.arguments).await?,
            "onelogin_list_condition_operators" => self.handle_list_condition_operators(&params.arguments).await?,
            "onelogin_list_condition_values" => self.handle_list_condition_values(&params.arguments).await?,
            "onelogin_list_action_values" => self.handle_list_action_values(&params.arguments).await?,
            "onelogin_sort_app_rules" => self.handle_sort_app_rules(&params.arguments).await?,

            // Connectors
            "onelogin_list_connectors" => self.handle_list_connectors(&params.arguments).await?,
            "onelogin_get_connector" => self.handle_get_connector(&params.arguments).await?,

            // Roles
            "onelogin_list_roles" => self.handle_list_roles(&params.arguments).await?,
            "onelogin_get_role" => self.handle_get_role(&params.arguments).await?,
            "onelogin_create_role" => self.handle_create_role(&params.arguments).await?,
            "onelogin_update_role" => self.handle_update_role(&params.arguments).await?,
            "onelogin_delete_role" => self.handle_delete_role(&params.arguments).await?,

            // Groups
            "onelogin_list_groups" => self.handle_list_groups(&params.arguments).await?,
            "onelogin_get_group" => self.handle_get_group(&params.arguments).await?,
            "onelogin_create_group" => self.handle_create_group(&params.arguments).await?,
            "onelogin_update_group" => self.handle_update_group(&params.arguments).await?,
            "onelogin_delete_group" => self.handle_delete_group(&params.arguments).await?,

            // User Operations
            "onelogin_assign_roles" => self.handle_assign_roles(&params.arguments).await?,
            "onelogin_remove_roles" => self.handle_remove_roles(&params.arguments).await?,
            "onelogin_lock_user" => self.handle_lock_user(&params.arguments).await?,
            "onelogin_set_password" => self.handle_set_password(&params.arguments).await?,
            "onelogin_set_custom_attributes" => self.handle_set_custom_attributes(&params.arguments).await?,

            // Smart Hooks
            "onelogin_create_smart_hook" => {
                self.handle_create_smart_hook(&params.arguments).await?
            }
            "onelogin_update_smart_hook" => {
                self.handle_update_smart_hook(&params.arguments).await?
            }
            "onelogin_list_smart_hooks" => self.handle_list_smart_hooks(&params.arguments).await?,
            "onelogin_get_smart_hook" => self.handle_get_smart_hook(&params.arguments).await?,
            "onelogin_delete_smart_hook" => self.handle_delete_smart_hook(&params.arguments).await?,
            "onelogin_get_smart_hook_logs" => self.handle_get_smart_hook_logs(&params.arguments).await?,
            // Hook Environment Variables (account-level)
            "onelogin_list_hook_env_vars" => self.handle_list_hook_env_vars(&params.arguments).await?,
            "onelogin_get_hook_env_var" => self.handle_get_hook_env_var(&params.arguments).await?,
            "onelogin_create_hook_env_var" => self.handle_create_hook_env_var(&params.arguments).await?,
            "onelogin_update_hook_env_var" => self.handle_update_hook_env_var(&params.arguments).await?,
            "onelogin_delete_hook_env_var" => self.handle_delete_hook_env_var(&params.arguments).await?,

            // Vigilance
            "onelogin_get_risk_score" => self.handle_get_risk_score(&params.arguments).await?,
            "onelogin_validate_user_smart_mfa" => {
                self.handle_validate_user_smart_mfa(&params.arguments)
                    .await?
            }

            // SAML
            "onelogin_get_saml_assertion" => self.handle_get_saml_assertion(&params.arguments).await?,
            "onelogin_verify_saml_factor" => self.handle_verify_saml_factor(&params.arguments).await?,

            // Privileges
            "onelogin_list_privileges" => self.handle_list_privileges(&params.arguments).await?,
            "onelogin_get_privilege" => self.handle_get_privilege(&params.arguments).await?,
            "onelogin_create_privilege" => self.handle_create_privilege(&params.arguments).await?,
            "onelogin_update_privilege" => self.handle_update_privilege(&params.arguments).await?,
            "onelogin_delete_privilege" => self.handle_delete_privilege(&params.arguments).await?,
            "onelogin_assign_user_to_privilege" => self.handle_assign_user_to_privilege(&params.arguments).await?,
            "onelogin_assign_role_to_privilege" => self.handle_assign_role_to_privilege(&params.arguments).await?,

            // MFA Operations
            "onelogin_list_mfa_factors" => self.handle_list_mfa_factors(&params.arguments).await?,
            "onelogin_enroll_mfa" => self.handle_enroll_mfa(&params.arguments).await?,
            "onelogin_verify_mfa" => self.handle_verify_mfa(&params.arguments).await?,
            "onelogin_remove_mfa" => self.handle_remove_mfa(&params.arguments).await?,
            "onelogin_generate_mfa_token" => self.handle_generate_mfa_token(&params.arguments).await?,
            "onelogin_verify_mfa_token" => self.handle_verify_mfa_token(&params.arguments).await?,

            // Events
            "onelogin_list_events" => self.handle_list_events(&params.arguments).await?,
            "onelogin_get_event" => self.handle_get_event(&params.arguments).await?,
            "onelogin_create_event" => self.handle_create_event(&params.arguments).await?,
            "onelogin_list_event_types" => self.handle_list_event_types(&params.arguments).await?,

            // User Mappings
            "onelogin_get_user_mapping" => self.handle_get_user_mapping(&params.arguments).await?,
            "onelogin_create_user_mapping" => self.handle_create_user_mapping(&params.arguments).await?,
            "onelogin_update_user_mapping" => self.handle_update_user_mapping(&params.arguments).await?,
            "onelogin_delete_user_mapping" => self.handle_delete_user_mapping(&params.arguments).await?,
            "onelogin_sort_mapping_order" => self.handle_sort_mapping_order(&params.arguments).await?,
            "onelogin_list_mapping_conditions" => self.handle_list_mapping_conditions(&params.arguments).await?,

            // Custom Attributes
            "onelogin_list_custom_attributes" => self.handle_list_custom_attributes(&params.arguments).await?,
            "onelogin_create_custom_attribute" => self.handle_create_custom_attribute(&params.arguments).await?,
            "onelogin_update_custom_attribute" => self.handle_update_custom_attribute(&params.arguments).await?,
            "onelogin_delete_custom_attribute" => self.handle_delete_custom_attribute(&params.arguments).await?,

            // Directories
            "onelogin_list_directory_connectors" => self.handle_list_directory_connectors(&params.arguments).await?,

            // Branding
            "onelogin_get_branding_settings" => self.handle_get_branding_settings(&params.arguments).await?,
            "onelogin_list_message_templates" => self.handle_list_message_templates(&params.arguments).await?,
            "onelogin_get_message_template" => self.handle_get_message_template(&params.arguments).await?,
            "onelogin_get_template_by_type" => self.handle_get_template_by_type(&params.arguments).await?,
            "onelogin_get_template_by_locale" => self.handle_get_template_by_locale(&params.arguments).await?,
            "onelogin_create_message_template" => self.handle_create_message_template(&params.arguments).await?,
            "onelogin_update_message_template" => self.handle_update_message_template(&params.arguments).await?,
            "onelogin_update_template_by_locale" => self.handle_update_template_by_locale(&params.arguments).await?,
            "onelogin_delete_message_template" => self.handle_delete_message_template(&params.arguments).await?,

            // Self-Registration
            "onelogin_list_self_registration_profiles" => self.handle_list_self_registration_profiles(&params.arguments).await?,
            "onelogin_get_self_registration_profile" => self.handle_get_self_registration_profile(&params.arguments).await?,
            "onelogin_create_self_registration_profile" => self.handle_create_self_registration_profile(&params.arguments).await?,
            "onelogin_update_self_registration_profile" => self.handle_update_self_registration_profile(&params.arguments).await?,
            "onelogin_delete_self_registration_profile" => self.handle_delete_self_registration_profile(&params.arguments).await?,
            "onelogin_list_registrations" => self.handle_list_registrations(&params.arguments).await?,
            "onelogin_approve_registration" => self.handle_approve_registration(&params.arguments).await?,

            // Reports
            "onelogin_list_reports" => self.handle_list_reports(&params.arguments).await?,
            "onelogin_get_report" => self.handle_get_report(&params.arguments).await?,
            "onelogin_run_report" => self.handle_run_report(&params.arguments).await?,
            "onelogin_get_report_results" => self.handle_get_report_results(&params.arguments).await?,

            // Login/Session
            "onelogin_create_session_login_token" => self.handle_create_session_login_token(&params.arguments).await?,
            "onelogin_verify_factor_login" => self.handle_verify_factor_login(&params.arguments).await?,
            "onelogin_create_session" => self.handle_create_session(&params.arguments).await?,

            // OIDC
            "onelogin_oidc_get_well_known_config" => {
                self.handle_oidc_get_well_known_config(&params.arguments).await?
            }
            "onelogin_oidc_get_jwks" => self.handle_oidc_get_jwks(&params.arguments).await?,

            // OAuth
            "onelogin_generate_oauth_tokens" => self.handle_generate_oauth_tokens(&params.arguments).await?,
            "onelogin_revoke_oauth_token" => self.handle_revoke_oauth_token(&params.arguments).await?,
            "onelogin_introspect_oauth_token" => self.handle_introspect_oauth_token(&params.arguments).await?,

            // Embed Tokens
            "onelogin_generate_embed_token" => self.handle_generate_embed_token(&params.arguments).await?,
            "onelogin_list_embeddable_apps" => self.handle_list_embeddable_apps(&params.arguments).await?,

            // API Auth
            "onelogin_list_api_authorizations" => self.handle_list_api_authorizations(&params.arguments).await?,
            "onelogin_get_api_authorization" => self.handle_get_api_authorization(&params.arguments).await?,
            "onelogin_create_api_authorization" => self.handle_create_api_authorization(&params.arguments).await?,
            "onelogin_update_api_authorization" => self.handle_update_api_authorization(&params.arguments).await?,
            "onelogin_delete_api_authorization" => self.handle_delete_api_authorization(&params.arguments).await?,

            // Additional SAML
            "onelogin_get_saml_assertion_v2" => self.handle_get_saml_assertion_v2(&params.arguments).await?,

            // Additional OIDC
            "onelogin_oidc_get_userinfo" => self.handle_oidc_get_userinfo(&params.arguments).await?,

            // Additional Vigilance
            "onelogin_list_risk_rules" => self.handle_list_risk_rules(&params.arguments).await?,
            "onelogin_create_risk_rule" => self.handle_create_risk_rule(&params.arguments).await?,
            "onelogin_update_risk_rule" => self.handle_update_risk_rule(&params.arguments).await?,
            "onelogin_delete_risk_rule" => self.handle_delete_risk_rule(&params.arguments).await?,
            "onelogin_get_risk_events" => self.handle_get_risk_events(&params.arguments).await?,
            "onelogin_track_risk_event" => self.handle_track_risk_event(&params.arguments).await?,

            // Directories
            "onelogin_get_directory_connector" => self.handle_get_directory_connector(&params.arguments).await?,
            "onelogin_update_directory_connector" => self.handle_update_directory_connector(&params.arguments).await?,
            "onelogin_create_directory_connector" => self.handle_create_directory_connector(&params.arguments).await?,
            "onelogin_delete_directory_connector" => self.handle_delete_directory_connector(&params.arguments).await?,
            "onelogin_sync_directory" => self.handle_sync_directory(&params.arguments).await?,
            "onelogin_get_sync_status" => self.handle_get_sync_status(&params.arguments).await?,

            // User Mappings
            "onelogin_list_user_mappings" => self.handle_list_user_mappings(&params.arguments).await?,
            "onelogin_sort_user_mappings" => self.handle_sort_user_mappings(&params.arguments).await?,
            "onelogin_enroll_mfa_factor" => self.handle_enroll_mfa_factor(&params.arguments).await?,
            "onelogin_verify_mfa_factor" => self.handle_verify_mfa_factor(&params.arguments).await?,
            "onelogin_remove_mfa_factor" => self.handle_remove_mfa_factor(&params.arguments).await?,

            // Invitations
            "onelogin_generate_invite_link" => self.handle_generate_invite_link(&params.arguments).await?,
            "onelogin_send_invite_link" => self.handle_send_invite_link(&params.arguments).await?,

            // Branding
            "onelogin_update_branding_settings" => self.handle_update_branding_settings(&params.arguments).await?,

            // Rate Limits
            "onelogin_get_rate_limit_status" => self.handle_get_rate_limit_status(&params.arguments).await?,
            "onelogin_get_rate_limits" => self.handle_get_rate_limits(&params.arguments).await?,

            // Account Settings
            "onelogin_get_account_settings" => self.handle_get_account_settings(&params.arguments).await?,
            "onelogin_update_account_settings" => self.handle_update_account_settings(&params.arguments).await?,
            "onelogin_get_account_features" => self.handle_get_account_features(&params.arguments).await?,
            "onelogin_get_account_usage" => self.handle_get_account_usage(&params.arguments).await?,

            // Password Policies
            "onelogin_list_password_policies" => self.handle_list_password_policies(&params.arguments).await?,
            "onelogin_get_password_policy" => self.handle_get_password_policy(&params.arguments).await?,
            "onelogin_create_password_policy" => self.handle_create_password_policy(&params.arguments).await?,
            "onelogin_update_password_policy" => self.handle_update_password_policy(&params.arguments).await?,

            // Certificates
            "onelogin_list_certificates" => self.handle_list_certificates(&params.arguments).await?,
            "onelogin_get_certificate" => self.handle_get_certificate(&params.arguments).await?,
            "onelogin_generate_certificate" => self.handle_generate_certificate(&params.arguments).await?,
            "onelogin_renew_certificate" => self.handle_renew_certificate(&params.arguments).await?,

            // Device Trust
            "onelogin_list_devices" => self.handle_list_devices(&params.arguments).await?,
            "onelogin_get_device" => self.handle_get_device(&params.arguments).await?,
            "onelogin_register_device" => self.handle_register_device(&params.arguments).await?,
            "onelogin_update_device" => self.handle_update_device(&params.arguments).await?,
            "onelogin_delete_device" => self.handle_delete_device(&params.arguments).await?,

            // Login Pages
            "onelogin_list_login_pages" => self.handle_list_login_pages(&params.arguments).await?,
            "onelogin_get_login_page" => self.handle_get_login_page(&params.arguments).await?,
            "onelogin_create_login_page" => self.handle_create_login_page(&params.arguments).await?,
            "onelogin_update_login_page" => self.handle_update_login_page(&params.arguments).await?,
            "onelogin_delete_login_page" => self.handle_delete_login_page(&params.arguments).await?,

            // Trusted IDPs
            "onelogin_list_trusted_idps" => self.handle_list_trusted_idps(&params.arguments).await?,
            "onelogin_get_trusted_idp" => self.handle_get_trusted_idp(&params.arguments).await?,
            "onelogin_create_trusted_idp" => self.handle_create_trusted_idp(&params.arguments).await?,
            "onelogin_update_trusted_idp" => self.handle_update_trusted_idp(&params.arguments).await?,
            "onelogin_delete_trusted_idp" => self.handle_delete_trusted_idp(&params.arguments).await?,
            "onelogin_get_trusted_idp_metadata" => self.handle_get_trusted_idp_metadata(&params.arguments).await?,
            "onelogin_update_trusted_idp_metadata" => self.handle_update_trusted_idp_metadata(&params.arguments).await?,
            "onelogin_get_trusted_idp_issuer" => self.handle_get_trusted_idp_issuer(&params.arguments).await?,

            // Roles (expanded - sub-resources)
            "onelogin_get_role_apps" => self.handle_get_role_apps(&params.arguments).await?,
            "onelogin_set_role_apps" => self.handle_set_role_apps(&params.arguments).await?,
            "onelogin_get_role_users" => self.handle_get_role_users(&params.arguments).await?,
            "onelogin_get_role_admins" => self.handle_get_role_admins(&params.arguments).await?,
            "onelogin_add_role_admins" => self.handle_add_role_admins(&params.arguments).await?,
            "onelogin_remove_role_admin" => self.handle_remove_role_admin(&params.arguments).await?,

            // Tenant Management
            "onelogin_list_tenants" => self.handle_list_tenants().await?,

            _ => return Err(anyhow!("Unknown tool: {}", params.name)),
        };

        Ok(serde_json::to_string_pretty(&result)?)
    }

    // Tool definitions
    fn tool_list_users(&self) -> Value {
        json!({
            "name": "onelogin_list_users",
            "description": "List users in OneLogin with optional filtering. Use filters to find specific users by email, username, name, role, or directory. Returns user objects with id, email, username, firstname, lastname, status, state, and more. To find a single user by email, use email filter. To find users in a role, use role_id filter.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "email": {
                        "type": "string",
                        "description": "Filter by exact email address. Use this to find a specific user by their email."
                    },
                    "username": {
                        "type": "string",
                        "description": "Filter by exact username. Use this to find a specific user by their login username."
                    },
                    "firstname": {
                        "type": "string",
                        "description": "Filter by first name (partial match supported)"
                    },
                    "lastname": {
                        "type": "string",
                        "description": "Filter by last name (partial match supported)"
                    },
                    "directory_id": {
                        "type": "integer",
                        "description": "Filter to users from a specific directory (AD/LDAP sync). Get directory IDs from onelogin_list_directory_connectors."
                    },
                    "role_id": {
                        "type": "integer",
                        "description": "Filter to users with a specific role. Get role IDs from onelogin_list_roles."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Results per page (default 100, max 200). Use with page for manual pagination."
                    },
                    "page": {
                        "type": "integer",
                        "description": "Page number starting at 1. Use with limit for manual pagination."
                    },
                    "auto_paginate": {
                        "type": "boolean",
                        "description": "Set to true to automatically fetch multiple pages. Useful for getting all users matching a filter."
                    },
                    "max_pages": {
                        "type": "integer",
                        "description": "Max pages to fetch when auto_paginate=true (default 10). Prevents runaway queries on large tenants."
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Max total users to return when auto_paginate=true. Stops pagination early once this limit is reached."
                    }
                }
            }
        })
    }

    fn tool_get_user(&self) -> Value {
        json!({
            "name": "onelogin_get_user",
            "description": "Get detailed information about a specific user by their ID. Returns full user profile including email, username, name, status, state, custom_attributes, group_id, role_ids, and directory info. Use onelogin_list_users to find user IDs first.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique numeric user ID (required). Get this from onelogin_list_users or from user events."
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_create_user(&self) -> Value {
        json!({
            "name": "onelogin_create_user",
            "description": "Create a new user in OneLogin. Supports importing users with hashed passwords using password_algorithm and salt fields.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "email": {"type": "string", "description": "User's email address (required)"},
                    "username": {"type": "string", "description": "Username (required)"},
                    "firstname": {"type": "string", "description": "First name"},
                    "lastname": {"type": "string", "description": "Last name"},
                    "title": {"type": "string", "description": "Job title"},
                    "department": {"type": "string", "description": "Department"},
                    "company": {"type": "string", "description": "Company name"},
                    "phone": {"type": "string", "description": "Phone number (E.164 format)"},
                    "comment": {"type": "string", "description": "Free text notes about the user"},
                    "password": {"type": "string", "description": "User's password (cleartext)"},
                    "password_confirmation": {"type": "string", "description": "Password confirmation (must match password)"},
                    "password_algorithm": {
                        "type": "string",
                        "enum": ["salt+sha256", "sha256+salt", "bcrypt"],
                        "description": "Hash algorithm for pre-hashed password import: 'salt+sha256' (salt prepended), 'sha256+salt' (salt appended), 'bcrypt'. Requires 'salt' field for SHA256 variants."
                    },
                    "salt": {"type": "string", "description": "Salt value used with password_algorithm"},
                    "state": {
                        "type": "integer",
                        "enum": [0, 1, 2, 3],
                        "description": "User licensing state: 0=Unapproved (pending approval), 1=Approved (licensed), 2=Rejected, 3=Unlicensed. Default is 1 (Approved)."
                    },
                    "status": {
                        "type": "integer",
                        "enum": [0, 1, 2, 3, 4, 5, 7, 8],
                        "description": "User login status: 0=Unactivated, 1=Active (can login), 2=Suspended, 3=Locked, 4=Password expired, 5=Awaiting reset, 7=Password pending, 8=Security questions required. Default is 0 (Unactivated)."
                    },
                    "directory_id": {"type": "integer", "description": "OneLogin Directory ID"},
                    "trusted_idp_id": {"type": "integer", "description": "Trusted IDP ID"},
                    "samaccountname": {"type": "string", "description": "Active Directory username"},
                    "userprincipalname": {"type": "string", "description": "User principal name"},
                    "distinguished_name": {"type": "string", "description": "Distinguished name"},
                    "external_id": {"type": "string", "description": "External directory ID"},
                    "member_of": {"type": "string", "description": "Directory membership"},
                    "openid_name": {"type": "string", "description": "OpenID sign-in name"},
                    "group_id": {"type": "integer", "description": "Group ID to assign"},
                    "role_ids": {"type": "array", "items": {"type": "integer"}, "description": "List of Role IDs to assign"},
                    "manager_ad_id": {"type": "string", "description": "Manager's Active Directory ID"},
                    "manager_user_id": {"type": "integer", "description": "Manager's OneLogin User ID"},
                    "invalid_login_attempts": {"type": "integer", "description": "Count of failed login attempts"},
                    "preferred_locale_code": {"type": "string", "description": "2-character language locale (e.g., 'en', 'es')"},
                    "custom_attributes": {"type": "object", "description": "Custom attribute key-value pairs"}
                },
                "required": ["email", "username"]
            }
        })
    }

    fn tool_update_user(&self) -> Value {
        json!({
            "name": "onelogin_update_user",
            "description": "Update an existing user. Only provide fields you want to change - omitted fields remain unchanged. Note: To update user roles, use onelogin_assign_roles or onelogin_remove_roles instead.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user to update (required). Get this from onelogin_list_users or onelogin_get_user."
                    },
                    "email": {
                        "type": "string",
                        "description": "User's email address. Must be unique within the OneLogin account."
                    },
                    "username": {
                        "type": "string",
                        "description": "User's username for login. Must be unique within the OneLogin account."
                    },
                    "firstname": {
                        "type": "string",
                        "description": "User's first name"
                    },
                    "lastname": {
                        "type": "string",
                        "description": "User's last name"
                    },
                    "title": {
                        "type": "string",
                        "description": "User's job title (e.g., 'Software Engineer', 'Sales Manager')"
                    },
                    "department": {
                        "type": "string",
                        "description": "User's department within the organization (e.g., 'Engineering', 'Human Resources')"
                    },
                    "company": {
                        "type": "string",
                        "description": "User's company or organization name"
                    },
                    "phone": {
                        "type": "string",
                        "description": "User's phone number. E.164 format recommended (e.g., '+15551234567')"
                    },
                    "status": {
                        "type": "integer",
                        "enum": [0, 1, 2, 3, 4, 5, 7, 8],
                        "description": "User login status. Values: 0=Unactivated (never logged in), 1=Active (can log in), 2=Suspended (admin disabled), 3=Locked (too many failed attempts), 4=Password expired, 5=Awaiting password reset, 7=Password pending (hasn't set password), 8=Security questions required. Only status=1 allows login."
                    },
                    "state": {
                        "type": "integer",
                        "enum": [0, 1, 2, 3],
                        "description": "User licensing state. Values: 0=Unapproved (pending admin approval), 1=Approved (licensed, normal user), 2=Rejected (denied access), 3=Unlicensed (no license assigned). Note: Changing FROM state=3 requires using onelogin_set_user_state, not this endpoint."
                    },
                    "custom_attributes": {
                        "type": "object",
                        "description": "Custom attribute key-value pairs. Keys must match custom attributes defined in OneLogin admin console. Values can be strings, numbers, or booleans.",
                        "additionalProperties": true
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_delete_user(&self) -> Value {
        json!({
            "name": "onelogin_delete_user",
            "description": "Permanently delete a user from OneLogin. WARNING: This action cannot be undone. All user data, app assignments, and audit history will be removed.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user to delete (required). Get from onelogin_list_users or onelogin_get_user. Example: 12345678"
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_get_user_apps(&self) -> Value {
        json!({
            "name": "onelogin_get_user_apps",
            "description": "Get all applications assigned to a user. Returns app details including ID, name, icon, and provisioning status.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user (required). Get from onelogin_list_users. Example: 12345678"
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_get_user_roles(&self) -> Value {
        json!({
            "name": "onelogin_get_user_roles",
            "description": "Get all role IDs assigned to a user. Returns an array of role IDs. Use onelogin_get_role to get role details.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user (required). Get from onelogin_list_users. Example: 12345678"
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_unlock_user(&self) -> Value {
        json!({
            "name": "onelogin_unlock_user",
            "description": "Unlock a user's account that was locked due to too many failed login attempts (automatic locking). NOTE: This does NOT work for users locked via onelogin_lock_user with a duration - for those, use onelogin_update_user with status=1 to unlock.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the locked user (required). Get from onelogin_list_users. Example: 12345678"
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_logout_user(&self) -> Value {
        json!({
            "name": "onelogin_logout_user",
            "description": "Force logout a user from all active sessions across all devices and applications. Use for security incidents or password resets.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user to log out (required). Get from onelogin_list_users. Example: 12345678"
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_assign_roles(&self) -> Value {
        json!({
            "name": "onelogin_assign_roles",
            "description": "Assign one or more roles to a user. ADDS to the user's existing roles (does not replace). Roles control which apps a user can access. Get role IDs from onelogin_list_roles. Note: This is API v1 endpoint (different from onelogin_assign_roles_to_user which is similar).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer", "description": "The unique ID of the user to assign roles to (required)"},
                    "role_ids": {
                        "type": "array",
                        "items": {"type": "integer"},
                        "description": "Array of role IDs to add to the user. Get IDs from onelogin_list_roles."
                    }
                },
                "required": ["user_id", "role_ids"]
            }
        })
    }

    fn tool_remove_roles(&self) -> Value {
        json!({
            "name": "onelogin_remove_roles",
            "description": "Remove one or more roles from a user. This removes the user's access to apps associated with those roles. Only removes the specified roles; other roles remain assigned.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer", "description": "The unique ID of the user to remove roles from (required)"},
                    "role_ids": {
                        "type": "array",
                        "items": {"type": "integer"},
                        "description": "Array of role IDs to remove from the user"
                    }
                },
                "required": ["user_id", "role_ids"]
            }
        })
    }

    fn tool_lock_user(&self) -> Value {
        json!({
            "name": "onelogin_lock_user",
            "description": "Lock a user's account for a specified duration. Sets status=3 (Locked) and locked_until timestamp. To unlock early, use onelogin_update_user with status=1. NOTE: Cannot lock Account Owner users.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer", "description": "The ID of the user to lock (required). Cannot be Account Owner."},
                    "locked_until": {"type": "integer", "description": "Minutes to lock the account (required). Use 0 to delegate to account policy. Example: 30 for 30 minutes."}
                },
                "required": ["user_id", "locked_until"]
            }
        })
    }

    fn tool_set_password(&self) -> Value {
        json!({
            "name": "onelogin_set_password",
            "description": "Set a user's password directly using cleartext. WARNING: The password is transmitted in cleartext. Use this for admin password resets. The user's status will be set to active (1) after password is set. For self-service password reset, use the password reset email flow instead.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer", "description": "The unique ID of the user whose password to set (required)"},
                    "password": {"type": "string", "description": "The new password in cleartext (required). Must meet password policy requirements."},
                    "password_confirmation": {"type": "string", "description": "Must exactly match the password field (required)"},
                    "validate_policy": {"type": "boolean", "description": "Set to true to validate password against the account's password policy. Default is true. Set to false to bypass policy (admin override)."}
                },
                "required": ["user_id", "password", "password_confirmation"]
            }
        })
    }

    fn tool_set_custom_attributes(&self) -> Value {
        json!({
            "name": "onelogin_set_custom_attributes",
            "description": "Set custom attributes on a user. Custom attributes must be pre-defined in OneLogin Admin > Users > Custom User Fields. This MERGES with existing attributes (does not delete unspecified ones). To see available custom attributes, use onelogin_list_custom_attributes.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer", "description": "The unique ID of the user (required)"},
                    "custom_attributes": {
                        "type": "object",
                        "description": "Key-value pairs where keys are custom attribute shortnames (as defined in OneLogin admin console). Values can be strings, numbers, or booleans depending on the attribute type.",
                        "additionalProperties": true
                    }
                },
                "required": ["user_id", "custom_attributes"]
            }
        })
    }

    // Apps API
    fn tool_list_apps(&self) -> Value {
        json!({
            "name": "onelogin_list_apps",
            "description": "List all applications (SSO-enabled services) in OneLogin. Returns app id, name, connector_id, visible status, and more. To see which users have access to an app, check role assignments (apps are assigned to roles, roles to users). To see app details including SSO configuration, use onelogin_get_app.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_app(&self) -> Value {
        json!({
            "name": "onelogin_get_app",
            "description": "Get detailed information about a specific application. Returns full app configuration including name, description, connector_id, icon_url, configuration (SSO settings), parameters, provisioning settings, and role assignments. Use onelogin_list_apps to find app IDs.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The unique numeric ID of the application (required). Get from onelogin_list_apps."
                    }
                },
                "required": ["app_id"]
            }
        })
    }

    fn tool_create_app(&self) -> Value {
        json!({
            "name": "onelogin_create_app",
            "description": "Create a new application (SSO connector instance) in OneLogin. IMPORTANT: You must first get a connector_id from onelogin_list_connectors - connectors are templates (e.g., 'SAML 2.0', 'Salesforce', 'AWS') and apps are instances of those templates. After creation, assign the app to roles using onelogin_set_role_apps to grant user access.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "connector_id": {
                        "type": "integer",
                        "description": "The connector template ID (required). Get from onelogin_list_connectors. Common connectors: SAML 2.0, OIDC, AWS, Salesforce, etc."
                    },
                    "name": {
                        "type": "string",
                        "description": "Display name for the application (required). This appears in the user portal."
                    },
                    "description": {
                        "type": "string",
                        "description": "Description of the application's purpose (optional)"
                    },
                    "visible": {
                        "type": "boolean",
                        "description": "Whether the app appears in users' portal. Set to false for backend-only apps. Default: true"
                    },
                    "configuration": {
                        "type": "object",
                        "description": "Connector-specific settings. Varies by connector type. For SAML: audience, recipient, acs_url. For OIDC: redirect_uri. Check connector documentation for required fields.",
                        "additionalProperties": true
                    }
                },
                "required": ["connector_id", "name"]
            }
        })
    }

    fn tool_update_app(&self) -> Value {
        json!({
            "name": "onelogin_update_app",
            "description": "Update an existing application's settings. Only provide fields you want to change - omitted fields remain unchanged. IMPORTANT: This updates app metadata and configuration only. To change which roles have access to this app, use onelogin_set_role_apps instead.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The unique ID of the application to update (required)"
                    },
                    "name": {
                        "type": "string",
                        "description": "New display name for the application"
                    },
                    "description": {
                        "type": "string",
                        "description": "New description for the application"
                    },
                    "visible": {
                        "type": "boolean",
                        "description": "Whether the app appears in users' portal"
                    },
                    "configuration": {
                        "type": "object",
                        "description": "Updated connector-specific configuration. Only include settings you want to change.",
                        "additionalProperties": true
                    }
                },
                "required": ["app_id"]
            }
        })
    }

    fn tool_delete_app(&self) -> Value {
        json!({
            "name": "onelogin_delete_app",
            "description": "Permanently delete an application from OneLogin. WARNING: This cannot be undone. All role assignments to this app will be removed. Users will lose access to the external service through OneLogin SSO. Consider setting visible=false to hide the app instead of deleting.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The unique ID of the application to delete (required)"
                    }
                },
                "required": ["app_id"]
            }
        })
    }

    // Roles API
    fn tool_list_roles(&self) -> Value {
        json!({
            "name": "onelogin_list_roles",
            "description": "List all roles in OneLogin. Roles group users and control access to applications. Returns basic role info (id, name). To see apps/users/admins for a role, use onelogin_get_role_apps, onelogin_get_role_users, or onelogin_get_role_admins.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_role(&self) -> Value {
        json!({
            "name": "onelogin_get_role",
            "description": "Get a specific role by ID. Returns the role's id and name. Note: To get detailed lists of apps/users/admins, use the dedicated endpoints: onelogin_get_role_apps, onelogin_get_role_users, onelogin_get_role_admins.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role to retrieve"
                    }
                },
                "required": ["role_id"]
            }
        })
    }

    fn tool_create_role(&self) -> Value {
        json!({
            "name": "onelogin_create_role",
            "description": "Create a new role with a name. IMPORTANT: This endpoint ONLY accepts 'name' - you cannot set apps, users, or admins during creation. After creating the role, use: onelogin_set_role_apps to assign apps, onelogin_assign_roles_to_user to assign users, onelogin_add_role_admins to assign admins.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "The name of the role to create (required). This is the ONLY field accepted during creation."
                    }
                },
                "required": ["name"]
            }
        })
    }

    fn tool_update_role(&self) -> Value {
        json!({
            "name": "onelogin_update_role",
            "description": "Update a role's name ONLY. IMPORTANT: This endpoint can ONLY update the 'name' field. It CANNOT modify apps, users, or admins. To manage apps on a role, use onelogin_set_role_apps. To manage users on a role, use onelogin_assign_roles_to_user. To manage admins on a role, use onelogin_add_role_admins or onelogin_remove_role_admin.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role to update (required)"
                    },
                    "name": {
                        "type": "string",
                        "description": "New name for the role. This is the ONLY field that can be updated via this endpoint."
                    }
                },
                "required": ["role_id", "name"]
            }
        })
    }

    fn tool_delete_role(&self) -> Value {
        json!({
            "name": "onelogin_delete_role",
            "description": "Permanently delete a role from OneLogin. WARNING: This removes the role from all users who have it and removes all app access granted through this role. This action cannot be undone.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role to delete"
                    }
                },
                "required": ["role_id"]
            }
        })
    }

    // Groups API
    fn tool_list_groups(&self) -> Value {
        json!({
            "name": "onelogin_list_groups",
            "description": "List all groups in OneLogin. Groups are typically synced from directory services (AD, LDAP) and used for user organization and User Mappings. IMPORTANT: Groups are different from Roles - groups organize users, while roles control app access. To assign a user to a group, use onelogin_update_user with group_id.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_group(&self) -> Value {
        json!({
            "name": "onelogin_get_group",
            "description": "Get details about a specific group by ID. Returns group name and reference ID. To see users in a group, use onelogin_list_users with appropriate filters or check user.group_id.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "group_id": {
                        "type": "integer",
                        "description": "The unique ID of the group (required). Get from onelogin_list_groups."
                    }
                },
                "required": ["group_id"]
            }
        })
    }

    fn tool_create_group(&self) -> Value {
        json!({
            "name": "onelogin_create_group",
            "description": "Create a new group in OneLogin. Groups are used to organize users and can be used in User Mappings to automatically assign roles. NOTE: For directory-synced groups (AD/LDAP), groups are typically created automatically during sync - only create groups manually for non-synced use cases.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "The display name of the group (required)"
                    },
                    "reference": {
                        "type": "string",
                        "description": "External reference ID (e.g., AD distinguished name or LDAP DN). Used for directory sync correlation."
                    }
                },
                "required": ["name"]
            }
        })
    }

    fn tool_update_group(&self) -> Value {
        json!({
            "name": "onelogin_update_group",
            "description": "Update an existing group's name or reference. Only provide fields you want to change. NOTE: Changing group names may affect User Mappings that reference this group by name.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "group_id": {
                        "type": "integer",
                        "description": "The unique ID of the group to update (required)"
                    },
                    "name": {
                        "type": "string",
                        "description": "New display name for the group"
                    },
                    "reference": {
                        "type": "string",
                        "description": "New external reference ID for directory sync correlation"
                    }
                },
                "required": ["group_id"]
            }
        })
    }

    fn tool_delete_group(&self) -> Value {
        json!({
            "name": "onelogin_delete_group",
            "description": "Delete a group from OneLogin. WARNING: Users in this group will have their group_id cleared. User Mappings referencing this group may stop working. Directory-synced groups may be recreated on next sync.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "group_id": {
                        "type": "integer",
                        "description": "The unique ID of the group to delete (required)"
                    }
                },
                "required": ["group_id"]
            }
        })
    }

    // MFA API
    fn tool_list_mfa_factors(&self) -> Value {
        json!({
            "name": "onelogin_list_mfa_factors",
            "description": "List all MFA devices/factors enrolled for a user. Returns device_id, device_type, active status, and default flag for each enrolled factor. Use this to see what MFA methods a user has set up before removing or verifying factors.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user (required). Get from onelogin_list_users."
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_enroll_mfa_factor(&self) -> Value {
        json!({
            "name": "onelogin_enroll_mfa_factor",
            "description": "Start MFA enrollment for a user. IMPORTANT: This begins a two-step process - after calling this, you must call onelogin_verify_mfa_factor with the OTP code to complete enrollment. For TOTP factors (Google Authenticator), returns a QR code URL the user must scan. For SMS/Voice, sends a code to the provided phone number.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user to enroll (required)"
                    },
                    "device_type": {
                        "type": "string",
                        "enum": ["OneLogin SMS", "OneLogin Voice", "OneLogin Email", "Google Authenticator", "OneLogin Protect", "Duo Security", "Yubico YubiKey", "RSA SecurID"],
                        "description": "MFA device type (required). SMS/Voice require phone_number. Google Authenticator returns QR code URL. OneLogin Protect uses push notifications to the mobile app."
                    },
                    "phone_number": {
                        "type": "string",
                        "description": "Required for 'OneLogin SMS' and 'OneLogin Voice'. Must be E.164 format: +[country code][number]. Example: '+15551234567'"
                    }
                },
                "required": ["user_id", "device_type"]
            }
        })
    }

    fn tool_remove_mfa_factor(&self) -> Value {
        json!({
            "name": "onelogin_remove_mfa_factor",
            "description": "Remove an MFA device from a user. WARNING: If this is the user's only MFA device and MFA is required by policy, they may be locked out. Check onelogin_list_mfa_factors first to see all enrolled devices.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user (required)"
                    },
                    "device_id": {
                        "type": "integer",
                        "description": "The device ID to remove (required). Get from onelogin_list_mfa_factors."
                    }
                },
                "required": ["user_id", "device_id"]
            }
        })
    }

    fn tool_verify_mfa_factor(&self) -> Value {
        json!({
            "name": "onelogin_verify_mfa_factor",
            "description": "Complete MFA verification by submitting an OTP code. Use this after onelogin_enroll_mfa_factor to complete enrollment, or during authentication when MFA is challenged. The state_token ties this verification to the specific authentication/enrollment session.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user (required)"
                    },
                    "state_token": {
                        "type": "string",
                        "description": "The state token from the enrollment or auth challenge response (required). This is a temporary token that expires."
                    },
                    "device_id": {
                        "type": "integer",
                        "description": "The device ID being verified (required). From onelogin_list_mfa_factors or the enrollment response."
                    },
                    "otp_code": {
                        "type": "string",
                        "description": "The 6-digit OTP code from the user's authenticator app, SMS, or email (required)"
                    }
                },
                "required": ["user_id", "state_token", "device_id", "otp_code"]
            }
        })
    }

    fn tool_enroll_mfa(&self) -> Value {
        json!({
            "name": "onelogin_enroll_mfa",
            "description": "Enroll a user in an MFA factor using the factor_id. Alternative to onelogin_enroll_mfa_factor which uses device_type. This endpoint may skip the verification step if verified=true (admin override).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user to enroll (required)"
                    },
                    "factor_id": {
                        "type": "integer",
                        "description": "The MFA factor ID to enroll (required). Get available factor IDs from account MFA settings."
                    },
                    "display_name": {
                        "type": "string",
                        "description": "Friendly name for the device shown in user portal (e.g., 'Work Phone', 'Personal YubiKey')"
                    },
                    "verified": {
                        "type": "boolean",
                        "description": "Admin override: set true to skip verification step. Use with caution - user won't prove possession of the factor. Default: false"
                    }
                },
                "required": ["user_id", "factor_id"]
            }
        })
    }

    fn tool_verify_mfa(&self) -> Value {
        json!({
            "name": "onelogin_verify_mfa",
            "description": "Verify an MFA factor using an OTP token. Simpler alternative to onelogin_verify_mfa_factor that doesn't require state_token. Use for standalone verification outside of authentication flows.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user (required)"
                    },
                    "device_id": {
                        "type": "integer",
                        "description": "The device ID to verify (required). Get from onelogin_list_mfa_factors."
                    },
                    "otp_token": {
                        "type": "string",
                        "description": "The OTP code from the user's MFA device (required)"
                    }
                },
                "required": ["user_id", "device_id", "otp_token"]
            }
        })
    }

    fn tool_remove_mfa(&self) -> Value {
        json!({
            "name": "onelogin_remove_mfa",
            "description": "Remove an MFA factor/device from a user. Same as onelogin_remove_mfa_factor. WARNING: Removing all MFA devices may lock users out if MFA is required by security policy.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user (required)"
                    },
                    "device_id": {
                        "type": "integer",
                        "description": "The device ID to remove (required). Get from onelogin_list_mfa_factors."
                    }
                },
                "required": ["user_id", "device_id"]
            }
        })
    }

    // SAML API
    fn tool_get_saml_assertion(&self) -> Value {
        json!({
            "name": "onelogin_get_saml_assertion",
            "description": "Generate a SAML assertion for programmatic SSO login. Authenticates a user and returns a base64-encoded SAML assertion for the specified SAML app. IMPORTANT: If MFA is required, this returns a state_token and MFA challenge - you must then call onelogin_verify_saml_factor to complete authentication. Use for service-to-service SSO or automated testing.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "username_or_email": {
                        "type": "string",
                        "description": "User's login identifier - either username or email address (required)"
                    },
                    "password": {
                        "type": "string",
                        "description": "User's password in cleartext (required). Handle securely."
                    },
                    "app_id": {
                        "type": "string",
                        "description": "The SAML application ID (required). Get from onelogin_list_apps - must be a SAML-enabled app."
                    },
                    "subdomain": {
                        "type": "string",
                        "description": "Your OneLogin subdomain (required). For 'company.onelogin.com', use 'company'."
                    }
                },
                "required": ["username_or_email", "password", "app_id", "subdomain"]
            }
        })
    }

    fn tool_verify_saml_factor(&self) -> Value {
        json!({
            "name": "onelogin_verify_saml_factor",
            "description": "Complete MFA verification during SAML assertion flow. MUST be called after onelogin_get_saml_assertion returns an MFA challenge (status='mfa_required'). Use the state_token and device_id from the challenge response. Returns the SAML assertion on successful verification.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The SAML application ID (required). Same app_id used in get_saml_assertion."
                    },
                    "device_id": {
                        "type": "string",
                        "description": "The MFA device ID from the challenge response (required). User may have multiple devices - use the one they want to verify with."
                    },
                    "state_token": {
                        "type": "string",
                        "description": "The state_token from the MFA challenge response (required). This is temporary and expires."
                    },
                    "otp_token": {
                        "type": "string",
                        "description": "The OTP code from the user's MFA device. Required for TOTP factors (Google Authenticator, etc). Not needed for push-based factors."
                    }
                },
                "required": ["app_id", "device_id", "state_token"]
            }
        })
    }

    fn tool_get_saml_assertion_v2(&self) -> Value {
        json!({
            "name": "onelogin_get_saml_assertion_v2",
            "description": "Generate a SAML assertion using API v2. Newer endpoint with improved response format. Same authentication flow as onelogin_get_saml_assertion - may return MFA challenge requiring onelogin_verify_saml_factor. Prefer this over v1 for new integrations.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "username_or_email": {
                        "type": "string",
                        "description": "User's login identifier (required)"
                    },
                    "password": {
                        "type": "string",
                        "description": "User's password (required)"
                    },
                    "app_id": {
                        "type": "string",
                        "description": "The SAML application ID (required)"
                    },
                    "subdomain": {
                        "type": "string",
                        "description": "Your OneLogin subdomain (required)"
                    }
                },
                "required": ["username_or_email", "password", "app_id", "subdomain"]
            }
        })
    }

    // Smart Hooks API
    fn tool_create_smart_hook(&self) -> Value {
        json!({
            "name": "onelogin_create_smart_hook",
            "description": "Create a Smart Hook for custom authentication logic. ⚠️ WARNING: Creating a hook with an invalid or broken function CAN BREAK ALL USER LOGINS. Always create with disabled=true first and test thoroughly. CRITICAL: Only ONE hook can exist per type (e.g., one 'pre-authentication' hook). Creating a second hook of the same type returns 409 Conflict. To modify an existing hook, use onelogin_update_smart_hook instead. To replace a hook, delete the existing one first with onelogin_delete_smart_hook. Use onelogin_list_smart_hooks to check if a hook already exists. Smart Hooks run JavaScript code during auth flows. The 'function' field accepts plain JavaScript - it will be auto-base64-encoded. If no function is provided, a safe passthrough function is used that allows all logins. The function MUST export a handler: 'exports.handler = async (context) => { return { success: true }; }'. For pre-auth hooks, return {success:true/false, user:{...}}. Requires Smart Hooks feature enabled on your OneLogin account.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": ["pre-authentication", "user-migration"],
                        "description": "Hook type (required): 'pre-authentication' runs before login completes (can block/allow/modify user context), 'user-migration' runs on first login to migrate users from external systems"
                    },
                    "function": {
                        "type": "string",
                        "description": "JavaScript code (optional). Auto-base64-encoded. Must export handler: exports.handler = async (context) => { return {success: true}; }. If omitted or empty, uses a SAFE passthrough function that allows all logins. ⚠️ A broken function will block ALL logins - always test with disabled=true first!"
                    },
                    "disabled": {
                        "type": "boolean",
                        "description": "Create hook in disabled state (won't run until enabled). Default: false (enabled immediately). Use disabled=true for testing before going live."
                    },
                    "runtime": {
                        "type": "string",
                        "enum": ["nodejs18.x", "nodejs16.x", "nodejs12.x"],
                        "description": "Node.js runtime version. Default: 'nodejs18.x'. Use latest unless you need specific compatibility."
                    },
                    "timeout": {
                        "type": "integer",
                        "description": "Max execution time in seconds (1-10). Default: 1. Increase for external API calls. Hooks exceeding timeout are killed."
                    },
                    "retries": {
                        "type": "integer",
                        "description": "Auto-retry count on failure (0-3). Default: 0. Useful for hooks calling unreliable external APIs."
                    },
                    "options": {
                        "type": "object",
                        "description": "Context enrichment options - adds extra data to the context object passed to your function",
                        "properties": {
                            "risk_enabled": {
                                "type": "boolean",
                                "description": "Include context.risk with score (0-100) and reasons array"
                            },
                            "location_enabled": {
                                "type": "boolean",
                                "description": "Include context.location with city, country, latitude, longitude"
                            },
                            "mfa_device_info_enabled": {
                                "type": "boolean",
                                "description": "Include context.mfa_device_info with device details"
                            }
                        }
                    },
                    "env_vars": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Names of ACCOUNT-LEVEL env vars to expose to this hook (e.g., ['API_KEY', 'SECRET']). The env vars must first be created at the account level using onelogin_create_hook_env_var. Access in your function via process.env.VAR_NAME."
                    },
                    "packages": {
                        "type": "object",
                        "description": "NPM packages available to your function. Format: {\"package\": \"version\"}. Example: {\"axios\": \"1.4.0\", \"lodash\": \"4.17.21\"}",
                        "additionalProperties": {"type": "string"}
                    }
                },
                "required": ["type"]
            }
        })
    }

    fn tool_update_smart_hook(&self) -> Value {
        json!({
            "name": "onelogin_update_smart_hook",
            "description": "Update an existing Smart Hook configuration. Only provide fields you want to change. Function code is automatically base64-encoded if provided as plain text.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {
                        "type": "string",
                        "description": "The ID of the smart hook to update (required). Get this from onelogin_list_smart_hooks."
                    },
                    "status": {
                        "type": "string",
                        "enum": ["enabled", "disabled"],
                        "description": "Hook status: 'enabled' to activate, 'disabled' to deactivate"
                    },
                    "function": {
                        "type": "string",
                        "description": "JavaScript function code. Plain text is automatically base64-encoded. Must export a handler function that receives context and returns a response object."
                    },
                    "runtime": {
                        "type": "string",
                        "enum": ["nodejs18.x", "nodejs16.x", "nodejs12.x"],
                        "description": "Node.js runtime version for execution"
                    },
                    "packages": {
                        "type": "object",
                        "description": "NPM packages available to the function. Format: {\"package-name\": \"version\"} (e.g., {\"lodash\": \"4.17.21\", \"axios\": \"1.4.0\"})",
                        "additionalProperties": {"type": "string"}
                    },
                    "env_vars": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Names of ACCOUNT-LEVEL env vars to expose to this hook. Env vars must first be created using onelogin_create_hook_env_var. Access via process.env.VAR_NAME."
                    },
                    "options": {
                        "type": "object",
                        "description": "Additional context options passed to the hook function",
                        "properties": {
                            "risk_enabled": {
                                "type": "boolean",
                                "description": "Include risk score and risk reasons in hook context"
                            },
                            "location_enabled": {
                                "type": "boolean",
                                "description": "Include geolocation data (city, country, coordinates) in hook context"
                            },
                            "mfa_device_info_enabled": {
                                "type": "boolean",
                                "description": "Include MFA device information in hook context"
                            }
                        }
                    }
                },
                "required": ["hook_id"]
            }
        })
    }

    fn tool_delete_smart_hook(&self) -> Value {
        json!({
            "name": "onelogin_delete_smart_hook",
            "description": "Permanently delete a Smart Hook. WARNING: Cannot be undone. The hook will stop running immediately. Consider using onelogin_update_smart_hook with status='disabled' instead to preserve the configuration.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {
                        "type": "string",
                        "description": "The hook ID to delete (required). Get from onelogin_list_smart_hooks."
                    }
                },
                "required": ["hook_id"]
            }
        })
    }

    fn tool_get_smart_hook(&self) -> Value {
        json!({
            "name": "onelogin_get_smart_hook",
            "description": "Get full details of a Smart Hook including its type, status, function code (base64), runtime, packages, env_vars, and options. Use to inspect configuration or before updates. Function code is returned base64-encoded.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {
                        "type": "string",
                        "description": "The hook ID to retrieve (required). Get from onelogin_list_smart_hooks."
                    }
                },
                "required": ["hook_id"]
            }
        })
    }

    fn tool_list_smart_hooks(&self) -> Value {
        json!({
            "name": "onelogin_list_smart_hooks",
            "description": "List all Smart Hooks in your OneLogin account. Returns hook IDs, types (pre-authentication/user-migration), and enabled/disabled status. Use hook_id with other Smart Hook tools. Returns empty array if no hooks configured.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_smart_hook_logs(&self) -> Value {
        json!({
            "name": "onelogin_get_smart_hook_logs",
            "description": "Get recent execution logs for a Smart Hook. Essential for debugging - shows execution time, input context, output response, and any console.log output from your code. Logs are retained for limited time. Check after authentication attempts to debug issues.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {
                        "type": "string",
                        "description": "The hook ID to get logs for (required)"
                    }
                },
                "required": ["hook_id"]
            }
        })
    }

    // ==================== HOOK ENVIRONMENT VARIABLES (Account-Level) ====================
    // Note: Env vars are ACCOUNT-LEVEL, shared by ALL hooks. Not per-hook.
    // To use an env var in a hook, declare its NAME in the hook's env_vars array.

    fn tool_list_hook_env_vars(&self) -> Value {
        json!({
            "name": "onelogin_list_hook_env_vars",
            "description": "List all environment variables defined in your OneLogin account. These are ACCOUNT-LEVEL secrets shared by ALL Smart Hooks. Returns name and ID (not values - values are write-only for security). To use an env var in a hook, declare its NAME in the hook's env_vars array.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_hook_env_var(&self) -> Value {
        json!({
            "name": "onelogin_get_hook_env_var",
            "description": "Get details of a specific environment variable by ID. Returns name and timestamps (not value - values are write-only for security).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "env_var_id": {
                        "type": "string",
                        "description": "The environment variable ID (required). Get from onelogin_list_hook_env_vars."
                    }
                },
                "required": ["env_var_id"]
            }
        })
    }

    fn tool_create_hook_env_var(&self) -> Value {
        json!({
            "name": "onelogin_create_hook_env_var",
            "description": "Create an ACCOUNT-LEVEL environment variable for Smart Hooks. The value is encrypted at rest and injected into hook functions at runtime. Access in your function via process.env.VAR_NAME. IMPORTANT: After creating, you must add the env var NAME to a hook's env_vars array for it to be available to that hook.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Variable name (required). Will be accessible as process.env.<name> in hook functions. Use UPPER_SNAKE_CASE convention (e.g., API_KEY, DATABASE_URL)."
                    },
                    "value": {
                        "type": "string",
                        "description": "Secret value (required). Will be encrypted at rest. Example: API keys, connection strings, secrets."
                    }
                },
                "required": ["name", "value"]
            }
        })
    }

    fn tool_update_hook_env_var(&self) -> Value {
        json!({
            "name": "onelogin_update_hook_env_var",
            "description": "Update the VALUE of an existing environment variable. The name CANNOT be changed - create a new one if you need a different name. Value is encrypted at rest.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "env_var_id": {
                        "type": "string",
                        "description": "The environment variable ID (required). Get from onelogin_list_hook_env_vars."
                    },
                    "value": {
                        "type": "string",
                        "description": "New secret value (required). Will replace the existing value."
                    }
                },
                "required": ["env_var_id", "value"]
            }
        })
    }

    fn tool_delete_hook_env_var(&self) -> Value {
        json!({
            "name": "onelogin_delete_hook_env_var",
            "description": "Delete an environment variable. ⚠️ WARNING: This will cause any hooks using this variable to fail if they expect it. Check hook configurations before deleting.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "env_var_id": {
                        "type": "string",
                        "description": "The environment variable ID (required). Get from onelogin_list_hook_env_vars."
                    }
                },
                "required": ["env_var_id"]
            }
        })
    }

    // Vigilance/Risk API (Adaptive MFA / Smart MFA)
    fn tool_get_risk_score(&self) -> Value {
        json!({
            "name": "onelogin_get_risk_score",
            "description": "Get real-time risk score for a login attempt. REQUIRES Adaptive MFA (Vigilance) feature enabled on your OneLogin account. Returns score 0-100 and risk reasons. Use to decide whether to require MFA for low-risk logins or block high-risk ones. Score factors: IP reputation, geolocation, device fingerprint, impossible travel.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "string",
                        "description": "The user ID to assess (required). Get from onelogin_list_users."
                    },
                    "ip": {
                        "type": "string",
                        "description": "Client IP address (required). Use the actual user's IP, not your server's IP. Example: '203.0.113.50'"
                    },
                    "user_agent": {
                        "type": "string",
                        "description": "Client's User-Agent header (required). Used for device fingerprinting and anomaly detection."
                    }
                },
                "required": ["user_id", "ip", "user_agent"]
            }
        })
    }

    fn tool_validate_user_smart_mfa(&self) -> Value {
        json!({
            "name": "onelogin_validate_user_smart_mfa",
            "description": "Evaluate risk and potentially trigger MFA for a user. REQUIRES Adaptive MFA feature. Call this during your custom login flow to check if MFA is needed based on risk. Returns mfa_required=true/false. If true, user must complete MFA. Useful for embedding OneLogin risk-based MFA into custom applications.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_identifier": {
                        "type": "string",
                        "description": "User identifier (required). Can be email, username, or numeric user_id."
                    },
                    "phone": {
                        "type": "string",
                        "description": "User's phone for SMS OTP if MFA is triggered. E.164 format: '+15551234567'"
                    },
                    "email": {
                        "type": "string",
                        "description": "User's email for email OTP if MFA is triggered"
                    },
                    "context": {
                        "type": "object",
                        "description": "Request context for risk assessment (required)",
                        "properties": {
                            "ip_address": {
                                "type": "string",
                                "description": "Client's IP address (required)"
                            },
                            "user_agent": {
                                "type": "string",
                                "description": "Client's User-Agent header (required)"
                            }
                        },
                        "required": ["ip_address", "user_agent"]
                    }
                },
                "required": ["user_identifier", "context"]
            }
        })
    }

    fn tool_list_risk_rules(&self) -> Value {
        json!({
            "name": "onelogin_list_risk_rules",
            "description": "List all risk rules (Adaptive MFA policies). Risk rules define when to require additional authentication based on conditions like IP, location, risk score, user groups. REQUIRES Adaptive MFA feature. Returns rule ID, name, enabled status, priority, conditions, and actions.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_create_risk_rule(&self) -> Value {
        json!({
            "name": "onelogin_create_risk_rule",
            "description": "Create a risk rule for conditional access/MFA. REQUIRES Adaptive MFA feature. Rules evaluate conditions and take actions (allow, mfa, deny). Rules are evaluated in priority order. Use for: geo-blocking, risk-based MFA, IP allowlists, device trust policies.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Descriptive rule name (required). Example: 'Block non-US logins' or 'MFA for new devices'"
                    },
                    "description": {
                        "type": "string",
                        "description": "Detailed description of rule purpose and behavior"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether rule is active (required). Set false to create in draft mode."
                    },
                    "conditions": {
                        "type": "array",
                        "description": "Array of condition objects (required). Conditions ANDed together. Example: [{\"type\": \"ip\", \"operator\": \"not_in\", \"values\": [\"10.0.0.0/8\"]}]"
                    },
                    "action": {
                        "type": "object",
                        "description": "Action when conditions match (required). {\"action\": \"allow\"} or {\"action\": \"mfa\"} or {\"action\": \"deny\"}"
                    },
                    "priority": {
                        "type": "integer",
                        "description": "Evaluation order (required). Lower = higher priority. First matching rule wins."
                    }
                },
                "required": ["name", "enabled", "conditions", "action", "priority"]
            }
        })
    }

    fn tool_update_risk_rule(&self) -> Value {
        json!({
            "name": "onelogin_update_risk_rule",
            "description": "Update an existing risk rule. Only provide fields to change. Use to enable/disable rules or modify conditions/actions. Changes take effect immediately for new login attempts.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "rule_id": {
                        "type": "string",
                        "description": "The rule ID to update (required). Get from onelogin_list_risk_rules."
                    },
                    "name": {
                        "type": "string",
                        "description": "New name for the rule"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Set false to disable rule without deleting"
                    }
                },
                "required": ["rule_id"]
            }
        })
    }

    fn tool_delete_risk_rule(&self) -> Value {
        json!({
            "name": "onelogin_delete_risk_rule",
            "description": "Permanently delete a risk rule. WARNING: Cannot be undone. Consider disabling (enabled=false) instead to preserve configuration. Deletion takes effect immediately.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "rule_id": {
                        "type": "string",
                        "description": "The rule ID to delete (required)"
                    }
                },
                "required": ["rule_id"]
            }
        })
    }

    fn tool_get_risk_events(&self) -> Value {
        json!({
            "name": "onelogin_get_risk_events",
            "description": "Get risk event history for a user. Shows login attempts with risk scores, locations, devices, and outcomes. Useful for security investigations and understanding user login patterns. REQUIRES Adaptive MFA feature.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "string",
                        "description": "The user ID to get events for (required)"
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_track_risk_event(&self) -> Value {
        json!({
            "name": "onelogin_track_risk_event",
            "description": "Report a custom risk event to OneLogin's risk engine. Use to feed external security signals (failed app logins, suspicious API calls, etc.) into OneLogin for holistic risk assessment. These events influence future risk scores for the user.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "string",
                        "description": "The user ID this event is for (required)"
                    },
                    "event_type": {
                        "type": "string",
                        "enum": ["suspicious_login", "failed_mfa", "impossible_travel", "unusual_activity", "password_spray", "credential_stuffing", "brute_force", "account_takeover"],
                        "description": "Event category (required). Use standard types for consistent risk scoring."
                    },
                    "risk_score": {
                        "type": "integer",
                        "minimum": 0,
                        "maximum": 100,
                        "description": "Severity 0-100 (required). 0=informational, 50=medium, 100=critical/breach"
                    },
                    "details": {
                        "type": "object",
                        "description": "Additional context for investigation. Include ip, location, reason, timestamps.",
                        "additionalProperties": true
                    }
                },
                "required": ["user_id", "event_type", "risk_score"]
            }
        })
    }

    // Privileges API (Delegated Administration)
    fn tool_list_privileges(&self) -> Value {
        json!({
            "name": "onelogin_list_privileges",
            "description": "List all privileges (delegated admin permission sets). REQUIRES Delegated Administration add-on. Privileges define what actions help desk or departmental admins can perform. Returns privilege IDs, names, and permission details.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_privilege(&self) -> Value {
        json!({
            "name": "onelogin_get_privilege",
            "description": "Get details of a specific privilege including its resource type, allowed actions, and scoping rules. Use to audit what a delegated admin can do.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "privilege_id": {
                        "type": "string",
                        "description": "The privilege ID (required). Get from onelogin_list_privileges."
                    }
                },
                "required": ["privilege_id"]
            }
        })
    }

    fn tool_create_privilege(&self) -> Value {
        json!({
            "name": "onelogin_create_privilege",
            "description": "Create a privilege for delegated administration. REQUIRES Delegated Administration add-on. Privileges are permission sets that can be assigned to users or roles. Example uses: help desk can reset passwords, department managers can manage their team's apps.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Descriptive name (required). Example: 'Help Desk - Password Reset' or 'Sales Department User Manager'"
                    },
                    "description": {
                        "type": "string",
                        "description": "Optional description of what this privilege allows"
                    },
                    "resource_type": {
                        "type": "string",
                        "enum": ["users", "apps", "roles", "groups", "policies", "mappings", "reports"],
                        "description": "What resource type this privilege controls (required)"
                    },
                    "actions": {
                        "type": "array",
                        "items": {
                            "type": "string",
                            "enum": ["read", "create", "update", "delete", "manage"]
                        },
                        "description": "Permitted operations (required). 'manage' grants all actions. Example: ['read', 'update'] for view+edit without delete."
                    },
                    "scope": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Optional scope restrictions. Defaults to '*' (all resources). Use specific resource IDs like 'apps/1234' to limit scope."
                    }
                },
                "required": ["name", "resource_type", "actions"]
            }
        })
    }

    fn tool_update_privilege(&self) -> Value {
        json!({
            "name": "onelogin_update_privilege",
            "description": "Update an existing privilege's name, actions, or scope. Changes affect all users/roles currently assigned this privilege. Only provide fields to change.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "privilege_id": {
                        "type": "string",
                        "description": "The privilege ID to update (required)"
                    }
                },
                "required": ["privilege_id"]
            }
        })
    }

    fn tool_delete_privilege(&self) -> Value {
        json!({
            "name": "onelogin_delete_privilege",
            "description": "Delete a privilege. WARNING: Users and roles assigned this privilege will immediately lose those permissions. Cannot be undone.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "privilege_id": {
                        "type": "string",
                        "description": "The privilege ID to delete (required)"
                    }
                },
                "required": ["privilege_id"]
            }
        })
    }

    fn tool_assign_privilege_to_user(&self) -> Value {
        json!({
            "name": "onelogin_assign_user_to_privilege",
            "description": "Grant a privilege directly to a user, making them a delegated admin. The user can then perform the privilege's allowed actions. For granting to multiple users, consider assigning the privilege to a role instead.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "privilege_id": {
                        "type": "string",
                        "description": "The privilege ID to assign (required)"
                    },
                    "user_id": {
                        "type": "integer",
                        "description": "The user ID who will receive this privilege (required)"
                    }
                },
                "required": ["privilege_id", "user_id"]
            }
        })
    }

    fn tool_assign_privilege_to_role(&self) -> Value {
        json!({
            "name": "onelogin_assign_role_to_privilege",
            "description": "Grant a privilege to all users in a role. ALL users with this role become delegated admins with these permissions. Preferred over user assignment when multiple people need the same privilege.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "privilege_id": {
                        "type": "string",
                        "description": "The privilege ID to assign (required)"
                    },
                    "role_id": {
                        "type": "integer",
                        "description": "The role ID whose members will receive this privilege (required)"
                    }
                },
                "required": ["privilege_id", "role_id"]
            }
        })
    }

    // User Mappings API
    fn tool_list_user_mappings(&self) -> Value {
        json!({
            "name": "onelogin_list_user_mappings",
            "description": "List all user mapping rules. User mappings automatically assign roles, set attributes, or modify users based on conditions (like email domain, AD group membership, department). Mappings run on user creation/sync and can be re-applied manually. Returns mapping IDs, names, enabled status, and position.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_user_mapping(&self) -> Value {
        json!({
            "name": "onelogin_get_user_mapping",
            "description": "Get full details of a user mapping including its conditions (what triggers it) and actions (what it does). Use to audit or debug mapping behavior.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "mapping_id": {
                        "type": "string",
                        "description": "The mapping ID (required). Get from onelogin_list_user_mappings."
                    }
                },
                "required": ["mapping_id"]
            }
        })
    }

    fn tool_create_user_mapping(&self) -> Value {
        json!({
            "name": "onelogin_create_user_mapping",
            "description": "Create a user mapping rule to automatically assign roles, set attributes, or modify users based on conditions. Example: auto-assign 'Engineering' role to users with email ending in '@eng.company.com'.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Descriptive name for the mapping rule. Example: 'Assign Engineering Role'"
                    },
                    "match_type": {
                        "type": "string",
                        "enum": ["all", "any"],
                        "description": "'all'=ALL conditions must match (AND logic), 'any'=ANY condition matches (OR logic)"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether the mapping is active. Default: true"
                    },
                    "position": {
                        "type": "integer",
                        "description": "Execution priority (lower=first). Example: 1"
                    },
                    "conditions": {
                        "type": "array",
                        "description": "Conditions that trigger this mapping",
                        "items": {
                            "type": "object",
                            "properties": {
                                "source": {
                                    "type": "string",
                                    "description": "Attribute to check. Common: 'email', 'member_of' (AD group), 'department', 'has_attribute', 'custom_attribute_*'. Use onelogin_list_mapping_conditions for all."
                                },
                                "operator": {
                                    "type": "string",
                                    "enum": ["=", "!=", "~", "!~", ">", "<", "ri"],
                                    "description": "'='=equals, '!='=not equals, '~'=contains, '!~'=not contains, '>'=starts with, '<'=ends with, 'ri'=regex"
                                },
                                "value": {
                                    "type": "string",
                                    "description": "Value to match. Example: '@eng.company.com' with operator '<' matches emails ending in that domain"
                                }
                            },
                            "required": ["source", "operator", "value"]
                        }
                    },
                    "actions": {
                        "type": "array",
                        "description": "Actions when conditions match",
                        "items": {
                            "type": "object",
                            "properties": {
                                "action": {
                                    "type": "string",
                                    "enum": ["set_status", "set_state", "set_role_ids", "set_groups", "set_userprincipalname", "set_samaccountname", "set_usertype", "set_directory"],
                                    "description": "Action type. 'set_status' (user status 0-8), 'set_state' (user state 0-3), 'set_role_ids' (assign roles), 'set_groups' (assign groups). Custom attributes use 'set_<shortname>'."
                                },
                                "value": {
                                    "type": "array",
                                    "items": {"type": "string"},
                                    "description": "Values as strings. Examples: ['1'] for status=Active, ['123456'] for role ID, ['Engineering'] for group name"
                                }
                            },
                            "required": ["action", "value"]
                        }
                    }
                },
                "required": ["name", "match_type", "conditions", "actions"]
            }
        })
    }

    fn tool_update_user_mapping(&self) -> Value {
        json!({
            "name": "onelogin_update_user_mapping",
            "description": "Update an existing user mapping rule. Only provide fields you want to change - omitted fields remain unchanged. Note: conditions and actions arrays replace existing values entirely when provided.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "mapping_id": {
                        "type": "string",
                        "description": "The ID of the user mapping to update (required). Get this from onelogin_list_user_mappings."
                    },
                    "name": {
                        "type": "string",
                        "description": "Display name for the mapping rule"
                    },
                    "match_type": {
                        "type": "string",
                        "enum": ["all", "any"],
                        "description": "How conditions are evaluated: 'all' requires ALL conditions match (AND logic), 'any' requires ANY condition matches (OR logic)"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether the mapping is active. Set to false to disable without deleting."
                    },
                    "position": {
                        "type": "integer",
                        "description": "Priority order for mapping execution. Lower numbers execute first. Use onelogin_sort_user_mappings to reorder multiple mappings."
                    },
                    "conditions": {
                        "type": "array",
                        "description": "Conditions that trigger this mapping. Replaces ALL existing conditions when provided.",
                        "items": {
                            "type": "object",
                            "properties": {
                                "source": {
                                    "type": "string",
                                    "description": "Attribute to evaluate (e.g., 'email', 'member_of', 'department', 'has_attribute'). Use onelogin_list_mapping_conditions for all options."
                                },
                                "operator": {
                                    "type": "string",
                                    "description": "Comparison operator: '=' (equals), '!=' (not equals), '~' (contains), '!~' (not contains), '>' (starts with), '<' (ends with), 'ri' (regex match)"
                                },
                                "value": {
                                    "type": "string",
                                    "description": "Value to compare against (e.g., '@company.com' for email domain matching)"
                                }
                            },
                            "required": ["source", "operator", "value"]
                        }
                    },
                    "actions": {
                        "type": "array",
                        "description": "Actions to perform when conditions match. Replaces ALL existing actions when provided.",
                        "items": {
                            "type": "object",
                            "properties": {
                                "action": {
                                    "type": "string",
                                    "enum": ["set_status", "set_state", "set_role_ids", "set_groups", "set_userprincipalname", "set_samaccountname", "set_usertype", "set_directory"],
                                    "description": "Action type. 'set_status' (0-8), 'set_state' (0-3), 'set_role_ids', 'set_groups'. Custom attributes use 'set_<shortname>'."
                                },
                                "value": {
                                    "type": "array",
                                    "items": {"type": "string"},
                                    "description": "Action values as strings. Example: ['1'] for Active status, ['123', '456'] for role IDs"
                                }
                            },
                            "required": ["action", "value"]
                        }
                    }
                },
                "required": ["mapping_id"]
            }
        })
    }

    fn tool_delete_user_mapping(&self) -> Value {
        json!({
            "name": "onelogin_delete_user_mapping",
            "description": "Delete a user mapping rule. WARNING: Users previously affected by this mapping retain their assigned roles/attributes - deletion only prevents future applications.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "mapping_id": {
                        "type": "string",
                        "description": "The ID of the mapping to delete (required). Get from onelogin_list_user_mappings."
                    }
                },
                "required": ["mapping_id"]
            }
        })
    }

    fn tool_sort_user_mappings(&self) -> Value {
        json!({
            "name": "onelogin_sort_user_mappings",
            "description": "Reorder user mapping execution priority. Mappings execute in order; first matching mapping wins. Provide mapping IDs in desired order (first=highest priority).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "mapping_ids": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Array of mapping IDs in desired execution order. Example: ['mapping_abc', 'mapping_def', 'mapping_ghi']"
                    }
                },
                "required": ["mapping_ids"]
            }
        })
    }

    fn tool_sort_mapping_order(&self) -> Value {
        json!({
            "name": "onelogin_sort_mapping_order",
            "description": "Reorder user mappings execution priority. Same as onelogin_sort_user_mappings. First matching mapping's actions are applied. Position matters when multiple mappings could match the same user.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "mapping_ids": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Complete list of mapping IDs in desired order. First = highest priority."
                    }
                },
                "required": ["mapping_ids"]
            }
        })
    }

    fn tool_list_mapping_conditions(&self) -> Value {
        json!({
            "name": "onelogin_list_mapping_conditions",
            "description": "List all available condition sources for user mappings. Returns condition types like 'email', 'member_of', 'department', 'has_attribute', and custom attribute names. Use this to discover what conditions you can use when creating mappings.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_generate_invite_link(&self) -> Value {
        json!({
            "name": "onelogin_generate_invite_link",
            "description": "Generate an invitation link for a user to activate their OneLogin account. Returns a URL that can be shared directly with the user. The invitation expires after 72 hours.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "email": {
                        "type": "string",
                        "description": "Email address of the user to invite (required). Must match an existing unactivated user in OneLogin."
                    },
                    "custom_message": {
                        "type": "string",
                        "description": "Custom message to include in the invitation. Useful for providing context or instructions to the user."
                    }
                },
                "required": ["email"]
            }
        })
    }

    fn tool_send_invite_link(&self) -> Value {
        json!({
            "name": "onelogin_send_invite_link",
            "description": "Send an invitation email to a user to activate their OneLogin account. OneLogin sends the email directly to the user with a secure activation link.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "email": {
                        "type": "string",
                        "description": "Email address of the user to invite (required). Must match an existing unactivated user in OneLogin."
                    },
                    "personal_email": {
                        "type": "string",
                        "description": "Alternative email address to send the invitation to. Useful when the user's OneLogin email is not yet accessible (e.g., corporate email requires SSO to access)."
                    },
                    "custom_message": {
                        "type": "string",
                        "description": "Custom message to include in the invitation email. Useful for providing context or instructions to the user."
                    }
                },
                "required": ["email"]
            }
        })
    }

    fn tool_list_custom_attributes(&self) -> Value {
        json!({
            "name": "onelogin_list_custom_attributes",
            "description": "List all custom attributes defined in your OneLogin account. Custom attributes allow you to store additional user data beyond standard fields. Returns attribute ID, name, shortname, data type, and visibility settings.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_create_custom_attribute(&self) -> Value {
        json!({
            "name": "onelogin_create_custom_attribute",
            "description": "Create a new custom attribute for storing additional user data. Custom attributes can be used in user mappings, provisioning rules, and application parameter mappings.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Display name for the attribute (required). Shown in the UI and reports (e.g., 'Employee ID', 'Cost Center')."
                    },
                    "shortname": {
                        "type": "string",
                        "description": "Short identifier for the attribute (required). Used in API calls and SAML assertions. Lowercase alphanumeric with underscores (e.g., 'employee_id', 'cost_center'). Cannot be changed after creation."
                    },
                    "data_type": {
                        "type": "string",
                        "enum": ["string", "integer", "boolean"],
                        "description": "Data type for the attribute (required). 'string' for text values, 'integer' for numeric values, 'boolean' for true/false values."
                    },
                    "required": {
                        "type": "boolean",
                        "description": "Whether users must have a value for this attribute. Default is false."
                    },
                    "user_visible": {
                        "type": "boolean",
                        "description": "Whether users can see this attribute in their profile. Default is true. Set to false for internal/administrative attributes."
                    }
                },
                "required": ["name", "shortname", "data_type"]
            }
        })
    }

    fn tool_update_custom_attribute(&self) -> Value {
        json!({
            "name": "onelogin_update_custom_attribute",
            "description": "Update an existing custom attribute. Only provide fields you want to change. Note: shortname and data_type cannot be changed after creation.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "attribute_id": {
                        "type": "integer",
                        "description": "The unique ID of the custom attribute to update (required). Get this from onelogin_list_custom_attributes."
                    },
                    "name": {
                        "type": "string",
                        "description": "New display name for the attribute."
                    },
                    "required": {
                        "type": "boolean",
                        "description": "Whether users must have a value for this attribute."
                    },
                    "user_visible": {
                        "type": "boolean",
                        "description": "Whether users can see this attribute in their profile."
                    }
                },
                "required": ["attribute_id"]
            }
        })
    }

    fn tool_delete_custom_attribute(&self) -> Value {
        json!({
            "name": "onelogin_delete_custom_attribute",
            "description": "Delete a custom attribute. WARNING: This permanently removes the attribute definition and all user values for this attribute. This action cannot be undone.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "attribute_id": {
                        "type": "integer",
                        "description": "The unique ID of the custom attribute to delete (required). Get this from onelogin_list_custom_attributes."
                    }
                },
                "required": ["attribute_id"]
            }
        })
    }

    fn tool_generate_embed_token(&self) -> Value {
        json!({
            "name": "onelogin_generate_embed_token",
            "description": "Generate an embed token for embedding OneLogin SSO into your application. The token allows seamless SSO without redirecting to the OneLogin portal. Used for custom login experiences.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "email": {
                        "type": "string",
                        "description": "Email address of the user to generate a token for (required). Must match an existing active user in OneLogin."
                    },
                    "session_duration": {
                        "type": "integer",
                        "description": "Session duration in minutes. Default is based on account settings. Maximum depends on account configuration."
                    },
                    "return_to_url": {
                        "type": "string",
                        "description": "URL to redirect the user to after successful authentication. Must be a valid URL within allowed domains."
                    }
                },
                "required": ["email"]
            }
        })
    }

    fn tool_list_embeddable_apps(&self) -> Value {
        json!({
            "name": "onelogin_list_embeddable_apps",
            "description": "List all applications configured for embedding. Embeddable apps can be launched via embed tokens for seamless SSO integration. Returns app ID, name, and icon URL.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_generate_oauth_tokens(&self) -> Value {
        json!({
            "name": "onelogin_generate_oauth_tokens",
            "description": "Generate OAuth 2.0 access and refresh tokens. Supports authorization_code, refresh_token, and client_credentials grant types.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "grant_type": {
                        "type": "string",
                        "enum": ["authorization_code", "refresh_token", "client_credentials"],
                        "description": "OAuth grant type (required). 'authorization_code' exchanges auth code for tokens, 'refresh_token' gets new access token using refresh token, 'client_credentials' for machine-to-machine auth."
                    },
                    "code": {
                        "type": "string",
                        "description": "Authorization code from OAuth redirect. Required when grant_type is 'authorization_code'."
                    },
                    "refresh_token": {
                        "type": "string",
                        "description": "Refresh token to exchange for new access token. Required when grant_type is 'refresh_token'."
                    },
                    "redirect_uri": {
                        "type": "string",
                        "description": "Redirect URI that was used in the authorization request. Required for 'authorization_code' grant type."
                    },
                    "scope": {
                        "type": "string",
                        "description": "Space-separated list of scopes to request (e.g., 'openid profile email'). If omitted, uses default scopes."
                    }
                },
                "required": ["grant_type"]
            }
        })
    }

    fn tool_revoke_oauth_token(&self) -> Value {
        json!({
            "name": "onelogin_revoke_oauth_token",
            "description": "Revoke an OAuth 2.0 access or refresh token. Use this to log out a user or invalidate compromised tokens.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "token": {
                        "type": "string",
                        "description": "The access token or refresh token to revoke (required)."
                    },
                    "token_type_hint": {
                        "type": "string",
                        "enum": ["access_token", "refresh_token"],
                        "description": "Hint about the type of token being revoked. Improves performance but is optional."
                    }
                },
                "required": ["token"]
            }
        })
    }

    fn tool_introspect_oauth_token(&self) -> Value {
        json!({
            "name": "onelogin_introspect_oauth_token",
            "description": "Introspect an OAuth 2.0 token to check if it's active and get metadata. Returns token validity, scopes, client_id, username, and expiration info.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "token": {
                        "type": "string",
                        "description": "The access token or refresh token to introspect (required)."
                    },
                    "token_type_hint": {
                        "type": "string",
                        "enum": ["access_token", "refresh_token"],
                        "description": "Hint about the type of token being introspected. Improves performance but is optional."
                    }
                },
                "required": ["token"]
            }
        })
    }

    fn tool_oidc_get_well_known_config(&self) -> Value {
        json!({
            "name": "onelogin_oidc_get_well_known_config",
            "description": "Get the OpenID Connect discovery document (/.well-known/openid-configuration). Returns issuer, endpoints (authorization, token, userinfo, JWKS), supported scopes, response types, grant types, and signing algorithms.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_oidc_get_jwks(&self) -> Value {
        json!({
            "name": "onelogin_oidc_get_jwks",
            "description": "Get the JSON Web Key Set (JWKS) containing public keys used to verify JWT signatures from OneLogin. Use these keys to validate ID tokens and access tokens.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_oidc_get_userinfo(&self) -> Value {
        json!({
            "name": "onelogin_oidc_get_userinfo",
            "description": "Get user information using an OIDC access token. Returns standard claims (sub, email, name) based on the scopes granted to the token.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "access_token": {
                        "type": "string",
                        "description": "A valid OAuth 2.0 access token (required). The token must have been issued with appropriate scopes (e.g., 'openid profile email')."
                    }
                },
                "required": ["access_token"]
            }
        })
    }

    fn tool_list_directory_connectors(&self) -> Value {
        json!({
            "name": "onelogin_list_directory_connectors",
            "description": "List all directory connectors configured in your OneLogin account. Directory connectors sync users from external directories like Active Directory, LDAP, or Workday.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_directory_connector(&self) -> Value {
        json!({
            "name": "onelogin_get_directory_connector",
            "description": "Get detailed information about a specific directory connector including its status, configuration, and last sync time.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "connector_id": {
                        "type": "string",
                        "description": "The unique ID of the directory connector (required). Get this from onelogin_list_directory_connectors."
                    }
                },
                "required": ["connector_id"]
            }
        })
    }

    fn tool_create_directory_connector(&self) -> Value {
        json!({
            "name": "onelogin_create_directory_connector",
            "description": "Create a new directory connector for syncing users from an external directory. Requires appropriate configuration based on connector type.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Display name for the connector (required). Use a descriptive name like 'Production AD' or 'HR Workday Sync'."
                    },
                    "connector_type": {
                        "type": "string",
                        "enum": ["active_directory", "ldap", "workday", "google", "scim"],
                        "description": "Type of directory to connect to (required). Each type requires specific configuration parameters."
                    },
                    "configuration": {
                        "type": "object",
                        "description": "Type-specific configuration (required). For AD/LDAP: host, port, base_dn, bind_user, bind_password. For cloud directories: API credentials and sync settings.",
                        "additionalProperties": true
                    }
                },
                "required": ["name", "connector_type", "configuration"]
            }
        })
    }

    fn tool_update_directory_connector(&self) -> Value {
        json!({
            "name": "onelogin_update_directory_connector",
            "description": "Update an existing directory connector. Only provide fields you want to change.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "connector_id": {
                        "type": "string",
                        "description": "The unique ID of the directory connector to update (required). Get this from onelogin_list_directory_connectors."
                    },
                    "name": {
                        "type": "string",
                        "description": "New display name for the connector."
                    },
                    "configuration": {
                        "type": "object",
                        "description": "Updated configuration parameters. Merged with existing configuration.",
                        "additionalProperties": true
                    }
                },
                "required": ["connector_id"]
            }
        })
    }

    fn tool_delete_directory_connector(&self) -> Value {
        json!({
            "name": "onelogin_delete_directory_connector",
            "description": "Delete a directory connector. WARNING: This stops all syncing from this directory. Users previously synced will remain but no longer be updated from the source directory.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "connector_id": {
                        "type": "string",
                        "description": "The unique ID of the directory connector to delete (required). Get this from onelogin_list_directory_connectors."
                    }
                },
                "required": ["connector_id"]
            }
        })
    }

    fn tool_sync_directory(&self) -> Value {
        json!({
            "name": "onelogin_sync_directory",
            "description": "Trigger an immediate directory sync for the specified connector. Syncs users, groups, and attributes from the external directory to OneLogin. Use onelogin_get_sync_status to check progress.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "connector_id": {
                        "type": "string",
                        "description": "The unique ID of the directory connector to sync (required). Get this from onelogin_list_directory_connectors."
                    }
                },
                "required": ["connector_id"]
            }
        })
    }

    fn tool_get_sync_status(&self) -> Value {
        json!({
            "name": "onelogin_get_sync_status",
            "description": "Get the status of a directory sync operation. Returns progress, users added/updated/deleted counts, and any errors encountered during sync.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "connector_id": {
                        "type": "string",
                        "description": "The unique ID of the directory connector to check (required). Get this from onelogin_list_directory_connectors."
                    }
                },
                "required": ["connector_id"]
            }
        })
    }

    fn tool_get_branding_settings(&self) -> Value {
        json!({
            "name": "onelogin_get_branding_settings",
            "description": "Get the current branding settings for your OneLogin account. Returns logo URLs, colors, custom CSS, login messages, and other branding customizations applied to the login portal.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_update_branding_settings(&self) -> Value {
        json!({
            "name": "onelogin_update_branding_settings",
            "description": "Update branding settings for your OneLogin login portal. Customize the look and feel of the login experience for your users.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "logo_url": {
                        "type": "string",
                        "description": "URL of the company logo to display on the login page. Must be HTTPS. Recommended size: 200x50 pixels."
                    },
                    "background_url": {
                        "type": "string",
                        "description": "URL of the background image for the login page. Must be HTTPS."
                    },
                    "primary_color": {
                        "type": "string",
                        "description": "Primary brand color in hex format (e.g., '#0066CC'). Used for buttons and links."
                    },
                    "secondary_color": {
                        "type": "string",
                        "description": "Secondary brand color in hex format (e.g., '#004499'). Used for hover states and accents."
                    },
                    "custom_css": {
                        "type": "string",
                        "description": "Custom CSS to apply to the login page. Use for advanced styling customizations."
                    },
                    "login_message": {
                        "type": "string",
                        "description": "Custom message displayed on the login page. Can include HTML for formatting."
                    },
                    "company_name": {
                        "type": "string",
                        "description": "Company name displayed in the browser title and login page."
                    },
                    "favicon_url": {
                        "type": "string",
                        "description": "URL of the favicon for the login page. Must be HTTPS. Recommended: .ico file, 16x16 or 32x32 pixels."
                    }
                }
            }
        })
    }

    fn tool_list_events(&self) -> Value {
        json!({
            "name": "onelogin_list_events",
            "description": "List audit events from OneLogin. Events track logins, user changes, app access, API calls, and admin actions. Common event_type_ids: 5=login success, 6=login failed, 8=app login, 13=user created, 14=user updated, 17=user deleted, 51=user provisioned, 510=API password update, 533=API user created. Use onelogin_list_event_types for the complete list.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "since": {
                        "type": "string",
                        "description": "Return events on or after this date/time. ISO 8601 format. Examples: '2024-01-01T00:00:00Z', '2024-01-01', '2024-01-15T14:30:00-05:00'"
                    },
                    "until": {
                        "type": "string",
                        "description": "Return events before this date/time. ISO 8601 format. Example: '2024-12-31T23:59:59Z'"
                    },
                    "user_id": {
                        "type": "integer",
                        "description": "Filter events for a specific user. Example: 12345678"
                    },
                    "event_type_id": {
                        "type": "integer",
                        "description": "Filter by event type. Common IDs: 5=login, 6=failed login, 8=app access, 13=user created, 14=user modified, 17=user deleted. Use onelogin_list_event_types for all IDs."
                    },
                    "client_id": {
                        "type": "string",
                        "description": "Filter by OAuth client ID. Useful for tracking API usage by specific integrations."
                    },
                    "directory_id": {
                        "type": "integer",
                        "description": "Filter events for a specific directory sync connector."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Max events to return (default 50, max 1000). Example: 100"
                    }
                }
            }
        })
    }

    fn tool_get_event(&self) -> Value {
        json!({
            "name": "onelogin_get_event",
            "description": "Get detailed information about a specific audit event. Returns full event data including user, app, IP address, timestamp, and any associated metadata.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "event_id": {
                        "type": "integer",
                        "description": "The unique ID of the event to retrieve (required). Get this from onelogin_list_events."
                    }
                },
                "required": ["event_id"]
            }
        })
    }

    fn tool_create_event(&self) -> Value {
        json!({
            "name": "onelogin_create_event",
            "description": "Create a custom audit event for tracking actions in OneLogin. Useful for logging external system integrations, custom workflows, or administrative actions.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "event_type_id": {
                        "type": "integer",
                        "description": "Event type ID (required). Common types: 13=user created, 14=user modified, 17=user deleted, 510=password updated via API. Use onelogin_list_event_types for all available types."
                    },
                    "account_id": {
                        "type": "integer",
                        "description": "OneLogin account ID (required). This is the account where the event will be logged."
                    },
                    "user_id": {
                        "type": "integer",
                        "description": "OneLogin user ID to associate with this event. Example: 12345678. Omit for system-level events."
                    },
                    "notes": {
                        "type": "string",
                        "description": "Free-text notes for audit trail. Example: 'Password reset triggered by helpdesk ticket #1234'"
                    }
                },
                "required": ["event_type_id", "account_id"]
            }
        })
    }

    fn tool_list_event_types(&self) -> Value {
        json!({
            "name": "onelogin_list_event_types",
            "description": "List all available event types with their IDs and descriptions. Returns 150+ event types including: authentication (5=login, 6=failed login), user management (13=created, 14=updated, 17=deleted), app access (8=app login, 51=provisioned), MFA (22=device registered, 40=device unlocked), API operations (510=password updated, 533=user created via API), and more. Cache results as this endpoint is rate-limited.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_list_api_authorizations(&self) -> Value {
        json!({
            "name": "onelogin_list_api_authorizations",
            "description": "List all API authorization servers configured in your OneLogin account. API authorizations define OAuth 2.0 token settings for custom APIs.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_api_authorization(&self) -> Value {
        json!({
            "name": "onelogin_get_api_authorization",
            "description": "Get detailed information about a specific API authorization server including its configuration, scopes, and token settings.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "auth_id": {
                        "type": "string",
                        "description": "The unique ID of the API authorization server (required). Get this from onelogin_list_api_authorizations."
                    }
                },
                "required": ["auth_id"]
            }
        })
    }

    fn tool_create_api_authorization(&self) -> Value {
        json!({
            "name": "onelogin_create_api_authorization",
            "description": "Create a new API authorization server for issuing OAuth 2.0 tokens to protect your custom APIs.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Display name for the API authorization server (required). Use a descriptive name like 'Internal API' or 'Partner API'."
                    },
                    "configuration": {
                        "type": "object",
                        "description": "API authorization configuration (required). Includes: audiences (array of allowed API identifiers), access_token_expiration_minutes (token lifetime), refresh_token_expiration_minutes.",
                        "additionalProperties": true
                    }
                },
                "required": ["name", "configuration"]
            }
        })
    }

    fn tool_update_api_authorization(&self) -> Value {
        json!({
            "name": "onelogin_update_api_authorization",
            "description": "Update an existing API authorization server. Only provide fields you want to change.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "auth_id": {
                        "type": "string",
                        "description": "The unique ID of the API authorization server to update (required). Get this from onelogin_list_api_authorizations."
                    },
                    "name": {
                        "type": "string",
                        "description": "New display name for the API authorization server."
                    },
                    "configuration": {
                        "type": "object",
                        "description": "Updated configuration. Can include: audiences, access_token_expiration_minutes, refresh_token_expiration_minutes.",
                        "additionalProperties": true
                    }
                },
                "required": ["auth_id"]
            }
        })
    }

    fn tool_delete_api_authorization(&self) -> Value {
        json!({
            "name": "onelogin_delete_api_authorization",
            "description": "Delete an API authorization server. WARNING: This immediately invalidates all tokens issued by this server. APIs protected by this authorization will stop accepting tokens.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "auth_id": {
                        "type": "string",
                        "description": "The unique ID of the API authorization server to delete (required). Get this from onelogin_list_api_authorizations."
                    }
                },
                "required": ["auth_id"]
            }
        })
    }

    // Tool handlers (implementations)
    async fn handle_list_users(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        debug!("Parsing list_users arguments: {}", serde_json::to_string_pretty(args).unwrap_or_default());

        let parsed_args: ListUsersArgs = serde_json::from_value(args.clone())
            .context(format!(
                "Failed to parse list_users arguments\n\
                 Raw arguments: {}\n\
                 Expected format: ListUsersArgs {{ email: Option<String>, username: Option<String>, ... }}",
                serde_json::to_string_pretty(args).unwrap_or_default()
            ))?;

        let mut base_params = UserQueryParams::default();
        base_params.limit = parsed_args.limit;
        base_params.email = parsed_args.email.clone();
        base_params.username = parsed_args.username.clone();
        base_params.firstname = parsed_args.firstname.clone();
        base_params.lastname = parsed_args.lastname.clone();
        base_params.directory_id = parsed_args.directory_id;
        base_params.role_id = parsed_args.role_id;
        base_params.page = parsed_args.page;

        debug!("Built query parameters: {:?}", base_params);

        if parsed_args.auto_paginate.unwrap_or(false) {
            info!("Auto-pagination enabled for list_users");
            let limit = base_params.limit.unwrap_or(100).clamp(1, 200);
            let limit_usize = limit as usize;
            let max_pages = parsed_args.max_pages.unwrap_or(10).max(1);
            let default_total = (limit as i64 * max_pages as i64) as usize;
            let max_results = parsed_args
                .max_results
                .map(|v| v as usize)
                .unwrap_or(default_total);

            debug!("Pagination config: limit={}, max_pages={}, max_results={}", limit, max_pages, max_results);

            let (results, pages_fetched, next_page_hint) = {
                let mut paged_params = base_params.clone();
                let mut collected: Vec<User> = Vec::new();
                let mut fetched_pages = 0;
                let mut current_page = paged_params.page.unwrap_or(1).max(1);

                loop {
                    if fetched_pages >= max_pages {
                        debug!("Reached max_pages limit ({}), stopping pagination", max_pages);
                        break (collected, fetched_pages, Some(current_page));
                    }

                    paged_params.page = Some(current_page);
                    paged_params.limit = Some(limit);

                    debug!("Fetching page {} with limit {}", current_page, limit);

                    let batch = client
                        .users
                        .list_users(Some(paged_params.clone()))
                        .await
                        .context(format!(
                            "Failed to list users (page {})\n\
                             Query Parameters:\n\
                             - limit: {:?}\n\
                             - page: {:?}\n\
                             - email: {:?}\n\
                             - username: {:?}\n\
                             - firstname: {:?}\n\
                             - lastname: {:?}\n\
                             - directory_id: {:?}\n\
                             - role_id: {:?}\n\
                             \n\
                             This error occurred while fetching users from the OneLogin API.\n\
                             Check the detailed error message above for more information.",
                            current_page,
                            paged_params.limit,
                            paged_params.page,
                            paged_params.email,
                            paged_params.username,
                            paged_params.firstname,
                            paged_params.lastname,
                            paged_params.directory_id,
                            paged_params.role_id
                        ))?;

                    let batch_len = batch.len();
                    debug!("Fetched {} users from page {}", batch_len, current_page);

                    if batch_len == 0 {
                        debug!("Empty page received, stopping pagination");
                        break (collected, fetched_pages, None);
                    }

                    fetched_pages += 1;
                    let next_page_candidate = current_page + 1;
                    let remaining_capacity = max_results.saturating_sub(collected.len());

                    if remaining_capacity == 0 {
                        debug!("Reached max_results limit ({}), stopping pagination", max_results);
                        break (collected, fetched_pages, Some(next_page_candidate));
                    }

                    if batch_len > remaining_capacity {
                        debug!("Batch size ({}) exceeds remaining capacity ({}), taking partial batch", batch_len, remaining_capacity);
                        collected.extend(batch.into_iter().take(remaining_capacity));
                        break (collected, fetched_pages, Some(next_page_candidate));
                    } else {
                        collected.extend(batch);
                    }

                    if batch_len < limit_usize {
                        debug!("Received fewer users than limit ({} < {}), assuming last page", batch_len, limit_usize);
                        break (collected, fetched_pages, None);
                    }

                    current_page = next_page_candidate;
                }
            };

            info!("Auto-pagination completed: fetched {} users across {} pages", results.len(), pages_fetched);
            return Ok(json!({
                "count": results.len(),
                "pagesFetched": pages_fetched,
                "nextPage": next_page_hint,
                "users": results
            }));
        }

        let params = if base_params == UserQueryParams::default() {
            debug!("Using default parameters (no filters)");
            None
        } else {
            debug!("Using filtered parameters: {:?}", base_params);
            Some(base_params.clone())
        };

        debug!("Calling OneLogin API to list users...");
        let users = client
            .users
            .list_users(params.clone())
            .await
            .context(format!(
                "Failed to list users from OneLogin API\n\
                 Query Parameters: {:?}\n\
                 \n\
                 This error occurred while fetching users. Check the detailed error above for:\n\
                 - The exact HTTP request that failed (method, URL)\n\
                 - The response status code and body\n\
                 - Whether it was a JSON parsing error (check 'Expected' vs 'Actual' structure)\n\
                 - Network or authentication issues",
                params
            ))?;

        info!("Successfully fetched {} users", users.len());

        let next_page = match (base_params.limit, base_params.page) {
            (Some(limit), Some(page)) if limit > 0 && users.len() as i32 == limit => Some(page + 1),
            (Some(limit), None) if limit > 0 && users.len() as i32 == limit => Some(2),
            _ => None,
        };

        Ok(json!({
            "count": users.len(),
            "nextPage": next_page,
            "users": users
        }))
    }

    async fn handle_get_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id: i64 = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let result = client.users.get_user(user_id).await;

        match result {
            Ok(user) => Ok(serde_json::to_value(user)?),
            Err(OneLoginError::NotFound(msg)) => Ok(json!({
                "status": "not_found",
                "message": msg,
            })),
            Err(e) => Err(anyhow!("Failed to get user: {}", e)),
        }
    }

    async fn handle_get_user_apps(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id: i64 = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let apps = client
            .users
            .get_user_apps(user_id)
            .await
            .map_err(|e| anyhow!("Failed to get user apps: {}", e))?;
        Ok(serde_json::to_value(apps)?)
    }

    async fn handle_get_user_roles(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id: i64 = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let roles = client
            .users
            .get_user_roles(user_id)
            .await
            .map_err(|e| anyhow!("Failed to get user roles: {}", e))?;
        Ok(serde_json::to_value(roles)?)
    }

    async fn handle_unlock_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id: i64 = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        client
            .users
            .unlock_user(user_id)
            .await
            .map_err(|e| anyhow!("Failed to unlock user: {}", e))?;
        Ok(json!({
            "status": "unlocked",
            "user_id": user_id
        }))
    }

    async fn handle_logout_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id: i64 = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        client
            .users
            .logout_user(user_id)
            .await
            .map_err(|e| anyhow!("Failed to logout user: {}", e))?;
        Ok(json!({
            "status": "logged_out",
            "user_id": user_id
        }))
    }

    async fn handle_list_apps(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let apps = client
            .apps
            .list_apps()
            .await
            .map_err(|e| anyhow!("Failed to list apps: {}", e))?;
        Ok(serde_json::to_value(apps)?)
    }

    async fn handle_create_role(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request: CreateRoleRequest =
            serde_json::from_value(args.clone()).map_err(|e| anyhow!("Invalid request: {}", e))?;
        let result = client.roles.create_role(request).await;

        match result {
            Ok(role) => Ok(serde_json::to_value(role)?),
            Err(e) => Err(anyhow!("Failed to create role: {}", e)),
        }
    }

    async fn handle_delete_role(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id: i64 = args
            .get("role_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let result = client.roles.delete_role(role_id).await;

        match result {
            Ok(_) => Ok(json!({"status": "deleted", "role_id": role_id})),
            Err(OneLoginError::NotFound(msg)) => Ok(json!({
                "status": "not_found",
                "message": msg,
            })),
            Err(e) => Err(anyhow!("Failed to delete role: {}", e)),
        }
    }

    async fn handle_get_role(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id: i64 = args
            .get("role_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let result = client.roles.get_role(role_id).await;

        match result {
            Ok(role) => Ok(serde_json::to_value(role)?),
            Err(OneLoginError::NotFound(msg)) => Ok(json!({
                "status": "not_found",
                "message": msg,
            })),
            Err(e) => Err(anyhow!("Failed to get role: {}", e)),
        }
    }

    async fn handle_update_role(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id: i64 = args
            .get("role_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let request = crate::models::roles::UpdateRoleRequest {
            name: args
                .get("name")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            description: args
                .get("description")
                .and_then(|v| v.as_str().map(|s| s.to_string())),
        };
        let result = client.roles.update_role(role_id, request).await;

        match result {
            Ok(role) => Ok(serde_json::to_value(role)?),
            Err(OneLoginError::NotFound(msg)) => Ok(json!({
                "status": "not_found",
                "message": msg,
            })),
            Err(e) => Err(anyhow!("Failed to update role: {}", e)),
        }
    }

    async fn handle_list_roles(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let roles = client
            .roles
            .list_roles()
            .await
            .map_err(|e| anyhow!("Failed to list roles: {}", e))?;
        Ok(serde_json::to_value(roles)?)
    }

    async fn handle_list_groups(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let groups = client
            .groups
            .list_groups()
            .await
            .map_err(|e| anyhow!("Failed to list groups: {}", e))?;
        Ok(serde_json::to_value(groups)?)
    }

    async fn handle_create_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request =
            serde_json::from_value(args.clone()).map_err(|e| anyhow!("Invalid request: {}", e))?;
        let user = client
            .users
            .create_user(request)
            .await
            .map_err(|e| anyhow!("Failed to create user: {}", e))?;
        Ok(serde_json::to_value(user)?)
    }

    async fn handle_update_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id: i64 = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let request =
            serde_json::from_value(args.clone()).map_err(|e| anyhow!("Invalid request: {}", e))?;
        let result = client.users.update_user(user_id, request).await;

        match result {
            Ok(user) => Ok(serde_json::to_value(user)?),
            Err(OneLoginError::NotFound(msg)) => Ok(json!({
                "status": "not_found",
                "message": msg,
            })),
            Err(e) => Err(anyhow!("Failed to update user: {}", e)),
        }
    }

    async fn handle_delete_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id: i64 = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let result = client.users.delete_user(user_id).await;

        match result {
            Ok(_) => Ok(json!({"status": "deleted", "user_id": user_id})),
            Err(OneLoginError::NotFound(msg)) => Ok(json!({
                "status": "not_found",
                "message": msg,
            })),
            Err(e) => Err(anyhow!("Failed to delete user: {}", e)),
        }
    }

    async fn handle_list_privileges(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let privileges = client
            .privileges
            .list_privileges()
            .await
            .map_err(|e| {
                if e.to_string().contains("Permission denied") || e.to_string().contains("403") {
                    anyhow!(
                        "Privileges API access denied. This feature requires the 'Manage All' permission or the Delegated Administration add-on. \
                        See: https://developers.onelogin.com/api-docs/2/privileges/overview"
                    )
                } else {
                    anyhow!("Failed to list privileges: {}", e)
                }
            })?;
        Ok(serde_json::to_value(privileges)?)
    }


    async fn handle_list_events(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let params: Option<EventQueryParams> =
            if args.as_object().map(|o| !o.is_empty()).unwrap_or(false) {
                Some(
                    serde_json::from_value(args.clone())
                        .map_err(|e| anyhow!("Invalid event query: {}", e))?,
                )
            } else {
                None
            };

        let events = client
            .events
            .list_events(params)
            .await
            .map_err(|e| anyhow!("Failed to list events: {}", e))?;
        Ok(serde_json::to_value(events)?)
    }

    async fn handle_list_custom_attributes(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let attributes = client
            .custom_attributes
            .list_custom_attributes()
            .await
            .map_err(|e| anyhow!("Failed to list custom attributes: {}", e))?;
        Ok(serde_json::to_value(attributes)?)
    }

    async fn handle_list_directory_connectors(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let connectors = client
            .connectors
            .list_connectors()
            .await
            .map_err(|e| anyhow!("Failed to list connectors: {}", e))?;
        Ok(serde_json::to_value(connectors)?)
    }

    async fn handle_get_branding_settings(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let branding = client
            .branding
            .get_branding_settings()
            .await
            .map_err(|e| anyhow!("Failed to get branding settings: {}", e))?;
        Ok(serde_json::to_value(branding)?)
    }

    async fn handle_oidc_get_well_known_config(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let config = client
            .oidc
            .get_well_known_configuration()
            .await
            .map_err(|e| anyhow!("Failed to get OIDC configuration: {}", e))?;
        Ok(serde_json::to_value(config)?)
    }

    async fn handle_oidc_get_jwks(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let jwks = client
            .oidc
            .get_jwks()
            .await
            .map_err(|e| anyhow!("Failed to get OIDC JWKS: {}", e))?;
        Ok(serde_json::to_value(jwks)?)
    }

    async fn handle_create_smart_hook(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let mut request: crate::models::smart_hooks::CreateHookRequest =
            serde_json::from_value(args.clone()).map_err(|e| anyhow!("Invalid request: {}", e))?;

        // Default minimal function if none provided
        const DEFAULT_FUNCTION: &str = r#"exports.handler = async (context) => {
  return {
    success: true,
    user: {
      policy_id: context.user.policy_id
    }
  }
}"#;

        // Use default function if empty or not provided
        if request.function.is_empty() {
            request.function = DEFAULT_FUNCTION.to_string();
        }

        // Base64 encode the function - OneLogin API requires base64 encoded JavaScript
        // Check if it's already base64 encoded by attempting to decode
        let is_already_base64 = base64_decode(&request.function).is_ok()
            && !request.function.contains("exports.handler")
            && !request.function.contains("function");

        if !is_already_base64 {
            request.function = base64_encode(&request.function);
        }

        if request.disabled.is_none() {
            request.disabled = Some(false);
        }
        if request.runtime.is_none() {
            request.runtime = Some("nodejs18.x".to_string());
        }
        if request.timeout.is_none() {
            request.timeout = Some(1);
        }
        if request.retries.is_none() {
            request.retries = Some(0);
        }
        if request.packages.is_none() {
            request.packages = Some(std::collections::HashMap::new());
        }
        if request.env_vars.is_none() {
            request.env_vars = Some(Vec::new());
        }

        let hook = client
            .smart_hooks
            .create_hook(request)
            .await
            .map_err(|e| anyhow!("Failed to create smart hook: {}", e))?;
        Ok(serde_json::to_value(hook)?)
    }

    async fn handle_update_smart_hook(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let hook_id = args
            .get("hook_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("hook_id is required"))?;

        // OneLogin API requires ALL fields for update, so fetch current hook first
        let current_hook = client
            .smart_hooks
            .get_hook(hook_id)
            .await
            .map_err(|e| anyhow!("Failed to get current hook: {}", e))?;

        // Handle 'status' field mapping to 'disabled' boolean
        let disabled = if let Some(status) = args.get("status").and_then(|v| v.as_str()) {
            match status {
                "disabled" => true,
                "enabled" => false,
                _ => current_hook.disabled.unwrap_or(false),
            }
        } else if let Some(disabled_val) = args.get("disabled").and_then(|v| v.as_bool()) {
            disabled_val
        } else {
            current_hook.disabled.unwrap_or(false)
        };

        // Get function, base64 encode if needed
        let function = if let Some(func) = args.get("function").and_then(|v| v.as_str()) {
            if func.is_empty() {
                current_hook.function.clone().unwrap_or_default()
            } else {
                let is_already_base64 = base64_decode(func).is_ok()
                    && !func.contains("exports.handler")
                    && !func.contains("function");
                if is_already_base64 {
                    func.to_string()
                } else {
                    base64_encode(func)
                }
            }
        } else {
            current_hook.function.clone().unwrap_or_default()
        };

        // Build full update request with all required fields
        let request = crate::models::smart_hooks::FullUpdateHookRequest {
            hook_type: current_hook.hook_type.clone(),
            function,
            disabled,
            runtime: args.get("runtime").and_then(|v| v.as_str()).map(|s| s.to_string())
                .unwrap_or(current_hook.runtime.clone()),
            timeout: args.get("timeout").and_then(|v| value_as_i64(v)).map(|n| n as i32)
                .unwrap_or(current_hook.timeout.unwrap_or(1)),
            retries: args.get("retries").and_then(|v| value_as_i64(v)).map(|n| n as i32)
                .unwrap_or(current_hook.retries.unwrap_or(0)),
            packages: args.get("packages").and_then(|v| serde_json::from_value(v.clone()).ok())
                .or_else(|| current_hook.packages.clone())
                .unwrap_or_default(),
            env_vars: args.get("env_vars").and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_else(|| {
                    current_hook.env_vars.as_ref()
                        .map(|vars| vars.iter().map(|v| v.name.clone()).collect())
                        .unwrap_or_default()
                }),
            options: args.get("options").and_then(|v| serde_json::from_value(v.clone()).ok())
                .or(current_hook.options.clone()),
        };

        let hook = client
            .smart_hooks
            .update_hook_full(hook_id, request)
            .await
            .map_err(|e| anyhow!("Failed to update smart hook: {}", e))?;
        Ok(serde_json::to_value(hook)?)
    }

    async fn handle_list_smart_hooks(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let hooks = client
            .smart_hooks
            .list_hooks()
            .await
            .map_err(|e| anyhow!("Failed to list smart hooks: {}", e))?;
        Ok(serde_json::to_value(hooks)?)
    }

    async fn handle_get_risk_score(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let ip = args
            .get("ip")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("ip is required"))?;
        let user_agent = args
            .get("user_agent")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("user_agent is required"))?;
        let score = client
            .vigilance
            .get_risk_score(user_id, ip, user_agent)
            .await
            .map_err(|e| anyhow!("Failed to get risk score: {}", e))?;
        Ok(serde_json::to_value(score)?)
    }

    async fn handle_validate_user_smart_mfa(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request =
            serde_json::from_value(args.clone()).map_err(|e| anyhow!("Invalid request: {}", e))?;
        let result = client
            .vigilance
            .validate_user(request)
            .await
            .map_err(|e| anyhow!("Failed to validate user: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    // ==================== GROUP HANDLERS ====================

    async fn handle_get_group(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let group_id: i64 = args
            .get("group_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("group_id is required"))?;
        let group = client
            .groups
            .get_group(group_id)
            .await
            .map_err(|e| anyhow!("Failed to get group: {}", e))?;
        Ok(serde_json::to_value(group)?)
    }

    async fn handle_create_group(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let group = client
            .groups
            .create_group(request)
            .await
            .map_err(|e| anyhow!("Failed to create group: {}", e))?;
        Ok(serde_json::to_value(group)?)
    }

    async fn handle_update_group(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let group_id: i64 = args
            .get("group_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("group_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let group = client
            .groups
            .update_group(group_id, request)
            .await
            .map_err(|e| anyhow!("Failed to update group: {}", e))?;
        Ok(serde_json::to_value(group)?)
    }

    async fn handle_delete_group(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let group_id: i64 = args
            .get("group_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("group_id is required"))?;
        client
            .groups
            .delete_group(group_id)
            .await
            .map_err(|e| anyhow!("Failed to delete group: {}", e))?;
        Ok(json!({"success": true, "message": "Group deleted successfully"}))
    }

    // ==================== APP HANDLERS ====================

    async fn handle_get_app(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id: i64 = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let app = client
            .apps
            .get_app(app_id)
            .await
            .map_err(|e| anyhow!("Failed to get app: {}", e))?;
        Ok(serde_json::to_value(app)?)
    }

    async fn handle_create_app(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let app = client
            .apps
            .create_app(request)
            .await
            .map_err(|e| anyhow!("Failed to create app: {}", e))?;
        Ok(serde_json::to_value(app)?)
    }

    async fn handle_update_app(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id: i64 = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let app = client
            .apps
            .update_app(app_id, request)
            .await
            .map_err(|e| anyhow!("Failed to update app: {}", e))?;
        Ok(serde_json::to_value(app)?)
    }

    async fn handle_delete_app(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id: i64 = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        client
            .apps
            .delete_app(app_id)
            .await
            .map_err(|e| anyhow!("Failed to delete app: {}", e))?;
        Ok(json!({"success": true, "message": "App deleted successfully"}))
    }

    // ==================== USER OPERATIONS ====================

    async fn handle_assign_roles(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let role_ids: Vec<i64> = args
            .get("role_ids")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| value_as_i64(v)).collect())
            .ok_or_else(|| anyhow!("role_ids array is required"))?;
        let request = crate::models::users::AssignRolesRequest {
            role_id_array: role_ids,
        };
        client
            .users
            .assign_roles(user_id, request)
            .await
            .map_err(|e| anyhow!("Failed to assign roles: {}", e))?;
        Ok(json!({"success": true, "message": "Roles assigned successfully"}))
    }

    async fn handle_remove_roles(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let role_ids: Vec<i64> = args
            .get("role_ids")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| value_as_i64(v)).collect())
            .ok_or_else(|| anyhow!("role_ids array is required"))?;
        let request = crate::models::users::RemoveRolesRequest {
            role_id_array: role_ids,
        };
        client
            .users
            .remove_roles(user_id, request)
            .await
            .map_err(|e| anyhow!("Failed to remove roles: {}", e))?;
        Ok(json!({"success": true, "message": "Roles removed successfully"}))
    }

    async fn handle_lock_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        client
            .users
            .lock_user(user_id, request)
            .await
            .map_err(|e| anyhow!("Failed to lock user: {}", e))?;
        Ok(json!({"success": true, "message": "User locked successfully"}))
    }

    async fn handle_set_password(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        client
            .users
            .set_password_clear_text(user_id, request)
            .await
            .map_err(|e| anyhow!("Failed to set password: {}", e))?;
        Ok(json!({"success": true, "message": "Password set successfully"}))
    }

    async fn handle_set_custom_attributes(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        client
            .users
            .set_custom_attributes(user_id, request)
            .await
            .map_err(|e| anyhow!("Failed to set custom attributes: {}", e))?;
        Ok(json!({"success": true, "message": "Custom attributes set successfully"}))
    }

    // ==================== PRIVILEGE OPERATIONS ====================
    // Note: Privileges API requires 'Manage All' permission or Delegated Administration add-on

    fn privileges_access_denied_error() -> anyhow::Error {
        anyhow!(
            "Privileges API access denied. This feature requires the 'Manage All' permission or the Delegated Administration add-on. \
            See: https://developers.onelogin.com/api-docs/2/privileges/overview"
        )
    }

    fn handle_privilege_error(e: crate::core::error::OneLoginError, action: &str) -> anyhow::Error {
        if e.to_string().contains("Permission denied") || e.to_string().contains("403") {
            Self::privileges_access_denied_error()
        } else {
            anyhow!("Failed to {}: {}", action, e)
        }
    }

    async fn handle_get_privilege(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let privilege_id = args
            .get("privilege_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("privilege_id is required"))?;
        let privilege = client
            .privileges
            .get_privilege(privilege_id)
            .await
            .map_err(|e| Self::handle_privilege_error(e, "get privilege"))?;
        Ok(serde_json::to_value(privilege)?)
    }

    async fn handle_create_privilege(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        use crate::models::privileges::{CreatePrivilegeRequest, PrivilegeStatement, StatementItem};

        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("name is required"))?
            .to_string();

        let description = args
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Build the privilege statement from simplified inputs
        let resource_type = args
            .get("resource_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("resource_type is required"))?;

        let actions: Vec<String> = args
            .get("actions")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .ok_or_else(|| anyhow!("actions is required"))?;

        // Convert simplified actions to OneLogin format (e.g., "read" -> "users:List")
        let formatted_actions: Vec<String> = actions
            .iter()
            .map(|action| {
                match action.as_str() {
                    "read" => format!("{}:List", resource_type),
                    "create" => format!("{}:Create", resource_type),
                    "update" => format!("{}:Update", resource_type),
                    "delete" => format!("{}:Delete", resource_type),
                    "manage" => "*".to_string(),
                    // If it's already in full format (e.g., "users:List"), use as-is
                    other => other.to_string(),
                }
            })
            .collect();

        // Get scope or default to all resources
        let scope_value = args
            .get("scope")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_else(|| vec!["*".to_string()]);

        let request = CreatePrivilegeRequest {
            name,
            description,
            privilege: PrivilegeStatement {
                version: "2018-05-18".to_string(),
                statement: vec![StatementItem {
                    effect: "Allow".to_string(),
                    action: formatted_actions,
                    scope: scope_value,
                }],
            },
        };

        let privilege = client
            .privileges
            .create_privilege(request)
            .await
            .map_err(|e| Self::handle_privilege_error(e, "create privilege"))?;
        Ok(serde_json::to_value(privilege)?)
    }

    async fn handle_update_privilege(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        use crate::models::privileges::UpdatePrivilegeRequest;

        let privilege_id = args
            .get("privilege_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("privilege_id is required"))?;

        // Get the current privilege to preserve the privilege statement (required by API)
        let current = client
            .privileges
            .get_privilege(privilege_id)
            .await
            .map_err(|e| Self::handle_privilege_error(e, "get privilege for update"))?;

        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let description = args
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Update requires the full privilege block, so we use the current one
        let request = UpdatePrivilegeRequest {
            name,
            description,
            privilege: Some(current.privilege),
        };

        let privilege = client
            .privileges
            .update_privilege(privilege_id, request)
            .await
            .map_err(|e| Self::handle_privilege_error(e, "update privilege"))?;
        Ok(serde_json::to_value(privilege)?)
    }

    async fn handle_delete_privilege(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let privilege_id = args
            .get("privilege_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("privilege_id is required"))?;
        client
            .privileges
            .delete_privilege(privilege_id)
            .await
            .map_err(|e| Self::handle_privilege_error(e, "delete privilege"))?;
        Ok(json!({"success": true, "message": "Privilege deleted successfully"}))
    }

    async fn handle_assign_user_to_privilege(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let privilege_id = args
            .get("privilege_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("privilege_id is required"))?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        client
            .privileges
            .assign_to_user(privilege_id, user_id)
            .await
            .map_err(|e| Self::handle_privilege_error(e, "assign user to privilege"))?;
        Ok(json!({"success": true, "message": "User assigned to privilege successfully"}))
    }

    async fn handle_assign_role_to_privilege(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let privilege_id = args
            .get("privilege_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("privilege_id is required"))?;
        let role_id = args
            .get("role_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        client
            .privileges
            .assign_to_role(privilege_id, role_id)
            .await
            .map_err(|e| Self::handle_privilege_error(e, "assign role to privilege"))?;
        Ok(json!({"success": true, "message": "Role assigned to privilege successfully"}))
    }

    // ==================== MFA OPERATIONS ====================

    async fn handle_list_mfa_factors(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let factors = client
            .mfa
            .list_factors(user_id)
            .await
            .map_err(|e| anyhow!("Failed to list MFA factors: {}", e))?;
        Ok(serde_json::to_value(factors)?)
    }

    async fn handle_enroll_mfa(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let factor_id = args
            .get("factor_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("factor_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let enrollment = client
            .mfa
            .enroll_factor(user_id, factor_id, request)
            .await
            .map_err(|e| anyhow!("Failed to enroll MFA factor: {}", e))?;
        Ok(serde_json::to_value(enrollment)?)
    }

    async fn handle_verify_mfa(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let device_id = args
            .get("device_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("device_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let result = client
            .mfa
            .verify_factor(user_id, device_id, request)
            .await
            .map_err(|e| anyhow!("Failed to verify MFA factor: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_remove_mfa(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let device_id = args
            .get("device_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("device_id is required"))?;
        client
            .mfa
            .remove_factor(user_id, device_id)
            .await
            .map_err(|e| anyhow!("Failed to remove MFA factor: {}", e))?;
        Ok(json!({"success": true, "message": "MFA factor removed successfully"}))
    }

    // ==================== SMART HOOKS OPERATIONS ====================

    async fn handle_get_smart_hook(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let hook_id = args
            .get("hook_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("hook_id is required"))?;
        let hook = client
            .smart_hooks
            .get_hook(hook_id)
            .await
            .map_err(|e| anyhow!("Failed to get smart hook: {}", e))?;
        Ok(serde_json::to_value(hook)?)
    }

    async fn handle_delete_smart_hook(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let hook_id = args
            .get("hook_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("hook_id is required"))?;
        client
            .smart_hooks
            .delete_hook(hook_id)
            .await
            .map_err(|e| anyhow!("Failed to delete smart hook: {}", e))?;
        Ok(json!({"success": true, "message": "Smart hook deleted successfully"}))
    }

    // ==================== HOOK ENVIRONMENT VARIABLES (Account-Level) ====================

    async fn handle_list_hook_env_vars(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let env_vars = client
            .smart_hooks
            .list_env_vars()
            .await
            .map_err(|e| anyhow!("Failed to list environment variables: {}", e))?;
        Ok(serde_json::to_value(env_vars)?)
    }

    async fn handle_get_hook_env_var(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let env_var_id = args
            .get("env_var_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("env_var_id is required"))?;
        let env_var = client
            .smart_hooks
            .get_env_var(env_var_id)
            .await
            .map_err(|e| anyhow!("Failed to get environment variable: {}", e))?;
        Ok(serde_json::to_value(env_var)?)
    }

    async fn handle_create_hook_env_var(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("name is required"))?;
        let value = args
            .get("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("value is required"))?;
        let request = crate::models::smart_hooks::CreateEnvVarRequest {
            name: name.to_string(),
            value: value.to_string(),
        };
        let env_var = client
            .smart_hooks
            .create_env_var(request)
            .await
            .map_err(|e| anyhow!("Failed to create environment variable: {}", e))?;
        Ok(serde_json::to_value(env_var)?)
    }

    async fn handle_update_hook_env_var(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let env_var_id = args
            .get("env_var_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("env_var_id is required"))?;
        let value = args
            .get("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("value is required"))?;
        let request = crate::models::smart_hooks::UpdateEnvVarRequest {
            value: value.to_string(),
        };
        let env_var = client
            .smart_hooks
            .update_env_var(env_var_id, request)
            .await
            .map_err(|e| anyhow!("Failed to update environment variable: {}", e))?;
        Ok(serde_json::to_value(env_var)?)
    }

    async fn handle_delete_hook_env_var(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let env_var_id = args
            .get("env_var_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("env_var_id is required"))?;
        client
            .smart_hooks
            .delete_env_var(env_var_id)
            .await
            .map_err(|e| anyhow!("Failed to delete environment variable: {}", e))?;
        Ok(json!({"success": true, "message": "Environment variable deleted successfully"}))
    }

    async fn handle_get_smart_hook_logs(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let hook_id = args
            .get("hook_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("hook_id is required"))?;
        let logs = client
            .smart_hooks
            .get_hook_logs(hook_id)
            .await
            .map_err(|e| anyhow!("Failed to get smart hook logs: {}", e))?;
        Ok(serde_json::to_value(logs)?)
    }

    // ==================== SAML OPERATIONS ====================

    async fn handle_get_saml_assertion(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let response = client
            .saml
            .get_saml_assertion(request)
            .await
            .map_err(|e| anyhow!("Failed to get SAML assertion: {}", e))?;
        Ok(serde_json::to_value(response)?)
    }

    async fn handle_verify_saml_factor(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let response = client
            .saml
            .verify_saml_factor(request)
            .await
            .map_err(|e| anyhow!("Failed to verify SAML factor: {}", e))?;
        Ok(serde_json::to_value(response)?)
    }

    // ==================== EVENTS OPERATIONS ====================

    async fn handle_get_event(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let event_id = args
            .get("event_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("event_id is required"))?;
        let event = client
            .events
            .get_event(event_id)
            .await
            .map_err(|e| anyhow!("Failed to get event: {}", e))?;
        Ok(serde_json::to_value(event)?)
    }

    async fn handle_create_event(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        client
            .events
            .create_event(request)
            .await
            .map_err(|e| anyhow!("Failed to create event: {}", e))?;
        Ok(serde_json::json!({
            "message": "Event created successfully",
            "success": true
        }))
    }

    async fn handle_list_event_types(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let event_types = client
            .events
            .list_event_types()
            .await
            .map_err(|e| anyhow!("Failed to list event types: {}", e))?;
        Ok(serde_json::to_value(event_types)?)
    }

    // ==================== USER MAPPINGS OPERATIONS ====================

    async fn handle_get_user_mapping(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let mapping_id = args
            .get("mapping_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("mapping_id is required"))?;
        let mapping = client
            .user_mappings
            .get_mapping(mapping_id)
            .await
            .map_err(|e| anyhow!("Failed to get user mapping: {}", e))?;
        Ok(serde_json::to_value(mapping)?)
    }

    async fn handle_create_user_mapping(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let mapping = client
            .user_mappings
            .create_mapping(request)
            .await
            .map_err(|e| anyhow!("Failed to create user mapping: {}", e))?;
        Ok(serde_json::to_value(mapping)?)
    }

    async fn handle_update_user_mapping(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let mapping_id = args
            .get("mapping_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("mapping_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let mapping = client
            .user_mappings
            .update_mapping(mapping_id, request)
            .await
            .map_err(|e| anyhow!("Failed to update user mapping: {}", e))?;
        Ok(serde_json::to_value(mapping)?)
    }

    async fn handle_delete_user_mapping(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let mapping_id = args
            .get("mapping_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("mapping_id is required"))?;
        client
            .user_mappings
            .delete_mapping(mapping_id)
            .await
            .map_err(|e| anyhow!("Failed to delete user mapping: {}", e))?;
        Ok(json!({"success": true, "message": "User mapping deleted successfully"}))
    }

    async fn handle_sort_mapping_order(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        client
            .user_mappings
            .sort_mapping_order(request)
            .await
            .map_err(|e| anyhow!("Failed to sort mapping order: {}", e))?;
        Ok(json!({"success": true, "message": "Mapping order updated successfully"}))
    }

    async fn handle_list_mapping_conditions(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let conditions = client
            .user_mappings
            .list_conditions()
            .await
            .map_err(|e| anyhow!("Failed to list mapping conditions: {}", e))?;
        Ok(serde_json::to_value(conditions)?)
    }

    // ==================== CUSTOM ATTRIBUTES OPERATIONS ====================

    async fn handle_create_custom_attribute(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let attribute = client
            .custom_attributes
            .create_custom_attribute(request)
            .await
            .map_err(|e| anyhow!("Failed to create custom attribute: {}", e))?;
        Ok(serde_json::to_value(attribute)?)
    }

    async fn handle_update_custom_attribute(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let attribute_id = args
            .get("attribute_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("attribute_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let attribute = client
            .custom_attributes
            .update_custom_attribute(attribute_id, request)
            .await
            .map_err(|e| anyhow!("Failed to update custom attribute: {}", e))?;
        Ok(serde_json::to_value(attribute)?)
    }

    async fn handle_delete_custom_attribute(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let attribute_id = args
            .get("attribute_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("attribute_id is required"))?;
        client
            .custom_attributes
            .delete_custom_attribute(attribute_id)
            .await
            .map_err(|e| anyhow!("Failed to delete custom attribute: {}", e))?;
        Ok(json!({"success": true, "message": "Custom attribute deleted successfully"}))
    }

    // ==================== OAUTH OPERATIONS ====================

    async fn handle_generate_oauth_tokens(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let tokens = client
            .oauth
            .generate_tokens(request)
            .await
            .map_err(|e| anyhow!("Failed to generate OAuth tokens: {}", e))?;
        Ok(serde_json::to_value(tokens)?)
    }

    async fn handle_revoke_oauth_token(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        client
            .oauth
            .revoke_token(request)
            .await
            .map_err(|e| anyhow!("Failed to revoke OAuth token: {}", e))?;
        Ok(json!({"success": true, "message": "OAuth token revoked successfully"}))
    }

    async fn handle_introspect_oauth_token(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let introspection = client
            .oauth
            .introspect_token(request)
            .await
            .map_err(|e| anyhow!("Failed to introspect OAuth token: {}", e))?;
        Ok(serde_json::to_value(introspection)?)
    }

    // ==================== EMBED TOKENS OPERATIONS ====================

    async fn handle_generate_embed_token(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let token = client
            .embed_tokens
            .generate_embed_token(request)
            .await
            .map_err(|e| anyhow!("Failed to generate embed token: {}", e))?;
        Ok(serde_json::to_value(token)?)
    }

    async fn handle_list_embeddable_apps(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let apps = client
            .embed_tokens
            .list_embeddable_apps()
            .await
            .map_err(|e| anyhow!("Failed to list embeddable apps: {}", e))?;
        Ok(serde_json::to_value(apps)?)
    }

    // ==================== API AUTH OPERATIONS ====================

    async fn handle_list_api_authorizations(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let authorizations = client
            .api_auth
            .list_api_authorizations()
            .await
            .map_err(|e| anyhow!("Failed to list API authorizations: {}", e))?;
        Ok(serde_json::to_value(authorizations)?)
    }

    async fn handle_get_api_authorization(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let auth_id = args
            .get("auth_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("auth_id is required"))?;
        let authorization = client
            .api_auth
            .get_api_authorization(auth_id)
            .await
            .map_err(|e| anyhow!("Failed to get API authorization: {}", e))?;
        Ok(serde_json::to_value(authorization)?)
    }

    async fn handle_create_api_authorization(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let authorization = client
            .api_auth
            .create_api_authorization(request)
            .await
            .map_err(|e| anyhow!("Failed to create API authorization: {}", e))?;
        Ok(serde_json::to_value(authorization)?)
    }

    async fn handle_update_api_authorization(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let auth_id = args
            .get("auth_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("auth_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let authorization = client
            .api_auth
            .update_api_authorization(auth_id, request)
            .await
            .map_err(|e| anyhow!("Failed to update API authorization: {}", e))?;
        Ok(serde_json::to_value(authorization)?)
    }

    async fn handle_delete_api_authorization(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let auth_id = args
            .get("auth_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("auth_id is required"))?;
        client
            .api_auth
            .delete_api_authorization(auth_id)
            .await
            .map_err(|e| anyhow!("Failed to delete API authorization: {}", e))?;
        Ok(json!({"success": true, "message": "API authorization deleted successfully"}))
    }

    // ==================== ADDITIONAL SAML OPERATIONS ====================

    async fn handle_get_saml_assertion_v2(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let response = client
            .saml
            .get_saml_assertion_v2(request)
            .await
            .map_err(|e| anyhow!("Failed to get SAML assertion (v2): {}", e))?;
        Ok(serde_json::to_value(response)?)
    }

    // ==================== ADDITIONAL OIDC OPERATIONS ====================

    async fn handle_oidc_get_userinfo(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let access_token = args
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("access_token is required"))?;
        let userinfo = client
            .oidc
            .get_userinfo(access_token)
            .await
            .map_err(|e| anyhow!("Failed to get OIDC userinfo: {}", e))?;
        Ok(serde_json::to_value(userinfo)?)
    }

    // ==================== ADDITIONAL VIGILANCE OPERATIONS ====================

    async fn handle_list_risk_rules(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let rules = client
            .vigilance
            .list_risk_rules()
            .await
            .map_err(|e| anyhow!("Failed to list risk rules: {}", e))?;
        Ok(serde_json::to_value(rules)?)
    }

    async fn handle_create_risk_rule(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let rule = client
            .vigilance
            .create_risk_rule(request)
            .await
            .map_err(|e| anyhow!("Failed to create risk rule: {}", e))?;
        Ok(serde_json::to_value(rule)?)
    }

    async fn handle_update_risk_rule(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let rule_id = args
            .get("rule_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("rule_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let rule = client
            .vigilance
            .update_risk_rule(rule_id, request)
            .await
            .map_err(|e| anyhow!("Failed to update risk rule: {}", e))?;
        Ok(serde_json::to_value(rule)?)
    }

    async fn handle_delete_risk_rule(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let rule_id = args
            .get("rule_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("rule_id is required"))?;
        client
            .vigilance
            .delete_risk_rule(rule_id)
            .await
            .map_err(|e| anyhow!("Failed to delete risk rule: {}", e))?;
        Ok(json!({"success": true, "message": "Risk rule deleted successfully"}))
    }

    async fn handle_get_risk_events(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let events = client
            .vigilance
            .get_risk_events(user_id)
            .await
            .map_err(|e| anyhow!("Failed to get risk events: {}", e))?;
        Ok(serde_json::to_value(events)?)
    }

    async fn handle_track_risk_event(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let event = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        client
            .vigilance
            .track_risk_event(event)
            .await
            .map_err(|e| anyhow!("Failed to track risk event: {}", e))?;
        Ok(json!({"success": true, "message": "Risk event tracked successfully"}))
    }

    // ==================== DIRECTORIES OPERATIONS ====================

    async fn handle_get_directory_connector(&self, args: &Value) -> Result<Value> {
        let _client = self.resolve_client(args)?;
        // OneLogin API v2 only supports listing connectors, not getting individual ones
        // Users should use list_connectors and filter client-side
        Err(anyhow!("OneLogin API does not provide an endpoint to get individual connectors. Use list_connectors instead."))
    }

    async fn handle_update_directory_connector(&self, args: &Value) -> Result<Value> {
        let _client = self.resolve_client(args)?;
        // OneLogin API v2 does not provide connector management endpoints
        Err(anyhow!("OneLogin API does not provide endpoints to update connectors. Connector management must be done through the OneLogin admin console."))
    }

    async fn handle_create_directory_connector(&self, args: &Value) -> Result<Value> {
        let _client = self.resolve_client(args)?;
        Err(anyhow!("OneLogin API does not provide endpoints to create connectors. Connector management must be done through the OneLogin admin console."))
    }

    async fn handle_delete_directory_connector(&self, args: &Value) -> Result<Value> {
        let _client = self.resolve_client(args)?;
        Err(anyhow!("OneLogin API does not provide endpoints to delete connectors. Connector management must be done through the OneLogin admin console."))
    }

    async fn handle_sync_directory(&self, args: &Value) -> Result<Value> {
        let _client = self.resolve_client(args)?;
        Err(anyhow!("OneLogin API does not provide endpoints to trigger directory sync. Directory sync must be done through the OneLogin admin console."))
    }

    async fn handle_get_sync_status(&self, args: &Value) -> Result<Value> {
        let _client = self.resolve_client(args)?;
        Err(anyhow!("OneLogin API does not provide endpoints to get sync status. Directory status must be checked through the OneLogin admin console."))
    }

    // ==================== USER MAPPINGS OPERATIONS ====================

    async fn handle_list_user_mappings(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let mappings = client
            .user_mappings
            .list_mappings()
            .await
            .map_err(|e| anyhow!("Failed to list user mappings: {}", e))?;
        Ok(serde_json::to_value(mappings)?)
    }

    async fn handle_sort_user_mappings(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        client
            .user_mappings
            .sort_mapping_order(request)
            .await
            .map_err(|e| anyhow!("Failed to sort user mappings: {}", e))?;
        Ok(json!({"success": true, "message": "User mappings sorted successfully"}))
    }

    // ==================== MFA FACTOR OPERATIONS ====================

    async fn handle_enroll_mfa_factor(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let factor_id = args
            .get("factor_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("factor_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let enrollment = client
            .mfa
            .enroll_factor(user_id, factor_id, request)
            .await
            .map_err(|e| anyhow!("Failed to enroll MFA factor: {}", e))?;
        Ok(serde_json::to_value(enrollment)?)
    }

    async fn handle_verify_mfa_factor(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let device_id = args
            .get("device_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("device_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let result = client
            .mfa
            .verify_factor(user_id, device_id, request)
            .await
            .map_err(|e| anyhow!("Failed to verify MFA factor: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_remove_mfa_factor(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let device_id = args
            .get("device_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("device_id is required"))?;
        client
            .mfa
            .remove_factor(user_id, device_id)
            .await
            .map_err(|e| anyhow!("Failed to remove MFA factor: {}", e))?;
        Ok(json!({"success": true, "message": "MFA factor removed successfully"}))
    }

    // ==================== INVITATIONS OPERATIONS ====================

    async fn handle_generate_invite_link(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let invitation = client
            .invitations
            .generate_invite_link(request)
            .await
            .map_err(|e| anyhow!("Failed to generate invite link: {}", e))?;
        Ok(serde_json::to_value(invitation)?)
    }

    async fn handle_send_invite_link(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let invitation = client
            .invitations
            .send_invite_link(request)
            .await
            .map_err(|e| anyhow!("Failed to send invite link: {}", e))?;
        Ok(serde_json::to_value(invitation)?)
    }

    // ==================== BRANDING OPERATIONS ====================

    async fn handle_update_branding_settings(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let settings = client
            .branding
            .update_branding_settings(request)
            .await
            .map_err(|e| anyhow!("Failed to update branding settings: {}", e))?;
        Ok(serde_json::to_value(settings)?)
    }

    // ==================== APP RULES TOOL DEFINITIONS ====================

    fn tool_list_app_rules(&self) -> Value {
        json!({
            "name": "onelogin_list_app_rules",
            "description": "List all provisioning rules for an application. Rules control user provisioning and entitlement assignment.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Filter by enabled status"
                    },
                    "has_condition": {
                        "type": "string",
                        "description": "Filter by condition name and value (supports wildcards)"
                    },
                    "has_condition_type": {
                        "type": "string",
                        "enum": ["builtin", "custom", "none"],
                        "description": "Filter by condition type"
                    },
                    "has_action": {
                        "type": "string",
                        "description": "Filter by action name and value (supports wildcards)"
                    },
                    "has_action_type": {
                        "type": "string",
                        "enum": ["builtin", "custom", "none"],
                        "description": "Filter by action type"
                    }
                },
                "required": ["app_id"]
            }
        })
    }

    fn tool_get_app_rule(&self) -> Value {
        json!({
            "name": "onelogin_get_app_rule",
            "description": "Get a specific provisioning rule by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "rule_id": {
                        "type": "integer",
                        "description": "The rule ID"
                    }
                },
                "required": ["app_id", "rule_id"]
            }
        })
    }

    fn tool_create_app_rule(&self) -> Value {
        json!({
            "name": "onelogin_create_app_rule",
            "description": "Create a new provisioning rule for an application",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "Name of the rule"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether the rule is active (default: true)"
                    },
                    "match": {
                        "type": "string",
                        "enum": ["all", "any"],
                        "description": "'all' requires ALL conditions match, 'any' requires ANY condition matches"
                    },
                    "position": {
                        "type": "integer",
                        "description": "Execution priority - lower numbers execute first"
                    },
                    "conditions": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "source": {"type": "string", "description": "Condition source field"},
                                "operator": {"type": "string", "description": "Comparison operator"},
                                "value": {"type": "string", "description": "Value to compare against"}
                            },
                            "required": ["source", "operator", "value"]
                        },
                        "description": "Array of condition objects"
                    },
                    "actions": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "action": {"type": "string", "description": "Action type"},
                                "value": {"type": "array", "items": {"type": "string"}, "description": "Action values"},
                                "expression": {"type": "string", "description": "Optional dynamic expression"},
                                "macro": {"type": "string", "description": "Optional macro for value transformation"}
                            },
                            "required": ["action", "value"]
                        },
                        "description": "Array of action objects"
                    }
                },
                "required": ["app_id", "name"]
            }
        })
    }

    fn tool_update_app_rule(&self) -> Value {
        json!({
            "name": "onelogin_update_app_rule",
            "description": "Update an existing provisioning rule",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "rule_id": {
                        "type": "integer",
                        "description": "The rule ID to update"
                    },
                    "name": {
                        "type": "string",
                        "description": "Updated name"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether the rule is active"
                    },
                    "match": {
                        "type": "string",
                        "enum": ["all", "any"],
                        "description": "Match type"
                    },
                    "position": {
                        "type": "integer",
                        "description": "Execution priority"
                    },
                    "conditions": {
                        "type": "array",
                        "description": "Updated conditions array"
                    },
                    "actions": {
                        "type": "array",
                        "description": "Updated actions array"
                    }
                },
                "required": ["app_id", "rule_id"]
            }
        })
    }

    fn tool_delete_app_rule(&self) -> Value {
        json!({
            "name": "onelogin_delete_app_rule",
            "description": "Delete a provisioning rule",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "rule_id": {
                        "type": "integer",
                        "description": "The rule ID to delete"
                    }
                },
                "required": ["app_id", "rule_id"]
            }
        })
    }

    fn tool_list_app_rule_conditions(&self) -> Value {
        json!({
            "name": "onelogin_list_app_rule_conditions",
            "description": "List available condition types for an application's rules",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    }
                },
                "required": ["app_id"]
            }
        })
    }

    fn tool_list_app_rule_actions(&self) -> Value {
        json!({
            "name": "onelogin_list_app_rule_actions",
            "description": "List available action types for an application's rules",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    }
                },
                "required": ["app_id"]
            }
        })
    }

    fn tool_list_condition_operators(&self) -> Value {
        json!({
            "name": "onelogin_list_condition_operators",
            "description": "List available operators for a specific condition type",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "condition_value": {
                        "type": "string",
                        "description": "The condition type value (from list_app_rule_conditions)"
                    }
                },
                "required": ["app_id", "condition_value"]
            }
        })
    }

    fn tool_list_condition_values(&self) -> Value {
        json!({
            "name": "onelogin_list_condition_values",
            "description": "List available values for a specific condition type",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "condition_value": {
                        "type": "string",
                        "description": "The condition type value (from list_app_rule_conditions)"
                    }
                },
                "required": ["app_id", "condition_value"]
            }
        })
    }

    fn tool_list_action_values(&self) -> Value {
        json!({
            "name": "onelogin_list_action_values",
            "description": "List available values for a specific action type",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "action_value": {
                        "type": "string",
                        "description": "The action type value (from list_app_rule_actions)"
                    }
                },
                "required": ["app_id", "action_value"]
            }
        })
    }

    fn tool_sort_app_rules(&self) -> Value {
        json!({
            "name": "onelogin_sort_app_rules",
            "description": "Reorder provisioning rules for an application. Rules are executed in order.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "integer",
                        "description": "The application ID"
                    },
                    "rule_ids": {
                        "type": "array",
                        "items": {"type": "integer"},
                        "description": "Array of rule IDs in desired execution order"
                    }
                },
                "required": ["app_id", "rule_ids"]
            }
        })
    }

    // ==================== APP RULES HANDLERS ====================

    async fn handle_list_app_rules(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;

        let params = crate::models::app_rules::AppRuleQueryParams {
            enabled: args.get("enabled").and_then(|v| v.as_bool()),
            has_condition: args.get("has_condition").and_then(|v| v.as_str()).map(String::from),
            has_condition_type: args.get("has_condition_type").and_then(|v| v.as_str()).map(String::from),
            has_action: args.get("has_action").and_then(|v| v.as_str()).map(String::from),
            has_action_type: args.get("has_action_type").and_then(|v| v.as_str()).map(String::from),
        };

        let rules = client.app_rules.list_rules(app_id, Some(params)).await
            .map_err(|e| anyhow!("Failed to list app rules: {}", e))?;
        Ok(serde_json::to_value(rules)?)
    }

    async fn handle_get_app_rule(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let rule_id = args
            .get("rule_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("rule_id is required"))?;

        let rule = client.app_rules.get_rule(app_id, rule_id).await
            .map_err(|e| anyhow!("Failed to get app rule: {}", e))?;
        Ok(serde_json::to_value(rule)?)
    }

    async fn handle_create_app_rule(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;

        let request: crate::models::app_rules::CreateAppRuleRequest = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;

        let rule = client.app_rules.create_rule(app_id, request).await
            .map_err(|e| anyhow!("Failed to create app rule: {}", e))?;
        Ok(serde_json::to_value(rule)?)
    }

    async fn handle_update_app_rule(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let rule_id = args
            .get("rule_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("rule_id is required"))?;

        let request: crate::models::app_rules::UpdateAppRuleRequest = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;

        let rule = client.app_rules.update_rule(app_id, rule_id, request).await
            .map_err(|e| anyhow!("Failed to update app rule: {}", e))?;
        Ok(serde_json::to_value(rule)?)
    }

    async fn handle_delete_app_rule(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let rule_id = args
            .get("rule_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("rule_id is required"))?;

        client.app_rules.delete_rule(app_id, rule_id).await
            .map_err(|e| anyhow!("Failed to delete app rule: {}", e))?;
        Ok(json!({"success": true, "message": "Rule deleted successfully"}))
    }

    async fn handle_list_app_rule_conditions(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;

        let conditions = client.app_rules.list_conditions(app_id).await
            .map_err(|e| anyhow!("Failed to list conditions: {}", e))?;
        Ok(serde_json::to_value(conditions)?)
    }

    async fn handle_list_app_rule_actions(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;

        let actions = client.app_rules.list_actions(app_id).await
            .map_err(|e| anyhow!("Failed to list actions: {}", e))?;
        Ok(serde_json::to_value(actions)?)
    }

    async fn handle_list_condition_operators(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let condition_value = args
            .get("condition_value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("condition_value is required"))?;

        let operators = client.app_rules.list_condition_operators(app_id, condition_value).await
            .map_err(|e| anyhow!("Failed to list condition operators: {}", e))?;
        Ok(serde_json::to_value(operators)?)
    }

    async fn handle_list_condition_values(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let condition_value = args
            .get("condition_value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("condition_value is required"))?;

        let values = client.app_rules.list_condition_values(app_id, condition_value).await
            .map_err(|e| anyhow!("Failed to list condition values: {}", e))?;
        Ok(serde_json::to_value(values)?)
    }

    async fn handle_list_action_values(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let action_value = args
            .get("action_value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("action_value is required"))?;

        let values = client.app_rules.list_action_values(app_id, action_value).await
            .map_err(|e| anyhow!("Failed to list action values: {}", e))?;
        Ok(serde_json::to_value(values)?)
    }

    async fn handle_sort_app_rules(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let app_id = args
            .get("app_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("app_id is required"))?;
        let rule_ids: Vec<i64> = args
            .get("rule_ids")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| value_as_i64(v)).collect())
            .ok_or_else(|| anyhow!("rule_ids array is required"))?;

        let request = crate::models::app_rules::SortRulesRequest { rule_ids };
        let sorted_ids = client.app_rules.sort_rules(app_id, request).await
            .map_err(|e| anyhow!("Failed to sort rules: {}", e))?;
        Ok(json!({"success": true, "rule_ids": sorted_ids}))
    }

    // ==================== MESSAGE TEMPLATE TOOL DEFINITIONS ====================

    fn tool_list_message_templates(&self) -> Value {
        json!({
            "name": "onelogin_list_message_templates",
            "description": "List all message templates for a brand",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "brand_id": {
                        "type": "integer",
                        "description": "The brand ID"
                    }
                },
                "required": ["brand_id"]
            }
        })
    }

    fn tool_get_message_template(&self) -> Value {
        json!({
            "name": "onelogin_get_message_template",
            "description": "Get a specific message template by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "brand_id": {
                        "type": "integer",
                        "description": "The brand ID"
                    },
                    "template_id": {
                        "type": "integer",
                        "description": "The template ID"
                    }
                },
                "required": ["brand_id", "template_id"]
            }
        })
    }

    fn tool_get_template_by_type(&self) -> Value {
        json!({
            "name": "onelogin_get_template_by_type",
            "description": "Get a message template by type",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "brand_id": {
                        "type": "integer",
                        "description": "The brand ID"
                    },
                    "template_type": {
                        "type": "string",
                        "description": "The template type (e.g., 'email_invitation', 'email_password_reset')"
                    }
                },
                "required": ["brand_id", "template_type"]
            }
        })
    }

    fn tool_get_template_by_locale(&self) -> Value {
        json!({
            "name": "onelogin_get_template_by_locale",
            "description": "Get a message template by type and locale",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "brand_id": {
                        "type": "integer",
                        "description": "The brand ID"
                    },
                    "template_type": {
                        "type": "string",
                        "description": "The template type"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale code (e.g., 'en', 'fr', 'es')"
                    }
                },
                "required": ["brand_id", "template_type", "locale"]
            }
        })
    }

    fn tool_create_message_template(&self) -> Value {
        json!({
            "name": "onelogin_create_message_template",
            "description": "Create a new message template for a brand",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "brand_id": {
                        "type": "integer",
                        "description": "The brand ID"
                    },
                    "type": {
                        "type": "string",
                        "description": "The template type (e.g., 'email_invitation', 'email_password_reset')"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale code"
                    },
                    "subject": {
                        "type": "string",
                        "description": "Email subject line"
                    },
                    "body": {
                        "type": "string",
                        "description": "Email body (HTML supported)"
                    }
                },
                "required": ["brand_id", "type"]
            }
        })
    }

    fn tool_update_message_template(&self) -> Value {
        json!({
            "name": "onelogin_update_message_template",
            "description": "Update a message template by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "brand_id": {
                        "type": "integer",
                        "description": "The brand ID"
                    },
                    "template_id": {
                        "type": "integer",
                        "description": "The template ID"
                    },
                    "subject": {
                        "type": "string",
                        "description": "Updated email subject"
                    },
                    "body": {
                        "type": "string",
                        "description": "Updated email body"
                    },
                    "locale": {
                        "type": "string",
                        "description": "Updated locale"
                    }
                },
                "required": ["brand_id", "template_id"]
            }
        })
    }

    fn tool_update_template_by_locale(&self) -> Value {
        json!({
            "name": "onelogin_update_template_by_locale",
            "description": "Update a message template by type and locale",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "brand_id": {
                        "type": "integer",
                        "description": "The brand ID"
                    },
                    "template_type": {
                        "type": "string",
                        "description": "The template type"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale code"
                    },
                    "subject": {
                        "type": "string",
                        "description": "Updated email subject"
                    },
                    "body": {
                        "type": "string",
                        "description": "Updated email body"
                    }
                },
                "required": ["brand_id", "template_type", "locale"]
            }
        })
    }

    fn tool_delete_message_template(&self) -> Value {
        json!({
            "name": "onelogin_delete_message_template",
            "description": "Delete a message template",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "brand_id": {
                        "type": "integer",
                        "description": "The brand ID"
                    },
                    "template_id": {
                        "type": "integer",
                        "description": "The template ID to delete"
                    }
                },
                "required": ["brand_id", "template_id"]
            }
        })
    }

    // ==================== MESSAGE TEMPLATE HANDLERS ====================

    async fn handle_list_message_templates(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let brand_id = args
            .get("brand_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("brand_id is required"))?;
        let templates = client.branding.list_message_templates(brand_id).await
            .map_err(|e| anyhow!("Failed to list message templates: {}", e))?;
        Ok(serde_json::to_value(templates)?)
    }

    async fn handle_get_message_template(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let brand_id = args
            .get("brand_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("brand_id is required"))?;
        let template_id = args
            .get("template_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("template_id is required"))?;
        let template = client.branding.get_message_template(brand_id, template_id).await
            .map_err(|e| anyhow!("Failed to get message template: {}", e))?;
        Ok(serde_json::to_value(template)?)
    }

    async fn handle_get_template_by_type(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let brand_id = args
            .get("brand_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("brand_id is required"))?;
        let template_type = args
            .get("template_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("template_type is required"))?;
        let template = client.branding.get_template_by_type(brand_id, template_type).await
            .map_err(|e| anyhow!("Failed to get template by type: {}", e))?;
        Ok(serde_json::to_value(template)?)
    }

    async fn handle_get_template_by_locale(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let brand_id = args
            .get("brand_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("brand_id is required"))?;
        let template_type = args
            .get("template_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("template_type is required"))?;
        let locale = args
            .get("locale")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("locale is required"))?;
        let template = client.branding.get_template_by_locale(brand_id, template_type, locale).await
            .map_err(|e| anyhow!("Failed to get template by locale: {}", e))?;
        Ok(serde_json::to_value(template)?)
    }

    async fn handle_create_message_template(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let brand_id = args
            .get("brand_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("brand_id is required"))?;
        let request: crate::models::branding::CreateMessageTemplateRequest = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let template = client.branding.create_message_template(brand_id, request).await
            .map_err(|e| anyhow!("Failed to create message template: {}", e))?;
        Ok(serde_json::to_value(template)?)
    }

    async fn handle_update_message_template(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let brand_id = args
            .get("brand_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("brand_id is required"))?;
        let template_id = args
            .get("template_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("template_id is required"))?;
        let request: crate::models::branding::UpdateMessageTemplateRequest = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let template = client.branding.update_message_template(brand_id, template_id, request).await
            .map_err(|e| anyhow!("Failed to update message template: {}", e))?;
        Ok(serde_json::to_value(template)?)
    }

    async fn handle_update_template_by_locale(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let brand_id = args
            .get("brand_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("brand_id is required"))?;
        let template_type = args
            .get("template_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("template_type is required"))?;
        let locale = args
            .get("locale")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("locale is required"))?;
        let request: crate::models::branding::UpdateMessageTemplateRequest = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let template = client.branding.update_template_by_locale(brand_id, template_type, locale, request).await
            .map_err(|e| anyhow!("Failed to update template by locale: {}", e))?;
        Ok(serde_json::to_value(template)?)
    }

    async fn handle_delete_message_template(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let brand_id = args
            .get("brand_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("brand_id is required"))?;
        let template_id = args
            .get("template_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("template_id is required"))?;
        client.branding.delete_message_template(brand_id, template_id).await
            .map_err(|e| anyhow!("Failed to delete message template: {}", e))?;
        Ok(json!({"success": true, "message": "Template deleted successfully"}))
    }

    // ==================== SELF-REGISTRATION TOOL DEFINITIONS ====================

    fn tool_list_self_registration_profiles(&self) -> Value {
        json!({
            "name": "onelogin_list_self_registration_profiles",
            "description": "List all self-registration profiles",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_self_registration_profile(&self) -> Value {
        json!({
            "name": "onelogin_get_self_registration_profile",
            "description": "Get a specific self-registration profile by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile_id": {
                        "type": "integer",
                        "description": "The profile ID"
                    }
                },
                "required": ["profile_id"]
            }
        })
    }

    fn tool_create_self_registration_profile(&self) -> Value {
        json!({
            "name": "onelogin_create_self_registration_profile",
            "description": "Create a new self-registration profile for user sign-up",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Profile name"
                    },
                    "url": {
                        "type": "string",
                        "description": "Unique URL path for the registration form"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether the profile is active"
                    },
                    "moderated": {
                        "type": "boolean",
                        "description": "If true, registrations require admin approval"
                    },
                    "email_verification_type": {
                        "type": "string",
                        "enum": ["email", "sms", "none"],
                        "description": "How to verify new user identity"
                    },
                    "default_role_id": {
                        "type": "integer",
                        "description": "Role to auto-assign to new users"
                    },
                    "default_group_id": {
                        "type": "integer",
                        "description": "Group to auto-assign to new users"
                    },
                    "domain_whitelist": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Allowed email domains"
                    },
                    "domain_blacklist": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Blocked email domains"
                    },
                    "helpdesk_message": {
                        "type": "string",
                        "description": "Custom message shown on registration form"
                    }
                },
                "required": ["name", "url", "enabled"]
            }
        })
    }

    fn tool_update_self_registration_profile(&self) -> Value {
        json!({
            "name": "onelogin_update_self_registration_profile",
            "description": "Update an existing self-registration profile",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile_id": {
                        "type": "integer",
                        "description": "The profile ID to update"
                    },
                    "name": {"type": "string"},
                    "url": {"type": "string"},
                    "enabled": {"type": "boolean"},
                    "moderated": {"type": "boolean"},
                    "email_verification_type": {"type": "string"},
                    "default_role_id": {"type": "integer"},
                    "default_group_id": {"type": "integer"},
                    "domain_whitelist": {"type": "array", "items": {"type": "string"}},
                    "domain_blacklist": {"type": "array", "items": {"type": "string"}},
                    "helpdesk_message": {"type": "string"}
                },
                "required": ["profile_id"]
            }
        })
    }

    fn tool_delete_self_registration_profile(&self) -> Value {
        json!({
            "name": "onelogin_delete_self_registration_profile",
            "description": "Delete a self-registration profile",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile_id": {
                        "type": "integer",
                        "description": "The profile ID to delete"
                    }
                },
                "required": ["profile_id"]
            }
        })
    }

    fn tool_list_registrations(&self) -> Value {
        json!({
            "name": "onelogin_list_registrations",
            "description": "List pending registrations for a self-registration profile",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile_id": {
                        "type": "integer",
                        "description": "The profile ID"
                    }
                },
                "required": ["profile_id"]
            }
        })
    }

    fn tool_approve_registration(&self) -> Value {
        json!({
            "name": "onelogin_approve_registration",
            "description": "Approve or reject a pending registration",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile_id": {
                        "type": "integer",
                        "description": "The profile ID"
                    },
                    "registration_id": {
                        "type": "integer",
                        "description": "The registration ID"
                    },
                    "status": {
                        "type": "string",
                        "enum": ["approved", "rejected"],
                        "description": "Approval status"
                    },
                    "rejection_reason": {
                        "type": "string",
                        "description": "Reason for rejection (if rejected)"
                    }
                },
                "required": ["profile_id", "registration_id", "status"]
            }
        })
    }

    // ==================== SELF-REGISTRATION HANDLERS ====================

    async fn handle_list_self_registration_profiles(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let profiles = client.self_registration.list_profiles().await
            .map_err(|e| anyhow!("Failed to list self-registration profiles: {}", e))?;
        Ok(serde_json::to_value(profiles)?)
    }

    async fn handle_get_self_registration_profile(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let profile_id = args
            .get("profile_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("profile_id is required"))?;
        let profile = client.self_registration.get_profile(profile_id).await
            .map_err(|e| anyhow!("Failed to get profile: {}", e))?;
        Ok(serde_json::to_value(profile)?)
    }

    async fn handle_create_self_registration_profile(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request: crate::models::self_registration::CreateSelfRegistrationProfileRequest =
            serde_json::from_value(args.clone())
                .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let profile = client.self_registration.create_profile(request).await
            .map_err(|e| anyhow!("Failed to create profile: {}", e))?;
        Ok(serde_json::to_value(profile)?)
    }

    async fn handle_update_self_registration_profile(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let profile_id = args
            .get("profile_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("profile_id is required"))?;
        let request: crate::models::self_registration::UpdateSelfRegistrationProfileRequest =
            serde_json::from_value(args.clone())
                .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let profile = client.self_registration.update_profile(profile_id, request).await
            .map_err(|e| anyhow!("Failed to update profile: {}", e))?;
        Ok(serde_json::to_value(profile)?)
    }

    async fn handle_delete_self_registration_profile(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let profile_id = args
            .get("profile_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("profile_id is required"))?;
        client.self_registration.delete_profile(profile_id).await
            .map_err(|e| anyhow!("Failed to delete profile: {}", e))?;
        Ok(json!({"success": true, "message": "Profile deleted successfully"}))
    }

    async fn handle_list_registrations(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let profile_id = args
            .get("profile_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("profile_id is required"))?;
        let registrations = client.self_registration.list_registrations(profile_id).await
            .map_err(|e| anyhow!("Failed to list registrations: {}", e))?;
        Ok(serde_json::to_value(registrations)?)
    }

    async fn handle_approve_registration(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let profile_id = args
            .get("profile_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("profile_id is required"))?;
        let registration_id = args
            .get("registration_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("registration_id is required"))?;
        let request: crate::models::self_registration::ApproveRegistrationRequest =
            serde_json::from_value(args.clone())
                .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let registration = client.self_registration.approve_registration(profile_id, registration_id, request).await
            .map_err(|e| anyhow!("Failed to approve/reject registration: {}", e))?;
        Ok(serde_json::to_value(registration)?)
    }

    // ==================== REPORTS TOOL DEFINITIONS ====================

    fn tool_list_reports(&self) -> Value {
        json!({
            "name": "onelogin_list_reports",
            "description": "List all available reports",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_report(&self) -> Value {
        json!({
            "name": "onelogin_get_report",
            "description": "Get a specific report definition by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "report_id": {
                        "type": "integer",
                        "description": "The report ID"
                    }
                },
                "required": ["report_id"]
            }
        })
    }

    fn tool_run_report(&self) -> Value {
        json!({
            "name": "onelogin_run_report",
            "description": "Run a report and return results",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "report_id": {
                        "type": "integer",
                        "description": "The report ID to run"
                    },
                    "filters": {
                        "type": "object",
                        "description": "Report-specific filter parameters"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["json", "csv"],
                        "description": "Output format (default: json)"
                    }
                },
                "required": ["report_id"]
            }
        })
    }

    fn tool_get_report_results(&self) -> Value {
        json!({
            "name": "onelogin_get_report_results",
            "description": "Get results from a report job",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "report_id": {
                        "type": "integer",
                        "description": "The report ID"
                    },
                    "job_id": {
                        "type": "string",
                        "description": "The job ID from a run_report call"
                    }
                },
                "required": ["report_id", "job_id"]
            }
        })
    }

    // ==================== REPORTS HANDLERS ====================

    async fn handle_list_reports(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let reports = client.reports.list_reports().await
            .map_err(|e| anyhow!("Failed to list reports: {}", e))?;
        Ok(serde_json::to_value(reports)?)
    }

    async fn handle_get_report(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let report_id = args
            .get("report_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("report_id is required"))?;
        let report = client.reports.get_report(report_id).await
            .map_err(|e| anyhow!("Failed to get report: {}", e))?;
        Ok(serde_json::to_value(report)?)
    }

    async fn handle_run_report(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let report_id = args
            .get("report_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("report_id is required"))?;
        let request: Option<crate::models::reports::RunReportRequest> =
            if args.get("filters").is_some() || args.get("format").is_some() {
                Some(serde_json::from_value(args.clone()).map_err(|e| anyhow!("Invalid request: {}", e))?)
            } else {
                None
            };
        let job = client.reports.run_report(report_id, request).await
            .map_err(|e| anyhow!("Failed to run report: {}", e))?;
        Ok(serde_json::to_value(job)?)
    }

    async fn handle_get_report_results(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let report_id = args
            .get("report_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("report_id is required"))?;
        let job_id = args
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("job_id is required"))?;
        let job = client.reports.get_report_results(report_id, job_id).await
            .map_err(|e| anyhow!("Failed to get report results: {}", e))?;
        Ok(serde_json::to_value(job)?)
    }

    // ==================== LOGIN/SESSION API ====================

    fn tool_create_session_login_token(&self) -> Value {
        json!({
            "name": "onelogin_create_session_login_token",
            "description": "Authenticate a user and create a session login token. This is the first step in the login flow. Returns a session token that can be used to create a browser session or may require MFA verification.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "username_or_email": {
                        "type": "string",
                        "description": "User's username or email address"
                    },
                    "password": {
                        "type": "string",
                        "description": "User's password"
                    },
                    "subdomain": {
                        "type": "string",
                        "description": "OneLogin subdomain (e.g., 'mycompany' for mycompany.onelogin.com)"
                    },
                    "fields": {
                        "type": "string",
                        "description": "Optional comma-separated list of fields to return (e.g., 'id,email')"
                    },
                    "ip_address": {
                        "type": "string",
                        "description": "Optional IP address of the end user for risk assessment"
                    }
                },
                "required": ["username_or_email", "password", "subdomain"]
            }
        })
    }

    fn tool_verify_factor_login(&self) -> Value {
        json!({
            "name": "onelogin_verify_factor_login",
            "description": "Verify an MFA factor during the login flow. Use this after create_session_login_token when MFA is required. The state_token and device_id come from the initial login response.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "state_token": {
                        "type": "string",
                        "description": "State token from the create_session_login_token response"
                    },
                    "device_id": {
                        "type": "integer",
                        "description": "Device ID of the MFA factor to verify"
                    },
                    "otp_token": {
                        "type": "string",
                        "description": "OTP code from the user's MFA device"
                    },
                    "do_not_notify": {
                        "type": "boolean",
                        "description": "If true, don't send push notification (for push-based MFA)"
                    }
                },
                "required": ["state_token"]
            }
        })
    }

    fn tool_create_session(&self) -> Value {
        json!({
            "name": "onelogin_create_session",
            "description": "Create a browser session from a session token. This converts the session token from login into an actual browser session cookie. Note: This endpoint is on the main domain, not the API domain.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "session_token": {
                        "type": "string",
                        "description": "Session token from successful login or MFA verification"
                    }
                },
                "required": ["session_token"]
            }
        })
    }

    async fn handle_create_session_login_token(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let username_or_email = args
            .get("username_or_email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("username_or_email is required"))?;
        let password = args
            .get("password")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("password is required"))?;
        let subdomain = args
            .get("subdomain")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("subdomain is required"))?;
        let fields = args.get("fields").and_then(|v| v.as_str()).map(String::from);
        let ip_address = args.get("ip_address").and_then(|v| v.as_str()).map(String::from);

        let request = crate::models::login::SessionLoginRequest {
            username_or_email: username_or_email.to_string(),
            password: password.to_string(),
            subdomain: subdomain.to_string(),
            fields,
            ip_address,
        };

        let response = client.login.create_session_login_token(request).await
            .map_err(|e| anyhow!("Failed to create session login token: {}", e))?;
        Ok(serde_json::to_value(response)?)
    }

    async fn handle_verify_factor_login(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let state_token = args
            .get("state_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("state_token is required"))?;
        let device_id = args.get("device_id").and_then(|v| value_as_i64(v));
        let otp_token = args.get("otp_token").and_then(|v| v.as_str()).map(String::from);
        let do_not_notify = args.get("do_not_notify").and_then(|v| v.as_bool());

        let request = crate::models::login::VerifyFactorLoginRequest {
            state_token: state_token.to_string(),
            device_id,
            otp_token,
            do_not_notify,
        };

        let response = client.login.verify_factor_login(request).await
            .map_err(|e| anyhow!("Failed to verify factor: {}", e))?;
        Ok(serde_json::to_value(response)?)
    }

    async fn handle_create_session(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let session_token = args
            .get("session_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("session_token is required"))?;

        let request = crate::models::login::CreateSessionRequest {
            session_token: session_token.to_string(),
        };

        let response = client.login.create_session(request).await
            .map_err(|e| anyhow!("Failed to create session: {}", e))?;
        Ok(serde_json::to_value(response)?)
    }

    // ==================== CONNECTORS API ====================

    fn tool_list_connectors(&self) -> Value {
        json!({
            "name": "onelogin_list_connectors",
            "description": "List all available application connectors. Connectors define the type and configuration template for applications (e.g., SAML, OIDC, WS-Fed). Use this to find the connector_id needed when creating a new application.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": []
            }
        })
    }

    fn tool_get_connector(&self) -> Value {
        json!({
            "name": "onelogin_get_connector",
            "description": "Get detailed information about a specific connector by ID. Returns the connector's configuration template and supported authentication methods.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "connector_id": {
                        "type": "integer",
                        "description": "The ID of the connector to retrieve"
                    }
                },
                "required": ["connector_id"]
            }
        })
    }

    async fn handle_list_connectors(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let connectors = client.connectors.list_connectors().await
            .map_err(|e| anyhow!("Failed to list connectors: {}", e))?;
        Ok(serde_json::to_value(connectors)?)
    }

    async fn handle_get_connector(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let connector_id = args
            .get("connector_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("connector_id is required"))?;
        let connector = client.connectors.get_connector(connector_id).await
            .map_err(|e| anyhow!("Failed to get connector: {}", e))?;
        Ok(serde_json::to_value(connector)?)
    }

    // ==================== MFA TOKEN API ====================

    fn tool_generate_mfa_token(&self) -> Value {
        json!({
            "name": "onelogin_generate_mfa_token",
            "description": "Generate a temporary MFA bypass token for a user. This is useful for helpdesk scenarios where a user needs to bypass MFA temporarily (e.g., lost phone). The token can be configured with an expiration time and reusability.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The ID of the user to generate the MFA token for"
                    },
                    "expires_in": {
                        "type": "integer",
                        "description": "Time in seconds until the token expires (default varies by account settings)"
                    },
                    "reusable": {
                        "type": "boolean",
                        "description": "Whether the token can be reused multiple times before expiration"
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_verify_mfa_token(&self) -> Value {
        json!({
            "name": "onelogin_verify_mfa_token",
            "description": "Verify if a temporary MFA token is still valid for a user. Returns whether the token is valid and any associated messages.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The ID of the user whose MFA token to verify"
                    },
                    "mfa_token": {
                        "type": "string",
                        "description": "The MFA token to verify"
                    }
                },
                "required": ["user_id", "mfa_token"]
            }
        })
    }

    async fn handle_generate_mfa_token(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let expires_in = args.get("expires_in").and_then(|v| value_as_i64(v)).map(|v| v as i32);
        let reusable = args.get("reusable").and_then(|v| v.as_bool());

        let request = crate::models::mfa::GenerateMfaTokenRequest {
            expires_in,
            reusable,
        };

        let token = client.mfa.generate_mfa_token(user_id, request).await
            .map_err(|e| anyhow!("Failed to generate MFA token: {}", e))?;
        Ok(serde_json::to_value(token)?)
    }

    async fn handle_verify_mfa_token(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args
            .get("user_id")
            .and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let mfa_token = args
            .get("mfa_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("mfa_token is required"))?;

        let request = crate::models::mfa::VerifyMfaTokenRequest {
            mfa_token: mfa_token.to_string(),
        };

        let response = client.mfa.verify_mfa_token(user_id, request).await
            .map_err(|e| anyhow!("Failed to verify MFA token: {}", e))?;
        Ok(serde_json::to_value(response)?)
    }

    // ===== RATE LIMITS API =====
    fn tool_get_rate_limit_status(&self) -> Value {
        json!({
            "name": "onelogin_get_rate_limit_status",
            "description": "Get current API rate limit status including remaining requests and reset time",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_rate_limits(&self) -> Value {
        json!({
            "name": "onelogin_get_rate_limits",
            "description": "Get rate limit configuration for the account",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    async fn handle_get_rate_limit_status(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let result = client.rate_limits.get_rate_limit_status().await
            .map_err(|e| anyhow!("Failed to get rate limit status: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_rate_limits(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let result = client.rate_limits.get_rate_limits().await
            .map_err(|e| anyhow!("Failed to get rate limits: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    // ===== ACCOUNT SETTINGS API =====
    fn tool_get_account_settings(&self) -> Value {
        json!({
            "name": "onelogin_get_account_settings",
            "description": "Get global OneLogin account settings",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_update_account_settings(&self) -> Value {
        json!({
            "name": "onelogin_update_account_settings",
            "description": "Update account settings",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "subdomain": {
                        "type": "string",
                        "description": "Account subdomain"
                    },
                    "allow_password_reset": {
                        "type": "boolean",
                        "description": "Allow users to reset their password"
                    },
                    "allow_remember_me": {
                        "type": "boolean",
                        "description": "Allow 'remember me' option on login"
                    },
                    "session_timeout": {
                        "type": "integer",
                        "description": "Session timeout in minutes"
                    }
                }
            }
        })
    }

    fn tool_get_account_features(&self) -> Value {
        json!({
            "name": "onelogin_get_account_features",
            "description": "Get list of enabled features for the account",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_account_usage(&self) -> Value {
        json!({
            "name": "onelogin_get_account_usage",
            "description": "Get account usage statistics",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "start_date": {
                        "type": "string",
                        "description": "Start date for usage period (YYYY-MM-DD)"
                    },
                    "end_date": {
                        "type": "string",
                        "description": "End date for usage period (YYYY-MM-DD)"
                    }
                }
            }
        })
    }

    async fn handle_get_account_settings(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let result = client.account.get_account_settings().await
            .map_err(|e| anyhow!("Failed to get account settings: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_update_account_settings(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = crate::models::account::UpdateAccountSettingsRequest {
            default_locale: args.get("default_locale").and_then(|v| v.as_str()).map(|s| s.to_string()),
            default_timezone: args.get("default_timezone").and_then(|v| v.as_str()).map(|s| s.to_string()),
            session_timeout: args.get("session_timeout").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            absolute_session_timeout: args.get("absolute_session_timeout").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            mfa_required: args.get("mfa_required").and_then(|v| v.as_bool()),
            allowed_ip_ranges: args.get("allowed_ip_ranges").and_then(|v| v.as_array()).map(|arr|
                arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
            ),
            allowed_countries: args.get("allowed_countries").and_then(|v| v.as_array()).map(|arr|
                arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
            ),
        };
        let result = client.account.update_account_settings(request).await
            .map_err(|e| anyhow!("Failed to update account settings: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_account_features(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let result = client.account.get_account_features().await
            .map_err(|e| anyhow!("Failed to get account features: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_account_usage(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let start_date = args.get("start_date").and_then(|v| v.as_str()).map(|s| s.to_string());
        let end_date = args.get("end_date").and_then(|v| v.as_str()).map(|s| s.to_string());
        let result = client.account.get_account_usage(start_date, end_date).await
            .map_err(|e| anyhow!("Failed to get account usage: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    // ===== PASSWORD POLICIES API =====
    fn tool_list_password_policies(&self) -> Value {
        json!({
            "name": "onelogin_list_password_policies",
            "description": "List all password policies",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_password_policy(&self) -> Value {
        json!({
            "name": "onelogin_get_password_policy",
            "description": "Get a specific password policy by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "policy_id": {
                        "type": "integer",
                        "description": "The password policy ID"
                    }
                },
                "required": ["policy_id"]
            }
        })
    }

    fn tool_create_password_policy(&self) -> Value {
        json!({
            "name": "onelogin_create_password_policy",
            "description": "Create a new password policy",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Policy name"
                    },
                    "min_length": {
                        "type": "integer",
                        "description": "Minimum password length"
                    },
                    "max_length": {
                        "type": "integer",
                        "description": "Maximum password length"
                    },
                    "require_uppercase": {
                        "type": "boolean",
                        "description": "Require uppercase letters"
                    },
                    "require_lowercase": {
                        "type": "boolean",
                        "description": "Require lowercase letters"
                    },
                    "require_numbers": {
                        "type": "boolean",
                        "description": "Require numbers"
                    },
                    "require_symbols": {
                        "type": "boolean",
                        "description": "Require special characters"
                    },
                    "password_expiration_days": {
                        "type": "integer",
                        "description": "Days until password expires"
                    },
                    "password_history_count": {
                        "type": "integer",
                        "description": "Number of previous passwords to remember"
                    }
                },
                "required": ["name"]
            }
        })
    }

    fn tool_update_password_policy(&self) -> Value {
        json!({
            "name": "onelogin_update_password_policy",
            "description": "Update an existing password policy",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "policy_id": {
                        "type": "integer",
                        "description": "The password policy ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "Policy name"
                    },
                    "min_length": {
                        "type": "integer",
                        "description": "Minimum password length"
                    },
                    "max_length": {
                        "type": "integer",
                        "description": "Maximum password length"
                    },
                    "require_uppercase": {
                        "type": "boolean",
                        "description": "Require uppercase letters"
                    },
                    "require_lowercase": {
                        "type": "boolean",
                        "description": "Require lowercase letters"
                    },
                    "require_numbers": {
                        "type": "boolean",
                        "description": "Require numbers"
                    },
                    "require_symbols": {
                        "type": "boolean",
                        "description": "Require special characters"
                    },
                    "password_expiration_days": {
                        "type": "integer",
                        "description": "Days until password expires"
                    },
                    "password_history_count": {
                        "type": "integer",
                        "description": "Number of previous passwords to remember"
                    }
                },
                "required": ["policy_id"]
            }
        })
    }

    async fn handle_list_password_policies(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let result = client.password_policies.list_password_policies().await
            .map_err(|e| anyhow!("Failed to list password policies: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_password_policy(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let policy_id = args.get("policy_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("policy_id is required"))?;
        let result = client.password_policies.get_password_policy(policy_id).await
            .map_err(|e| anyhow!("Failed to get password policy: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_create_password_policy(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let name = args.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("name is required"))?;
        let request = crate::models::password_policies::CreatePasswordPolicyRequest {
            name: name.to_string(),
            min_length: args.get("min_length").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            require_uppercase: args.get("require_uppercase").and_then(|v| v.as_bool()),
            require_lowercase: args.get("require_lowercase").and_then(|v| v.as_bool()),
            require_numbers: args.get("require_numbers").and_then(|v| v.as_bool()),
            require_special_chars: args.get("require_special_chars").and_then(|v| v.as_bool()),
            password_history: args.get("password_history").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            expiration_days: args.get("expiration_days").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            max_failed_attempts: args.get("max_failed_attempts").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            lockout_duration_minutes: args.get("lockout_duration_minutes").and_then(|v| value_as_i64(v)).map(|v| v as i32),
        };
        let result = client.password_policies.create_password_policy(request).await
            .map_err(|e| anyhow!("Failed to create password policy: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_update_password_policy(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let policy_id = args.get("policy_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("policy_id is required"))?;
        let request = crate::models::password_policies::UpdatePasswordPolicyRequest {
            name: args.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            min_length: args.get("min_length").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            require_uppercase: args.get("require_uppercase").and_then(|v| v.as_bool()),
            require_lowercase: args.get("require_lowercase").and_then(|v| v.as_bool()),
            require_numbers: args.get("require_numbers").and_then(|v| v.as_bool()),
            require_special_chars: args.get("require_special_chars").and_then(|v| v.as_bool()),
            password_history: args.get("password_history").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            expiration_days: args.get("expiration_days").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            max_failed_attempts: args.get("max_failed_attempts").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            lockout_duration_minutes: args.get("lockout_duration_minutes").and_then(|v| value_as_i64(v)).map(|v| v as i32),
        };
        let result = client.password_policies.update_password_policy(policy_id, request).await
            .map_err(|e| anyhow!("Failed to update password policy: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    // ===== CERTIFICATES API =====
    fn tool_list_certificates(&self) -> Value {
        json!({
            "name": "onelogin_list_certificates",
            "description": "List all X.509 certificates",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_certificate(&self) -> Value {
        json!({
            "name": "onelogin_get_certificate",
            "description": "Get a specific certificate by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cert_id": {
                        "type": "integer",
                        "description": "The certificate ID"
                    }
                },
                "required": ["cert_id"]
            }
        })
    }

    fn tool_generate_certificate(&self) -> Value {
        json!({
            "name": "onelogin_generate_certificate",
            "description": "Generate a new X.509 certificate",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Certificate name"
                    },
                    "valid_years": {
                        "type": "integer",
                        "description": "Number of years the certificate is valid"
                    },
                    "common_name": {
                        "type": "string",
                        "description": "Common name for the certificate"
                    }
                },
                "required": ["name"]
            }
        })
    }

    fn tool_renew_certificate(&self) -> Value {
        json!({
            "name": "onelogin_renew_certificate",
            "description": "Renew an expiring certificate",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cert_id": {
                        "type": "integer",
                        "description": "The certificate ID to renew"
                    }
                },
                "required": ["cert_id"]
            }
        })
    }

    async fn handle_list_certificates(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let result = client.certificates.list_certificates().await
            .map_err(|e| anyhow!("Failed to list certificates: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_certificate(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let cert_id = args.get("cert_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("cert_id is required"))?;
        let result = client.certificates.get_certificate(cert_id).await
            .map_err(|e| anyhow!("Failed to get certificate: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_generate_certificate(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let request = crate::models::certificates::GenerateCertificateRequest {
            name: args.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            validity_years: args.get("validity_years").and_then(|v| value_as_i64(v)).map(|v| v as i32),
        };
        let result = client.certificates.generate_certificate(request).await
            .map_err(|e| anyhow!("Failed to generate certificate: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_renew_certificate(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let cert_id = args.get("cert_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("cert_id is required"))?;
        let result = client.certificates.renew_certificate(cert_id).await
            .map_err(|e| anyhow!("Failed to renew certificate: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    // ===== DEVICE TRUST API =====
    fn tool_list_devices(&self) -> Value {
        json!({
            "name": "onelogin_list_devices",
            "description": "List trusted devices with optional filters",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "Filter by user ID"
                    },
                    "device_type": {
                        "type": "string",
                        "description": "Filter by device type"
                    },
                    "status": {
                        "type": "string",
                        "description": "Filter by status"
                    }
                }
            }
        })
    }

    fn tool_get_device(&self) -> Value {
        json!({
            "name": "onelogin_get_device",
            "description": "Get a specific trusted device by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "device_id": {
                        "type": "integer",
                        "description": "The device ID"
                    }
                },
                "required": ["device_id"]
            }
        })
    }

    fn tool_register_device(&self) -> Value {
        json!({
            "name": "onelogin_register_device",
            "description": "Register a new trusted device",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "User ID to associate the device with"
                    },
                    "device_type": {
                        "type": "string",
                        "description": "Type of device (e.g., mobile, desktop)"
                    },
                    "device_name": {
                        "type": "string",
                        "description": "Friendly name for the device"
                    },
                    "platform": {
                        "type": "string",
                        "description": "Platform (e.g., iOS, Android, Windows)"
                    }
                },
                "required": ["user_id", "device_type"]
            }
        })
    }

    fn tool_update_device(&self) -> Value {
        json!({
            "name": "onelogin_update_device",
            "description": "Update a trusted device configuration",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "device_id": {
                        "type": "integer",
                        "description": "The device ID"
                    },
                    "device_name": {
                        "type": "string",
                        "description": "Updated device name"
                    },
                    "status": {
                        "type": "string",
                        "description": "Updated status"
                    }
                },
                "required": ["device_id"]
            }
        })
    }

    fn tool_delete_device(&self) -> Value {
        json!({
            "name": "onelogin_delete_device",
            "description": "Remove a trusted device",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "device_id": {
                        "type": "integer",
                        "description": "The device ID to remove"
                    }
                },
                "required": ["device_id"]
            }
        })
    }

    async fn handle_list_devices(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let query = crate::models::device_trust::DeviceQuery {
            user_id: args.get("user_id").and_then(|v| value_as_i64(v)),
            device_type: args.get("device_type").and_then(|v| v.as_str()).map(|s| s.to_string()),
            limit: args.get("limit").and_then(|v| value_as_i64(v)).map(|v| v as i32),
            page: args.get("page").and_then(|v| value_as_i64(v)).map(|v| v as i32),
        };
        let result = client.device_trust.list_devices(query).await
            .map_err(|e| anyhow!("Failed to list devices: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_device(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let device_id = args.get("device_id").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("device_id is required"))?;
        let result = client.device_trust.get_device(device_id).await
            .map_err(|e| anyhow!("Failed to get device: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_register_device(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args.get("user_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let device_name = args.get("device_name").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("device_name is required"))?;
        let request = crate::models::device_trust::RegisterDeviceRequest {
            user_id,
            device_name: device_name.to_string(),
            device_type: args.get("device_type").and_then(|v| v.as_str()).map(|s| s.to_string()),
            platform: args.get("platform").and_then(|v| v.as_str()).map(|s| s.to_string()),
            browser: args.get("browser").and_then(|v| v.as_str()).map(|s| s.to_string()),
        };
        let result = client.device_trust.register_device(request).await
            .map_err(|e| anyhow!("Failed to register device: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_update_device(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let device_id = args.get("device_id").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("device_id is required"))?;
        let request = crate::models::device_trust::UpdateDeviceRequest {
            device_name: args.get("device_name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            trust_level: args.get("trust_level").and_then(|v| v.as_str()).map(|s| s.to_string()),
        };
        let result = client.device_trust.update_device(device_id, request).await
            .map_err(|e| anyhow!("Failed to update device: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_delete_device(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let device_id = args.get("device_id").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("device_id is required"))?;
        client.device_trust.delete_device(device_id).await
            .map_err(|e| anyhow!("Failed to delete device: {}", e))?;
        Ok(json!({"success": true}))
    }

    // ===== LOGIN PAGES API =====
    fn tool_list_login_pages(&self) -> Value {
        json!({
            "name": "onelogin_list_login_pages",
            "description": "List all custom login pages",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_login_page(&self) -> Value {
        json!({
            "name": "onelogin_get_login_page",
            "description": "Get a specific login page by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "page_id": {
                        "type": "integer",
                        "description": "The login page ID"
                    }
                },
                "required": ["page_id"]
            }
        })
    }

    fn tool_create_login_page(&self) -> Value {
        json!({
            "name": "onelogin_create_login_page",
            "description": "Create a new custom login page",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Login page name"
                    },
                    "html": {
                        "type": "string",
                        "description": "HTML content for the login page"
                    },
                    "css": {
                        "type": "string",
                        "description": "CSS styles for the login page"
                    },
                    "javascript": {
                        "type": "string",
                        "description": "JavaScript for the login page"
                    }
                },
                "required": ["name"]
            }
        })
    }

    fn tool_update_login_page(&self) -> Value {
        json!({
            "name": "onelogin_update_login_page",
            "description": "Update an existing login page",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "page_id": {
                        "type": "integer",
                        "description": "The login page ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "Login page name"
                    },
                    "html": {
                        "type": "string",
                        "description": "HTML content for the login page"
                    },
                    "css": {
                        "type": "string",
                        "description": "CSS styles for the login page"
                    },
                    "javascript": {
                        "type": "string",
                        "description": "JavaScript for the login page"
                    }
                },
                "required": ["page_id"]
            }
        })
    }

    fn tool_delete_login_page(&self) -> Value {
        json!({
            "name": "onelogin_delete_login_page",
            "description": "Delete a custom login page",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "page_id": {
                        "type": "integer",
                        "description": "The login page ID to delete"
                    }
                },
                "required": ["page_id"]
            }
        })
    }

    async fn handle_list_login_pages(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let result = client.login_pages.list_login_pages().await
            .map_err(|e| anyhow!("Failed to list login pages: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_login_page(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let page_id = args.get("page_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("page_id is required"))?;
        let result = client.login_pages.get_login_page(page_id).await
            .map_err(|e| anyhow!("Failed to get login page: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_create_login_page(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let name = args.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("name is required"))?;
        let request = crate::models::login_pages::CreateLoginPageRequest {
            name: name.to_string(),
            html_content: args.get("html_content").and_then(|v| v.as_str()).map(|s| s.to_string()),
            css_content: args.get("css_content").and_then(|v| v.as_str()).map(|s| s.to_string()),
            javascript_content: args.get("javascript_content").and_then(|v| v.as_str()).map(|s| s.to_string()),
            subdomain: args.get("subdomain").and_then(|v| v.as_str()).map(|s| s.to_string()),
            enabled: args.get("enabled").and_then(|v| v.as_bool()),
        };
        let result = client.login_pages.create_login_page(request).await
            .map_err(|e| anyhow!("Failed to create login page: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_update_login_page(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let page_id = args.get("page_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("page_id is required"))?;
        let request = crate::models::login_pages::UpdateLoginPageRequest {
            name: args.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            html_content: args.get("html_content").and_then(|v| v.as_str()).map(|s| s.to_string()),
            css_content: args.get("css_content").and_then(|v| v.as_str()).map(|s| s.to_string()),
            javascript_content: args.get("javascript_content").and_then(|v| v.as_str()).map(|s| s.to_string()),
            subdomain: args.get("subdomain").and_then(|v| v.as_str()).map(|s| s.to_string()),
            enabled: args.get("enabled").and_then(|v| v.as_bool()),
        };
        let result = client.login_pages.update_login_page(page_id, request).await
            .map_err(|e| anyhow!("Failed to update login page: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_delete_login_page(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        client.login_pages.delete_login_page(
            args.get("page_id").and_then(|v| value_as_i64(v))
                .ok_or_else(|| anyhow!("page_id is required"))?
        ).await.map_err(|e| anyhow!("Failed to delete login page: {}", e))?;
        Ok(json!({"success": true}))
    }

    // ===== TRUSTED IDPS API =====
    fn tool_list_trusted_idps(&self) -> Value {
        json!({
            "name": "onelogin_list_trusted_idps",
            "description": "List all trusted identity providers (federation)",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_trusted_idp(&self) -> Value {
        json!({
            "name": "onelogin_get_trusted_idp",
            "description": "Get a specific trusted IDP by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "idp_id": {
                        "type": "integer",
                        "description": "The trusted IDP ID"
                    }
                },
                "required": ["idp_id"]
            }
        })
    }

    fn tool_create_trusted_idp(&self) -> Value {
        json!({
            "name": "onelogin_create_trusted_idp",
            "description": "Create a new trusted IDP (SAML or OIDC)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "IDP name"
                    },
                    "idp_type": {
                        "type": "string",
                        "description": "Type of IDP (saml, oidc)"
                    },
                    "issuer_url": {
                        "type": "string",
                        "description": "IDP issuer URL"
                    },
                    "sso_url": {
                        "type": "string",
                        "description": "Single sign-on URL"
                    },
                    "certificate": {
                        "type": "string",
                        "description": "X.509 certificate for SAML"
                    }
                },
                "required": ["name", "idp_type"]
            }
        })
    }

    fn tool_update_trusted_idp(&self) -> Value {
        json!({
            "name": "onelogin_update_trusted_idp",
            "description": "Update a trusted IDP configuration",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "idp_id": {
                        "type": "integer",
                        "description": "The trusted IDP ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "IDP name"
                    },
                    "issuer_url": {
                        "type": "string",
                        "description": "IDP issuer URL"
                    },
                    "sso_url": {
                        "type": "string",
                        "description": "Single sign-on URL"
                    },
                    "certificate": {
                        "type": "string",
                        "description": "X.509 certificate for SAML"
                    }
                },
                "required": ["idp_id"]
            }
        })
    }

    fn tool_delete_trusted_idp(&self) -> Value {
        json!({
            "name": "onelogin_delete_trusted_idp",
            "description": "Delete a trusted IDP",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "idp_id": {
                        "type": "integer",
                        "description": "The trusted IDP ID to delete"
                    }
                },
                "required": ["idp_id"]
            }
        })
    }

    fn tool_get_trusted_idp_metadata(&self) -> Value {
        json!({
            "name": "onelogin_get_trusted_idp_metadata",
            "description": "Get SAML metadata for a trusted IDP",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "idp_id": {
                        "type": "integer",
                        "description": "The trusted IDP ID"
                    }
                },
                "required": ["idp_id"]
            }
        })
    }

    fn tool_update_trusted_idp_metadata(&self) -> Value {
        json!({
            "name": "onelogin_update_trusted_idp_metadata",
            "description": "Upload/update SAML metadata XML for a trusted IDP",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "idp_id": {
                        "type": "integer",
                        "description": "The trusted IDP ID"
                    },
                    "metadata_xml": {
                        "type": "string",
                        "description": "SAML metadata XML content"
                    }
                },
                "required": ["idp_id", "metadata_xml"]
            }
        })
    }

    fn tool_get_trusted_idp_issuer(&self) -> Value {
        json!({
            "name": "onelogin_get_trusted_idp_issuer",
            "description": "Get the issuer URL for a trusted IDP",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "idp_id": {
                        "type": "integer",
                        "description": "The trusted IDP ID"
                    }
                },
                "required": ["idp_id"]
            }
        })
    }

    async fn handle_list_trusted_idps(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let result = client.trusted_idps.list_trusted_idps().await
            .map_err(|e| anyhow!("Failed to list trusted IDPs: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_trusted_idp(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let idp_id = args.get("idp_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("idp_id is required"))?;
        let result = client.trusted_idps.get_trusted_idp(idp_id).await
            .map_err(|e| anyhow!("Failed to get trusted IDP: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_create_trusted_idp(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let name = args.get("name").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("name is required"))?;
        let idp_type = args.get("idp_type").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("idp_type is required"))?;
        let request = crate::models::trusted_idps::CreateTrustedIdpRequest {
            name: name.to_string(),
            idp_type: idp_type.to_string(),
            enabled: args.get("enabled").and_then(|v| v.as_bool()),
            issuer: args.get("issuer").and_then(|v| v.as_str()).map(|s| s.to_string()),
            sso_endpoint: args.get("sso_endpoint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            slo_endpoint: args.get("slo_endpoint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            certificate: args.get("certificate").and_then(|v| v.as_str()).map(|s| s.to_string()),
            client_id: args.get("client_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
            client_secret: args.get("client_secret").and_then(|v| v.as_str()).map(|s| s.to_string()),
            authorization_endpoint: args.get("authorization_endpoint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            token_endpoint: args.get("token_endpoint").and_then(|v| v.as_str()).map(|s| s.to_string()),
        };
        let result = client.trusted_idps.create_trusted_idp(request).await
            .map_err(|e| anyhow!("Failed to create trusted IDP: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_update_trusted_idp(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let idp_id = args.get("idp_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("idp_id is required"))?;
        let request = crate::models::trusted_idps::UpdateTrustedIdpRequest {
            name: args.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
            enabled: args.get("enabled").and_then(|v| v.as_bool()),
            issuer: args.get("issuer").and_then(|v| v.as_str()).map(|s| s.to_string()),
            sso_endpoint: args.get("sso_endpoint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            slo_endpoint: args.get("slo_endpoint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            certificate: args.get("certificate").and_then(|v| v.as_str()).map(|s| s.to_string()),
        };
        let result = client.trusted_idps.update_trusted_idp(idp_id, request).await
            .map_err(|e| anyhow!("Failed to update trusted IDP: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_delete_trusted_idp(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        client.trusted_idps.delete_trusted_idp(
            args.get("idp_id").and_then(|v| value_as_i64(v))
                .ok_or_else(|| anyhow!("idp_id is required"))?
        ).await.map_err(|e| anyhow!("Failed to delete trusted IDP: {}", e))?;
        Ok(json!({"success": true}))
    }

    async fn handle_get_trusted_idp_metadata(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let idp_id = args.get("idp_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("idp_id is required"))?;
        let result = client.trusted_idps.get_trusted_idp_metadata(idp_id).await
            .map_err(|e| anyhow!("Failed to get trusted IDP metadata: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_update_trusted_idp_metadata(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let idp_id = args.get("idp_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("idp_id is required"))?;
        let metadata = args.get("metadata").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("metadata is required"))?;
        let request = crate::models::trusted_idps::UpdateTrustedIdpMetadataRequest {
            metadata: metadata.to_string(),
        };
        let result = client.trusted_idps.update_trusted_idp_metadata(idp_id, request).await
            .map_err(|e| anyhow!("Failed to update trusted IDP metadata: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_trusted_idp_issuer(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let idp_id = args.get("idp_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("idp_id is required"))?;
        let result = client.trusted_idps.get_trusted_idp_issuer(idp_id).await
            .map_err(|e| anyhow!("Failed to get trusted IDP issuer: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    // ===== ROLES API (EXPANDED - SUB-RESOURCES) =====
    fn tool_get_role_apps(&self) -> Value {
        json!({
            "name": "onelogin_get_role_apps",
            "description": "Get the list of apps assigned to a specific role. Use this to see which applications users with this role can access.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role"
                    }
                },
                "required": ["role_id"]
            }
        })
    }

    fn tool_set_role_apps(&self) -> Value {
        json!({
            "name": "onelogin_set_role_apps",
            "description": "Set the apps assigned to a role. WARNING: This REPLACES all existing app assignments - provide the complete list of app IDs you want assigned. This is the ONLY way to modify apps on a role (onelogin_update_role cannot do this).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role"
                    },
                    "app_ids": {
                        "type": "array",
                        "items": {"type": "integer"},
                        "description": "Complete array of app IDs to assign. This replaces ALL existing apps on the role."
                    }
                },
                "required": ["role_id", "app_ids"]
            }
        })
    }

    fn tool_get_role_users(&self) -> Value {
        json!({
            "name": "onelogin_get_role_users",
            "description": "Get the list of users assigned to a specific role. Note: To assign users to a role, use onelogin_assign_roles_to_user instead.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role"
                    }
                },
                "required": ["role_id"]
            }
        })
    }

    fn tool_get_role_admins(&self) -> Value {
        json!({
            "name": "onelogin_get_role_admins",
            "description": "Get the list of administrators for a specific role. Admins can manage the role's configuration and user assignments.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role"
                    }
                },
                "required": ["role_id"]
            }
        })
    }

    fn tool_add_role_admins(&self) -> Value {
        json!({
            "name": "onelogin_add_role_admins",
            "description": "Add administrators to a role. This is the ONLY way to assign admins to a role (onelogin_update_role and onelogin_create_role cannot do this). Admins can manage the role's configuration.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role"
                    },
                    "admin_ids": {
                        "type": "array",
                        "items": {"type": "integer"},
                        "description": "Array of user IDs to add as administrators of this role"
                    }
                },
                "required": ["role_id", "admin_ids"]
            }
        })
    }

    fn tool_remove_role_admin(&self) -> Value {
        json!({
            "name": "onelogin_remove_role_admin",
            "description": "Remove an administrator from a role. This removes their ability to manage the role.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {
                        "type": "integer",
                        "description": "The unique ID of the role"
                    },
                    "admin_id": {
                        "type": "integer",
                        "description": "The user ID of the admin to remove from the role"
                    }
                },
                "required": ["role_id", "admin_id"]
            }
        })
    }

    fn tool_assign_roles_to_user(&self) -> Value {
        json!({
            "name": "onelogin_assign_roles_to_user",
            "description": "Assign one or more roles to a user. This is the ONLY way to add users to a role (onelogin_update_role and onelogin_create_role cannot do this). The roles will be added to the user's existing roles.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user to assign roles to"
                    },
                    "role_ids": {
                        "type": "array",
                        "items": {"type": "integer"},
                        "description": "Array of role IDs to assign to this user"
                    }
                },
                "required": ["user_id", "role_ids"]
            }
        })
    }

    fn tool_remove_roles_from_user(&self) -> Value {
        json!({
            "name": "onelogin_remove_roles_from_user",
            "description": "Remove one or more roles from a user. This removes the user's access to apps associated with those roles.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The unique ID of the user to remove roles from"
                    },
                    "role_ids": {
                        "type": "array",
                        "items": {"type": "integer"},
                        "description": "Array of role IDs to remove from this user"
                    }
                },
                "required": ["user_id", "role_ids"]
            }
        })
    }

    async fn handle_get_role_apps(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id = args.get("role_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let result = client.roles.get_role_apps(role_id).await
            .map_err(|e| anyhow!("Failed to get role apps: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_set_role_apps(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id = args.get("role_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let app_id_array = args.get("app_ids").and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("app_ids is required"))?
            .iter()
            .filter_map(|v| value_as_i64(v))
            .collect();
        let request = crate::models::roles::SetRoleAppsRequest { app_id_array };
        let result = client.roles.set_role_apps(role_id, request).await
            .map_err(|e| anyhow!("Failed to set role apps: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_role_users(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id = args.get("role_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let result = client.roles.get_role_users(role_id).await
            .map_err(|e| anyhow!("Failed to get role users: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_get_role_admins(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id = args.get("role_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let result = client.roles.get_role_admins(role_id).await
            .map_err(|e| anyhow!("Failed to get role admins: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_add_role_admins(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id = args.get("role_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let admin_id_array = args.get("admin_ids").and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("admin_ids is required"))?
            .iter()
            .filter_map(|v| value_as_i64(v))
            .collect();
        let request = crate::models::roles::AddRoleAdminsRequest { admin_id_array };
        client.roles.add_role_admins(role_id, request).await
            .map_err(|e| anyhow!("Failed to add role admins: {}", e))?;
        Ok(json!({"success": true}))
    }

    async fn handle_remove_role_admin(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let role_id = args.get("role_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("role_id is required"))?;
        let admin_id = args.get("admin_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("admin_id is required"))?;
        client.roles.remove_role_admin(role_id, admin_id).await
            .map_err(|e| anyhow!("Failed to remove role admin: {}", e))?;
        Ok(json!({"success": true}))
    }

    async fn handle_assign_roles_to_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args.get("user_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let role_id_array = args.get("role_ids").and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("role_ids is required"))?
            .iter()
            .filter_map(|v| value_as_i64(v))
            .collect();
        let request = crate::models::roles::RoleIdsRequest { role_id_array };
        client.roles.assign_roles_to_user(user_id, request).await
            .map_err(|e| anyhow!("Failed to assign roles to user: {}", e))?;
        Ok(json!({"success": true}))
    }

    async fn handle_remove_roles_from_user(&self, args: &Value) -> Result<Value> {
        let client = self.resolve_client(args)?;
        let user_id = args.get("user_id").and_then(|v| value_as_i64(v))
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let role_id_array = args.get("role_ids").and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("role_ids is required"))?
            .iter()
            .filter_map(|v| value_as_i64(v))
            .collect();
        let request = crate::models::roles::RoleIdsRequest { role_id_array };
        client.roles.remove_roles_from_user(user_id, request).await
            .map_err(|e| anyhow!("Failed to remove roles from user: {}", e))?;
        Ok(json!({"success": true}))
    }

    fn tool_list_tenants(&self) -> Value {
        json!({
            "name": "onelogin_list_tenants",
            "description": "List all configured OneLogin tenants. Shows tenant name, subdomain, region, and which is the default. Use the tenant name as the 'tenant' parameter in other tools to target a specific tenant.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    async fn handle_list_tenants(&self) -> Result<Value> {
        let info = self.tenant_manager.tenant_info();
        Ok(json!({
            "tenants": info,
            "default_tenant": self.tenant_manager.default_tenant_name(),
            "multi_tenant_mode": self.tenant_manager.is_multi_tenant()
        }))
    }

}
