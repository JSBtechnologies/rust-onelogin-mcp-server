use crate::api::OneLoginClient;
use crate::core::auth::AuthManager;
use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::config::Config;
use crate::core::rate_limit::RateLimiter;
use crate::mcp::tools::ToolRegistry;
use anyhow::Result;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{error, info};

pub struct McpServer {
    config: Arc<Config>,
    client: Arc<OneLoginClient>,
    tool_registry: ToolRegistry,
}

impl McpServer {
    pub async fn new(config: Config) -> Result<Self> {
        let config = Arc::new(config);

        // Initialize auth manager
        let auth_manager = Arc::new(AuthManager::new(config.clone()));

        // Initialize rate limiter
        let rate_limiter = Arc::new(RateLimiter::new(
            config.rate_limit_requests_per_second,
        ));

        // Initialize HTTP client
        let http_client = Arc::new(HttpClient::new(
            config.clone(),
            auth_manager,
            rate_limiter,
        ));

        // Initialize cache
        let cache = Arc::new(CacheManager::new(config.cache_ttl_seconds, 10000));

        // Initialize OneLogin API client
        let client = Arc::new(OneLoginClient::new(http_client, cache));

        // Initialize tool registry
        let tool_registry = ToolRegistry::new(client.clone());

        Ok(Self {
            config,
            client,
            tool_registry,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("OneLogin MCP Server started");

        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut writer = stdout;

        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                break; // EOF
            }

            let request: Request = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    error!("Failed to parse request: {}", e);
                    continue;
                }
            };

            let response = self.handle_request(request).await;

            let response_json = serde_json::to_string(&response)?;
            writer.write_all(response_json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
        }

        Ok(())
    }

    async fn handle_request(&self, request: Request) -> Response {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request).await,
            "tools/list" => self.handle_list_tools(request).await,
            "tools/call" => self.handle_call_tool(request).await,
            _ => Response {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(ResponseError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
            },
        }
    }

    async fn handle_initialize(&self, request: Request) -> Response {
        Response {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "onelogin-mcp-server",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            error: None,
        }
    }

    async fn handle_list_tools(&self, request: Request) -> Response {
        let tools = self.tool_registry.list_tools();

        Response {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(serde_json::json!({
                "tools": tools
            })),
            error: None,
        }
    }

    async fn handle_call_tool(&self, request: Request) -> Response {
        let params: CallToolParams = match serde_json::from_value(request.params.clone()) {
            Ok(p) => p,
            Err(e) => {
                return Response {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(ResponseError {
                        code: -32602,
                        message: format!("Invalid params: {}", e),
                        data: None,
                    }),
                }
            }
        };

        match self.tool_registry.call_tool(&params).await {
            Ok(result) => Response {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(serde_json::json!({
                    "content": [{
                        "type": "text",
                        "text": result
                    }]
                })),
                error: None,
            },
            Err(e) => Response {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(ResponseError {
                    code: -32000,
                    message: format!("Tool execution failed: {}", e),
                    data: None,
                }),
            },
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct Request {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    #[serde(default)]
    params: serde_json::Value,
}

#[derive(Debug, serde::Serialize)]
struct Response {
    jsonrpc: String,
    id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ResponseError>,
}

#[derive(Debug, serde::Serialize)]
struct ResponseError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CallToolParams {
    pub name: String,
    pub arguments: serde_json::Value,
}
