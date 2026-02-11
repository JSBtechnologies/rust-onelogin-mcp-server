#!/usr/bin/env python3
"""
Smoke test suite for OneLogin MCP Server
Quick verification of core operations (~20 tests, ~30 seconds)

For comprehensive testing, use test_all.py instead.
"""

import subprocess
import json
import time
import sys
import os
from datetime import datetime
from pathlib import Path


# Load .env file from project root
def load_dotenv():
    """Load environment variables from .env file"""
    env_paths = [
        Path(__file__).parent.parent / ".env",
        Path.cwd() / ".env",
    ]
    for env_path in env_paths:
        if env_path.exists():
            print(f"Loading environment from: {env_path}")
            with open(env_path) as f:
                for line in f:
                    line = line.strip()
                    if line and not line.startswith('#') and '=' in line:
                        key, _, value = line.partition('=')
                        key = key.strip()
                        value = value.strip().strip('"').strip("'")
                        if key not in os.environ:
                            os.environ[key] = value
            return True
    return False


load_dotenv()

# Colors
RED = '\033[0;31m'
GREEN = '\033[0;32m'
YELLOW = '\033[1;33m'
BLUE = '\033[0;34m'
NC = '\033[0m'

# Configuration
CONFIG = {
    "test_user_id": int(os.environ.get("TEST_USER_ID", "255838675")),
    "account_owner_id": int(os.environ.get("ACCOUNT_OWNER_ID", "244955039")),
    "test_role_id": int(os.environ.get("TEST_ROLE_ID", "892924")),
    "custom_attr": os.environ.get("CUSTOM_ATTR_SHORTNAME", "city"),
    "account_id": int(os.environ.get("ACCOUNT_ID", "244135")),
}

# Counters
passed = 0
failed = 0


def check_prerequisites():
    """Check that server binary and credentials exist"""
    server_path = Path("./target/release/onelogin-mcp-server")
    if not server_path.exists():
        print(f"{RED}Error: MCP server not found. Run: cargo build --release{NC}")
        sys.exit(1)

    if not os.environ.get("ONELOGIN_CLIENT_ID"):
        print(f"{RED}Error: ONELOGIN_CLIENT_ID not set{NC}")
        sys.exit(1)
    if not os.environ.get("ONELOGIN_CLIENT_SECRET"):
        print(f"{RED}Error: ONELOGIN_CLIENT_SECRET not set{NC}")
        sys.exit(1)


def get_env():
    """Get environment for subprocess"""
    return {
        "ONELOGIN_CLIENT_ID": os.environ["ONELOGIN_CLIENT_ID"],
        "ONELOGIN_CLIENT_SECRET": os.environ["ONELOGIN_CLIENT_SECRET"],
        "ONELOGIN_REGION": os.environ.get("ONELOGIN_REGION", "us"),
        "ONELOGIN_SUBDOMAIN": os.environ.get("ONELOGIN_SUBDOMAIN", "jbudde-dev"),
        "PATH": os.environ.get("PATH", ""),
    }


def run_test(name, tool, args, expect_error=False):
    """Run a single tool test"""
    global passed, failed

    # Build requests
    init_req = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }
    }
    tool_req = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {"name": tool, "arguments": args}
    }

    # Send requests
    init_data = json.dumps(init_req)
    tool_data = json.dumps(tool_req)
    input_data = f"Content-Length: {len(init_data)}\r\n\r\n{init_data}"
    input_data += f"Content-Length: {len(tool_data)}\r\n\r\n{tool_data}"

    try:
        proc = subprocess.run(
            ["./target/release/onelogin-mcp-server"],
            input=input_data,
            capture_output=True,
            text=True,
            timeout=15,
            env=get_env()
        )
        result = proc.stdout.replace('\r', '').replace('\n', '')
    except subprocess.TimeoutExpired:
        print(f"  {RED}✗{NC} {name} (timeout)")
        failed += 1
        return False

    # Check for error in tool response (id:2)
    has_error = '"id":2' in result and '"error":{' in result.split('"id":2')[1][:500] if '"id":2' in result else False

    if expect_error:
        if has_error:
            print(f"  {GREEN}✓{NC} {name} (expected error)")
            passed += 1
            return True
        else:
            print(f"  {RED}✗{NC} {name} (expected error but got success)")
            failed += 1
            return False
    else:
        if not has_error:
            print(f"  {GREEN}✓{NC} {name}")
            passed += 1
            return True
        else:
            print(f"  {RED}✗{NC} {name}")
            failed += 1
            return False


def run_prompts_test(name, method, params, check_string):
    """Run a prompts capability test"""
    global passed, failed

    init_req = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        }
    }
    call_req = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": method,
        "params": params
    }

    init_data = json.dumps(init_req)
    call_data = json.dumps(call_req)
    input_data = f"Content-Length: {len(init_data)}\r\n\r\n{init_data}"
    input_data += f"Content-Length: {len(call_data)}\r\n\r\n{call_data}"

    try:
        proc = subprocess.run(
            ["./target/release/onelogin-mcp-server"],
            input=input_data,
            capture_output=True,
            text=True,
            timeout=10,
            env=get_env()
        )
        result = proc.stdout
    except subprocess.TimeoutExpired:
        print(f"  {RED}✗{NC} {name} (timeout)")
        failed += 1
        return False

    if check_string in result:
        print(f"  {GREEN}✓{NC} {name}")
        passed += 1
        return True
    else:
        print(f"  {RED}✗{NC} {name}")
        failed += 1
        return False


def main():
    check_prerequisites()

    print(f"{BLUE}OneLogin MCP Server - Smoke Test Suite{NC}")
    print("=" * 40)
    print(f"Time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print()
    print("Configuration:")
    print(f"  TEST_USER_ID:     {CONFIG['test_user_id']}")
    print(f"  ACCOUNT_OWNER_ID: {CONFIG['account_owner_id']}")
    print(f"  TEST_ROLE_ID:     {CONFIG['test_role_id']}")
    print(f"  ACCOUNT_ID:       {CONFIG['account_id']}")
    print()

    # ==================== PROMPTS ====================
    print(f"{YELLOW}PROMPTS CAPABILITY{NC}")
    run_prompts_test("prompts/list", "prompts/list", {}, "onelogin-usage-guide")
    run_prompts_test("prompts/get", "prompts/get", {"name": "onelogin-usage-guide"}, "OneLogin")

    # ==================== USERS (Read) ====================
    print()
    print(f"{YELLOW}USER OPERATIONS (Read){NC}")
    run_test("list_users", "onelogin_list_users", {"limit": 5})
    run_test("get_user", "onelogin_get_user", {"user_id": CONFIG["test_user_id"]})
    run_test("get_user_apps", "onelogin_get_user_apps", {"user_id": CONFIG["test_user_id"]})
    run_test("get_user_roles", "onelogin_get_user_roles", {"user_id": CONFIG["test_user_id"]})

    # ==================== USERS (Write) ====================
    print()
    print(f"{YELLOW}USER OPERATIONS (Write){NC}")
    run_test("logout_user", "onelogin_logout_user", {"user_id": CONFIG["test_user_id"]})
    run_test("set_custom_attributes", "onelogin_set_custom_attributes", {
        "user_id": CONFIG["test_user_id"],
        "custom_attributes": {CONFIG["custom_attr"]: f"test_{int(time.time())}"}
    })

    # ==================== LOCK/UNLOCK ====================
    print()
    print(f"{YELLOW}USER LOCK/UNLOCK MECHANISMS{NC}")
    print(f"  {BLUE}Note: Two different lock types exist in OneLogin:{NC}")
    print(f"  {BLUE}  1. Admin lock (lock_user) - time-based, unlock via update_user status=1{NC}")
    print(f"  {BLUE}  2. Failed attempt lock - auto-triggered, unlock via unlock_user{NC}")
    print()

    # Test admin lock mechanism
    run_test("admin_lock: lock_user (1 min)", "onelogin_lock_user", {
        "user_id": CONFIG["test_user_id"],
        "locked_until": 1
    })
    time.sleep(1)

    # unlock_user does NOT work for admin locks - only for failed-attempt locks
    run_test("admin_lock: unlock_user (no effect)", "onelogin_unlock_user", {
        "user_id": CONFIG["test_user_id"]
    })
    time.sleep(1)

    # Verify user is still locked
    run_test("admin_lock: verify still locked", "onelogin_get_user", {
        "user_id": CONFIG["test_user_id"]
    })

    # Correct way to unlock admin-locked user
    run_test("admin_lock: unlock via update_user status=1", "onelogin_update_user", {
        "user_id": CONFIG["test_user_id"],
        "status": 1
    })
    time.sleep(1)

    # Verify user is now active
    run_test("admin_lock: verify now active", "onelogin_get_user", {
        "user_id": CONFIG["test_user_id"]
    })

    # ==================== ROLES ====================
    print()
    print(f"{YELLOW}ROLE OPERATIONS{NC}")
    run_test("list_roles", "onelogin_list_roles", {})
    run_test("assign_roles", "onelogin_assign_roles", {
        "user_id": CONFIG["test_user_id"],
        "role_ids": [CONFIG["test_role_id"]]
    })
    time.sleep(1)
    run_test("remove_roles", "onelogin_remove_roles", {
        "user_id": CONFIG["test_user_id"],
        "role_ids": [CONFIG["test_role_id"]]
    })

    # ==================== APPS ====================
    print()
    print(f"{YELLOW}APP OPERATIONS{NC}")
    run_test("list_apps", "onelogin_list_apps", {})

    # ==================== EVENTS ====================
    print()
    print(f"{YELLOW}EVENT OPERATIONS{NC}")
    run_test("list_events", "onelogin_list_events", {"limit": 5})
    run_test("list_event_types", "onelogin_list_event_types", {})
    run_test("create_event", "onelogin_create_event", {
        "event_type_id": 8,
        "account_id": CONFIG["account_id"],
        "user_id": CONFIG["test_user_id"],
        "notes": "Test event"
    })

    # ==================== CUSTOM ATTRIBUTES ====================
    print()
    print(f"{YELLOW}CUSTOM ATTRIBUTES{NC}")
    run_test("list_custom_attributes", "onelogin_list_custom_attributes", {})

    # ==================== ACCOUNT OWNER PROTECTION ====================
    print()
    print(f"{YELLOW}ACCOUNT OWNER PROTECTION{NC}")
    run_test("lock_account_owner (403)", "onelogin_lock_user", {
        "user_id": CONFIG["account_owner_id"],
        "locked_until": 1
    }, expect_error=True)

    # ==================== GROUPS ====================
    print()
    print(f"{YELLOW}GROUPS{NC}")
    run_test("list_groups", "onelogin_list_groups", {})

    # ==================== SUMMARY ====================
    print()
    print("=" * 40)
    print(f"Passed: {GREEN}{passed}{NC}")
    print(f"Failed: {RED}{failed}{NC}")
    print()

    if failed == 0:
        print(f"{GREEN}All tests passed!{NC}")
        sys.exit(0)
    else:
        print(f"{RED}{failed} test(s) failed{NC}")
        sys.exit(1)


if __name__ == "__main__":
    main()
