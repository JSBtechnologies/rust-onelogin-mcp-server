use crate::core::config::Config;
use crate::core::tenant_manager::TenantManager;
use crate::core::tool_config::ToolConfig;
use crate::mcp::tools::ToolRegistry;
use anyhow::{anyhow, Context, Result};
use notify::RecommendedWatcher;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tracing::{debug, error, info};

#[allow(dead_code)]
pub struct McpServer {
    config: Arc<Config>,
    tenant_manager: Arc<TenantManager>,
    tool_registry: ToolRegistry,
    tool_config: Arc<ToolConfig>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransportMode {
    ContentLength,
    RawJson,
}

#[derive(Debug)]
struct IncomingFrame {
    payload: String,
    mode: TransportMode,
}

#[allow(dead_code)]
impl McpServer {
    pub async fn new(config: Config) -> Result<Self> {
        let config_arc = Arc::new(config.clone());

        // Initialize tool configuration (controls which tools are enabled)
        let tool_config = Arc::new(
            ToolConfig::load(config.tool_config_path.clone())
                .context("Failed to load tool configuration")?
        );

        // Determine single vs multi-tenant mode
        let tenant_manager = match Config::load_tenants_file()? {
            Some(tenants_file) if !tenants_file.tenants.is_empty() => {
                info!(
                    "Multi-tenant mode: {} tenant(s) configured",
                    tenants_file.tenants.len()
                );
                for t in &tenants_file.tenants {
                    info!(
                        "  Tenant '{}': subdomain={}, region={}{}",
                        t.name, t.subdomain, t.region,
                        if t.default { " (default)" } else { "" }
                    );
                }
                Arc::new(
                    TenantManager::from_entries(&tenants_file.tenants, &config)
                        .context("Failed to initialize multi-tenant manager")?
                )
            }
            _ => {
                info!("Single-tenant mode (credentials from environment)");
                Arc::new(TenantManager::from_single(config))
            }
        };

        // Initialize tool registry with tenant manager and tool config
        let tool_registry = ToolRegistry::new(tenant_manager.clone(), tool_config.clone());

        Ok(Self {
            config: config_arc,
            tenant_manager,
            tool_registry,
            tool_config,
        })
    }

    /// Start file watcher for hot reload if enabled
    pub fn start_config_watcher(&self) -> Result<Option<RecommendedWatcher>> {
        self.tool_config.start_watcher()
    }

    /// Get the tool config for external access
    pub fn tool_config(&self) -> &Arc<ToolConfig> {
        &self.tool_config
    }

    pub async fn run(&self) -> Result<()> {
        info!("OneLogin MCP Server started");

        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut writer = stdout;
        let mut negotiated_transport: Option<TransportMode> = None;

        loop {
            let frame = match Self::read_frame(&mut reader).await {
                Ok(Some(frame)) => {
                    info!("Received MCP frame: {} bytes", frame.payload.len());
                    if negotiated_transport.is_none() {
                        negotiated_transport = Some(frame.mode);
                    }
                    frame
                }
                Ok(None) => {
                    info!("Client closed connection");
                    break;
                }
                Err(e) => {
                    error!("Failed to read MCP frame: {}", e);
                    continue;
                }
            };

            let request: Request = match serde_json::from_str::<Request>(&frame.payload) {
                Ok(req) => {
                    info!("Parsed request: method={}", req.method);
                    req
                }
                Err(e) => {
                    error!("Failed to parse request: {}", e);
                    continue;
                }
            };

            if let Some(response) = self.handle_request(request).await {
                let response_json = serde_json::to_string(&response)?;
                info!("Sending response: {} bytes", response_json.len());
                let mode = negotiated_transport.unwrap_or(TransportMode::ContentLength);
                Self::write_frame(&mut writer, &response_json, mode).await?;
                writer.flush().await?;
            } else {
                info!("No response needed (notification)");
            }
        }

        Ok(())
    }

    async fn handle_request(&self, request: Request) -> Option<Response> {
        // Notifications don't have an id and don't require a response
        if request.id.is_none() {
            // Handle notifications (currently we just ignore them)
            return None;
        }

        let response = match request.method.as_str() {
            "initialize" => self.handle_initialize(request).await,
            "tools/list" => self.handle_list_tools(request).await,
            "tools/call" => self.handle_call_tool(request).await,
            "prompts/list" => self.handle_list_prompts(request).await,
            "prompts/get" => self.handle_get_prompt(request).await,
            _ => Response {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(ResponseError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                    tool_name: None,
                }),
            },
        };

        Some(response)
    }

    async fn handle_initialize(&self, request: Request) -> Response {
        Response {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "prompts": {}
                },
                "serverInfo": {
                    "name": "onelogin-mcp-server",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            error: None,
        }
    }

    async fn handle_list_prompts(&self, request: Request) -> Response {
        Response {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(serde_json::json!({
                "prompts": [
                    {
                        "name": "onelogin-usage-guide",
                        "description": "Important guidelines for using OneLogin MCP tools effectively"
                    }
                ]
            })),
            error: None,
        }
    }

    async fn handle_get_prompt(&self, request: Request) -> Response {
        let name = request.params
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if name == "onelogin-usage-guide" {
            Response {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(serde_json::json!({
                    "description": "Important guidelines for using OneLogin MCP tools effectively",
                    "messages": [
                        {
                            "role": "user",
                            "content": {
                                "type": "text",
                                "text": r#"# OneLogin MCP Server Usage Guide

## IMPORTANT: Always Read Tool Descriptions
Before using any tool, carefully read its description. The descriptions contain critical information about:
- Required vs optional parameters
- Valid enum values and their meanings
- Which tools to use together (e.g., unlock vs update_user)
- Limitations and edge cases

## Key User Status Values
- 0 = Unactivated (never logged in)
- 1 = Active (can log in)
- 2 = Suspended (admin disabled)
- 3 = Locked (too many failed attempts OR manually locked)
- 4 = Password expired
- 5 = Awaiting password reset

## Key User State Values
- 0 = Unapproved (pending approval)
- 1 = Approved (licensed, normal user)
- 2 = Rejected
- 3 = Unlicensed

## Lock/Unlock Behavior
- `onelogin_lock_user`: Locks user for specified duration. To unlock early, use `onelogin_update_user` with status=1
- `onelogin_unlock_user`: ONLY works for users locked due to failed login attempts, NOT for manually locked users

## Role Management
- Use `onelogin_assign_roles` and `onelogin_remove_roles` instead of update_user for role changes
- These are additive/subtractive operations, not replacements

## Error Handling
- 403 errors on Account Owner users are expected - you cannot lock/modify the Account Owner
- Always check error messages for specific guidance

## Best Practices
1. Use `onelogin_list_users` with filters (email, username) to find users before operations
2. Verify user status after lock/unlock operations
3. Use `onelogin_get_user` to confirm changes were applied"#
                            }
                        }
                    ]
                })),
                error: None,
            }
        } else {
            Response {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(ResponseError {
                    code: -32602,
                    message: format!("Prompt not found: {}", name),
                    data: None,
                    tool_name: None,
                }),
            }
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
                error!(
                    "INVALID TOOL CALL PARAMS\n\
                     Request ID: {:?}\n\
                     Raw Params: {}\n\
                     Parse Error: {}\n\
                     \n\
                     The parameters provided do not match the expected schema.",
                    request.id,
                    serde_json::to_string_pretty(&request.params).unwrap_or_else(|_| "<failed to serialize>".to_string()),
                    e
                );
                return Response {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(ResponseError {
                        code: -32602,
                        message: format!(
                            "Invalid tool call parameters\n\
                             Parse Error: {}\n\
                             Raw Params: {}",
                            e,
                            serde_json::to_string_pretty(&request.params).unwrap_or_else(|_| "<failed to serialize>".to_string())
                        ),
                        data: None,
                        tool_name: None,
                    }),
                }
            }
        };

        info!("Calling tool: {}", params.name);
        debug!(
            "Tool call arguments: {}",
            serde_json::to_string_pretty(&params.arguments).unwrap_or_else(|_| "<failed to serialize>".to_string())
        );

        match self.tool_registry.call_tool(&params).await {
            Ok(result) => {
                info!("Tool {} completed successfully", params.name);
                debug!("Tool result (first 500 chars): {}", &result.chars().take(500).collect::<String>());
                Response {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "content": [{
                            "type": "text",
                            "text": result
                        }]
                    })),
                    error: None,
                }
            },
            Err(e) => {
                error!(
                    "TOOL EXECUTION FAILED\n\
                     Tool Name: {}\n\
                     Request ID: {:?}\n\
                     Arguments: {}\n\
                     Error: {}\n\
                     Error Type: {:?}\n\
                     \n\
                     This error occurred while executing the tool. Check the error message above for details.",
                    params.name,
                    request.id,
                    serde_json::to_string_pretty(&params.arguments).unwrap_or_else(|_| "<failed to serialize>".to_string()),
                    e,
                    e
                );
                Response {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(ResponseError {
                        code: -32000,
                        message: format!(
                            "Tool '{}' execution failed\n\
                             \n\
                             Arguments provided:\n{}\n\
                             \n\
                             Error details:\n{}\n\
                             \n\
                             Please check the logs for more detailed information about this error.",
                            params.name,
                            serde_json::to_string_pretty(&params.arguments).unwrap_or_else(|_| "<failed to serialize>".to_string()),
                            e
                        ),
                        data: Some(serde_json::json!({
                            "tool_name": params.name,
                            "arguments": params.arguments,
                            "error_message": e.to_string()
                        })),
                        tool_name: Some(params.name.clone()),
                    }),
                }
            },
        }
    }
    async fn read_frame<R>(reader: &mut BufReader<R>) -> Result<Option<IncomingFrame>>
    where
        R: tokio::io::AsyncRead + Unpin,
    {
        let mut headers = HashMap::new();
        let mut line = String::new();
        let mut raw_mode = false;
        let mut raw_payload = String::new();

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await?;

            if bytes_read == 0 {
                if raw_mode {
                    if raw_payload.trim().is_empty() {
                        return Ok(None);
                    } else {
                        return Err(anyhow!("Unexpected EOF while reading JSON payload"));
                    }
                }

                if headers.is_empty() {
                    return Ok(None);
                } else {
                    return Err(anyhow!("Unexpected EOF while reading headers"));
                }
            }

            let trimmed = line.trim_end_matches(|c| c == '\r' || c == '\n');

            if raw_mode {
                raw_payload.push_str(trimmed);
                match serde_json::from_str::<serde_json::Value>(&raw_payload) {
                    Ok(_) => {
                        return Ok(Some(IncomingFrame {
                            payload: raw_payload,
                            mode: TransportMode::RawJson,
                        }))
                    }
                    Err(e) if e.is_eof() => {
                        raw_payload.push('\n');
                        continue;
                    }
                    Err(e) => {
                        return Err(anyhow!("Invalid JSON payload: {}", e));
                    }
                }
            }

            if trimmed.is_empty() {
                if headers.is_empty() {
                    continue;
                } else {
                    break;
                }
            }

            let trimmed_start = trimmed.trim_start_matches(|c| c == ' ' || c == '\t');
            if headers.is_empty()
                && (trimmed_start.starts_with('{') || trimmed_start.starts_with('['))
            {
                raw_mode = true;
                raw_payload.push_str(trimmed);
                match serde_json::from_str::<serde_json::Value>(&raw_payload) {
                    Ok(_) => {
                        return Ok(Some(IncomingFrame {
                            payload: raw_payload,
                            mode: TransportMode::RawJson,
                        }))
                    }
                    Err(e) if e.is_eof() => {
                        raw_payload.push('\n');
                        continue;
                    }
                    Err(e) => {
                        return Err(anyhow!("Invalid JSON payload: {}", e));
                    }
                }
            }

            if let Some((name, value)) = trimmed.split_once(':') {
                headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
            } else {
                return Err(anyhow!("Invalid header line: {}", trimmed));
            }
        }

        let content_length = headers
            .get("content-length")
            .ok_or_else(|| anyhow!("Missing Content-Length header"))?
            .parse::<usize>()
            .map_err(|e| anyhow!("Invalid Content-Length header: {}", e))?;

        let mut buffer = vec![0u8; content_length];
        reader.read_exact(&mut buffer).await?;

        Ok(Some(IncomingFrame {
            payload: String::from_utf8(buffer)?,
            mode: TransportMode::ContentLength,
        }))
    }

    async fn write_frame<W>(writer: &mut W, payload: &str, mode: TransportMode) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        match mode {
            TransportMode::ContentLength => {
                let bytes = payload.as_bytes();
                let header = format!("Content-Length: {}\r\n\r\n", bytes.len());
                writer.write_all(header.as_bytes()).await?;
                writer.write_all(bytes).await?;
            }
            TransportMode::RawJson => {
                writer.write_all(payload.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct Request {
    jsonrpc: String,
    #[serde(default)]
    id: Option<serde_json::Value>,
    method: String,
    #[serde(default)]
    params: serde_json::Value,
}

#[derive(Debug, serde::Serialize)]
struct Response {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<serde_json::Value>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CallToolParams {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::McpServer;
    use tokio::io::{self, AsyncWriteExt, BufReader};

    #[tokio::test]
    async fn read_frame_supports_content_length_transport() {
        let (mut client, server) = io::duplex(1024);
        let mut reader = BufReader::new(server);
        let payload = r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#;
        let frame = format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload);

        client.write_all(frame.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();

        let frame = McpServer::read_frame(&mut reader).await.unwrap();
        assert!(frame.is_some());
        let frame = frame.unwrap();
        assert_eq!(frame.payload, payload.to_string());
        assert_eq!(frame.mode, super::TransportMode::ContentLength);
    }

    #[tokio::test]
    async fn read_frame_supports_raw_json_transport() {
        let (mut client, server) = io::duplex(1024);
        let mut reader = BufReader::new(server);
        let payload = r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#;
        let frame = format!("{}\n", payload);

        client.write_all(frame.as_bytes()).await.unwrap();
        client.shutdown().await.unwrap();

        let frame = McpServer::read_frame(&mut reader).await.unwrap();
        assert!(frame.is_some());
        let frame = frame.unwrap();
        assert_eq!(frame.payload, payload.to_string());
        assert_eq!(frame.mode, super::TransportMode::RawJson);
    }
}
