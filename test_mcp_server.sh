#!/bin/bash
# OneLogin MCP Server - Automated Test Script
# This script tests basic MCP protocol functionality

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test results tracking
PASSED=0
FAILED=0
SKIPPED=0

# Server process PID
SERVER_PID=""

# Output file for test results
RESULTS_FILE="TEST_RESULTS_$(date +%Y%m%d_%H%M%S).md"

# Function to print test result
print_result() {
    local test_name="$1"
    local status="$2"
    local message="$3"

    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}[PASS]${NC} $test_name"
        ((PASSED++))
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}[FAIL]${NC} $test_name: $message"
        ((FAILED++))
    else
        echo -e "${YELLOW}[SKIP]${NC} $test_name: $message"
        ((SKIPPED++))
    fi
}

# Function to send MCP request
send_mcp_request() {
    local payload="$1"
    local length=${#payload}
    printf "Content-Length: %d\r\n\r\n%s" "$length" "$payload"
}

# Function to run a test
run_test() {
    local test_name="$1"
    local method="$2"
    local params="$3"
    local expected_field="$4"

    echo "Running: $test_name..."

    local payload=$(cat <<EOF
{"jsonrpc":"2.0","id":1,"method":"$method","params":$params}
EOF
)

    local response=$(echo "$payload" | timeout 10 ./target/release/onelogin-mcp-server 2>/dev/null | head -c 10000 || true)

    if [ -z "$response" ]; then
        print_result "$test_name" "FAIL" "No response received"
        return 1
    fi

    # Extract JSON from response (skip Content-Length header)
    local json_response=$(echo "$response" | grep -o '{.*}' | tail -1)

    if [ -z "$json_response" ]; then
        print_result "$test_name" "FAIL" "Could not parse JSON response"
        return 1
    fi

    # Check for error
    if echo "$json_response" | grep -q '"error"'; then
        local error=$(echo "$json_response" | grep -o '"error"[^}]*}' | head -1)
        print_result "$test_name" "FAIL" "Error: $error"
        return 1
    fi

    # Check for expected field if specified
    if [ -n "$expected_field" ]; then
        if echo "$json_response" | grep -q "\"$expected_field\""; then
            print_result "$test_name" "PASS" ""
            return 0
        else
            print_result "$test_name" "FAIL" "Expected field '$expected_field' not found"
            return 1
        fi
    fi

    print_result "$test_name" "PASS" ""
    return 0
}

# Check prerequisites
echo "============================================"
echo "OneLogin MCP Server Test Suite"
echo "============================================"
echo ""

# Check if binary exists
if [ ! -f "./target/release/onelogin-mcp-server" ]; then
    echo -e "${RED}Error: Server binary not found. Please run 'cargo build --release' first.${NC}"
    exit 1
fi

# Check environment variables
if [ -z "$ONELOGIN_CLIENT_ID" ] || [ -z "$ONELOGIN_CLIENT_SECRET" ]; then
    echo -e "${YELLOW}Warning: ONELOGIN_CLIENT_ID or ONELOGIN_CLIENT_SECRET not set.${NC}"
    echo "Some API tests will fail without valid credentials."
    echo ""
fi

echo "Starting tests..."
echo ""

# ==================================
# MCP Protocol Tests
# ==================================
echo "--- MCP Protocol Tests ---"

# Test: Initialize
run_test "MCP Initialize" "initialize" '{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}' "serverInfo"

# Test: Tools List
run_test "MCP Tools List" "tools/list" '{}' "tools"

# ==================================
# User API Tests (Read-only)
# ==================================
echo ""
echo "--- User API Tests ---"

run_test "List Users" "tools/call" '{"name":"onelogin_list_users","arguments":{"limit":5}}' "content"

# ==================================
# Apps API Tests (Read-only)
# ==================================
echo ""
echo "--- Apps API Tests ---"

run_test "List Apps" "tools/call" '{"name":"onelogin_list_apps","arguments":{}}' "content"

# ==================================
# Roles API Tests (Read-only)
# ==================================
echo ""
echo "--- Roles API Tests ---"

run_test "List Roles" "tools/call" '{"name":"onelogin_list_roles","arguments":{}}' "content"

# ==================================
# Groups API Tests (Read-only)
# ==================================
echo ""
echo "--- Groups API Tests ---"

run_test "List Groups" "tools/call" '{"name":"onelogin_list_groups","arguments":{}}' "content"

# ==================================
# Events API Tests (Read-only)
# ==================================
echo ""
echo "--- Events API Tests ---"

run_test "List Events" "tools/call" '{"name":"onelogin_list_events","arguments":{"limit":5}}' "content"

run_test "List Event Types" "tools/call" '{"name":"onelogin_list_event_types","arguments":{}}' "content"

# ==================================
# Smart Hooks API Tests (Read-only)
# ==================================
echo ""
echo "--- Smart Hooks API Tests ---"

run_test "List Smart Hooks" "tools/call" '{"name":"onelogin_list_smart_hooks","arguments":{}}' "content"

# ==================================
# Privileges API Tests (Read-only)
# ==================================
echo ""
echo "--- Privileges API Tests ---"

run_test "List Privileges" "tools/call" '{"name":"onelogin_list_privileges","arguments":{}}' "content"

# ==================================
# Custom Attributes API Tests (Read-only)
# ==================================
echo ""
echo "--- Custom Attributes API Tests ---"

run_test "List Custom Attributes" "tools/call" '{"name":"onelogin_list_custom_attributes","arguments":{}}' "content"

# ==================================
# Branding API Tests (Read-only)
# ==================================
echo ""
echo "--- Branding API Tests ---"

run_test "Get Branding Settings" "tools/call" '{"name":"onelogin_get_branding_settings","arguments":{}}' "content"

# ==================================
# Risk/Vigilance API Tests (Read-only)
# ==================================
echo ""
echo "--- Risk/Vigilance API Tests ---"

run_test "List Risk Rules" "tools/call" '{"name":"onelogin_list_risk_rules","arguments":{}}' "content"

# ==================================
# Directory API Tests (Read-only)
# ==================================
echo ""
echo "--- Directory API Tests ---"

run_test "List Directory Connectors" "tools/call" '{"name":"onelogin_list_directory_connectors","arguments":{}}' "content"

# ==================================
# Sessions API Tests (Read-only)
# ==================================
echo ""
echo "--- Sessions API Tests ---"

run_test "List Sessions" "tools/call" '{"name":"onelogin_list_sessions","arguments":{}}' "content"

# ==================================
# OIDC API Tests (Read-only)
# ==================================
echo ""
echo "--- OIDC API Tests ---"

run_test "OIDC Well-Known Config" "tools/call" '{"name":"onelogin_oidc_get_well_known_config","arguments":{}}' "content"

run_test "OIDC JWKS" "tools/call" '{"name":"onelogin_oidc_get_jwks","arguments":{}}' "content"

# ==================================
# Error Handling Tests
# ==================================
echo ""
echo "--- Error Handling Tests ---"

# Test unknown tool
payload='{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"unknown_tool","arguments":{}}}'
response=$(echo "$payload" | timeout 5 ./target/release/onelogin-mcp-server 2>/dev/null | head -c 5000 || true)
if echo "$response" | grep -q "error"; then
    print_result "Unknown Tool Error" "PASS" ""
else
    print_result "Unknown Tool Error" "FAIL" "Expected error for unknown tool"
fi

# Test invalid method
payload='{"jsonrpc":"2.0","id":1,"method":"invalid/method","params":{}}'
response=$(echo "$payload" | timeout 5 ./target/release/onelogin-mcp-server 2>/dev/null | head -c 5000 || true)
if echo "$response" | grep -q "error"; then
    print_result "Invalid Method Error" "PASS" ""
else
    print_result "Invalid Method Error" "FAIL" "Expected error for invalid method"
fi

# ==================================
# Summary
# ==================================
echo ""
echo "============================================"
echo "Test Results Summary"
echo "============================================"
echo -e "Passed:  ${GREEN}$PASSED${NC}"
echo -e "Failed:  ${RED}$FAILED${NC}"
echo -e "Skipped: ${YELLOW}$SKIPPED${NC}"
echo "Total:   $((PASSED + FAILED + SKIPPED))"
echo ""

# Generate results file
cat > "$RESULTS_FILE" << EOF
# OneLogin MCP Server Test Results

**Date:** $(date)
**Server Version:** $(./target/release/onelogin-mcp-server --version 2>/dev/null || echo "Unknown")

## Summary

| Result | Count |
|--------|-------|
| Passed | $PASSED |
| Failed | $FAILED |
| Skipped | $SKIPPED |
| **Total** | **$((PASSED + FAILED + SKIPPED))** |

## Environment

- ONELOGIN_REGION: ${ONELOGIN_REGION:-not set}
- ONELOGIN_SUBDOMAIN: ${ONELOGIN_SUBDOMAIN:-not set}
- ONELOGIN_CLIENT_ID: ${ONELOGIN_CLIENT_ID:+set (hidden)}${ONELOGIN_CLIENT_ID:-not set}

## Notes

Tests were run using the automated test script. See MCP_SERVER_TESTS.md for manual testing instructions.
EOF

echo "Results saved to: $RESULTS_FILE"

if [ $FAILED -gt 0 ]; then
    exit 1
fi

exit 0
