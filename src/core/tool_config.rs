//! Tool configuration module for enabling/disabling MCP tools.
//!
//! Supports category-level and tool-level granularity with sensible defaults.

use anyhow::{Context, Result};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tracing::{error, info, warn};

/// Configuration version for future migrations
const CURRENT_VERSION: &str = "1";

/// Category configuration - either a simple bool or detailed config with tool overrides
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CategoryConfig {
    /// Simple enable/disable for entire category
    Simple(bool),
    /// Detailed config with optional tool-level overrides
    Detailed {
        enabled: bool,
        #[serde(default)]
        tools: HashMap<String, bool>,
    },
}

impl Default for CategoryConfig {
    fn default() -> Self {
        CategoryConfig::Simple(false)
    }
}

#[allow(dead_code)]
impl CategoryConfig {
    /// Check if the category is enabled
    pub fn is_enabled(&self) -> bool {
        match self {
            CategoryConfig::Simple(enabled) => *enabled,
            CategoryConfig::Detailed { enabled, .. } => *enabled,
        }
    }

    /// Check if a specific tool is enabled (with tool-level override support)
    pub fn is_tool_enabled(&self, tool_name: &str) -> bool {
        match self {
            CategoryConfig::Simple(enabled) => *enabled,
            CategoryConfig::Detailed { enabled, tools } => {
                // Tool-level override takes precedence over category setting
                tools.get(tool_name).copied().unwrap_or(*enabled)
            }
        }
    }
}

/// Tool category definition mapping tools to their category
#[derive(Debug, Clone)]
pub struct ToolCategory {
    pub name: &'static str,
    pub tools: &'static [&'static str],
    pub default_enabled: bool,
}

/// All category definitions with their tools and default enabled state
pub static TOOL_CATEGORIES: &[ToolCategory] = &[
    // === ENABLED BY DEFAULT ===
    ToolCategory {
        name: "users",
        tools: &[
            "onelogin_list_users",
            "onelogin_get_user",
            "onelogin_create_user",
            "onelogin_update_user",
            "onelogin_delete_user",
            "onelogin_get_user_apps",
            "onelogin_get_user_roles",
            "onelogin_unlock_user",
            "onelogin_logout_user",
            "onelogin_assign_roles",
            "onelogin_remove_roles",
            "onelogin_lock_user",
            "onelogin_set_password",
            "onelogin_set_custom_attributes",
        ],
        default_enabled: true,
    },
    ToolCategory {
        name: "apps",
        tools: &[
            "onelogin_list_apps",
            "onelogin_get_app",
            "onelogin_create_app",
            "onelogin_update_app",
            "onelogin_delete_app",
        ],
        default_enabled: true,
    },
    ToolCategory {
        name: "roles",
        tools: &[
            "onelogin_list_roles",
            "onelogin_get_role",
            "onelogin_create_role",
            "onelogin_update_role",
            "onelogin_delete_role",
        ],
        default_enabled: true,
    },
    ToolCategory {
        name: "groups",
        tools: &[
            "onelogin_list_groups",
            "onelogin_get_group",
            "onelogin_create_group",
            "onelogin_update_group",
            "onelogin_delete_group",
        ],
        default_enabled: true,
    },
    ToolCategory {
        name: "connectors",
        tools: &["onelogin_list_connectors", "onelogin_get_connector"],
        default_enabled: true,
    },
    ToolCategory {
        name: "custom_attributes",
        tools: &[
            "onelogin_list_custom_attributes",
            "onelogin_create_custom_attribute",
            "onelogin_update_custom_attribute",
            "onelogin_delete_custom_attribute",
        ],
        default_enabled: true,
    },
    ToolCategory {
        name: "invitations",
        tools: &[
            "onelogin_generate_invite_link",
            "onelogin_send_invite_link",
        ],
        default_enabled: true,
    },
    ToolCategory {
        name: "events",
        tools: &[
            "onelogin_list_events",
            "onelogin_get_event",
            "onelogin_create_event",
            "onelogin_list_event_types",
        ],
        default_enabled: true,
    },
    ToolCategory {
        name: "reports",
        tools: &[
            "onelogin_list_reports",
            "onelogin_get_report",
            "onelogin_run_report",
            "onelogin_get_report_results",
        ],
        default_enabled: true,
    },
    // === DISABLED BY DEFAULT ===
    ToolCategory {
        name: "app_rules",
        tools: &[
            "onelogin_list_app_rules",
            "onelogin_get_app_rule",
            "onelogin_create_app_rule",
            "onelogin_update_app_rule",
            "onelogin_delete_app_rule",
            "onelogin_list_app_rule_conditions",
            "onelogin_list_app_rule_actions",
            "onelogin_list_condition_operators",
            "onelogin_list_condition_values",
            "onelogin_list_action_values",
            "onelogin_sort_app_rules",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "mfa",
        tools: &[
            "onelogin_list_mfa_factors",
            "onelogin_enroll_mfa_factor",
            "onelogin_remove_mfa_factor",
            "onelogin_verify_mfa_factor",
            "onelogin_enroll_mfa",
            "onelogin_verify_mfa",
            "onelogin_remove_mfa",
            "onelogin_generate_mfa_token",
            "onelogin_verify_mfa_token",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "saml",
        tools: &[
            "onelogin_get_saml_assertion",
            "onelogin_verify_saml_factor",
            "onelogin_get_saml_assertion_v2",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "smart_hooks",
        tools: &[
            "onelogin_create_smart_hook",
            "onelogin_update_smart_hook",
            "onelogin_delete_smart_hook",
            "onelogin_get_smart_hook",
            "onelogin_list_smart_hooks",
            "onelogin_get_smart_hook_logs",
            // Hook environment variables (account-level, shared by all hooks)
            "onelogin_list_hook_env_vars",
            "onelogin_get_hook_env_var",
            "onelogin_create_hook_env_var",
            "onelogin_update_hook_env_var",
            "onelogin_delete_hook_env_var",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "vigilance",
        tools: &[
            "onelogin_get_risk_score",
            "onelogin_validate_user_smart_mfa",
            "onelogin_list_risk_rules",
            "onelogin_create_risk_rule",
            "onelogin_update_risk_rule",
            "onelogin_delete_risk_rule",
            "onelogin_get_risk_events",
            "onelogin_track_risk_event",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "privileges",
        tools: &[
            "onelogin_list_privileges",
            "onelogin_get_privilege",
            "onelogin_create_privilege",
            "onelogin_update_privilege",
            "onelogin_delete_privilege",
            "onelogin_assign_user_to_privilege",
            "onelogin_assign_role_to_privilege",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "user_mappings",
        tools: &[
            "onelogin_list_user_mappings",
            "onelogin_get_user_mapping",
            "onelogin_create_user_mapping",
            "onelogin_update_user_mapping",
            "onelogin_delete_user_mapping",
            "onelogin_sort_user_mappings",
            "onelogin_sort_mapping_order",
            "onelogin_list_mapping_conditions",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "embed_tokens",
        tools: &["onelogin_generate_embed_token", "onelogin_list_embeddable_apps"],
        default_enabled: false,
    },
    ToolCategory {
        name: "oauth",
        tools: &[
            "onelogin_generate_oauth_tokens",
            "onelogin_revoke_oauth_token",
            "onelogin_introspect_oauth_token",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "oidc",
        tools: &[
            "onelogin_oidc_get_well_known_config",
            "onelogin_oidc_get_jwks",
            "onelogin_oidc_get_userinfo",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "directories",
        tools: &[
            "onelogin_list_directory_connectors",
            "onelogin_get_directory_connector",
            "onelogin_create_directory_connector",
            "onelogin_update_directory_connector",
            "onelogin_delete_directory_connector",
            "onelogin_sync_directory",
            "onelogin_get_sync_status",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "branding",
        tools: &[
            "onelogin_get_branding_settings",
            "onelogin_update_branding_settings",
            "onelogin_get_email_settings",
            "onelogin_update_email_settings",
            "onelogin_list_message_templates",
            "onelogin_get_message_template",
            "onelogin_get_template_by_type",
            "onelogin_get_template_by_locale",
            "onelogin_create_message_template",
            "onelogin_update_message_template",
            "onelogin_update_template_by_locale",
            "onelogin_delete_message_template",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "self_registration",
        tools: &[
            "onelogin_list_self_registration_profiles",
            "onelogin_get_self_registration_profile",
            "onelogin_create_self_registration_profile",
            "onelogin_update_self_registration_profile",
            "onelogin_delete_self_registration_profile",
            "onelogin_list_registrations",
            "onelogin_approve_registration",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "login",
        tools: &[
            "onelogin_create_session_login_token",
            "onelogin_verify_factor_login",
            "onelogin_create_session",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "api_auth",
        tools: &[
            "onelogin_list_api_authorizations",
            "onelogin_get_api_authorization",
            "onelogin_create_api_authorization",
            "onelogin_update_api_authorization",
            "onelogin_delete_api_authorization",
        ],
        default_enabled: false,
    },
    // NOTE: The following categories were removed because no public OneLogin API exists for them:
    // - account (no /api/2/account endpoint)
    // - password_policies (no /api/2/password_policies endpoint)
    // - certificates (no /api/2/certificates endpoint)
    // - devices (no /api/2/devices endpoint - Device Trust managed via admin portal)
    // - login_pages (no management API - /api/1/login-page is for session creation only)
    // - trusted_idps (no /api/2/trusted_idps endpoint - configured via admin portal)
    // - webhooks_crud (no CRUD API - webhooks must be configured via admin portal)
    ToolCategory {
        name: "role_resources",
        tools: &[
            "onelogin_get_role_apps",
            "onelogin_set_role_apps",
            "onelogin_get_role_users",
            "onelogin_get_role_admins",
            "onelogin_add_role_admins",
            "onelogin_remove_role_admin",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "rate_limits",
        tools: &[
            "onelogin_get_rate_limit_status",
            "onelogin_get_rate_limits",
        ],
        default_enabled: false,
    },
    ToolCategory {
        name: "risk",
        tools: &[
            "onelogin_get_risk_rule",
        ],
        default_enabled: false,
    },
];

/// Main configuration file structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolConfigFile {
    /// Schema version for future migrations
    #[serde(default = "default_version")]
    pub version: String,

    /// Enable hot reload (file watching)
    #[serde(default)]
    pub hot_reload: bool,

    /// Category configurations
    #[serde(default)]
    pub categories: HashMap<String, CategoryConfig>,
}

fn default_version() -> String {
    CURRENT_VERSION.to_string()
}

impl Default for ToolConfigFile {
    fn default() -> Self {
        let mut categories = HashMap::new();
        for cat in TOOL_CATEGORIES {
            categories.insert(
                cat.name.to_string(),
                CategoryConfig::Simple(cat.default_enabled),
            );
        }
        Self {
            version: CURRENT_VERSION.to_string(),
            hot_reload: false,
            categories,
        }
    }
}

/// Runtime tool configuration manager
#[allow(dead_code)]
pub struct ToolConfig {
    config_path: Option<PathBuf>,
    config: RwLock<ToolConfigFile>,
    enabled_tools: RwLock<HashSet<String>>,
}

#[allow(dead_code)]
impl ToolConfig {
    /// Create from config file path, falling back to defaults if file doesn't exist
    pub fn load(config_path: Option<PathBuf>) -> Result<Self> {
        let config = match &config_path {
            Some(path) if path.exists() => {
                info!("Loading tool config from: {}", path.display());
                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("Failed to read config file: {}", path.display()))?;
                let config: ToolConfigFile = serde_json::from_str(&content)
                    .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

                // Validate version
                if config.version != CURRENT_VERSION {
                    warn!(
                        "Config version mismatch: expected {}, got {}. Some settings may be ignored.",
                        CURRENT_VERSION, config.version
                    );
                }

                // Warn about unknown categories
                for cat_name in config.categories.keys() {
                    if !TOOL_CATEGORIES.iter().any(|c| c.name == cat_name) {
                        warn!("Unknown category in config: '{}' (will be ignored)", cat_name);
                    }
                }

                config
            }
            Some(path) => {
                info!(
                    "Config file not found at {}, using defaults",
                    path.display()
                );
                ToolConfigFile::default()
            }
            None => {
                info!("No config path specified, using defaults");
                ToolConfigFile::default()
            }
        };

        let enabled_tools = Self::compute_enabled_tools(&config);

        info!(
            "Tool config loaded: {} tools enabled out of {} total",
            enabled_tools.len(),
            TOOL_CATEGORIES.iter().map(|c| c.tools.len()).sum::<usize>()
        );

        Ok(Self {
            config_path,
            config: RwLock::new(config),
            enabled_tools: RwLock::new(enabled_tools),
        })
    }

    /// Get default config file path (~/.config/onelogin-mcp/config.json)
    pub fn default_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("onelogin-mcp").join("config.json"))
    }

    /// Get the config file path that was used (if any)
    pub fn config_path(&self) -> Option<&PathBuf> {
        self.config_path.as_ref()
    }

    /// Check if a tool is enabled
    pub fn is_tool_enabled(&self, tool_name: &str) -> bool {
        self.enabled_tools
            .read()
            .expect("RwLock poisoned")
            .contains(tool_name)
    }

    /// Get all enabled tool names
    pub fn enabled_tools(&self) -> HashSet<String> {
        self.enabled_tools.read().expect("RwLock poisoned").clone()
    }

    /// Get count of enabled tools
    pub fn enabled_count(&self) -> usize {
        self.enabled_tools.read().expect("RwLock poisoned").len()
    }

    /// Check if hot reload is enabled
    pub fn hot_reload_enabled(&self) -> bool {
        self.config.read().expect("RwLock poisoned").hot_reload
    }

    /// Reload configuration from file
    pub fn reload(&self) -> Result<()> {
        let Some(path) = &self.config_path else {
            warn!("No config path set, cannot reload");
            return Ok(());
        };

        if !path.exists() {
            warn!("Config file no longer exists: {}", path.display());
            return Ok(());
        }

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let new_config: ToolConfigFile = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        let new_enabled = Self::compute_enabled_tools(&new_config);

        let old_count = self.enabled_count();
        let new_count = new_enabled.len();

        // Update atomically
        *self.config.write().expect("RwLock poisoned") = new_config;
        *self.enabled_tools.write().expect("RwLock poisoned") = new_enabled;

        info!(
            "Tool config reloaded: {} tools enabled (was {})",
            new_count, old_count
        );
        Ok(())
    }

    /// Compute which tools are enabled based on config
    fn compute_enabled_tools(config: &ToolConfigFile) -> HashSet<String> {
        let mut enabled = HashSet::new();

        for category in TOOL_CATEGORIES {
            // Get category config, falling back to default if not specified
            let cat_config = config
                .categories
                .get(category.name)
                .cloned()
                .unwrap_or(CategoryConfig::Simple(category.default_enabled));

            for tool_name in category.tools {
                if cat_config.is_tool_enabled(tool_name) {
                    enabled.insert((*tool_name).to_string());
                }
            }
        }

        enabled
    }

    /// Start watching config file for changes (hot reload)
    pub fn start_watcher(self: &Arc<Self>) -> Result<Option<RecommendedWatcher>> {
        if !self.hot_reload_enabled() {
            info!("Hot reload disabled in config");
            return Ok(None);
        }

        let Some(path) = &self.config_path else {
            warn!("No config path set, cannot enable hot reload");
            return Ok(None);
        };

        let config = Arc::clone(self);
        let path_for_watch = path.clone();

        let mut watcher =
            notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
                Ok(event) => {
                    // Only reload on modify events
                    if matches!(
                        event.kind,
                        EventKind::Modify(_) | EventKind::Create(_)
                    ) {
                        info!("Config file changed, reloading...");
                        if let Err(e) = config.reload() {
                            error!("Failed to reload config: {}", e);
                        }
                    }
                }
                Err(e) => error!("File watch error: {:?}", e),
            })?;

        // Watch the config file's parent directory (more reliable than watching the file directly)
        if let Some(parent) = path_for_watch.parent() {
            watcher.watch(parent, RecursiveMode::NonRecursive)?;
            info!("Hot reload enabled, watching: {}", path_for_watch.display());
        } else {
            warn!("Cannot determine parent directory for config file");
        }

        Ok(Some(watcher))
    }

    /// Generate default config file content (for documentation/init purposes)
    pub fn generate_default_config() -> String {
        let config = ToolConfigFile::default();
        serde_json::to_string_pretty(&config).expect("Failed to serialize default config")
    }

    /// Generate example config with all options shown
    pub fn generate_example_config() -> String {
        let mut categories = HashMap::new();

        // Show simple boolean for most categories
        for cat in TOOL_CATEGORIES {
            if cat.name == "users" {
                // Show detailed config example for users
                let mut tools = HashMap::new();
                tools.insert("onelogin_delete_user".to_string(), false);
                tools.insert("onelogin_set_password".to_string(), false);
                categories.insert(
                    cat.name.to_string(),
                    CategoryConfig::Detailed {
                        enabled: true,
                        tools,
                    },
                );
            } else {
                categories.insert(
                    cat.name.to_string(),
                    CategoryConfig::Simple(cat.default_enabled),
                );
            }
        }

        let config = ToolConfigFile {
            version: CURRENT_VERSION.to_string(),
            hot_reload: true,
            categories,
        };

        serde_json::to_string_pretty(&config).expect("Failed to serialize example config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_config_simple() {
        let config = CategoryConfig::Simple(true);
        assert!(config.is_enabled());
        assert!(config.is_tool_enabled("any_tool"));

        let config = CategoryConfig::Simple(false);
        assert!(!config.is_enabled());
        assert!(!config.is_tool_enabled("any_tool"));
    }

    #[test]
    fn test_category_config_detailed() {
        let mut tools = HashMap::new();
        tools.insert("disabled_tool".to_string(), false);
        tools.insert("enabled_tool".to_string(), true);

        let config = CategoryConfig::Detailed {
            enabled: true,
            tools,
        };

        assert!(config.is_enabled());
        assert!(!config.is_tool_enabled("disabled_tool"));
        assert!(config.is_tool_enabled("enabled_tool"));
        assert!(config.is_tool_enabled("unspecified_tool")); // Falls back to enabled
    }

    #[test]
    fn test_category_config_detailed_disabled() {
        let mut tools = HashMap::new();
        tools.insert("enabled_tool".to_string(), true);

        let config = CategoryConfig::Detailed {
            enabled: false,
            tools,
        };

        assert!(!config.is_enabled());
        assert!(config.is_tool_enabled("enabled_tool")); // Override takes precedence
        assert!(!config.is_tool_enabled("unspecified_tool")); // Falls back to disabled
    }

    #[test]
    fn test_default_config() {
        let config = ToolConfigFile::default();
        assert_eq!(config.version, CURRENT_VERSION);
        assert!(!config.hot_reload);

        // Check that all categories are present
        for cat in TOOL_CATEGORIES {
            assert!(config.categories.contains_key(cat.name));
        }
    }

    #[test]
    fn test_compute_enabled_tools_defaults() {
        let config = ToolConfigFile::default();
        let enabled = ToolConfig::compute_enabled_tools(&config);

        // Users should be enabled by default
        assert!(enabled.contains("onelogin_list_users"));
        assert!(enabled.contains("onelogin_get_user"));

        // MFA should be disabled by default
        assert!(!enabled.contains("onelogin_list_mfa_factors"));
        assert!(!enabled.contains("onelogin_enroll_mfa"));
    }

    #[test]
    fn test_parse_simple_config() {
        let json = r#"{
            "version": "1",
            "categories": {
                "users": true,
                "mfa": true
            }
        }"#;

        let config: ToolConfigFile = serde_json::from_str(json).unwrap();
        let enabled = ToolConfig::compute_enabled_tools(&config);

        assert!(enabled.contains("onelogin_list_users"));
        assert!(enabled.contains("onelogin_list_mfa_factors"));
    }

    #[test]
    fn test_parse_detailed_config() {
        let json = r#"{
            "version": "1",
            "categories": {
                "users": {
                    "enabled": true,
                    "tools": {
                        "onelogin_delete_user": false
                    }
                }
            }
        }"#;

        let config: ToolConfigFile = serde_json::from_str(json).unwrap();
        let enabled = ToolConfig::compute_enabled_tools(&config);

        assert!(enabled.contains("onelogin_list_users"));
        assert!(enabled.contains("onelogin_get_user"));
        assert!(!enabled.contains("onelogin_delete_user"));
    }

    #[test]
    fn test_tool_config_load_no_file() {
        let config = ToolConfig::load(None).unwrap();
        assert!(config.is_tool_enabled("onelogin_list_users"));
        assert!(!config.is_tool_enabled("onelogin_list_mfa_factors"));
    }

    #[test]
    fn test_generate_configs() {
        // Just ensure they don't panic and produce valid JSON
        let default = ToolConfig::generate_default_config();
        let _: ToolConfigFile = serde_json::from_str(&default).unwrap();

        let example = ToolConfig::generate_example_config();
        let _: ToolConfigFile = serde_json::from_str(&example).unwrap();
    }
}
