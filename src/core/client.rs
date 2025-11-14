use crate::core::auth::AuthManager;
use crate::core::config::Config;
use crate::core::error::{OneLoginError, Result};
use crate::core::rate_limit::RateLimiter;
use reqwest::{header, Method, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tracing::{debug, error, instrument};

pub struct HttpClient {
    config: Arc<Config>,
    client: reqwest::Client,
    auth_manager: Arc<AuthManager>,
    rate_limiter: Arc<RateLimiter>,
}

impl HttpClient {
    pub fn new(
        config: Arc<Config>,
        auth_manager: Arc<AuthManager>,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            config,
            client,
            auth_manager,
            rate_limiter,
        }
    }

    #[instrument(skip(self))]
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request(Method::GET, path, None::<&()>).await
    }

    #[instrument(skip(self, body))]
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        self.request(Method::POST, path, body).await
    }

    #[instrument(skip(self, body))]
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        self.request(Method::PUT, path, body).await
    }

    #[instrument(skip(self))]
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request(Method::DELETE, path, None::<&()>).await
    }

    #[instrument(skip(self, body))]
    async fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        // Apply rate limiting
        self.rate_limiter.wait().await;

        // Get access token
        let token = self.auth_manager.get_token().await?;

        // Build URL
        let url = self.config.api_url(path);
        debug!("Making {} request to {}", method, url);

        // Build request
        let mut request = self.client.request(method.clone(), &url).header(
            header::AUTHORIZATION,
            format!("Bearer {}", token),
        );

        // Add body if provided
        if let Some(b) = body {
            request = request.json(b);
        }

        // Execute request
        let response = request.send().await.map_err(|e| {
            error!("HTTP request failed: {}", e);
            OneLoginError::HttpClientError(e)
        })?;

        let status = response.status();
        debug!("Received response with status: {}", status);

        // Handle error responses
        if !status.is_success() {
            return self.handle_error_response(status, response).await;
        }

        // Parse successful response
        response.json::<T>().await.map_err(|e| {
            error!("Failed to parse response: {}", e);
            OneLoginError::InvalidResponse(format!("JSON parsing failed: {}", e))
        })
    }

    async fn handle_error_response<T>(
        &self,
        status: reqwest::StatusCode,
        response: reqwest::Response,
    ) -> Result<T> {
        let body = response.text().await.unwrap_or_default();

        match status.as_u16() {
            401 => {
                error!("Authentication failed");
                self.auth_manager.invalidate_token().await;
                Err(OneLoginError::AuthenticationFailed(body))
            }
            403 => {
                error!("Permission denied");
                Err(OneLoginError::PermissionDenied(body))
            }
            404 => {
                error!("Resource not found");
                Err(OneLoginError::NotFound(body))
            }
            429 => {
                error!("Rate limit exceeded");
                Err(OneLoginError::RateLimitExceeded)
            }
            _ => {
                error!("API request failed with status {}: {}", status, body);
                Err(OneLoginError::ApiRequestFailed(format!(
                    "Status {}: {}",
                    status, body
                )))
            }
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn http_client(&self) -> &reqwest::Client {
        &self.client
    }
}
