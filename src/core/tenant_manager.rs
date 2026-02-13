use crate::api::OneLoginClient;
use crate::core::auth::AuthManager;
use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::config::{Config, TenantEntry};
use crate::core::rate_limit::RateLimiter;
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

/// Describes a tenant's identity (for listing via the onelogin_list_tenants tool).
#[derive(Debug, Clone, Serialize)]
pub struct TenantInfo {
    pub name: String,
    pub subdomain: String,
    pub region: String,
    pub is_default: bool,
}

pub struct TenantManager {
    clients: HashMap<String, Arc<OneLoginClient>>,
    default_tenant: String,
    tenant_info: Vec<TenantInfo>,
}

impl TenantManager {
    /// Build a full client stack for a single Config.
    fn build_client(config: Config) -> Arc<OneLoginClient> {
        let config = Arc::new(config);
        let auth_manager = Arc::new(AuthManager::new(config.clone()));
        let rate_limiter = Arc::new(RateLimiter::new(config.rate_limit_requests_per_second));
        let http_client = Arc::new(HttpClient::new(config.clone(), auth_manager, rate_limiter));
        let cache = Arc::new(CacheManager::new(config.cache_ttl_seconds, 10000));
        Arc::new(OneLoginClient::new(http_client, cache))
    }

    /// Create a single-tenant manager from environment config (backward compatible).
    pub fn from_single(config: Config) -> Self {
        let tenant_name = config.onelogin_subdomain.clone();
        let info = TenantInfo {
            name: tenant_name.clone(),
            subdomain: config.onelogin_subdomain.clone(),
            region: format!("{:?}", config.onelogin_region).to_lowercase(),
            is_default: true,
        };
        let client = Self::build_client(config);
        let mut clients = HashMap::new();
        clients.insert(tenant_name.clone(), client);

        TenantManager {
            clients,
            default_tenant: tenant_name,
            tenant_info: vec![info],
        }
    }

    /// Create a multi-tenant manager from tenant entries, inheriting shared settings from base config.
    pub fn from_entries(entries: &[TenantEntry], base_config: &Config) -> Result<Self> {
        let mut clients = HashMap::new();
        let mut tenant_info = Vec::new();
        let mut default_tenant: Option<String> = None;

        for entry in entries {
            let config = entry.to_config(base_config)?;
            let client = Self::build_client(config);

            let info = TenantInfo {
                name: entry.name.clone(),
                subdomain: entry.subdomain.clone(),
                region: entry.region.clone(),
                is_default: entry.default,
            };

            clients.insert(entry.name.clone(), client);
            tenant_info.push(info);

            if entry.default {
                if default_tenant.is_some() {
                    anyhow::bail!("Multiple tenants marked as default in tenants.json");
                }
                default_tenant = Some(entry.name.clone());
            }
        }

        // If no default specified, use the first tenant
        let default_tenant = default_tenant
            .or_else(|| entries.first().map(|e| e.name.clone()))
            .ok_or_else(|| anyhow!("No tenants configured"))?;

        Ok(TenantManager {
            clients,
            default_tenant,
            tenant_info,
        })
    }

    /// Resolve tenant name to client. None or empty string means default.
    pub fn resolve(&self, tenant: Option<&str>) -> Result<Arc<OneLoginClient>> {
        let name = match tenant {
            Some(t) if !t.is_empty() => t,
            _ => &self.default_tenant,
        };
        self.clients.get(name).cloned().ok_or_else(|| {
            let available: Vec<&str> = self.clients.keys().map(|s| s.as_str()).collect();
            anyhow!(
                "Unknown tenant '{}'. Available tenants: {}",
                name,
                available.join(", ")
            )
        })
    }

    pub fn default_tenant_name(&self) -> &str {
        &self.default_tenant
    }

    pub fn tenant_info(&self) -> &[TenantInfo] {
        &self.tenant_info
    }

    pub fn is_multi_tenant(&self) -> bool {
        self.clients.len() > 1
    }
}
