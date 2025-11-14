use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber;

mod api;
mod core;
mod mcp;
mod models;
mod utils;

use crate::core::config::Config;
use crate::mcp::server::McpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    info!("Starting OneLogin MCP Server");

    // Load configuration
    let config = Config::from_env()?;

    // Create and run MCP server
    let server = McpServer::new(config).await?;
    server.run().await?;

    Ok(())
}
