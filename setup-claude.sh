#!/bin/bash
# Setup script for integrating OneLogin MCP Server with Claude Desktop

set -e

echo "🚀 OneLogin MCP Server - Claude Desktop Setup"
echo "=============================================="
echo ""

# Get the absolute path to the binary
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BINARY_PATH="$SCRIPT_DIR/target/release/onelogin-mcp-server"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Binary not found. Building now..."
    cargo build --release
    echo "✅ Build complete"
else
    echo "✅ Binary found at: $BINARY_PATH"
fi

# Determine Claude config path based on OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    CLAUDE_CONFIG="$HOME/Library/Application Support/Claude/claude_desktop_config.json"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    CLAUDE_CONFIG="$APPDATA/Claude/claude_desktop_config.json"
else
    CLAUDE_CONFIG="$HOME/.config/Claude/claude_desktop_config.json"
fi

echo ""
echo "📁 Claude config location: $CLAUDE_CONFIG"
echo ""

# Check if .env file exists
if [ ! -f "$SCRIPT_DIR/.env" ]; then
    echo "⚠️  No .env file found. Creating from template..."
    cp "$SCRIPT_DIR/.env.example" "$SCRIPT_DIR/.env"
    echo ""
    echo "📝 Please edit .env with your OneLogin credentials:"
    echo "   $SCRIPT_DIR/.env"
    echo ""
    echo "Then run this script again."
    exit 1
fi

echo "✅ Found .env file"
echo ""

# Create backup of existing config
if [ -f "$CLAUDE_CONFIG" ]; then
    BACKUP="$CLAUDE_CONFIG.backup.$(date +%Y%m%d_%H%M%S)"
    cp "$CLAUDE_CONFIG" "$BACKUP"
    echo "📦 Backed up existing config to: $BACKUP"
fi

# Create config directory if it doesn't exist
mkdir -p "$(dirname "$CLAUDE_CONFIG")"

# Load environment variables from .env file
source "$SCRIPT_DIR/.env"

# Normalize subdomain (strip domain suffix if user included it)
SUBDOMAIN_SLUG="${ONELOGIN_SUBDOMAIN}"
SUBDOMAIN_SLUG="${SUBDOMAIN_SLUG%.onelogin.com}"
SUBDOMAIN_SLUG="${SUBDOMAIN_SLUG%.eu.onelogin.com}"

if [ "$SUBDOMAIN_SLUG" != "$ONELOGIN_SUBDOMAIN" ]; then
    echo "ℹ️  Normalized ONELOGIN_SUBDOMAIN to '$SUBDOMAIN_SLUG' (removed domain suffix)"
fi

# Generate Claude config with environment variables
cat > "$CLAUDE_CONFIG" << EOF
{
  "mcpServers": {
    "onelogin": {
      "command": "$BINARY_PATH",
      "env": {
        "ONELOGIN_CLIENT_ID": "$ONELOGIN_CLIENT_ID",
        "ONELOGIN_CLIENT_SECRET": "$ONELOGIN_CLIENT_SECRET",
        "ONELOGIN_REGION": "$ONELOGIN_REGION",
        "ONELOGIN_SUBDOMAIN": "$SUBDOMAIN_SLUG",
        "CACHE_TTL_SECONDS": "$CACHE_TTL_SECONDS",
        "RATE_LIMIT_RPS": "$RATE_LIMIT_RPS",
        "ENABLE_METRICS": "$ENABLE_METRICS"
      }
    }
  }
}
EOF

echo "✅ Updated Claude Desktop configuration"
echo ""
echo "🎯 Next steps:"
echo "   1. Restart Claude Desktop"
echo "   2. Ask Claude: 'What MCP tools do you have available?'"
echo "   3. Start using OneLogin tools!"
echo ""
echo "📚 Example usage:"
echo "   - 'List the first 10 users in OneLogin'"
echo "   - 'Create a user with email test@company.com'"
echo "   - 'Get the risk score for user@company.com'"
echo ""
echo "✨ Setup complete!"
