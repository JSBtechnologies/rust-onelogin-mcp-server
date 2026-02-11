//! CLI module for managing tool configuration.

use crate::core::tool_config::{CategoryConfig, ToolConfig, ToolConfigFile, TOOL_CATEGORIES};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "onelogin-mcp-server")]
#[command(author, version, about = "OneLogin MCP Server - A comprehensive MCP server for OneLogin API")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage tool configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Run the MCP server (default if no command specified)
    Serve,
}

#[derive(Subcommand, Clone)]
pub enum ConfigAction {
    /// Show current configuration and enabled tools
    Show,
    /// Initialize a new configuration file with defaults
    Init {
        /// Overwrite existing config file
        #[arg(short, long)]
        force: bool,
    },
    /// List all available categories
    Categories,
    /// List all available tools (optionally filtered by category)
    Tools {
        /// Filter by category name
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Enable a category or specific tool
    Enable {
        /// Category name (e.g., "mfa", "saml"), tool name (e.g., "onelogin_list_users"), or "all" to enable all categories
        name: String,
    },
    /// Disable a category or specific tool
    Disable {
        /// Category name (e.g., "users", "apps") or tool name (e.g., "onelogin_delete_user")
        name: String,
    },
    /// Show the config file path
    Path,
    /// Open config file in default editor
    Edit,
    /// Reset configuration to defaults
    Reset {
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

/// Get the config file path
pub fn get_config_path() -> Result<PathBuf> {
    std::env::var("ONELOGIN_MCP_CONFIG")
        .map(PathBuf::from)
        .ok()
        .or_else(|| dirs::config_dir().map(|d| d.join("onelogin-mcp").join("config.json")))
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))
}

/// Load existing config or return default
fn load_config(path: &PathBuf) -> ToolConfigFile {
    if path.exists() {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(config) => config,
                Err(_) => ToolConfigFile::default(),
            },
            Err(_) => ToolConfigFile::default(),
        }
    } else {
        ToolConfigFile::default()
    }
}

/// Save config to file
fn save_config(path: &PathBuf, config: &ToolConfigFile) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(config)
        .context("Failed to serialize config")?;

    fs::write(path, json)
        .with_context(|| format!("Failed to write config file: {}", path.display()))?;

    Ok(())
}

/// Check if a name is a category
fn is_category(name: &str) -> bool {
    TOOL_CATEGORIES.iter().any(|c| c.name == name)
}

/// Check if a name is a tool
fn is_tool(name: &str) -> Option<&'static str> {
    for cat in TOOL_CATEGORIES {
        if cat.tools.contains(&name) {
            return Some(cat.name);
        }
    }
    None
}

/// Execute a config action
pub fn execute_config_action(action: ConfigAction) -> Result<()> {
    let config_path = get_config_path()?;

    match action {
        ConfigAction::Path => {
            println!("{}", config_path.display());
        }

        ConfigAction::Show => {
            println!("Configuration file: {}", config_path.display());
            println!("Status: {}\n", if config_path.exists() { "exists" } else { "not found (using defaults)" });

            let config = load_config(&config_path);

            println!("Hot reload: {}\n", if config.hot_reload { "enabled" } else { "disabled" });

            let tool_config = ToolConfig::load(Some(config_path.clone()))?;
            let enabled_count = tool_config.enabled_count();
            let total_count: usize = TOOL_CATEGORIES.iter().map(|c| c.tools.len()).sum();

            println!("Tools: {}/{} enabled\n", enabled_count, total_count);

            println!("Categories:");
            println!("{:-<60}", "");

            for cat in TOOL_CATEGORIES {
                let cat_config = config.categories.get(cat.name);
                let enabled_in_cat: usize = cat.tools.iter()
                    .filter(|t| tool_config.is_tool_enabled(t))
                    .count();

                let status = match cat_config {
                    Some(CategoryConfig::Simple(true)) => "enabled".to_string(),
                    Some(CategoryConfig::Simple(false)) => "disabled".to_string(),
                    Some(CategoryConfig::Detailed { enabled, tools }) => {
                        if tools.is_empty() {
                            if *enabled { "enabled".to_string() } else { "disabled".to_string() }
                        } else {
                            format!("{} (with {} overrides)",
                                if *enabled { "enabled" } else { "disabled" },
                                tools.len())
                        }
                    }
                    None => format!("default ({})", if cat.default_enabled { "enabled" } else { "disabled" }),
                };

                println!("  {:<20} {:>3}/{:<3} tools  [{}]",
                    cat.name, enabled_in_cat, cat.tools.len(), status);
            }
        }

        ConfigAction::Init { force } => {
            if config_path.exists() && !force {
                println!("Config file already exists at: {}", config_path.display());
                println!("Use --force to overwrite.");
                return Ok(());
            }

            let config = ToolConfigFile::default();
            save_config(&config_path, &config)?;
            println!("Created config file at: {}", config_path.display());
            println!("\nDefault configuration:");
            println!("  - {} categories enabled by default",
                TOOL_CATEGORIES.iter().filter(|c| c.default_enabled).count());
            println!("  - {} categories disabled by default",
                TOOL_CATEGORIES.iter().filter(|c| !c.default_enabled).count());
            println!("\nEdit with: onelogin-mcp-server config edit");
        }

        ConfigAction::Categories => {
            println!("Available categories:\n");
            println!("{:<20} {:<8} {:<6} Description", "NAME", "DEFAULT", "TOOLS");
            println!("{:-<60}", "");

            for cat in TOOL_CATEGORIES {
                let default = if cat.default_enabled { "enabled" } else { "disabled" };
                println!("{:<20} {:<8} {:<6}", cat.name, default, cat.tools.len());
            }

            println!("\nUse 'config tools --category <name>' to see tools in a category.");
        }

        ConfigAction::Tools { category } => {
            let config_path_opt = if config_path.exists() { Some(config_path) } else { None };
            let tool_config = ToolConfig::load(config_path_opt)?;

            if let Some(cat_name) = category {
                let cat = TOOL_CATEGORIES.iter()
                    .find(|c| c.name == cat_name)
                    .ok_or_else(|| anyhow::anyhow!("Unknown category: {}", cat_name))?;

                println!("Tools in '{}' category:\n", cat_name);
                println!("{:<45} {:<8}", "TOOL NAME", "STATUS");
                println!("{:-<55}", "");

                for tool in cat.tools {
                    let status = if tool_config.is_tool_enabled(tool) { "enabled" } else { "disabled" };
                    println!("{:<45} {:<8}", tool, status);
                }
            } else {
                println!("All tools:\n");
                println!("{:<45} {:<15} {:<8}", "TOOL NAME", "CATEGORY", "STATUS");
                println!("{:-<70}", "");

                for cat in TOOL_CATEGORIES {
                    for tool in cat.tools {
                        let status = if tool_config.is_tool_enabled(tool) { "enabled" } else { "disabled" };
                        println!("{:<45} {:<15} {:<8}", tool, cat.name, status);
                    }
                }
            }
        }

        ConfigAction::Enable { name } => {
            let mut config = load_config(&config_path);

            if name == "all" {
                // Enable all categories
                let mut total_tools = 0;
                for cat in TOOL_CATEGORIES {
                    config.categories.insert(cat.name.to_string(), CategoryConfig::Simple(true));
                    total_tools += cat.tools.len();
                }
                save_config(&config_path, &config)?;
                println!("Enabled all {} categories ({} tools)", TOOL_CATEGORIES.len(), total_tools);
            } else if is_category(&name) {
                config.categories.insert(name.clone(), CategoryConfig::Simple(true));
                save_config(&config_path, &config)?;

                let cat = TOOL_CATEGORIES.iter().find(|c| c.name == name).unwrap();
                println!("Enabled category '{}' ({} tools)", name, cat.tools.len());
            } else if let Some(cat_name) = is_tool(&name) {
                // Get or create category config
                let cat_config = config.categories
                    .entry(cat_name.to_string())
                    .or_insert_with(|| {
                        let cat = TOOL_CATEGORIES.iter().find(|c| c.name == cat_name).unwrap();
                        CategoryConfig::Simple(cat.default_enabled)
                    });

                // Convert to detailed config if needed
                match cat_config {
                    CategoryConfig::Simple(enabled) => {
                        let mut tools = HashMap::new();
                        tools.insert(name.clone(), true);
                        *cat_config = CategoryConfig::Detailed {
                            enabled: *enabled,
                            tools,
                        };
                    }
                    CategoryConfig::Detailed { tools, .. } => {
                        tools.insert(name.clone(), true);
                    }
                }

                save_config(&config_path, &config)?;
                println!("Enabled tool '{}' in category '{}'", name, cat_name);
            } else {
                return Err(anyhow::anyhow!(
                    "Unknown category or tool: '{}'\n\nUse 'config categories' to list categories\nUse 'config tools' to list tools",
                    name
                ));
            }
        }

        ConfigAction::Disable { name } => {
            let mut config = load_config(&config_path);

            if is_category(&name) {
                config.categories.insert(name.clone(), CategoryConfig::Simple(false));
                save_config(&config_path, &config)?;

                let cat = TOOL_CATEGORIES.iter().find(|c| c.name == name).unwrap();
                println!("Disabled category '{}' ({} tools)", name, cat.tools.len());
            } else if let Some(cat_name) = is_tool(&name) {
                // Get or create category config
                let cat_config = config.categories
                    .entry(cat_name.to_string())
                    .or_insert_with(|| {
                        let cat = TOOL_CATEGORIES.iter().find(|c| c.name == cat_name).unwrap();
                        CategoryConfig::Simple(cat.default_enabled)
                    });

                // Convert to detailed config if needed
                match cat_config {
                    CategoryConfig::Simple(enabled) => {
                        let mut tools = HashMap::new();
                        tools.insert(name.clone(), false);
                        *cat_config = CategoryConfig::Detailed {
                            enabled: *enabled,
                            tools,
                        };
                    }
                    CategoryConfig::Detailed { tools, .. } => {
                        tools.insert(name.clone(), false);
                    }
                }

                save_config(&config_path, &config)?;
                println!("Disabled tool '{}' in category '{}'", name, cat_name);
            } else {
                return Err(anyhow::anyhow!(
                    "Unknown category or tool: '{}'\n\nUse 'config categories' to list categories\nUse 'config tools' to list tools",
                    name
                ));
            }
        }

        ConfigAction::Edit => {
            // Ensure config exists
            if !config_path.exists() {
                let config = ToolConfigFile::default();
                save_config(&config_path, &config)?;
                println!("Created default config file.");
            }

            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| {
                    if cfg!(target_os = "windows") {
                        "notepad".to_string()
                    } else {
                        "vi".to_string()
                    }
                });

            println!("Opening {} with {}...", config_path.display(), editor);

            let status = std::process::Command::new(&editor)
                .arg(&config_path)
                .status()
                .with_context(|| format!("Failed to open editor: {}", editor))?;

            if !status.success() {
                return Err(anyhow::anyhow!("Editor exited with error"));
            }
        }

        ConfigAction::Reset { yes } => {
            if !yes {
                println!("This will reset the config to defaults.");
                println!("Config file: {}", config_path.display());
                print!("Continue? [y/N] ");

                use std::io::{self, Write};
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            let config = ToolConfigFile::default();
            save_config(&config_path, &config)?;
            println!("Reset config to defaults at: {}", config_path.display());
        }
    }

    Ok(())
}
