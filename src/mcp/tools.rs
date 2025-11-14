use crate::api::OneLoginClient;
use crate::core::error::Result as OneLoginResult;
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{error, info};

pub struct ToolRegistry {
    client: Arc<OneLoginClient>,
}

impl ToolRegistry {
    pub fn new(client: Arc<OneLoginClient>) -> Self {
        Self { client }
    }

    pub fn list_tools(&self) -> Vec<Value> {
        vec![
            // Users API
            self.tool_list_users(),
            self.tool_get_user(),
            self.tool_create_user(),
            self.tool_update_user(),
            self.tool_delete_user(),
            self.tool_get_user_apps(),
            self.tool_get_user_roles(),
            self.tool_lock_user(),
            self.tool_logout_user(),

            // Apps API
            self.tool_list_apps(),
            self.tool_get_app(),
            self.tool_create_app(),
            self.tool_update_app(),
            self.tool_delete_app(),

            // Roles API
            self.tool_list_roles(),
            self.tool_get_role(),
            self.tool_create_role(),
            self.tool_update_role(),
            self.tool_delete_role(),

            // Groups API
            self.tool_list_groups(),
            self.tool_get_group(),
            self.tool_create_group(),
            self.tool_update_group(),
            self.tool_delete_group(),

            // MFA API
            self.tool_list_mfa_factors(),
            self.tool_enroll_mfa_factor(),
            self.tool_remove_mfa_factor(),
            self.tool_verify_mfa_factor(),

            // SAML API
            self.tool_get_saml_assertion(),
            self.tool_verify_saml_factor(),

            // Smart Hooks API
            self.tool_create_smart_hook(),
            self.tool_update_smart_hook(),
            self.tool_delete_smart_hook(),
            self.tool_get_smart_hook(),
            self.tool_list_smart_hooks(),
            self.tool_get_smart_hook_logs(),
            self.tool_update_hook_env_vars(),

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

            // Policies API
            self.tool_list_policies(),
            self.tool_get_policy(),
            self.tool_create_policy(),
            self.tool_update_policy(),
            self.tool_delete_policy(),
            self.tool_assign_policy_to_user(),

            // Invitations API
            self.tool_generate_invite_link(),
            self.tool_send_invite_link(),
            self.tool_get_invitation(),
            self.tool_cancel_invitation(),
            self.tool_list_pending_invitations(),

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

            // Webhooks API
            self.tool_list_webhook_events(),

            // SCIM API
            self.tool_scim_get_users(),
            self.tool_scim_create_user(),
            self.tool_scim_get_user(),
            self.tool_scim_update_user(),
            self.tool_scim_patch_user(),
            self.tool_scim_delete_user(),
            self.tool_scim_get_groups(),
            self.tool_scim_create_group(),
            self.tool_scim_bulk_operations(),

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

            // Events API
            self.tool_list_events(),
            self.tool_get_event(),
            self.tool_create_event(),

            // Sessions API
            self.tool_list_sessions(),
            self.tool_get_session(),
            self.tool_delete_session(),

            // API Authorization API
            self.tool_list_api_authorizations(),
            self.tool_get_api_authorization(),
            self.tool_create_api_authorization(),
            self.tool_update_api_authorization(),
            self.tool_delete_api_authorization(),
        ]
    }

    pub async fn call_tool(&self, params: &super::server::CallToolParams) -> Result<String> {
        info!("Calling tool: {}", params.name);

        let result = match params.name.as_str() {
            // Users
            "onelogin_list_users" => self.handle_list_users(&params.arguments).await?,
            "onelogin_get_user" => self.handle_get_user(&params.arguments).await?,
            "onelogin_create_user" => self.handle_create_user(&params.arguments).await?,
            "onelogin_update_user" => self.handle_update_user(&params.arguments).await?,
            "onelogin_delete_user" => self.handle_delete_user(&params.arguments).await?,

            // Smart Hooks
            "onelogin_create_smart_hook" => self.handle_create_smart_hook(&params.arguments).await?,
            "onelogin_update_smart_hook" => self.handle_update_smart_hook(&params.arguments).await?,
            "onelogin_list_smart_hooks" => self.handle_list_smart_hooks(&params.arguments).await?,

            // Vigilance
            "onelogin_get_risk_score" => self.handle_get_risk_score(&params.arguments).await?,
            "onelogin_validate_user_smart_mfa" => self.handle_validate_user_smart_mfa(&params.arguments).await?,

            // SCIM
            "onelogin_scim_get_users" => self.handle_scim_get_users(&params.arguments).await?,
            "onelogin_scim_create_user" => self.handle_scim_create_user(&params.arguments).await?,

            // Add more tool handlers...
            _ => return Err(anyhow!("Unknown tool: {}", params.name)),
        };

        Ok(serde_json::to_string_pretty(&result)?)
    }

    // Tool definitions
    fn tool_list_users(&self) -> Value {
        json!({
            "name": "onelogin_list_users",
            "description": "List all users in OneLogin",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "email": {
                        "type": "string",
                        "description": "Filter by email"
                    },
                    "username": {
                        "type": "string",
                        "description": "Filter by username"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results"
                    }
                }
            }
        })
    }

    fn tool_get_user(&self) -> Value {
        json!({
            "name": "onelogin_get_user",
            "description": "Get a specific user by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "integer",
                        "description": "The user ID"
                    }
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_create_user(&self) -> Value {
        json!({
            "name": "onelogin_create_user",
            "description": "Create a new user in OneLogin",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "email": {"type": "string"},
                    "username": {"type": "string"},
                    "firstname": {"type": "string"},
                    "lastname": {"type": "string"},
                    "title": {"type": "string"},
                    "department": {"type": "string"},
                    "company": {"type": "string"},
                    "phone": {"type": "string"}
                },
                "required": ["email", "username"]
            }
        })
    }

    fn tool_update_user(&self) -> Value {
        json!({
            "name": "onelogin_update_user",
            "description": "Update an existing user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"},
                    "email": {"type": "string"},
                    "username": {"type": "string"},
                    "firstname": {"type": "string"},
                    "lastname": {"type": "string"}
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_delete_user(&self) -> Value {
        json!({
            "name": "onelogin_delete_user",
            "description": "Delete a user from OneLogin",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"}
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_get_user_apps(&self) -> Value {
        json!({
            "name": "onelogin_get_user_apps",
            "description": "Get all apps assigned to a user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"}
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_get_user_roles(&self) -> Value {
        json!({
            "name": "onelogin_get_user_roles",
            "description": "Get all roles assigned to a user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"}
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_lock_user(&self) -> Value {
        json!({
            "name": "onelogin_lock_user",
            "description": "Lock a user account for a specified duration",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"},
                    "minutes": {"type": "integer", "description": "Duration in minutes"}
                },
                "required": ["user_id", "minutes"]
            }
        })
    }

    fn tool_logout_user(&self) -> Value {
        json!({
            "name": "onelogin_logout_user",
            "description": "Log out a user from all sessions",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"}
                },
                "required": ["user_id"]
            }
        })
    }

    // Apps API
    fn tool_list_apps(&self) -> Value {
        json!({
            "name": "onelogin_list_apps",
            "description": "List all applications",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_app(&self) -> Value {
        json!({
            "name": "onelogin_get_app",
            "description": "Get a specific app by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {"type": "integer"}
                },
                "required": ["app_id"]
            }
        })
    }

    fn tool_create_app(&self) -> Value {
        json!({
            "name": "onelogin_create_app",
            "description": "Create a new application",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "connector_id": {"type": "integer"},
                    "name": {"type": "string"},
                    "description": {"type": "string"},
                    "visible": {"type": "boolean"}
                },
                "required": ["connector_id", "name"]
            }
        })
    }

    fn tool_update_app(&self) -> Value {
        json!({
            "name": "onelogin_update_app",
            "description": "Update an existing application",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {"type": "integer"},
                    "name": {"type": "string"},
                    "description": {"type": "string"},
                    "visible": {"type": "boolean"}
                },
                "required": ["app_id"]
            }
        })
    }

    fn tool_delete_app(&self) -> Value {
        json!({
            "name": "onelogin_delete_app",
            "description": "Delete an application",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {"type": "integer"}
                },
                "required": ["app_id"]
            }
        })
    }

    // Roles API
    fn tool_list_roles(&self) -> Value {
        json!({
            "name": "onelogin_list_roles",
            "description": "List all roles",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_role(&self) -> Value {
        json!({
            "name": "onelogin_get_role",
            "description": "Get a specific role by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {"type": "integer"}
                },
                "required": ["role_id"]
            }
        })
    }

    fn tool_create_role(&self) -> Value {
        json!({
            "name": "onelogin_create_role",
            "description": "Create a new role",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "description": {"type": "string"}
                },
                "required": ["name"]
            }
        })
    }

    fn tool_update_role(&self) -> Value {
        json!({
            "name": "onelogin_update_role",
            "description": "Update an existing role",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {"type": "integer"},
                    "name": {"type": "string"},
                    "description": {"type": "string"}
                },
                "required": ["role_id"]
            }
        })
    }

    fn tool_delete_role(&self) -> Value {
        json!({
            "name": "onelogin_delete_role",
            "description": "Delete a role",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "role_id": {"type": "integer"}
                },
                "required": ["role_id"]
            }
        })
    }

    // Groups API
    fn tool_list_groups(&self) -> Value {
        json!({
            "name": "onelogin_list_groups",
            "description": "List all groups",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_group(&self) -> Value {
        json!({
            "name": "onelogin_get_group",
            "description": "Get a specific group by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "group_id": {"type": "integer"}
                },
                "required": ["group_id"]
            }
        })
    }

    fn tool_create_group(&self) -> Value {
        json!({
            "name": "onelogin_create_group",
            "description": "Create a new group",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "reference": {"type": "string"}
                },
                "required": ["name"]
            }
        })
    }

    fn tool_update_group(&self) -> Value {
        json!({
            "name": "onelogin_update_group",
            "description": "Update an existing group",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "group_id": {"type": "integer"},
                    "name": {"type": "string"},
                    "reference": {"type": "string"}
                },
                "required": ["group_id"]
            }
        })
    }

    fn tool_delete_group(&self) -> Value {
        json!({
            "name": "onelogin_delete_group",
            "description": "Delete a group",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "group_id": {"type": "integer"}
                },
                "required": ["group_id"]
            }
        })
    }

    // MFA API
    fn tool_list_mfa_factors(&self) -> Value {
        json!({
            "name": "onelogin_list_mfa_factors",
            "description": "List MFA devices for a user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"}
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_enroll_mfa_factor(&self) -> Value {
        json!({
            "name": "onelogin_enroll_mfa_factor",
            "description": "Enroll a new MFA device for a user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"},
                    "device_type": {"type": "string"},
                    "phone_number": {"type": "string"}
                },
                "required": ["user_id", "device_type"]
            }
        })
    }

    fn tool_remove_mfa_factor(&self) -> Value {
        json!({
            "name": "onelogin_remove_mfa_factor",
            "description": "Remove an MFA device from a user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"},
                    "device_id": {"type": "integer"}
                },
                "required": ["user_id", "device_id"]
            }
        })
    }

    fn tool_verify_mfa_factor(&self) -> Value {
        json!({
            "name": "onelogin_verify_mfa_factor",
            "description": "Verify an MFA code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "integer"},
                    "state_token": {"type": "string"},
                    "device_id": {"type": "integer"},
                    "otp_code": {"type": "string"}
                },
                "required": ["user_id", "state_token", "device_id", "otp_code"]
            }
        })
    }

    // SAML API
    fn tool_get_saml_assertion(&self) -> Value {
        json!({
            "name": "onelogin_get_saml_assertion",
            "description": "Get SAML assertion for SSO",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "username_or_email": {"type": "string"},
                    "password": {"type": "string"},
                    "app_id": {"type": "string"},
                    "subdomain": {"type": "string"}
                },
                "required": ["username_or_email", "password", "app_id", "subdomain"]
            }
        })
    }

    fn tool_verify_saml_factor(&self) -> Value {
        json!({
            "name": "onelogin_verify_saml_factor",
            "description": "Verify MFA for SAML assertion",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {"type": "string"},
                    "device_id": {"type": "string"},
                    "state_token": {"type": "string"},
                    "otp_token": {"type": "string"}
                },
                "required": ["app_id", "device_id", "state_token"]
            }
        })
    }

    // Smart Hooks API
    fn tool_create_smart_hook(&self) -> Value {
        json!({
            "name": "onelogin_create_smart_hook",
            "description": "Create a Smart Hook for custom authentication logic",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": ["pre-authentication", "user-migration"],
                        "description": "Type of smart hook"
                    },
                    "function": {
                        "type": "string",
                        "description": "Base64 encoded JavaScript function"
                    },
                    "runtime": {
                        "type": "string",
                        "default": "nodejs18.x"
                    },
                    "options": {
                        "type": "object",
                        "properties": {
                            "risk_enabled": {"type": "boolean"},
                            "location_enabled": {"type": "boolean"},
                            "mfa_device_info_enabled": {"type": "boolean"}
                        }
                    },
                    "env_vars": {
                        "type": "array",
                        "items": {"type": "string"}
                    },
                    "packages": {
                        "type": "object",
                        "description": "NPM packages to include"
                    }
                },
                "required": ["type", "function"]
            }
        })
    }

    fn tool_update_smart_hook(&self) -> Value {
        json!({
            "name": "onelogin_update_smart_hook",
            "description": "Update an existing Smart Hook",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {"type": "string"},
                    "status": {"type": "string"},
                    "function": {"type": "string"},
                    "runtime": {"type": "string"}
                },
                "required": ["hook_id"]
            }
        })
    }

    fn tool_delete_smart_hook(&self) -> Value {
        json!({
            "name": "onelogin_delete_smart_hook",
            "description": "Delete a Smart Hook",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {"type": "string"}
                },
                "required": ["hook_id"]
            }
        })
    }

    fn tool_get_smart_hook(&self) -> Value {
        json!({
            "name": "onelogin_get_smart_hook",
            "description": "Get details of a specific Smart Hook",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {"type": "string"}
                },
                "required": ["hook_id"]
            }
        })
    }

    fn tool_list_smart_hooks(&self) -> Value {
        json!({
            "name": "onelogin_list_smart_hooks",
            "description": "List all Smart Hooks",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_get_smart_hook_logs(&self) -> Value {
        json!({
            "name": "onelogin_get_smart_hook_logs",
            "description": "Get execution logs for a Smart Hook",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {"type": "string"}
                },
                "required": ["hook_id"]
            }
        })
    }

    fn tool_update_hook_env_vars(&self) -> Value {
        json!({
            "name": "onelogin_update_hook_env_vars",
            "description": "Update environment variables for a Smart Hook",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "hook_id": {"type": "string"},
                    "env_vars": {"type": "object"}
                },
                "required": ["hook_id", "env_vars"]
            }
        })
    }

    // Vigilance/Risk API
    fn tool_get_risk_score(&self) -> Value {
        json!({
            "name": "onelogin_get_risk_score",
            "description": "Get real-time risk score for a user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_identifier": {"type": "string"},
                    "ip_address": {"type": "string"},
                    "user_agent": {"type": "string"}
                },
                "required": ["user_identifier", "ip_address", "user_agent"]
            }
        })
    }

    fn tool_validate_user_smart_mfa(&self) -> Value {
        json!({
            "name": "onelogin_validate_user_smart_mfa",
            "description": "Validate user risk and trigger Smart MFA if needed",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_identifier": {"type": "string"},
                    "phone": {"type": "string"},
                    "email": {"type": "string"},
                    "context": {
                        "type": "object",
                        "properties": {
                            "ip_address": {"type": "string"},
                            "user_agent": {"type": "string"}
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
            "description": "List all risk rules",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    fn tool_create_risk_rule(&self) -> Value {
        json!({
            "name": "onelogin_create_risk_rule",
            "description": "Create a new risk rule",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "description": {"type": "string"},
                    "enabled": {"type": "boolean"},
                    "conditions": {"type": "array"},
                    "action": {"type": "object"},
                    "priority": {"type": "integer"}
                },
                "required": ["name", "enabled", "conditions", "action", "priority"]
            }
        })
    }

    fn tool_update_risk_rule(&self) -> Value {
        json!({
            "name": "onelogin_update_risk_rule",
            "description": "Update an existing risk rule",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "rule_id": {"type": "string"},
                    "name": {"type": "string"},
                    "enabled": {"type": "boolean"}
                },
                "required": ["rule_id"]
            }
        })
    }

    fn tool_delete_risk_rule(&self) -> Value {
        json!({
            "name": "onelogin_delete_risk_rule",
            "description": "Delete a risk rule",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "rule_id": {"type": "string"}
                },
                "required": ["rule_id"]
            }
        })
    }

    fn tool_get_risk_events(&self) -> Value {
        json!({
            "name": "onelogin_get_risk_events",
            "description": "Get risk events for a user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "string"}
                },
                "required": ["user_id"]
            }
        })
    }

    fn tool_track_risk_event(&self) -> Value {
        json!({
            "name": "onelogin_track_risk_event",
            "description": "Track a custom risk event",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {"type": "string"},
                    "event_type": {"type": "string"},
                    "risk_score": {"type": "integer"},
                    "details": {"type": "object"}
                },
                "required": ["user_id", "event_type", "risk_score"]
            }
        })
    }

    // Continuing with remaining tool definitions...
    // (Abbreviated for space - would include all 100+ tools)

    fn tool_list_privileges(&self) -> Value {
        json!({"name": "onelogin_list_privileges", "description": "List all privileges", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_get_privilege(&self) -> Value {
        json!({"name": "onelogin_get_privilege", "description": "Get a specific privilege", "inputSchema": {"type": "object", "properties": {"privilege_id": {"type": "string"}}, "required": ["privilege_id"]}})
    }

    fn tool_create_privilege(&self) -> Value {
        json!({"name": "onelogin_create_privilege", "description": "Create a new privilege", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}, "resource_type": {"type": "string"}, "actions": {"type": "array"}}, "required": ["name", "resource_type", "actions"]}})
    }

    fn tool_update_privilege(&self) -> Value {
        json!({"name": "onelogin_update_privilege", "description": "Update a privilege", "inputSchema": {"type": "object", "properties": {"privilege_id": {"type": "string"}}, "required": ["privilege_id"]}})
    }

    fn tool_delete_privilege(&self) -> Value {
        json!({"name": "onelogin_delete_privilege", "description": "Delete a privilege", "inputSchema": {"type": "object", "properties": {"privilege_id": {"type": "string"}}, "required": ["privilege_id"]}})
    }

    fn tool_assign_privilege_to_user(&self) -> Value {
        json!({"name": "onelogin_assign_privilege_to_user", "description": "Assign privilege to user", "inputSchema": {"type": "object", "properties": {"privilege_id": {"type": "string"}, "user_id": {"type": "integer"}}, "required": ["privilege_id", "user_id"]}})
    }

    fn tool_assign_privilege_to_role(&self) -> Value {
        json!({"name": "onelogin_assign_privilege_to_role", "description": "Assign privilege to role", "inputSchema": {"type": "object", "properties": {"privilege_id": {"type": "string"}, "role_id": {"type": "integer"}}, "required": ["privilege_id", "role_id"]}})
    }

    fn tool_list_user_mappings(&self) -> Value {
        json!({"name": "onelogin_list_user_mappings", "description": "List all user mappings", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_get_user_mapping(&self) -> Value {
        json!({"name": "onelogin_get_user_mapping", "description": "Get a user mapping", "inputSchema": {"type": "object", "properties": {"mapping_id": {"type": "string"}}, "required": ["mapping_id"]}})
    }

    fn tool_create_user_mapping(&self) -> Value {
        json!({"name": "onelogin_create_user_mapping", "description": "Create a user mapping", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}, "match_type": {"type": "string"}, "rules": {"type": "array"}, "actions": {"type": "array"}}, "required": ["name", "match_type", "rules", "actions"]}})
    }

    fn tool_update_user_mapping(&self) -> Value {
        json!({"name": "onelogin_update_user_mapping", "description": "Update a user mapping", "inputSchema": {"type": "object", "properties": {"mapping_id": {"type": "string"}}, "required": ["mapping_id"]}})
    }

    fn tool_delete_user_mapping(&self) -> Value {
        json!({"name": "onelogin_delete_user_mapping", "description": "Delete a user mapping", "inputSchema": {"type": "object", "properties": {"mapping_id": {"type": "string"}}, "required": ["mapping_id"]}})
    }

    fn tool_sort_user_mappings(&self) -> Value {
        json!({"name": "onelogin_sort_user_mappings", "description": "Sort user mappings order", "inputSchema": {"type": "object", "properties": {"mapping_ids": {"type": "array"}}, "required": ["mapping_ids"]}})
    }

    fn tool_list_policies(&self) -> Value {
        json!({"name": "onelogin_list_policies", "description": "List all policies", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_get_policy(&self) -> Value {
        json!({"name": "onelogin_get_policy", "description": "Get a policy", "inputSchema": {"type": "object", "properties": {"policy_id": {"type": "string"}}, "required": ["policy_id"]}})
    }

    fn tool_create_policy(&self) -> Value {
        json!({"name": "onelogin_create_policy", "description": "Create a policy", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}, "policy_type": {"type": "string"}, "conditions": {"type": "array"}, "actions": {"type": "array"}}, "required": ["name", "policy_type", "conditions", "actions"]}})
    }

    fn tool_update_policy(&self) -> Value {
        json!({"name": "onelogin_update_policy", "description": "Update a policy", "inputSchema": {"type": "object", "properties": {"policy_id": {"type": "string"}}, "required": ["policy_id"]}})
    }

    fn tool_delete_policy(&self) -> Value {
        json!({"name": "onelogin_delete_policy", "description": "Delete a policy", "inputSchema": {"type": "object", "properties": {"policy_id": {"type": "string"}}, "required": ["policy_id"]}})
    }

    fn tool_assign_policy_to_user(&self) -> Value {
        json!({"name": "onelogin_assign_policy_to_user", "description": "Assign policy to user", "inputSchema": {"type": "object", "properties": {"policy_id": {"type": "string"}, "user_id": {"type": "integer"}}, "required": ["policy_id", "user_id"]}})
    }

    fn tool_generate_invite_link(&self) -> Value {
        json!({"name": "onelogin_generate_invite_link", "description": "Generate an invitation link", "inputSchema": {"type": "object", "properties": {"email": {"type": "string"}}, "required": ["email"]}})
    }

    fn tool_send_invite_link(&self) -> Value {
        json!({"name": "onelogin_send_invite_link", "description": "Send an invitation link", "inputSchema": {"type": "object", "properties": {"email": {"type": "string"}}, "required": ["email"]}})
    }

    fn tool_get_invitation(&self) -> Value {
        json!({"name": "onelogin_get_invitation", "description": "Get an invitation", "inputSchema": {"type": "object", "properties": {"invitation_id": {"type": "string"}}, "required": ["invitation_id"]}})
    }

    fn tool_cancel_invitation(&self) -> Value {
        json!({"name": "onelogin_cancel_invitation", "description": "Cancel an invitation", "inputSchema": {"type": "object", "properties": {"invitation_id": {"type": "string"}}, "required": ["invitation_id"]}})
    }

    fn tool_list_pending_invitations(&self) -> Value {
        json!({"name": "onelogin_list_pending_invitations", "description": "List pending invitations", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_list_custom_attributes(&self) -> Value {
        json!({"name": "onelogin_list_custom_attributes", "description": "List custom attributes", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_create_custom_attribute(&self) -> Value {
        json!({"name": "onelogin_create_custom_attribute", "description": "Create a custom attribute", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}, "shortname": {"type": "string"}, "data_type": {"type": "string"}}, "required": ["name", "shortname", "data_type"]}})
    }

    fn tool_update_custom_attribute(&self) -> Value {
        json!({"name": "onelogin_update_custom_attribute", "description": "Update a custom attribute", "inputSchema": {"type": "object", "properties": {"attribute_id": {"type": "integer"}}, "required": ["attribute_id"]}})
    }

    fn tool_delete_custom_attribute(&self) -> Value {
        json!({"name": "onelogin_delete_custom_attribute", "description": "Delete a custom attribute", "inputSchema": {"type": "object", "properties": {"attribute_id": {"type": "integer"}}, "required": ["attribute_id"]}})
    }

    fn tool_generate_embed_token(&self) -> Value {
        json!({"name": "onelogin_generate_embed_token", "description": "Generate an embed token for SSO", "inputSchema": {"type": "object", "properties": {"email": {"type": "string"}, "session_duration": {"type": "integer"}}, "required": ["email"]}})
    }

    fn tool_list_embeddable_apps(&self) -> Value {
        json!({"name": "onelogin_list_embeddable_apps", "description": "List embeddable apps", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_generate_oauth_tokens(&self) -> Value {
        json!({"name": "onelogin_generate_oauth_tokens", "description": "Generate OAuth tokens", "inputSchema": {"type": "object", "properties": {"grant_type": {"type": "string"}}, "required": ["grant_type"]}})
    }

    fn tool_revoke_oauth_token(&self) -> Value {
        json!({"name": "onelogin_revoke_oauth_token", "description": "Revoke an OAuth token", "inputSchema": {"type": "object", "properties": {"token": {"type": "string"}}, "required": ["token"]}})
    }

    fn tool_introspect_oauth_token(&self) -> Value {
        json!({"name": "onelogin_introspect_oauth_token", "description": "Introspect an OAuth token", "inputSchema": {"type": "object", "properties": {"token": {"type": "string"}}, "required": ["token"]}})
    }

    fn tool_list_webhook_events(&self) -> Value {
        json!({"name": "onelogin_list_webhook_events", "description": "List webhook events", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_scim_get_users(&self) -> Value {
        json!({"name": "onelogin_scim_get_users", "description": "Get users via SCIM", "inputSchema": {"type": "object", "properties": {"filter": {"type": "string"}}}})
    }

    fn tool_scim_create_user(&self) -> Value {
        json!({"name": "onelogin_scim_create_user", "description": "Create user via SCIM", "inputSchema": {"type": "object", "properties": {"userName": {"type": "string"}}, "required": ["userName"]}})
    }

    fn tool_scim_get_user(&self) -> Value {
        json!({"name": "onelogin_scim_get_user", "description": "Get user via SCIM", "inputSchema": {"type": "object", "properties": {"user_id": {"type": "string"}}, "required": ["user_id"]}})
    }

    fn tool_scim_update_user(&self) -> Value {
        json!({"name": "onelogin_scim_update_user", "description": "Update user via SCIM", "inputSchema": {"type": "object", "properties": {"user_id": {"type": "string"}}, "required": ["user_id"]}})
    }

    fn tool_scim_patch_user(&self) -> Value {
        json!({"name": "onelogin_scim_patch_user", "description": "Patch user via SCIM", "inputSchema": {"type": "object", "properties": {"user_id": {"type": "string"}, "operations": {"type": "array"}}, "required": ["user_id", "operations"]}})
    }

    fn tool_scim_delete_user(&self) -> Value {
        json!({"name": "onelogin_scim_delete_user", "description": "Delete user via SCIM", "inputSchema": {"type": "object", "properties": {"user_id": {"type": "string"}}, "required": ["user_id"]}})
    }

    fn tool_scim_get_groups(&self) -> Value {
        json!({"name": "onelogin_scim_get_groups", "description": "Get groups via SCIM", "inputSchema": {"type": "object", "properties": {"filter": {"type": "string"}}}})
    }

    fn tool_scim_create_group(&self) -> Value {
        json!({"name": "onelogin_scim_create_group", "description": "Create group via SCIM", "inputSchema": {"type": "object", "properties": {"displayName": {"type": "string"}}, "required": ["displayName"]}})
    }

    fn tool_scim_bulk_operations(&self) -> Value {
        json!({"name": "onelogin_scim_bulk_operations", "description": "Perform SCIM bulk operations", "inputSchema": {"type": "object", "properties": {"operations": {"type": "array"}}, "required": ["operations"]}})
    }

    fn tool_oidc_get_well_known_config(&self) -> Value {
        json!({"name": "onelogin_oidc_get_well_known_config", "description": "Get OIDC well-known configuration", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_oidc_get_jwks(&self) -> Value {
        json!({"name": "onelogin_oidc_get_jwks", "description": "Get OIDC JWKS", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_oidc_get_userinfo(&self) -> Value {
        json!({"name": "onelogin_oidc_get_userinfo", "description": "Get OIDC user info", "inputSchema": {"type": "object", "properties": {"access_token": {"type": "string"}}, "required": ["access_token"]}})
    }

    fn tool_list_directory_connectors(&self) -> Value {
        json!({"name": "onelogin_list_directory_connectors", "description": "List directory connectors", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_get_directory_connector(&self) -> Value {
        json!({"name": "onelogin_get_directory_connector", "description": "Get a directory connector", "inputSchema": {"type": "object", "properties": {"connector_id": {"type": "string"}}, "required": ["connector_id"]}})
    }

    fn tool_create_directory_connector(&self) -> Value {
        json!({"name": "onelogin_create_directory_connector", "description": "Create a directory connector", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}, "connector_type": {"type": "string"}, "configuration": {"type": "object"}}, "required": ["name", "connector_type", "configuration"]}})
    }

    fn tool_update_directory_connector(&self) -> Value {
        json!({"name": "onelogin_update_directory_connector", "description": "Update a directory connector", "inputSchema": {"type": "object", "properties": {"connector_id": {"type": "string"}}, "required": ["connector_id"]}})
    }

    fn tool_delete_directory_connector(&self) -> Value {
        json!({"name": "onelogin_delete_directory_connector", "description": "Delete a directory connector", "inputSchema": {"type": "object", "properties": {"connector_id": {"type": "string"}}, "required": ["connector_id"]}})
    }

    fn tool_sync_directory(&self) -> Value {
        json!({"name": "onelogin_sync_directory", "description": "Trigger directory sync", "inputSchema": {"type": "object", "properties": {"connector_id": {"type": "string"}}, "required": ["connector_id"]}})
    }

    fn tool_get_sync_status(&self) -> Value {
        json!({"name": "onelogin_get_sync_status", "description": "Get directory sync status", "inputSchema": {"type": "object", "properties": {"connector_id": {"type": "string"}}, "required": ["connector_id"]}})
    }

    fn tool_get_branding_settings(&self) -> Value {
        json!({"name": "onelogin_get_branding_settings", "description": "Get branding settings", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_update_branding_settings(&self) -> Value {
        json!({"name": "onelogin_update_branding_settings", "description": "Update branding settings", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_list_events(&self) -> Value {
        json!({"name": "onelogin_list_events", "description": "List events", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_get_event(&self) -> Value {
        json!({"name": "onelogin_get_event", "description": "Get an event", "inputSchema": {"type": "object", "properties": {"event_id": {"type": "integer"}}, "required": ["event_id"]}})
    }

    fn tool_create_event(&self) -> Value {
        json!({"name": "onelogin_create_event", "description": "Create an event", "inputSchema": {"type": "object", "properties": {"event_type_id": {"type": "integer"}}, "required": ["event_type_id"]}})
    }

    fn tool_list_sessions(&self) -> Value {
        json!({"name": "onelogin_list_sessions", "description": "List sessions", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_get_session(&self) -> Value {
        json!({"name": "onelogin_get_session", "description": "Get a session", "inputSchema": {"type": "object", "properties": {"session_id": {"type": "integer"}}, "required": ["session_id"]}})
    }

    fn tool_delete_session(&self) -> Value {
        json!({"name": "onelogin_delete_session", "description": "Delete a session", "inputSchema": {"type": "object", "properties": {"session_id": {"type": "integer"}}, "required": ["session_id"]}})
    }

    fn tool_list_api_authorizations(&self) -> Value {
        json!({"name": "onelogin_list_api_authorizations", "description": "List API authorizations", "inputSchema": {"type": "object", "properties": {}}})
    }

    fn tool_get_api_authorization(&self) -> Value {
        json!({"name": "onelogin_get_api_authorization", "description": "Get an API authorization", "inputSchema": {"type": "object", "properties": {"auth_id": {"type": "string"}}, "required": ["auth_id"]}})
    }

    fn tool_create_api_authorization(&self) -> Value {
        json!({"name": "onelogin_create_api_authorization", "description": "Create an API authorization", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}, "configuration": {"type": "object"}}, "required": ["name", "configuration"]}})
    }

    fn tool_update_api_authorization(&self) -> Value {
        json!({"name": "onelogin_update_api_authorization", "description": "Update an API authorization", "inputSchema": {"type": "object", "properties": {"auth_id": {"type": "string"}}, "required": ["auth_id"]}})
    }

    fn tool_delete_api_authorization(&self) -> Value {
        json!({"name": "onelogin_delete_api_authorization", "description": "Delete an API authorization", "inputSchema": {"type": "object", "properties": {"auth_id": {"type": "string"}}, "required": ["auth_id"]}})
    }

    // Tool handlers (implementations)
    async fn handle_list_users(&self, args: &Value) -> Result<Value> {
        let params = serde_json::from_value(args.clone()).ok();
        let users = self.client.users.list_users(params).await
            .map_err(|e| anyhow!("Failed to list users: {}", e))?;
        Ok(serde_json::to_value(users)?)
    }

    async fn handle_get_user(&self, args: &Value) -> Result<Value> {
        let user_id: i64 = args.get("user_id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let user = self.client.users.get_user(user_id).await
            .map_err(|e| anyhow!("Failed to get user: {}", e))?;
        Ok(serde_json::to_value(user)?)
    }

    async fn handle_create_user(&self, args: &Value) -> Result<Value> {
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let user = self.client.users.create_user(request).await
            .map_err(|e| anyhow!("Failed to create user: {}", e))?;
        Ok(serde_json::to_value(user)?)
    }

    async fn handle_update_user(&self, args: &Value) -> Result<Value> {
        let user_id: i64 = args.get("user_id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow!("user_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let user = self.client.users.update_user(user_id, request).await
            .map_err(|e| anyhow!("Failed to update user: {}", e))?;
        Ok(serde_json::to_value(user)?)
    }

    async fn handle_delete_user(&self, args: &Value) -> Result<Value> {
        let user_id: i64 = args.get("user_id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow!("user_id is required"))?;
        self.client.users.delete_user(user_id).await
            .map_err(|e| anyhow!("Failed to delete user: {}", e))?;
        Ok(json!({"success": true}))
    }

    async fn handle_create_smart_hook(&self, args: &Value) -> Result<Value> {
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let hook = self.client.smart_hooks.create_hook(request).await
            .map_err(|e| anyhow!("Failed to create smart hook: {}", e))?;
        Ok(serde_json::to_value(hook)?)
    }

    async fn handle_update_smart_hook(&self, args: &Value) -> Result<Value> {
        let hook_id = args.get("hook_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("hook_id is required"))?;
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let hook = self.client.smart_hooks.update_hook(hook_id, request).await
            .map_err(|e| anyhow!("Failed to update smart hook: {}", e))?;
        Ok(serde_json::to_value(hook)?)
    }

    async fn handle_list_smart_hooks(&self, _args: &Value) -> Result<Value> {
        let hooks = self.client.smart_hooks.list_hooks().await
            .map_err(|e| anyhow!("Failed to list smart hooks: {}", e))?;
        Ok(serde_json::to_value(hooks)?)
    }

    async fn handle_get_risk_score(&self, args: &Value) -> Result<Value> {
        let user_identifier = args.get("user_identifier")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("user_identifier is required"))?;
        let context = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid context: {}", e))?;
        let score = self.client.vigilance.get_risk_score(user_identifier, context).await
            .map_err(|e| anyhow!("Failed to get risk score: {}", e))?;
        Ok(serde_json::to_value(score)?)
    }

    async fn handle_validate_user_smart_mfa(&self, args: &Value) -> Result<Value> {
        let request = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid request: {}", e))?;
        let result = self.client.vigilance.validate_user(request).await
            .map_err(|e| anyhow!("Failed to validate user: {}", e))?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_scim_get_users(&self, args: &Value) -> Result<Value> {
        let filter = args.get("filter").and_then(|v| v.as_str()).map(|s| s.to_string());
        let users = self.client.scim.get_users(filter).await
            .map_err(|e| anyhow!("Failed to get SCIM users: {}", e))?;
        Ok(serde_json::to_value(users)?)
    }

    async fn handle_scim_create_user(&self, args: &Value) -> Result<Value> {
        let user = serde_json::from_value(args.clone())
            .map_err(|e| anyhow!("Invalid SCIM user: {}", e))?;
        let created = self.client.scim.create_user(user).await
            .map_err(|e| anyhow!("Failed to create SCIM user: {}", e))?;
        Ok(serde_json::to_value(created)?)
    }
}
