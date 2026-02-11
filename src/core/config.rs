use anyhow::{Context, Result};
use secrecy::Secret;
use std::env;
use std::path::PathBuf;

// Allow dead code - config fields defined for completeness even if not all used yet
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub onelogin_client_id: String,
    pub onelogin_client_secret: Secret<String>,
    pub onelogin_region: OneLoginRegion,
    pub onelogin_subdomain: String,
    pub cache_ttl_seconds: u64,
    pub rate_limit_requests_per_second: u32,
    pub enable_metrics: bool,
    pub max_retries: u32,
    pub retry_initial_delay_ms: u64,
    pub retry_max_delay_ms: u64,
    /// Path to tool configuration file (JSON)
    pub tool_config_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OneLoginRegion {
    US,
    EU,
}

impl OneLoginRegion {
    pub fn tenant_base_url(&self, subdomain: &str) -> String {
        match self {
            OneLoginRegion::US => format!("https://{}.onelogin.com", subdomain),
            OneLoginRegion::EU => format!("https://{}.eu.onelogin.com", subdomain),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let client_id = env::var("ONELOGIN_CLIENT_ID")
            .context("ONELOGIN_CLIENT_ID environment variable not set")?;

        let client_secret = env::var("ONELOGIN_CLIENT_SECRET")
            .context("ONELOGIN_CLIENT_SECRET environment variable not set")?;

        let region_str = env::var("ONELOGIN_REGION").unwrap_or_else(|_| "us".to_string());
        let region = match region_str.to_lowercase().as_str() {
            "us" => OneLoginRegion::US,
            "eu" => OneLoginRegion::EU,
            _ => anyhow::bail!("Invalid ONELOGIN_REGION. Must be 'us' or 'eu'"),
        };

        let subdomain = env::var("ONELOGIN_SUBDOMAIN")
            .context("ONELOGIN_SUBDOMAIN environment variable not set")?;

        let cache_ttl_seconds = env::var("CACHE_TTL_SECONDS")
            .unwrap_or_else(|_| "300".to_string())
            .parse()
            .context("Invalid CACHE_TTL_SECONDS")?;

        let rate_limit_requests_per_second = env::var("RATE_LIMIT_RPS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .context("Invalid RATE_LIMIT_RPS")?;

        let enable_metrics = env::var("ENABLE_METRICS")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        let max_retries = env::var("MAX_RETRIES")
            .unwrap_or_else(|_| "3".to_string())
            .parse()
            .context("Invalid MAX_RETRIES")?;

        let retry_initial_delay_ms = env::var("RETRY_INITIAL_DELAY_MS")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .context("Invalid RETRY_INITIAL_DELAY_MS")?;

        let retry_max_delay_ms = env::var("RETRY_MAX_DELAY_MS")
            .unwrap_or_else(|_| "10000".to_string())
            .parse()
            .context("Invalid RETRY_MAX_DELAY_MS")?;

        // Tool config path: check env var first, then default to ~/.config/onelogin-mcp/config.json
        let tool_config_path = env::var("ONELOGIN_MCP_CONFIG")
            .map(PathBuf::from)
            .ok()
            .or_else(|| dirs::config_dir().map(|d| d.join("onelogin-mcp").join("config.json")));

        Ok(Config {
            onelogin_client_id: client_id,
            onelogin_client_secret: Secret::new(client_secret),
            onelogin_region: region,
            onelogin_subdomain: subdomain,
            cache_ttl_seconds,
            rate_limit_requests_per_second,
            enable_metrics,
            max_retries,
            retry_initial_delay_ms,
            retry_max_delay_ms,
            tool_config_path,
        })
    }

    pub fn tenant_base_url(&self) -> String {
        self.onelogin_region
            .tenant_base_url(&self.onelogin_subdomain)
    }

    pub fn token_url(&self) -> String {
        format!("{}/auth/oauth2/v2/token", self.tenant_base_url())
    }

    pub fn api_url(&self, path: &str) -> String {
        let trimmed = path.trim_start_matches('/');
        let base = self.tenant_base_url();
        let needs_absolute = trimmed.starts_with("api/")
            || trimmed.starts_with("auth/")
            || trimmed.starts_with("scim/")
            || trimmed.starts_with("oidc/")
            || trimmed.starts_with(".well-known/");

        if needs_absolute {
            format!("{}/{}", base, trimmed)
        } else {
            format!("{}/api/2/{}", base, trimmed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_base_url() {
        assert_eq!(
            OneLoginRegion::US.tenant_base_url("mycompany"),
            "https://mycompany.onelogin.com"
        );
        assert_eq!(
            OneLoginRegion::EU.tenant_base_url("mycompany"),
            "https://mycompany.eu.onelogin.com"
        );
    }

    #[test]
    fn test_api_url_prefixing() {
        let config = Config {
            onelogin_client_id: "id".to_string(),
            onelogin_client_secret: Secret::new("secret".to_string()),
            onelogin_region: OneLoginRegion::US,
            onelogin_subdomain: "tenant".to_string(),
            cache_ttl_seconds: 300,
            rate_limit_requests_per_second: 10,
            enable_metrics: false,
            max_retries: 3,
            retry_initial_delay_ms: 100,
            retry_max_delay_ms: 10000,
            tool_config_path: None,
        };

        assert_eq!(
            config.api_url("/users"),
            "https://tenant.onelogin.com/api/2/users"
        );
        assert_eq!(
            config.api_url("/scim/v2/Users"),
            "https://tenant.onelogin.com/scim/v2/Users"
        );
        assert_eq!(
            config.api_url("/auth/oauth2/v2/token"),
            "https://tenant.onelogin.com/auth/oauth2/v2/token"
        );
    }
}
