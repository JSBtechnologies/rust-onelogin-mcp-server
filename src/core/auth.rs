use crate::core::config::Config;
use crate::core::error::{OneLoginError, Result};
use chrono::{DateTime, Duration, Utc};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub token_type: String,
}

#[allow(dead_code)]
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

        let token_url = self.config.token_url();
        debug!("Token URL: {}", token_url);
        debug!("Client ID: {}", self.config.onelogin_client_id);

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
                error!(
                    "AUTHENTICATION REQUEST FAILED\n\
                     \n\
                     Failed to send token request to OneLogin:\n\
                     URL: {}\n\
                     Error: {}\n\
                     \n\
                     This could be due to:\n\
                     - Network connectivity issues\n\
                     - Invalid token URL\n\
                     - DNS resolution failure\n\
                     - Timeout",
                    token_url, e
                );
                OneLoginError::AuthenticationFailed(format!(
                    "Failed to request token from {}\nError: {}\n\
                     \n\
                     Check your network connection and ensure the OneLogin region and subdomain are correct.",
                    token_url, e
                ))
            })?;

        let status = response.status();
        debug!("Token request response status: {}", status);

        if !status.is_success() {
            let headers = response.headers().clone();
            let content_type = headers.get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");
            let body = response.text().await.unwrap_or_else(|e| {
                error!("Failed to read error response body: {}", e);
                format!("<failed to read body: {}>", e)
            });

            // Try to pretty-print JSON error responses
            let formatted_body = if content_type.contains("json") {
                match serde_json::from_str::<serde_json::Value>(&body) {
                    Ok(json) => serde_json::to_string_pretty(&json).unwrap_or(body.clone()),
                    Err(_) => body.clone()
                }
            } else {
                body.clone()
            };

            error!(
                "AUTHENTICATION FAILED\n\
                 \n\
                 Token request failed:\n\
                 URL: {}\n\
                 Status: {}\n\
                 Content-Type: {}\n\
                 Response Body:\n{}\n\
                 \n\
                 Common causes:\n\
                 - Invalid client ID or client secret\n\
                 - Incorrect OneLogin region (us vs eu)\n\
                 - Incorrect subdomain\n\
                 - API credentials not activated in OneLogin admin panel\n\
                 - IP restrictions on the API credentials\n\
                 \n\
                 Please verify your OneLogin API credentials and configuration.",
                token_url, status, content_type, formatted_body
            );

            return Err(OneLoginError::AuthenticationFailed(format!(
                "Token request failed\n\
                 URL: {}\n\
                 Status: {}\n\
                 Response:\n{}\n\
                 \n\
                 Please verify your ONELOGIN_CLIENT_ID, ONELOGIN_CLIENT_SECRET, \
                 ONELOGIN_REGION, and ONELOGIN_SUBDOMAIN environment variables.",
                token_url, status, formatted_body
            )));
        }

        let body_bytes = response.bytes().await.map_err(|e| {
            error!("Failed to read token response body: {}", e);
            OneLoginError::AuthenticationFailed(format!(
                "Failed to read token response body: {}",
                e
            ))
        })?;

        let body_str = String::from_utf8_lossy(&body_bytes);
        debug!("Token response body: {}", body_str);

        let token_response: TokenResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
            error!(
                "FAILED TO PARSE TOKEN RESPONSE\n\
                 \n\
                 Received a successful response but failed to parse it:\n\
                 Parse Error: {}\n\
                 Response Body:\n{}\n\
                 \n\
                 This is unexpected and may indicate an API change or issue.",
                e, body_str
            );
            OneLoginError::AuthenticationFailed(format!(
                "Failed to parse token response: {}\nResponse: {}",
                e, body_str
            ))
        })?;

        let access_token = AccessToken {
            token: token_response.access_token.clone(),
            expires_at: Utc::now() + Duration::seconds(token_response.expires_in),
            token_type: token_response.token_type.clone(),
        };

        debug!(
            "Token obtained successfully, expires in {} seconds (at {})",
            token_response.expires_in,
            access_token.expires_at
        );

        // Update cached token
        {
            let mut token_guard = self.token.write().await;
            *token_guard = Some(access_token);
        }

        info!("Successfully obtained new access token (expires in {} seconds)", token_response.expires_in);
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
