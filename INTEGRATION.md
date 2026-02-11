# Claude Desktop Integration Guide

This guide explains how to integrate the OneLogin MCP Server with Claude Desktop.

## Quick Setup (Recommended)

### 1. Run the Setup Script

```bash
cd /path/to/onelogin-mcp-server

# First time: Configure credentials
cp .env.example .env
# Edit .env with your OneLogin credentials

# Run setup
./setup-claude.sh
```

The script will:
- ✅ Build the server (if needed)
- ✅ Backup existing Claude config
- ✅ Update Claude Desktop configuration
- ✅ Show next steps

### 2. Restart Claude Desktop

**Important:** Completely quit and restart Claude Desktop for changes to take effect.

### 3. Verify Integration

In Claude, ask:
```
What MCP tools do you have available?
```

You should see 177 OneLogin tools listed.

## Manual Setup

If you prefer to configure manually:

### 1. Build the Server

```bash
cd /path/to/onelogin-mcp-server
cargo build --release
```

Binary location: `target/release/onelogin-mcp-server`

### 2. Configure Credentials

**Option A: Interactive Setup (Recommended)**
```bash
python3 scripts/setup.py
```

**Option B: Manual Configuration**

Create `.env` file:
```bash
cp .env.example .env
```

Edit `.env` with your OneLogin API credentials:
```env
ONELOGIN_CLIENT_ID=your_client_id_here
ONELOGIN_CLIENT_SECRET=your_client_secret_here
ONELOGIN_REGION=us
ONELOGIN_SUBDOMAIN=your_company
```

### 3. Add to Claude Desktop Config

Edit the Claude Desktop configuration file:

**macOS:**
```bash
nano ~/Library/Application\ Support/Claude/claude_desktop_config.json
```

**Windows:**
```powershell
notepad %APPDATA%\Claude\claude_desktop_config.json
```

**Linux:**
```bash
nano ~/.config/Claude/claude_desktop_config.json
```

Add this configuration:

```json
{
  "mcpServers": {
    "onelogin": {
      "command": "/path/to/onelogin-mcp-server/target/release/onelogin-mcp-server"
    }
  }
}
```

**Note:** Replace the path with your actual installation path.

### 4. Restart Claude Desktop

Completely quit and restart Claude Desktop.

## Usage Examples

Once integrated, you can ask Claude to perform OneLogin operations naturally:

### User Management

**List Users:**
```
Show me the first 20 users in OneLogin
```

**Create User:**
```
Create a OneLogin user with:
- Email: john.doe@company.com
- Username: jdoe
- First name: John
- Last name: Doe
- Title: Software Engineer
```

**Update User:**
```
Update user ID 12345 to set their title to "Senior Engineer"
```

**Lock User:**
```
Lock user ID 12345 for 30 minutes
```

### Smart Hooks

**Create Smart Hook:**
```
Create a pre-authentication Smart Hook that:
1. Checks if the user's email domain is @company.com
2. Allows login if yes, deny if no
3. Log all attempts
```

**View Hook Logs:**
```
Show me the execution logs for Smart Hook abc123
```

### Risk & Security

**Check Risk Score:**
```
What's the risk score for user jane@company.com logging in from IP 192.168.1.100?
```

**Create Risk Rule:**
```
Create a risk rule that requires MFA when users login from a new country
```

**Validate with Smart MFA:**
```
Validate user john@company.com and trigger Smart MFA if needed
```

### Reports

**Run a Report:**
```
Run the user activity report and show me the results
```

**List Available Reports:**
```
What reports are available in OneLogin?
```

### Directory Sync

**Setup Directory Connector:**
```
Create an Active Directory connector with:
- Name: Corporate AD
- Host: ad.company.com
- Port: 389
- Base DN: DC=company,DC=com
```

**Trigger Sync:**
```
Trigger a sync for directory connector dir123 and show me the status
```

### Applications

**List Apps:**
```
Show me all applications in OneLogin
```

**Get User's Apps:**
```
What applications is user ID 12345 assigned to?
```

### Roles & Groups

**Create Role:**
```
Create a role called "Developers" with access to GitHub and AWS apps
```

**List Groups:**
```
Show me all groups in OneLogin
```

### MFA Management

**List MFA Devices:**
```
What MFA devices does user ID 12345 have?
```

**Enroll MFA:**
```
Enroll a new OTP device for user ID 12345
```

### Custom Attributes

**Create Custom Attribute:**
```
Create a custom user attribute called "employee_id" of type string
```

**List Custom Attributes:**
```
Show me all custom attributes defined in OneLogin
```

### Events & Monitoring

**List Recent Events:**
```
Show me the last 50 events from OneLogin
```

**Filter Events:**
```
Show me all failed login events from the last 24 hours
```

### User Mappings

**Create Mapping:**
```
Create a user mapping that automatically assigns users from the Engineering department to the Developers role
```

### Branding

**Update Branding:**
```
Update the OneLogin login page branding with our company logo and colors
```

## Advanced Usage

### Chaining Multiple Operations

Claude can intelligently chain multiple OneLogin operations:

```
Create a new user for sarah@company.com, then assign them to the
"Marketing" role, enroll them in MFA, and send them an invitation email
```

Claude will:
1. Use `onelogin_create_user`
2. Use `onelogin_assign_role` or update the user
3. Use `onelogin_enroll_mfa_factor`
4. Use `onelogin_send_invite_link`

### Data Analysis

Ask Claude to analyze OneLogin data:

```
List all users, analyze their departments, and tell me which
department has the most users
```

### Migration Tasks

Use for OneLogin migrations:

```
I'm migrating from Okta. Help me:
1. Create a user mapping rule for automatic role assignment
2. Set up a Smart Hook for password migration
3. Create custom attributes to preserve Okta user IDs
```

### Bulk Operations

Perform bulk operations:

```
Create 10 test users with sequential email addresses
(test1@company.com through test10@company.com)
```

### Compliance & Auditing

```
Show me all users who haven't logged in for the last 90 days
```

```
List all privileged users and their assigned privileges
```

## Troubleshooting

### Claude Doesn't See the Tools

**Problem:** Claude says "I don't have access to OneLogin tools"

**Solutions:**
1. Verify Claude config file exists and is valid JSON
2. Check the binary path is correct
3. Restart Claude Desktop completely (quit from menu bar)
4. Check logs in Claude Desktop developer tools

### Authentication Errors

**Problem:** Tools fail with authentication errors

**Solutions:**
1. Verify credentials in `.env` file
2. Check OneLogin API permissions
3. Verify region (US/EU) matches your instance
4. Test credentials manually: `cargo run --release`

### Server Not Starting

**Problem:** Claude shows "Server failed to start"

**Solutions:**
1. Rebuild: `cargo build --release`
2. Check binary permissions: `chmod +x target/release/onelogin-mcp-server`
3. Test manually: `./target/release/onelogin-mcp-server`
4. Check `.env` file exists

### Rate Limiting

**Problem:** Getting rate limit errors

**Solutions:**
1. Reduce `RATE_LIMIT_RPS` in `.env`
2. Increase `CACHE_TTL_SECONDS`
3. Batch operations where possible

## Configuration Options

### Environment Variables

Configure in `.env` file:

```env
# Required
ONELOGIN_CLIENT_ID=your_client_id
ONELOGIN_CLIENT_SECRET=your_secret
ONELOGIN_REGION=us                  # or 'eu'
ONELOGIN_SUBDOMAIN=yourcompany

# Optional (with defaults)
CACHE_TTL_SECONDS=300              # 5 minutes
RATE_LIMIT_RPS=10                  # requests per second
ENABLE_METRICS=false               # Prometheus metrics
```

### Alternative: Inline Configuration

Instead of `.env`, you can put credentials directly in Claude config:

```json
{
  "mcpServers": {
    "onelogin": {
      "command": "/path/to/onelogin-mcp-server",
      "env": {
        "ONELOGIN_CLIENT_ID": "your_id",
        "ONELOGIN_CLIENT_SECRET": "your_secret",
        "ONELOGIN_REGION": "us",
        "ONELOGIN_SUBDOMAIN": "yourcompany",
        "CACHE_TTL_SECONDS": "300",
        "RATE_LIMIT_RPS": "10"
      }
    }
  }
}
```

**⚠️ Security Note:** Using `.env` is more secure as credentials aren't in Claude's config.

## Debugging

### Enable Debug Logging

Add to Claude config:

```json
{
  "mcpServers": {
    "onelogin": {
      "command": "/path/to/onelogin-mcp-server",
      "env": {
        "RUST_LOG": "debug"
      }
    }
  }
}
```

### View Logs

Claude Desktop logs location:
- **macOS**: `~/Library/Logs/Claude/`
- **Windows**: `%APPDATA%\Claude\logs\`
- **Linux**: `~/.config/Claude/logs/`

### Test Manually

Test the server outside of Claude:

```bash
cd /path/to/onelogin-mcp-server

# Set credentials
export ONELOGIN_CLIENT_ID=your_id
export ONELOGIN_CLIENT_SECRET=your_secret
export ONELOGIN_REGION=us
export ONELOGIN_SUBDOMAIN=yourcompany

# Run server
./target/release/onelogin-mcp-server

# In another terminal, send test request
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | \
  ./target/release/onelogin-mcp-server
```

## Security Best Practices

1. **Never commit `.env`** - Already in `.gitignore`
2. **Use API credentials with minimum required permissions**
3. **Rotate credentials regularly**
4. **Use read-only credentials when possible**
5. **Monitor API usage in OneLogin admin portal**
6. **Keep Claude Desktop updated**

## Performance Tips

1. **Increase cache TTL** for rarely changing data
2. **Reduce rate limits** if hitting API limits
3. **Use specific queries** instead of listing all data
4. **Batch operations** when creating multiple resources

## Available Tools (177)

See the full list in Claude by asking:
```
List all OneLogin MCP tools you have available
```

Or check the documentation:
- [README.md](README.md#api-coverage) - Full API coverage table

## Support

- **Issues**: Check existing documentation first
- **Errors**: Review logs in Claude Desktop
- **Questions**: Ask Claude directly about OneLogin capabilities
- **Updates**: Pull latest changes and rebuild

## Next Steps

1. ✅ Complete setup using `./setup-claude.sh`
2. ✅ Restart Claude Desktop
3. ✅ Verify tools are available
4. 🚀 Start automating OneLogin tasks with Claude!

---

**You're all set!** Claude now has access to 177 OneLogin tools across 28 API domains for complete API automation.

---

## Other MCP Clients

This server works with any MCP-compatible client, not just Claude Desktop. To use with other clients:

- **Claude Code CLI** - Add the server to your Claude Code MCP settings
- **Other MCP clients** - Configure with the same binary path and environment variables
