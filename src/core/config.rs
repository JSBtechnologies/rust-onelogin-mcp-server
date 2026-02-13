use anyhow::{Context, Result};
use secrecy::Secret;
use serde::Deserialize;
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

    /// Load only shared operational settings from env vars, using placeholder credentials.
    /// Used when tenants.json provides all tenant-specific credentials.
    pub fn from_env_base() -> Result<Self> {
        dotenv::dotenv().ok();

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

        let tool_config_path = env::var("ONELOGIN_MCP_CONFIG")
            .map(PathBuf::from)
            .ok()
            .or_else(|| dirs::config_dir().map(|d| d.join("onelogin-mcp").join("config.json")));

        Ok(Config {
            onelogin_client_id: String::new(),
            onelogin_client_secret: Secret::new(String::new()),
            onelogin_region: OneLoginRegion::US,
            onelogin_subdomain: String::new(),
            cache_ttl_seconds,
            rate_limit_requests_per_second,
            enable_metrics,
            max_retries,
            retry_initial_delay_ms,
            retry_max_delay_ms,
            tool_config_path,
        })
    }

    /// Load multi-tenant configuration from file.
    /// Checks ONELOGIN_TENANTS_CONFIG env var first, then default platform path.
    pub fn load_tenants_file() -> Result<Option<TenantsConfigFile>> {
        let path = env::var("ONELOGIN_TENANTS_CONFIG")
            .map(PathBuf::from)
            .ok()
            .or_else(|| dirs::config_dir().map(|d| d.join("onelogin-mcp").join("tenants.json")));

        match path {
            Some(p) if p.exists() => {
                let content = std::fs::read_to_string(&p)
                    .with_context(|| format!("Failed to read tenants config: {}", p.display()))?;
                let config: TenantsConfigFile = serde_json::from_str(&content)
                    .with_context(|| format!("Failed to parse tenants config: {}", p.display()))?;
                Ok(Some(config))
            }
            _ => Ok(None),
        }
    }
}

/// A single tenant's connection credentials for multi-tenant mode.
#[derive(Debug, Clone, Deserialize)]
pub struct TenantEntry {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub region: String,
    pub subdomain: String,
    #[serde(default)]
    pub default: bool,
}

impl TenantEntry {
    /// Convert this entry into a full Config, inheriting shared operational settings from the base.
    pub fn to_config(&self, base: &Config) -> Result<Config> {
        let region = match self.region.to_lowercase().as_str() {
            "us" => OneLoginRegion::US,
            "eu" => OneLoginRegion::EU,
            _ => anyhow::bail!("Invalid region '{}' for tenant '{}'", self.region, self.name),
        };
        Ok(Config {
            onelogin_client_id: self.client_id.clone(),
            onelogin_client_secret: Secret::new(self.client_secret.clone()),
            onelogin_region: region,
            onelogin_subdomain: self.subdomain.clone(),
            cache_ttl_seconds: base.cache_ttl_seconds,
            rate_limit_requests_per_second: base.rate_limit_requests_per_second,
            enable_metrics: base.enable_metrics,
            max_retries: base.max_retries,
            retry_initial_delay_ms: base.retry_initial_delay_ms,
            retry_max_delay_ms: base.retry_max_delay_ms,
            tool_config_path: base.tool_config_path.clone(),
        })
    }
}

/// Multi-tenant configuration file structure.
#[derive(Debug, Clone, Deserialize)]
pub struct TenantsConfigFile {
    pub tenants: Vec<TenantEntry>,
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
