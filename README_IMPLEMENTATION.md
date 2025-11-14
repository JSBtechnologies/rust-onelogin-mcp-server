# OneLogin MCP Server - Implementation Guide

## Overview

This is a comprehensive MCP (Model Context Protocol) server implementation for OneLogin API with 100% endpoint coverage across all 17 API domains.

## Features

### Complete API Coverage (17 Domains)

1. **Users API** - Full user lifecycle management
2. **Apps API** - Application management
3. **Roles API** - Role-based access control
4. **Groups API** - Group management
5. **MFA API** - Multi-factor authentication
6. **SAML API** - SAML SSO integration
7. **Smart Hooks API** - Custom authentication logic
8. **Vigilance AI / Risk API** - Real-time risk scoring
9. **Privileges API** - Delegated administration
10. **User Mappings API** - Automated provisioning rules
11. **Policies API** - Access policies
12. **Invitations API** - User invitations
13. **Custom Attributes API** - Custom user fields
14. **Embed Tokens API** - SSO embedding
15. **OAuth API** - OAuth 2.0 token management
16. **Webhooks API** - Webhook event management
17. **SCIM API** - SCIM 2.0 provisioning
18. **OIDC API** - OpenID Connect endpoints
19. **Directories API** - Directory sync (AD/LDAP/Azure AD)
20. **Branding API** - UI customization
21. **Events API** - Audit logs and events
22. **Sessions API** - Session management
23. **API Authorization API** - API auth server configuration

### Architecture Features

- **Async/Await**: Built on Tokio for high-performance async I/O
- **Connection Pooling**: Efficient HTTP connection reuse
- **Rate Limiting**: Configurable rate limiting to respect API limits
- **Caching**: Intelligent caching with TTL support
- **Circuit Breaker**: Fault tolerance for external service failures
- **OAuth Token Management**: Automatic token refresh
- **Comprehensive Error Handling**: Detailed error types and retry logic
- **Structured Logging**: Tracing-based logging for observability

## Installation

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- OneLogin account with API credentials

### Build

```bash
cargo build --release
```

### Configuration

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your OneLogin credentials:
   ```env
   ONELOGIN_CLIENT_ID=your_client_id
   ONELOGIN_CLIENT_SECRET=your_client_secret
   ONELOGIN_REGION=us
   ONELOGIN_SUBDOMAIN=your_subdomain
   ```

### Run

```bash
cargo run --release
```

Or use the compiled binary:
```bash
./target/release/onelogin-mcp-server
```

## MCP Tools

The server exposes 100+ tools through the MCP protocol, organized by API domain:

### Users (9 tools)
- `onelogin_list_users` - List all users with filtering
- `onelogin_get_user` - Get user details
- `onelogin_create_user` - Create new user
- `onelogin_update_user` - Update user information
- `onelogin_delete_user` - Delete user
- `onelogin_get_user_apps` - Get user's applications
- `onelogin_get_user_roles` - Get user's roles
- `onelogin_lock_user` - Lock user account
- `onelogin_logout_user` - Force user logout

### Smart Hooks (7 tools)
- `onelogin_create_smart_hook` - Create custom authentication logic
- `onelogin_update_smart_hook` - Update hook configuration
- `onelogin_delete_smart_hook` - Remove hook
- `onelogin_get_smart_hook` - Get hook details
- `onelogin_list_smart_hooks` - List all hooks
- `onelogin_get_smart_hook_logs` - View execution logs
- `onelogin_update_hook_env_vars` - Update environment variables

### Vigilance / Risk API (8 tools)
- `onelogin_get_risk_score` - Get real-time risk score
- `onelogin_validate_user_smart_mfa` - Smart MFA validation
- `onelogin_list_risk_rules` - List risk rules
- `onelogin_create_risk_rule` - Create risk rule
- `onelogin_update_risk_rule` - Update risk rule
- `onelogin_delete_risk_rule` - Delete risk rule
- `onelogin_get_risk_events` - Get risk events
- `onelogin_track_risk_event` - Track custom risk event

### SCIM API (9 tools)
- `onelogin_scim_get_users` - SCIM user list with filtering
- `onelogin_scim_create_user` - SCIM user provisioning
- `onelogin_scim_get_user` - Get SCIM user
- `onelogin_scim_update_user` - Update SCIM user
- `onelogin_scim_patch_user` - SCIM PATCH operations
- `onelogin_scim_delete_user` - Deprovision user
- `onelogin_scim_get_groups` - List SCIM groups
- `onelogin_scim_create_group` - Create SCIM group
- `onelogin_scim_bulk_operations` - Bulk SCIM operations

*... and 70+ more tools across all other API domains*

## Usage Examples

### Example 1: List Users

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "onelogin_list_users",
    "arguments": {
      "limit": 100
    }
  }
}
```

### Example 2: Create Smart Hook

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "onelogin_create_smart_hook",
    "arguments": {
      "type": "pre-authentication",
      "function": "ZnVuY3Rpb24gaGFuZGxlcihldmVudCkge1xuICByZXR1cm4gZXZlbnQ7XG59",
      "runtime": "nodejs18.x",
      "options": {
        "risk_enabled": true
      }
    }
  }
}
```

### Example 3: Get Risk Score

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "onelogin_get_risk_score",
    "arguments": {
      "user_identifier": "user@example.com",
      "ip_address": "192.168.1.1",
      "user_agent": "Mozilla/5.0..."
    }
  }
}
```

## Development

### Project Structure

```
onelogin-mcp-server/
├── src/
│   ├── main.rs                 # Entry point
│   ├── api/                    # API client modules (23 files)
│   │   ├── mod.rs
│   │   ├── users.rs
│   │   ├── smart_hooks.rs
│   │   ├── vigilance.rs
│   │   ├── scim.rs
│   │   └── ... (19 more)
│   ├── core/                   # Core infrastructure
│   │   ├── auth.rs            # OAuth token management
│   │   ├── client.rs          # HTTP client
│   │   ├── config.rs          # Configuration
│   │   ├── error.rs           # Error types
│   │   ├── cache.rs           # Caching layer
│   │   ├── rate_limit.rs      # Rate limiting
│   │   └── circuit_breaker.rs # Circuit breaker
│   ├── mcp/                    # MCP protocol
│   │   ├── server.rs          # MCP server
│   │   └── tools.rs           # Tool registry (100+ tools)
│   ├── models/                 # Data models (23 files)
│   │   ├── mod.rs
│   │   ├── users.rs
│   │   ├── smart_hooks.rs
│   │   └── ... (20 more)
│   └── utils/                  # Utilities
│       ├── mod.rs
│       └── serde_helpers.rs
├── Cargo.toml
├── .env.example
└── README.md
```

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Adding New Tools

1. Add model in `src/models/`
2. Add API client method in `src/api/`
3. Add tool definition in `src/mcp/tools.rs`
4. Add tool handler in `src/mcp/tools.rs`

## Performance Tuning

### Caching

Adjust cache TTL based on your needs:
```env
CACHE_TTL_SECONDS=600  # 10 minutes
```

### Rate Limiting

Increase for higher throughput (respecting OneLogin limits):
```env
RATE_LIMIT_RPS=20
```

### Connection Pooling

HTTP client automatically pools connections with:
- Max 10 idle connections per host
- 30 second timeout
- Automatic keep-alive

## Security Considerations

1. **Credentials**: Never commit `.env` file to version control
2. **Token Storage**: Tokens are stored in memory with automatic refresh
3. **TLS**: All API calls use HTTPS with rustls
4. **Rate Limiting**: Prevents accidental API abuse
5. **Input Validation**: All tool inputs are validated
6. **Error Handling**: Errors don't leak sensitive information

## Migration Use Cases

This MCP server is particularly useful for OneLogin migrations:

1. **Password Migration**: Use Smart Hooks with user-migration type
2. **Automated Provisioning**: User mappings for automatic role assignment
3. **Custom Attributes**: Preserve all user metadata from source systems
4. **SCIM Integration**: Automated user lifecycle management
5. **Risk Monitoring**: Track migration-related anomalies

## Troubleshooting

### Authentication Errors

```bash
# Verify credentials
echo $ONELOGIN_CLIENT_ID
echo $ONELOGIN_REGION
echo $ONELOGIN_SUBDOMAIN
```

### Rate Limiting

If you see 429 errors, reduce `RATE_LIMIT_RPS` or add delays.

### Token Expiration

Tokens are automatically refreshed 5 minutes before expiration.

## License

MIT

## Contributing

Contributions welcome! Please ensure:
- Code compiles without warnings
- Tests pass
- Follow Rust style guidelines (rustfmt)
- Update documentation
