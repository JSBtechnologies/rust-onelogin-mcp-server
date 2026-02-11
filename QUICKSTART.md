# OneLogin MCP Server - Quick Start Guide

## Get Started in 5 Minutes

### 1. Prerequisites

Ensure you have:
- **Rust** 1.70+ installed ([rustup.rs](https://rustup.rs/))
- **OneLogin API credentials** (Client ID and Secret)

### 2. Configuration

**Option A: Interactive Setup (Recommended)**

Run the setup wizard for guided configuration with credential testing:

```bash
python3 scripts/setup.py
```

The wizard will prompt for your credentials, validate them against OneLogin's API, and create the `.env` file automatically.

**Option B: Manual Configuration**

```bash
cp .env.example .env
```

Edit `.env` with your credentials:

```env
ONELOGIN_CLIENT_ID=your_client_id_here
ONELOGIN_CLIENT_SECRET=your_client_secret_here
ONELOGIN_REGION=us
ONELOGIN_SUBDOMAIN=your_company
CACHE_TTL_SECONDS=300
RATE_LIMIT_RPS=10
```

### 3. Build

```bash
cargo build --release
```

This will:
- Download dependencies
- Compile the project
- Create optimized binary at `target/release/onelogin-mcp-server`

### 4. Run

```bash
cargo run --release
```

Or use the binary directly:

```bash
./target/release/onelogin-mcp-server
```

### 5. Test with MCP Client

Send an initialization request:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {}
}
```

List all available tools:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}
```

Call a tool (e.g., list users):

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

## Common Use Cases

### Create a User

```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "onelogin_create_user",
    "arguments": {
      "email": "newuser@example.com",
      "username": "newuser",
      "firstname": "New",
      "lastname": "User"
    }
  }
}
```

### Get Risk Score

```json
{
  "jsonrpc": "2.0",
  "id": 5,
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

### Create Smart Hook

```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "tools/call",
  "params": {
    "name": "onelogin_create_smart_hook",
    "arguments": {
      "type": "pre-authentication",
      "function": "ZnVuY3Rpb24gaGFuZGxlcihldmVudCkge1xuICByZXR1cm4gZXZlbnQ7XG59",
      "runtime": "nodejs18.x"
    }
  }
}
```

### Assign User to Role

```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "tools/call",
  "params": {
    "name": "onelogin_assign_roles_to_user",
    "arguments": {
      "user_id": 12345,
      "role_ids": [892924]
    }
  }
}
```

## Available Tools (177)

The server exposes 177 tools across 28 API domains including Users, Apps, Roles, Groups, Smart Hooks, Vigilance/Risk, MFA, App Rules, Branding, Connectors, Events, Privileges, Reports, SAML, and more.

See the [README](README.md#api-coverage) for the full API coverage table.

## Troubleshooting

### Build Errors

If you encounter build errors:

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Authentication Issues

Verify your credentials:

```bash
echo $ONELOGIN_CLIENT_ID
echo $ONELOGIN_REGION
echo $ONELOGIN_SUBDOMAIN
```

### Rate Limiting

If you see 429 errors, adjust rate limiting:

```env
RATE_LIMIT_RPS=5  # Reduce requests per second
```

### Debugging

Enable debug logging:

```bash
RUST_LOG=debug cargo run
```

## Next Steps

1. **Explore Tools**: Use `tools/list` to see all 177 available tools
2. **Read Documentation**: See [README.md](README.md) for full documentation
3. **Integrate with Claude**: See [INTEGRATION.md](INTEGRATION.md) for Claude Desktop setup
4. **Customize**: Modify source code for your specific needs
5. **Deploy**: Containerize with Docker for production deployment

## Docker Deployment (Optional)

Create `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/onelogin-mcp-server /usr/local/bin/
CMD ["onelogin-mcp-server"]
```

Build and run:

```bash
docker build -t onelogin-mcp-server .
docker run -e ONELOGIN_CLIENT_ID=... -e ONELOGIN_CLIENT_SECRET=... onelogin-mcp-server
```

## Support

For issues or questions:
- Check the implementation documentation
- Review source code comments
- Open an issue on GitHub (if applicable)

## License

MIT License - See LICENSE file

---

**You're all set!** 🚀

The OneLogin MCP Server is now ready to use with complete API coverage across 28 domains and 177 tools.
