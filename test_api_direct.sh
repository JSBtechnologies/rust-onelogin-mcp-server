#!/bin/bash

# OneLogin API Direct Testing Script
# This bypasses the MCP server to test APIs directly

REGION="us"
SUBDOMAIN="jbudde-dev"
CLIENT_ID="a7477ffafa919e42888811d7943e35f4e4c37e26386966d3547fdd545dba27ab"
CLIENT_SECRET="3d54694d52fcff0ccad43d18b53fdcc835bc50d7b62f05ea397e067e6c0593cd"

# Get access token
echo "=== Getting Access Token ==="
TOKEN_RESPONSE=$(curl -s -X POST "https://${SUBDOMAIN}.onelogin.com/auth/oauth2/v2/token" \
  -H "Content-Type: application/json" \
  -d '{
    "grant_type": "client_credentials"
  }' \
  -u "${CLIENT_ID}:${CLIENT_SECRET}")

ACCESS_TOKEN=$(echo $TOKEN_RESPONSE | jq -r '.access_token')
echo "Access Token: ${ACCESS_TOKEN:0:50}..."
echo ""

# Test various endpoints
echo "=== TEST 1: List Users (WORKING) ==="
curl -s -X GET "https://${SUBDOMAIN}.onelogin.com/api/2/users?limit=1" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.data[0].id, .data[0].email' || echo "FAILED"
echo ""

echo "=== TEST 2: List Groups (FAILING?) ==="
curl -s -X GET "https://${SUBDOMAIN}.onelogin.com/api/1/groups" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.' || echo "FAILED"
echo ""

echo "=== TEST 3: List Privileges (403 expected) ==="
curl -s -X GET "https://${SUBDOMAIN}.onelogin.com/api/1/privileges" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.' || echo "FAILED"
echo ""

echo "=== TEST 4: List Custom Attributes ==="
curl -s -X GET "https://${SUBDOMAIN}.onelogin.com/api/2/users/custom_attributes" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.' || echo "FAILED"
echo ""

echo "=== TEST 5: List Events ==="
curl -s -X GET "https://${SUBDOMAIN}.onelogin.com/api/1/events?limit=1" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.' || echo "FAILED"
echo ""

echo "=== TEST 6: Create User (jeffs test) ==="
CREATE_USER_RESPONSE=$(curl -s -X POST "https://${SUBDOMAIN}.onelogin.com/api/2/users" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "jeffstest@example.com",
    "firstname": "Jeffs",
    "lastname": "Test",
    "username": "jeffstest"
  }')
echo "$CREATE_USER_RESPONSE" | jq '.'
USER_ID=$(echo "$CREATE_USER_RESPONSE" | jq -r '.id // .data.id // empty')
echo "Created User ID: $USER_ID"
echo ""

if [ ! -z "$USER_ID" ]; then
  echo "=== TEST 8: Update User (jeffs test updated) ==="
  curl -s -X PUT "https://${SUBDOMAIN}.onelogin.com/api/2/users/${USER_ID}" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}" \
    -H "Content-Type: application/json" \
    -d '{
      "firstname": "Jeffs",
      "lastname": "Test Updated"
    }' | jq '.'
  echo ""

  echo "=== TEST 9: Get User Apps ==="
  curl -s -X GET "https://${SUBDOMAIN}.onelogin.com/api/2/users/${USER_ID}/apps" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.'
  echo ""

  echo "=== TEST 10: Delete User ==="
  curl -s -X DELETE "https://${SUBDOMAIN}.onelogin.com/api/2/users/${USER_ID}" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.'
  echo ""
fi

echo "=== TEST 11: List User Mappings ==="
curl -s -X GET "https://${SUBDOMAIN}.onelogin.com/api/2/mappings" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.'
echo ""

echo "=== TEST 12: List Smart Hooks ==="
curl -s -X GET "https://${SUBDOMAIN}.onelogin.com/api/2/hooks" \
  -H "Authorization: Bearer ${ACCESS_TOKEN}" | jq '.'
echo ""
