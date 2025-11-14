use anyhow::{Context, Result};
use secrecy::{ExposeSecret, Secret};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub onelogin_client_id: String,
    pub onelogin_client_secret: Secret<String>,
    pub onelogin_region: OneLoginRegion,
    pub onelogin_subdomain: String,
    pub cache_ttl_seconds: u64,
    pub rate_limit_requests_per_second: u32,
    pub enable_metrics: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OneLoginRegion {
    US,
    EU,
}

impl OneLoginRegion {
    pub fn base_url(&self, subdomain: &str) -> String {
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

        Ok(Config {
            onelogin_client_id: client_id,
            onelogin_client_secret: Secret::new(client_secret),
            onelogin_region: region,
            onelogin_subdomain: subdomain,
            cache_ttl_seconds,
            rate_limit_requests_per_second,
            enable_metrics,
        })
    }

    pub fn base_url(&self) -> String {
        self.onelogin_region.base_url(&self.onelogin_subdomain)
    }

    pub fn api_url(&self, path: &str) -> String {
        format!("{}/api/2{}", self.base_url(), path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_base_url() {
        assert_eq!(
            OneLoginRegion::US.base_url("mycompany"),
            "https://mycompany.onelogin.com"
        );
        assert_eq!(
            OneLoginRegion::EU.base_url("mycompany"),
            "https://mycompany.eu.onelogin.com"
        );
    }
}
