#!/bin/bash
# Test script to verify the onelogin_update_role fix.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if binary exists
if [ ! -f "./target/release/onelogin-mcp-server" ]; then
    echo -e "${RED}Error: Server binary not found. Please run 'cargo build --release' first.${NC}"
    exit 1
fi

# Check environment variables
if [ -z "$ONELOGIN_CLIENT_ID" ] || [ -z "$ONELOGIN_CLIENT_SECRET" ]; then
    echo -e "${YELLOW}Warning: ONELOGIN_CLIENT_ID or ONELOGIN_CLIENT_SECRET not set.${NC}"
    echo "This test will fail without valid credentials."
    echo ""
fi

# Function to send MCP request and extract the "content" field from the response
call_tool() {
    local method="$1"
    local params="$2"

    local payload=$(cat <<EOF
{"jsonrpc":"2.0","id":1,"method":"$method","params":$params}
EOF
)

    # The response is a string containing JSON, we need to extract it
    local response_json=$(echo "$payload" | ./target/release/onelogin-mcp-server 2>/dev/null | grep -o '{.*}' | tail -1)

    if [ -z "$response_json" ]; then
        echo "Error: No response from server" >&2
        exit 1
    fi
    
    if echo "$response_json" | jq -e '.error' >/dev/null; then
        echo "Error: Tool call failed with error:" >&2
        echo "$response_json" | jq '.error' >&2
        exit 1
    fi

    echo "$response_json" | jq -r '.result.content'
}

echo "Running Update Role integration test..."

# 1. Create a new role
ROLE_NAME="mcp-test-role-$(date +%s)"
echo "Creating role with name: $ROLE_NAME"
create_response_str=$(call_tool "tools/call" "{\"name\":\"onelogin_create_role\",\"arguments\":{\"name\":\"$ROLE_NAME\"}}")
create_response=$(echo "$create_response_str")

ROLE_ID=$(echo "$create_response" | jq -r '.id')
if [ -z "$ROLE_ID" ] || [ "$ROLE_ID" == "null" ]; then
    echo -e "${RED}FAIL: Could not get role ID from create response.${NC}"
    echo "Response: $create_response"
    exit 1
fi
echo "Role created with ID: $ROLE_ID"

# 2. Update the role
NEW_NAME="mcp-test-role-updated-$(date +%s)"
NEW_DESCRIPTION="This is a test description."
echo "Updating role $ROLE_ID with new name: $NEW_NAME"
update_response_str=$(call_tool "tools/call" "{\"name\":\"onelogin_update_role\",\"arguments\":{\"role_id\":$ROLE_ID,\"name\":\"$NEW_NAME\",\"description\":\"$NEW_DESCRIPTION\"}}")
update_response=$(echo "$update_response_str")

# 3. Verify the role was updated
echo "Verifying update for role $ROLE_ID..."
get_response_str=$(call_tool "tools/call" "{\"name\":\"onelogin_get_role\",\"arguments\":{\"role_id\":$ROLE_ID}}")
get_response=$(echo "$get_response_str")

UPDATED_NAME=$(echo "$get_response" | jq -r '.name')
UPDATED_DESCRIPTION=$(echo "$get_response" | jq -r '.description')

echo "Retrieved name: $UPDATED_NAME"
echo "Retrieved description: $UPDATED_DESCRIPTION"

if [ "$UPDATED_NAME" != "$NEW_NAME" ] || [ "$UPDATED_DESCRIPTION" != "$NEW_DESCRIPTION" ]; then
    echo -e "${RED}FAIL: Role was not updated correctly.${NC}"
    echo "Expected name: $NEW_NAME, but got: $UPDATED_NAME"
    echo "Expected description: $NEW_DESCRIPTION, but got: $UPDATED_DESCRIPTION"
    
    # Cleanup before exiting
    echo "Cleaning up role $ROLE_ID..."
    call_tool "tools/call" "{\"name\":\"onelogin_delete_role\",\"arguments\":{\"role_id\":$ROLE_ID}}" > /dev/null
    echo "Role $ROLE_ID deleted."
    exit 1
fi

echo -e "${GREEN}PASS: Role updated successfully.${NC}"

# 4. Clean up the created role
echo "Cleaning up role $ROLE_ID..."
delete_response_str=$(call_tool "tools/call" "{\"name\":\"onelogin_delete_role\",\"arguments\":{\"role_id\":$ROLE_ID}}")
delete_response=$(echo "$delete_response_str")

DELETED_STATUS=$(echo "$delete_response" | jq -r '.status')

if [ "$DELETED_STATUS" == "deleted" ]; then
    echo "Role $ROLE_ID deleted successfully."
else
    echo -e "${YELLOW}WARN: Could not delete role $ROLE_ID automatically.${NC}"
    echo "Deletion response: $delete_response"
fi

exit 0
