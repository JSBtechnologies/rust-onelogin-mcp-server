#!/usr/bin/env python3
"""
OneLogin MCP Server Setup Script

Interactive setup wizard for configuring the OneLogin MCP Server.
Creates the .env file with your OneLogin API credentials.

Usage:
    python scripts/setup.py

    Or make executable:
    chmod +x scripts/setup.py
    ./scripts/setup.py
"""

import getpass
import os
import sys
from pathlib import Path


# ANSI color codes for terminal output
class Colors:
    HEADER = '\033[95m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
    END = '\033[0m'

def print_header():
    """Print the setup wizard header."""
    print(f"""
{Colors.CYAN}{Colors.BOLD}╔══════════════════════════════════════════════════════════════╗
║           OneLogin MCP Server - Setup Wizard                  ║
╚══════════════════════════════════════════════════════════════╝{Colors.END}
""")

def print_step(step_num: int, total: int, description: str):
    """Print a step header."""
    print(f"\n{Colors.BLUE}[Step {step_num}/{total}]{Colors.END} {Colors.BOLD}{description}{Colors.END}")

def print_success(message: str):
    """Print a success message."""
    print(f"{Colors.GREEN}✓ {message}{Colors.END}")

def print_error(message: str):
    """Print an error message."""
    print(f"{Colors.RED}✗ {message}{Colors.END}")

def print_warning(message: str):
    """Print a warning message."""
    print(f"{Colors.YELLOW}! {message}{Colors.END}")

def print_info(message: str):
    """Print an info message."""
    print(f"{Colors.CYAN}ℹ {message}{Colors.END}")

def get_project_root() -> Path:
    """Get the project root directory."""
    script_dir = Path(__file__).parent
    return script_dir.parent

def prompt_with_default(prompt: str, default: str = None, required: bool = True) -> str:
    """Prompt for input with an optional default value."""
    if default:
        display_prompt = f"{prompt} [{default}]: "
    else:
        display_prompt = f"{prompt}: "

    while True:
        value = input(display_prompt).strip()

        if not value and default:
            return default
        elif not value and required:
            print_error("This field is required. Please enter a value.")
        else:
            return value

def prompt_secret(prompt: str) -> str:
    """Prompt for a secret value (hidden input)."""
    while True:
        value = getpass.getpass(f"{prompt}: ")
        if not value:
            print_error("This field is required. Please enter a value.")
        else:
            return value

def prompt_choice(prompt: str, choices: list, default: str = None) -> str:
    """Prompt for a choice from a list of options."""
    choices_str = "/".join(choices)
    if default:
        display_prompt = f"{prompt} ({choices_str}) [{default}]: "
    else:
        display_prompt = f"{prompt} ({choices_str}): "

    while True:
        value = input(display_prompt).strip().lower()

        if not value and default:
            return default
        elif value in [c.lower() for c in choices]:
            return value
        else:
            print_error(f"Please enter one of: {choices_str}")

def prompt_yes_no(prompt: str, default: bool = True) -> bool:
    """Prompt for a yes/no answer."""
    default_str = "Y/n" if default else "y/N"
    display_prompt = f"{prompt} [{default_str}]: "

    value = input(display_prompt).strip().lower()

    if not value:
        return default
    return value in ['y', 'yes', 'true', '1']

def test_credentials(client_id: str, client_secret: str, subdomain: str, region: str) -> bool:
    """Test OneLogin API credentials by attempting to get an access token."""
    try:
        import json
        import ssl
        import urllib.parse
        import urllib.request
    except ImportError:
        print_warning("Could not import required modules for credential testing.")
        return None

    api_url = f"https://{subdomain}.onelogin.com/auth/oauth2/v2/token"

    # Prepare request
    data = urllib.parse.urlencode({
        'grant_type': 'client_credentials'
    }).encode('utf-8')

    # Create authorization header
    import base64
    credentials = base64.b64encode(f"{client_id}:{client_secret}".encode()).decode()

    headers = {
        'Authorization': f'Basic {credentials}',
        'Content-Type': 'application/x-www-form-urlencoded'
    }

    try:
        request = urllib.request.Request(api_url, data=data, headers=headers, method='POST')

        # Create SSL context
        context = ssl.create_default_context()

        with urllib.request.urlopen(request, timeout=10, context=context) as response:
            if response.status == 200:
                return True
            else:
                return False
    except urllib.error.HTTPError as e:
        if e.code == 401:
            print_error(f"Authentication failed (401). Check your Client ID and Secret.")
        elif e.code == 404:
            print_error(f"Subdomain not found (404). Check your subdomain: {subdomain}")
        else:
            print_error(f"HTTP Error: {e.code} - {e.reason}")
        return False
    except urllib.error.URLError as e:
        print_error(f"Connection error: {e.reason}")
        print_info("Check your network connection and subdomain.")
        return False
    except Exception as e:
        print_error(f"Error testing credentials: {e}")
        return False

def create_env_file(config: dict, env_path: Path) -> bool:
    """Create the .env file with the provided configuration."""
    env_content = f"""# OneLogin MCP Server Configuration
# Generated by setup.py

# Required: OneLogin API Credentials
# Get these from OneLogin Admin > Developers > API Credentials
ONELOGIN_CLIENT_ID={config['client_id']}
ONELOGIN_CLIENT_SECRET={config['client_secret']}

# Required: OneLogin Region
# us = United States datacenter
# eu = European Union datacenter
ONELOGIN_REGION={config['region']}

# Required: Your OneLogin subdomain
# This is the part before .onelogin.com in your login URL
# Example: If your URL is https://mycompany.onelogin.com, use "mycompany"
ONELOGIN_SUBDOMAIN={config['subdomain']}

# Optional: Performance Settings
# Uncomment and modify as needed

# Rate limiting (requests per second)
# RATE_LIMIT_RPS=10

# Cache TTL in seconds (default 300 = 5 minutes)
# CACHE_TTL_SECONDS=300

# Logging level (error, warn, info, debug, trace)
# RUST_LOG=info
"""

    try:
        with open(env_path, 'w') as f:
            f.write(env_content)
        return True
    except Exception as e:
        print_error(f"Failed to write .env file: {e}")
        return False

def main():
    """Main setup wizard flow."""
    print_header()

    project_root = get_project_root()
    env_path = project_root / '.env'

    # Check if .env already exists
    if env_path.exists():
        print_warning(f"An existing .env file was found at: {env_path}")
        if not prompt_yes_no("Do you want to overwrite it?", default=False):
            print_info("Setup cancelled. Your existing .env file was not modified.")
            sys.exit(0)
        print()

    print(f"""This wizard will help you configure the OneLogin MCP Server.

You'll need the following information from your OneLogin Admin Console:
  • API Client ID
  • API Client Secret
  • Your OneLogin subdomain
  • Your datacenter region (US or EU)

{Colors.CYAN}To get API credentials:{Colors.END}
  1. Log into OneLogin Admin Console
  2. Go to Developers → API Credentials
  3. Click "New Credential"
  4. Select "Manage All" scope for full access
  5. Copy the Client ID and Client Secret
""")

    total_steps = 5
    config = {}

    # Step 1: Client ID
    print_step(1, total_steps, "OneLogin Client ID")
    print("  Enter your API Client ID (looks like: abc123def456...)")
    config['client_id'] = prompt_with_default("  Client ID")

    # Step 2: Client Secret
    print_step(2, total_steps, "OneLogin Client Secret")
    print("  Enter your API Client Secret (input will be hidden)")
    config['client_secret'] = prompt_secret("  Client Secret")

    # Step 3: Region
    print_step(3, total_steps, "OneLogin Region")
    print("  Select your OneLogin datacenter region:")
    print("    • us - United States (api.us.onelogin.com)")
    print("    • eu - European Union (api.eu.onelogin.com)")
    config['region'] = prompt_choice("  Region", ['us', 'eu'], default='us')

    # Step 4: Subdomain
    print_step(4, total_steps, "OneLogin Subdomain")
    print("  Enter your OneLogin subdomain.")
    print("  Example: If your login URL is https://mycompany.onelogin.com")
    print("           then your subdomain is 'mycompany'")
    config['subdomain'] = prompt_with_default("  Subdomain")

    # Step 5: Test credentials
    print_step(5, total_steps, "Verify Configuration")
    print()

    if prompt_yes_no("Would you like to test your credentials?", default=True):
        print_info("Testing connection to OneLogin API...")

        if test_credentials(
            config['client_id'],
            config['client_secret'],
            config['subdomain'],
            config['region']
        ):
            print_success("Credentials verified successfully!")
        else:
            print()
            if not prompt_yes_no("Credentials could not be verified. Continue anyway?", default=False):
                print_info("Setup cancelled. Please check your credentials and try again.")
                sys.exit(1)

    # Create .env file
    print()
    print_info(f"Creating .env file at: {env_path}")

    if create_env_file(config, env_path):
        print_success(".env file created successfully!")
    else:
        print_error("Failed to create .env file.")
        sys.exit(1)

    # Print next steps
    print(f"""
{Colors.GREEN}{Colors.BOLD}╔══════════════════════════════════════════════════════════════╗
║                    Setup Complete!                            ║
╚══════════════════════════════════════════════════════════════╝{Colors.END}

{Colors.BOLD}Next steps:{Colors.END}

  1. Build the server (if not already built):
     {Colors.CYAN}cargo build --release{Colors.END}

  2. Run the server:
     {Colors.CYAN}cargo run --release{Colors.END}

  3. Or configure Claude Desktop - see wiki/Claude-Desktop-Integration.md

{Colors.BOLD}Configuration file:{Colors.END} {env_path}

{Colors.YELLOW}Security Note:{Colors.END} Keep your .env file secure and never commit it to git.
              The .gitignore should already exclude it.

For more information, see the documentation at: wiki/
""")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n")
        print_info("Setup cancelled by user.")
        sys.exit(0)
        sys.exit(0)
