use anyhow::{Context, Result};
use clap::Parser;
use tracing::{error, info, Level};

mod api;
mod cli;
mod core;
mod mcp;
mod models;
mod utils;

use crate::cli::{Cli, Commands};
use crate::core::config::Config;
use crate::mcp::server::McpServer;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle config commands without initializing tracing to stderr
    // (config commands should output to stdout normally)
    match &cli.command {
        Some(Commands::Config { action }) => {
            return cli::execute_config_action(action.clone());
        }
        _ => {}
    }

    // Initialize tracing for server mode - IMPORTANT: Write logs to stderr, not stdout
    // MCP protocol requires stdout to only contain JSON-RPC messages
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_writer(std::io::stderr) // Logs go to stderr
        .with_ansi(false) // Disable color codes
        .init();

    // Run the MCP server (default command)
    run_server().await
}

async fn run_server() -> Result<()> {
    info!("Starting OneLogin MCP Server v{}", env!("CARGO_PKG_VERSION"));
    info!("Logs are written to stderr, MCP messages to stdout");

    // Load configuration
    info!("Loading configuration from environment variables...");
    let config = Config::from_env().context(
        "Failed to load configuration from environment variables.\n\
         \n\
         Required environment variables:\n\
         - ONELOGIN_CLIENT_ID: Your OneLogin API client ID\n\
         - ONELOGIN_CLIENT_SECRET: Your OneLogin API client secret\n\
         - ONELOGIN_REGION: Your OneLogin region (us or eu)\n\
         - ONELOGIN_SUBDOMAIN: Your OneLogin subdomain\n\
         \n\
         Please ensure all required environment variables are set and valid."
    )?;
    info!(
        "Configuration loaded successfully: region={:?}, subdomain={}",
        config.onelogin_region, config.onelogin_subdomain
    );
    if let Some(ref path) = config.tool_config_path {
        info!("Tool config path: {}", path.display());
    }

    // Create and run MCP server
    info!("Initializing MCP server...");
    let server = McpServer::new(config).await.context(
        "Failed to initialize MCP server.\n\
         \n\
         This could be due to:\n\
         - Invalid configuration\n\
         - Network connectivity issues\n\
         - Authentication problems with OneLogin API\n\
         \n\
         Check the detailed error message above for more information."
    )?;
    info!("MCP server initialized successfully");

    // Start hot reload watcher if enabled in config
    // Note: _watcher must be kept alive for the duration of the server
    let _watcher = server.start_config_watcher().context(
        "Failed to start configuration file watcher for hot reload"
    )?;

    info!("Starting MCP server main loop...");
    if let Err(e) = server.run().await {
        error!(
            "MCP SERVER ERROR\n\
             \n\
             The MCP server encountered a fatal error and will shut down:\n\
             {:#}\n\
             \n\
             Full error chain is shown above.",
            e
        );

        return Err(e);
    }

    info!("MCP server shut down gracefully");
    Ok(())
}
