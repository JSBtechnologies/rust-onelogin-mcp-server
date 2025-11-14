use crate::core::config::Config;
use crate::core::error::{OneLoginError, Result};
use chrono::{DateTime, Duration, Utc};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub token_type: String,
}

impl AccessToken {
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn needs_refresh(&self) -> bool {
        // Refresh 5 minutes before expiration
        Utc::now() >= self.expires_at - Duration::minutes(5)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
}

pub struct AuthManager {
    config: Arc<Config>,
    client: reqwest::Client,
    token: Arc<RwLock<Option<AccessToken>>>,
}

impl AuthManager {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config: config.clone(),
            client: reqwest::Client::new(),
            token: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_token(&self) -> Result<String> {
        // Check if we have a valid token
        {
            let token_guard = self.token.read().await;
            if let Some(ref token) = *token_guard {
                if !token.needs_refresh() {
                    debug!("Using cached access token");
                    return Ok(token.token.clone());
                }
                warn!("Access token needs refresh");
            }
        }

        // Acquire new token
        self.refresh_token().await
    }

    async fn refresh_token(&self) -> Result<String> {
        info!("Requesting new access token from OneLogin");

        let token_url = self.config.api_url("/auth/oauth2/v2/token");

        let response = self
            .client
            .post(&token_url)
            .json(&serde_json::json!({
                "grant_type": "client_credentials"
            }))
            .basic_auth(
                &self.config.onelogin_client_id,
                Some(self.config.onelogin_client_secret.expose_secret()),
            )
            .send()
            .await
            .map_err(|e| {
                OneLoginError::AuthenticationFailed(format!("Failed to request token: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(OneLoginError::AuthenticationFailed(format!(
                "Token request failed with status {}: {}",
                status, body
            )));
        }

        let token_response: TokenResponse = response.json().await.map_err(|e| {
            OneLoginError::AuthenticationFailed(format!("Failed to parse token response: {}", e))
        })?;

        let access_token = AccessToken {
            token: token_response.access_token.clone(),
            expires_at: Utc::now() + Duration::seconds(token_response.expires_in),
            token_type: token_response.token_type,
        };

        // Update cached token
        {
            let mut token_guard = self.token.write().await;
            *token_guard = Some(access_token);
        }

        info!("Successfully obtained new access token");
        Ok(token_response.access_token)
    }

    pub async fn invalidate_token(&self) {
        let mut token_guard = self.token.write().await;
        *token_guard = None;
        info!("Access token invalidated");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_expiration() {
        let token = AccessToken {
            token: "test_token".to_string(),
            expires_at: Utc::now() - Duration::seconds(10),
            token_type: "Bearer".to_string(),
        };
        assert!(token.is_expired());
        assert!(token.needs_refresh());
    }

    #[test]
    fn test_token_needs_refresh() {
        let token = AccessToken {
            token: "test_token".to_string(),
            expires_at: Utc::now() + Duration::minutes(3),
            token_type: "Bearer".to_string(),
        };
        assert!(!token.is_expired());
        assert!(token.needs_refresh());
    }
}
