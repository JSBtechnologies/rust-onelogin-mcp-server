# OneLogin MCP Server

<div align="center">

**A comprehensive Model Context Protocol (MCP) server for OneLogin API**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![API Coverage](https://img.shields.io/badge/API%20Coverage-100%25-brightgreen.svg)](#api-coverage)

[Features](#features) • [Quick Start](#quick-start) • [Documentation](#documentation) • [API Coverage](#api-coverage) • [Contributing](#contributing)

</div>

---

## Overview

A production-ready MCP server implementation providing **100% coverage** of the OneLogin API across all 23 API domains. Built in Rust for performance, reliability, and type safety, this server exposes 100+ tools through the Model Context Protocol for seamless integration with AI assistants and automation workflows.

### Key Features

- ✅ **Complete API Coverage** - All 23 OneLogin API domains fully implemented
- 🚀 **100+ MCP Tools** - Every OneLogin capability accessible via MCP protocol
- ⚡ **High Performance** - Built with Tokio async runtime for concurrent operations
- 🔒 **Secure** - OAuth 2.0 token management, TLS encryption, secret handling
- 📊 **Production Ready** - Rate limiting, caching, circuit breaker, comprehensive error handling
- 🎯 **Migration Focused** - Special features for OneLogin migration scenarios
- 🛠️ **Type Safe** - Full Rust type system ensures reliability
- 📝 **Well Documented** - Extensive inline documentation and usage examples

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- OneLogin account with API credentials (Client ID and Secret)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd onelogin-mcp-server
   ```

2. **Configure environment**
   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your OneLogin credentials:
   ```env
   ONELOGIN_CLIENT_ID=your_client_id_here
   ONELOGIN_CLIENT_SECRET=your_client_secret_here
   ONELOGIN_REGION=us                    # or 'eu'
   ONELOGIN_SUBDOMAIN=your_company
   CACHE_TTL_SECONDS=300
   RATE_LIMIT_RPS=10
   ```

3. **Build and run**
   ```bash
   cargo build --release
   cargo run --release
   ```

The server will start and listen for MCP protocol messages on stdin/stdout.

## Usage

### MCP Protocol Interaction

The server implements the [Model Context Protocol](https://modelcontextprotocol.io/) and responds to JSON-RPC requests.

#### Initialize the server

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {}
}
```

#### List available tools

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}
```

#### Call a tool

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "onelogin_list_users",
    "arguments": {
      "limit": 10
    }
  }
}
```

### Common Use Cases

<details>
<summary><b>User Management</b></summary>

**Create a user:**
```json
{
  "name": "onelogin_create_user",
  "arguments": {
    "email": "newuser@example.com",
    "username": "newuser",
    "firstname": "Jane",
    "lastname": "Doe",
    "title": "Software Engineer"
  }
}
```

**Update a user:**
```json
{
  "name": "onelogin_update_user",
  "arguments": {
    "user_id": 12345,
    "title": "Senior Software Engineer",
    "department": "Engineering"
  }
}
```

**Lock a user account:**
```json
{
  "name": "onelogin_lock_user",
  "arguments": {
    "user_id": 12345,
    "minutes": 30
  }
}
```
</details>

<details>
<summary><b>Smart Hooks (Custom Authentication Logic)</b></summary>

**Create a pre-authentication hook:**
```json
{
  "name": "onelogin_create_smart_hook",
  "arguments": {
    "type": "pre-authentication",
    "function": "ZnVuY3Rpb24gaGFuZGxlcihldmVudCkge1xuICByZXR1cm4gZXZlbnQ7XG59",
    "runtime": "nodejs18.x",
    "options": {
      "risk_enabled": true,
      "location_enabled": true
    }
  }
}
```

**Get hook execution logs:**
```json
{
  "name": "onelogin_get_smart_hook_logs",
  "arguments": {
    "hook_id": "abc123"
  }
}
```
</details>

<details>
<summary><b>Risk & Security (Vigilance AI)</b></summary>

**Get real-time risk score:**
```json
{
  "name": "onelogin_get_risk_score",
  "arguments": {
    "user_identifier": "user@example.com",
    "ip_address": "192.168.1.100",
    "user_agent": "Mozilla/5.0..."
  }
}
```

**Validate user with Smart MFA:**
```json
{
  "name": "onelogin_validate_user_smart_mfa",
  "arguments": {
    "user_identifier": "user@example.com",
    "context": {
      "ip_address": "192.168.1.100",
      "user_agent": "Mozilla/5.0..."
    }
  }
}
```

**Create risk rule:**
```json
{
  "name": "onelogin_create_risk_rule",
  "arguments": {
    "name": "Detect Impossible Travel",
    "enabled": true,
    "conditions": [
      {
        "field": "location_change_rate",
        "operator": "greater_than",
        "value": "500"
      }
    ],
    "action": {
      "action_type": "require_mfa"
    },
    "priority": 1
  }
}
```
</details>

<details>
<summary><b>SCIM Provisioning</b></summary>

**Create user via SCIM:**
```json
{
  "name": "onelogin_scim_create_user",
  "arguments": {
    "schemas": ["urn:ietf:params:scim:schemas:core:2.0:User"],
    "userName": "user@example.com",
    "name": {
      "givenName": "John",
      "familyName": "Smith"
    },
    "emails": [
      {
        "value": "user@example.com",
        "type": "work",
        "primary": true
      }
    ],
    "active": true
  }
}
```

**Bulk SCIM operations:**
```json
{
  "name": "onelogin_scim_bulk_operations",
  "arguments": {
    "schemas": ["urn:ietf:params:scim:api:messages:2.0:BulkRequest"],
    "operations": [
      {
        "method": "POST",
        "path": "/Users",
        "data": { /* user data */ }
      },
      {
        "method": "PATCH",
        "path": "/Users/123",
        "data": { /* patch operations */ }
      }
    ]
  }
}
```
</details>

<details>
<summary><b>Directory Synchronization</b></summary>

**Create directory connector:**
```json
{
  "name": "onelogin_create_directory_connector",
  "arguments": {
    "name": "Corporate AD",
    "connector_type": "active_directory",
    "configuration": {
      "host": "ad.company.com",
      "port": 389,
      "bind_dn": "CN=Service,DC=company,DC=com",
      "base_dn": "DC=company,DC=com"
    }
  }
}
```

**Trigger sync:**
```json
{
  "name": "onelogin_sync_directory",
  "arguments": {
    "connector_id": "dir123"
  }
}
```
</details>

## API Coverage

This server provides **100% coverage** of the OneLogin API across all 23 domains:

### Core Identity Management
| Domain | Tools | Description |
|--------|-------|-------------|
| 👤 **Users** | 9 | Complete user lifecycle management |
| 🎭 **Roles** | 5 | Role-based access control |
| 👥 **Groups** | 5 | Group management and assignments |

### Application & Access
| Domain | Tools | Description |
|--------|-------|-------------|
| 📱 **Apps** | 5 | Application configuration and management |
| 🔐 **MFA** | 4 | Multi-factor authentication devices |
| 🎫 **SAML** | 2 | SAML SSO assertion generation |
| 🔑 **OAuth** | 3 | OAuth 2.0 token management |
| 🌐 **OIDC** | 3 | OpenID Connect endpoints |
| 💺 **Sessions** | 3 | User session management |

### Advanced Security
| Domain | Tools | Description |
|--------|-------|-------------|
| ⚡ **Smart Hooks** | 7 | Custom authentication logic (JavaScript) |
| 🛡️ **Vigilance AI** | 8 | Real-time risk scoring and Smart MFA |
| 📋 **Policies** | 6 | Access policies and enforcement |

### Administration & Governance
| Domain | Tools | Description |
|--------|-------|-------------|
| 👑 **Privileges** | 7 | Delegated administration privileges |
| 🏷️ **Custom Attributes** | 4 | Custom user fields and metadata |

### Provisioning & Integration
| Domain | Tools | Description |
|--------|-------|-------------|
| 🔄 **User Mappings** | 6 | Automated provisioning rules |
| 🔌 **SCIM** | 9 | SCIM 2.0 provisioning protocol |
| 📁 **Directories** | 7 | AD/LDAP/Azure AD synchronization |

### Communication & Branding
| Domain | Tools | Description |
|--------|-------|-------------|
| ✉️ **Invitations** | 5 | User invitation management |
| 🎨 **Branding** | 2 | UI customization and theming |

### Monitoring & Events
| Domain | Tools | Description |
|--------|-------|-------------|
| 📊 **Events** | 3 | Audit logs and event tracking |
| 🪝 **Webhooks** | 1 | Webhook event management |

### Developer Tools
| Domain | Tools | Description |
|--------|-------|-------------|
| 🔧 **API Authorization** | 5 | API auth server configuration |
| 🎁 **Embed Tokens** | 2 | SSO embedding capabilities |

**Total: 23 API Domains • 100+ Tools • 100% Coverage**

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                    MCP Server (JSON-RPC)                     │
├─────────────────────────────────────────────────────────────┤
│                   Tool Registry (100+ tools)                 │
├─────────────────────────────────────────────────────────────┤
│                   OneLogin API Client                        │
│  ┌──────────┬──────────┬──────────┬──────────┬───────────┐ │
│  │  Users   │  Smart   │Vigilance │   SCIM   │    ...    │ │
│  │   API    │  Hooks   │   API    │   API    │  19 more  │ │
│  └──────────┴──────────┴──────────┴──────────┴───────────┘ │
├─────────────────────────────────────────────────────────────┤
│                     Core Infrastructure                      │
│  ┌────────┬──────────┬─────────┬────────────┬───────────┐  │
│  │ Auth   │  HTTP    │  Cache  │Rate Limit  │  Circuit  │  │
│  │Manager │  Client  │         │            │  Breaker  │  │
│  └────────┴──────────┴─────────┴────────────┴───────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    OneLogin API (HTTPS)
```

### Key Components

- **MCP Server** - Handles JSON-RPC protocol, routes tool calls
- **Tool Registry** - Manages 100+ tool definitions and execution
- **API Clients** - 23 domain-specific API clients with typed models
- **Auth Manager** - OAuth 2.0 token lifecycle management
- **HTTP Client** - Connection pooling, retry logic, error handling
- **Cache Layer** - Moka-based caching with configurable TTL
- **Rate Limiter** - Governor-based rate limiting
- **Circuit Breaker** - Fault tolerance for API failures

### Technology Stack

- **Runtime**: Tokio (async/await)
- **HTTP**: Reqwest with rustls
- **Serialization**: Serde (JSON)
- **Caching**: Moka
- **Rate Limiting**: Governor
- **Logging**: Tracing
- **Error Handling**: Anyhow + Thiserror

## Configuration

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `ONELOGIN_CLIENT_ID` | ✅ Yes | - | OneLogin API Client ID |
| `ONELOGIN_CLIENT_SECRET` | ✅ Yes | - | OneLogin API Client Secret |
| `ONELOGIN_REGION` | ✅ Yes | - | Region: `us` or `eu` |
| `ONELOGIN_SUBDOMAIN` | ✅ Yes | - | Your OneLogin subdomain |
| `CACHE_TTL_SECONDS` | No | `300` | Cache time-to-live in seconds |
| `RATE_LIMIT_RPS` | No | `10` | Requests per second limit |
| `ENABLE_METRICS` | No | `false` | Enable Prometheus metrics |

### Getting OneLogin API Credentials

1. Log in to your OneLogin admin portal
2. Navigate to **Administration** → **Developers** → **API Credentials**
3. Click **New Credential**
4. Select **Read users**, **Manage users**, and other required permissions
5. Copy the **Client ID** and **Client Secret**

## Development

### Project Structure

```
onelogin-mcp-server/
├── Cargo.toml                   # Project configuration and dependencies
├── .env.example                 # Environment template
├── .gitignore                   # Git ignore rules
├── README.md                    # This file
├── QUICKSTART.md                # 5-minute getting started
├── IMPLEMENTATION_SUMMARY.md    # Detailed implementation info
└── src/
    ├── main.rs                  # Application entry point
    ├── api/                     # API client implementations
    │   ├── mod.rs              # OneLoginClient aggregator
    │   ├── users.rs            # Users API
    │   ├── smart_hooks.rs      # Smart Hooks API
    │   ├── vigilance.rs        # Vigilance/Risk API
    │   ├── scim.rs             # SCIM API
    │   └── ... (19 more)
    ├── core/                    # Core infrastructure
    │   ├── auth.rs             # OAuth token management
    │   ├── client.rs           # HTTP client
    │   ├── config.rs           # Configuration
    │   ├── error.rs            # Error types
    │   ├── cache.rs            # Caching layer
    │   ├── rate_limit.rs       # Rate limiting
    │   └── circuit_breaker.rs  # Circuit breaker
    ├── mcp/                     # MCP protocol
    │   ├── server.rs           # JSON-RPC server
    │   └── tools.rs            # Tool registry
    ├── models/                  # Data models (23 files)
    │   ├── users.rs
    │   ├── smart_hooks.rs
    │   └── ...
    └── utils/                   # Utility functions
        ├── mod.rs
        └── serde_helpers.rs
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check for errors without building
cargo check

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests with specific log level
RUST_LOG=debug cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Fix auto-fixable issues
cargo fix
```

## Deployment

### Docker

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/onelogin-mcp-server /usr/local/bin/
CMD ["onelogin-mcp-server"]
```

Build and run:

```bash
docker build -t onelogin-mcp-server .
docker run -e ONELOGIN_CLIENT_ID=... \
           -e ONELOGIN_CLIENT_SECRET=... \
           -e ONELOGIN_REGION=us \
           -e ONELOGIN_SUBDOMAIN=... \
           onelogin-mcp-server
```

### Kubernetes

Example deployment:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: onelogin-mcp-server
spec:
  replicas: 1
  selector:
    matchLabels:
      app: onelogin-mcp-server
  template:
    metadata:
      labels:
        app: onelogin-mcp-server
    spec:
      containers:
      - name: server
        image: onelogin-mcp-server:latest
        env:
        - name: ONELOGIN_CLIENT_ID
          valueFrom:
            secretKeyRef:
              name: onelogin-credentials
              key: client-id
        - name: ONELOGIN_CLIENT_SECRET
          valueFrom:
            secretKeyRef:
              name: onelogin-credentials
              key: client-secret
        - name: ONELOGIN_REGION
          value: "us"
        - name: ONELOGIN_SUBDOMAIN
          value: "mycompany"
```

### Production Checklist

- [ ] Use secret management (AWS Secrets Manager, Vault, etc.)
- [ ] Enable structured logging with log aggregation
- [ ] Set appropriate rate limits for your use case
- [ ] Configure cache TTL based on data freshness requirements
- [ ] Set up monitoring and alerting
- [ ] Implement health checks
- [ ] Use HTTPS for all external communications
- [ ] Rotate API credentials regularly
- [ ] Review and configure resource limits
- [ ] Set up automated backups if needed

## Migration Use Cases

This MCP server is particularly valuable for OneLogin migration scenarios:

### Password Migration
Use **Smart Hooks** with `user-migration` type to:
- Transparently migrate user passwords during first login
- Validate credentials against legacy system
- Store migrated passwords in OneLogin

### Automated Provisioning
Use **User Mappings** to:
- Automatically assign roles based on user attributes
- Map department to appropriate applications
- Create consistent user profiles

### Metadata Preservation
Use **Custom Attributes** to:
- Store legacy system identifiers
- Preserve custom user fields
- Maintain audit trails from source systems

### Continuous Sync
Use **Directory Connectors** to:
- Sync with AD/LDAP in real-time
- Keep Azure AD in sync
- Maintain Google Workspace integration

### Risk Monitoring
Use **Vigilance AI** to:
- Detect unusual login patterns during migration
- Identify potential security issues
- Enforce Smart MFA for suspicious activities

## Troubleshooting

### Common Issues

<details>
<summary><b>Authentication Errors</b></summary>

**Problem**: `Authentication failed` error

**Solutions**:
1. Verify credentials in `.env` file
2. Check that API credentials have correct permissions
3. Ensure region (US/EU) matches your OneLogin instance
4. Verify subdomain is correct

```bash
# Test credentials
echo $ONELOGIN_CLIENT_ID
echo $ONELOGIN_REGION
echo $ONELOGIN_SUBDOMAIN
```
</details>

<details>
<summary><b>Rate Limiting</b></summary>

**Problem**: Getting 429 (Too Many Requests) errors

**Solutions**:
1. Reduce `RATE_LIMIT_RPS` in `.env`
2. Increase cache TTL to reduce API calls
3. Batch operations where possible

```env
RATE_LIMIT_RPS=5  # Lower rate limit
CACHE_TTL_SECONDS=600  # Increase cache duration
```
</details>

<details>
<summary><b>Build Errors</b></summary>

**Problem**: Compilation fails

**Solutions**:
1. Update Rust: `rustup update`
2. Clean build artifacts: `cargo clean`
3. Check dependency versions: `cargo update`
4. Rebuild: `cargo build --release`
</details>

<details>
<summary><b>Connection Issues</b></summary>

**Problem**: Cannot connect to OneLogin API

**Solutions**:
1. Check internet connectivity
2. Verify firewall rules allow HTTPS (443)
3. Check if behind corporate proxy
4. Verify OneLogin service status
</details>

### Debug Mode

Enable detailed logging:

```bash
# Debug level
RUST_LOG=debug cargo run

# Trace level (very verbose)
RUST_LOG=trace cargo run

# Specific module
RUST_LOG=onelogin_mcp_server::api::users=debug cargo run
```

## Performance Tuning

### Optimization Tips

1. **Increase Cache TTL** - For rarely changing data
   ```env
   CACHE_TTL_SECONDS=600  # 10 minutes
   ```

2. **Adjust Rate Limits** - Based on your API tier
   ```env
   RATE_LIMIT_RPS=20  # If you have higher limits
   ```

3. **Connection Pool** - Already optimized (10 connections per host)

4. **Batch Operations** - Use bulk endpoints when available
   ```json
   {
     "name": "onelogin_scim_bulk_operations",
     "arguments": { /* bulk operations */ }
   }
   ```

### Performance Metrics

Expected performance characteristics:
- **Throughput**: 10-20 requests/second (configurable)
- **Latency**: <100ms for cached requests, ~200-500ms for API calls
- **Memory**: ~50MB baseline, scales with cache size
- **CPU**: Low (I/O bound, async runtime)

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Code Standards

- Follow Rust style guidelines (`cargo fmt`)
- Ensure code passes linter (`cargo clippy`)
- Add tests for new functionality
- Update documentation
- Write clear commit messages

## Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Get started in 5 minutes
- **[IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)** - Detailed implementation information
- **[README_IMPLEMENTATION.md](README_IMPLEMENTATION.md)** - Comprehensive usage guide
- **[.env.example](.env.example)** - Configuration template

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [Tokio](https://tokio.rs/) async runtime
- Implements [Model Context Protocol](https://modelcontextprotocol.io/)
- Integrates with [OneLogin API](https://developers.onelogin.com/)

## Support

For issues, questions, or contributions:
- Open an issue on GitHub
- Check existing documentation
- Review OneLogin API documentation

---

<div align="center">

**Built with ❤️ for OneLogin migrations and automation**

[⬆ Back to Top](#onelogin-mcp-server)

</div>
