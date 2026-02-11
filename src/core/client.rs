use crate::core::auth::AuthManager;
use crate::core::config::Config;
use crate::core::error::{OneLoginError, Result};
use crate::core::rate_limit::RateLimiter;
use reqwest::{header, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tracing::{debug, error, warn, instrument};

#[allow(dead_code)]
pub struct HttpClient {
    config: Arc<Config>,
    client: reqwest::Client,
    auth_manager: Arc<AuthManager>,
    rate_limiter: Arc<RateLimiter>,
}

#[allow(dead_code)]
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
    pub async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        self.request(Method::PATCH, path, body).await
    }

    #[instrument(skip(self, body))]
    async fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let max_retries = self.config.max_retries;
        let mut attempt = 0;

        loop {
            // Apply rate limiting
            self.rate_limiter.wait().await;

            // Get access token
            let token = self.auth_manager.get_token().await?;

            // Build URL
            let url = self.config.api_url(path);
            if attempt == 0 {
                debug!("Making {} request to {}", method, url);
            } else {
                debug!("Retry attempt {} for {} request to {}", attempt, method, url);
            }

            // Build request
            let mut request = self
                .client
                .request(method.clone(), &url)
                .header(header::AUTHORIZATION, format!("Bearer {}", token));

            // Add body if provided
            let request_body_debug = if let Some(b) = body {
                let body_json = serde_json::to_string(b).unwrap_or_else(|_| "<serialization error>".to_string());
                debug!("Request body: {}", body_json);
                request = request.json(b);
                Some(body_json)
            } else {
                None
            };

            // Execute request
            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => {
                    let error = OneLoginError::HttpClientError(e);
                    if attempt < max_retries && error.is_retriable() {
                        attempt += 1;
                        error!(
                            "HTTP request failed (attempt {}/{}): {} {} - Error: {} - Will retry after backoff",
                            attempt, max_retries, method, url, error
                        );
                        self.exponential_backoff(attempt).await;
                        continue;
                    }
                    error!(
                        "HTTP request failed permanently: {} {} - Error: {} - Request body: {:?}",
                        method, url, error, request_body_debug
                    );
                    return Err(error);
                }
            };

            let status = response.status();
            debug!("Received response with status: {} for {} {}", status, method, url);

            if !status.is_success() {
                let result = self.handle_error_response(status, response, &method, &url).await;
                if let Err(ref e) = result {
                    if attempt < max_retries && e.is_retriable() {
                        attempt += 1;
                        warn!(
                            "Non-success response (attempt {}/{}): {} {} - Status: {} - Error: {} - Will retry",
                            attempt, max_retries, method, url, status, e
                        );
                        self.exponential_backoff(attempt).await;
                        continue;
                    }
                }
                return result;
            }

            return self.parse_success_response(response, &method, &url).await;
        }
    }

    async fn exponential_backoff(&self, attempt: u32) {
        let delay_ms = std::cmp::min(
            self.config.retry_initial_delay_ms * 2u64.pow(attempt - 1),
            self.config.retry_max_delay_ms,
        );
        debug!("Waiting {}ms before retry", delay_ms);
        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
    }

    async fn parse_success_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
        method: &Method,
        url: &str,
    ) -> Result<T> {
        let status = response.status();
        let headers = response.headers().clone();

        let bytes = response.bytes().await.map_err(|e| {
            error!(
                "Failed to read response body for {} {} - Status: {} - Error: {}",
                method, url, status, e
            );
            OneLoginError::InvalidResponse(format!(
                "Failed to read response body for {} {} (status {}): {}",
                method, url, status, e
            ))
        })?;

        // Handle various "empty" or "success indicator" responses
        let body = if bytes.is_empty() {
            debug!("Empty response body for {} {}, treating as null", method, url);
            b"null".to_vec()
        } else {
            // Check for plain text success indicators (e.g., "Accepted" for 202 responses)
            let text = String::from_utf8_lossy(&bytes);
            let trimmed = text.trim();

            // If it's a simple success indicator text (not JSON), treat as null
            // This handles APIs that return "Accepted", "OK", "Success", etc.
            if matches!(trimmed, "Accepted" | "OK" | "Success" | "Created" | "Deleted" | "Updated") {
                debug!(
                    "Plain text success indicator '{}' for {} {} (status {}), treating as null",
                    trimmed, method, url, status
                );
                b"null".to_vec()
            } else {
                bytes.to_vec()
            }
        };

        // Try to parse the response
        let body_str = String::from_utf8_lossy(&body);
        debug!("Response body for {} {} (first 500 chars): {}", method, url, &body_str.chars().take(500).collect::<String>());

        serde_json::from_slice::<T>(&body).map_err(|e| {
            // Get the expected type name if possible
            let type_name = std::any::type_name::<T>();

            // Try to parse as generic JSON to see what we actually got
            let actual_structure = match serde_json::from_slice::<serde_json::Value>(&body) {
                Ok(v) => {
                    match &v {
                        serde_json::Value::Null => "null".to_string(),
                        serde_json::Value::Bool(_) => "boolean".to_string(),
                        serde_json::Value::Number(_) => "number".to_string(),
                        serde_json::Value::String(_) => "string".to_string(),
                        serde_json::Value::Array(arr) => {
                            if arr.is_empty() {
                                "empty array []".to_string()
                            } else {
                                "array [...]".to_string()
                            }
                        },
                        serde_json::Value::Object(obj) => {
                            let keys: Vec<&str> = obj.keys().take(10).map(|s| s.as_str()).collect();
                            if keys.is_empty() {
                                "empty object {}".to_string()
                            } else {
                                format!("object with keys: {}", keys.join(", "))
                            }
                        }
                    }
                },
                Err(_) => "invalid JSON".to_string()
            };

            let content_type = headers.get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown");

            error!(
                "JSON PARSING FAILED for {} {}\n\
                 Status: {}\n\
                 Expected Type: {}\n\
                 Actual Structure: {}\n\
                 Content-Type: {}\n\
                 Parse Error: {}\n\
                 Response Body (first 1000 chars): {}\n\
                 Full Response Body Length: {} bytes",
                method, url, status, type_name, actual_structure, content_type, e,
                &body_str.chars().take(1000).collect::<String>(),
                body.len()
            );

            OneLoginError::InvalidResponse(format!(
                "JSON parsing failed for {} {} (status {})\n\
                 Expected: {}\n\
                 Actual: {}\n\
                 Parse Error: {}\n\
                 Content-Type: {}\n\
                 Response Preview (first 500 chars): {}\n\
                 Full Response Length: {} bytes\n\
                 \n\
                 This usually means the API returned a different structure than expected.\n\
                 Check the response body above to see what was actually returned.",
                method, url, status, type_name, actual_structure, e, content_type,
                &body_str.chars().take(500).collect::<String>(),
                body.len()
            ))
        })
    }

    async fn handle_error_response<T>(
        &self,
        status: StatusCode,
        response: reqwest::Response,
        method: &Method,
        url: &str,
    ) -> Result<T> {
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

        match status.as_u16() {
            401 => {
                error!(
                    "AUTHENTICATION FAILED for {} {}\n\
                     Status: 401 Unauthorized\n\
                     Content-Type: {}\n\
                     Response Body:\n{}\n\
                     \n\
                     The access token may be invalid or expired. Will invalidate and retry with new token.",
                    method, url, content_type, formatted_body
                );
                self.auth_manager.invalidate_token().await;
                Err(OneLoginError::AuthenticationFailed(format!(
                    "Authentication failed for {} {}\nStatus: 401\nResponse: {}",
                    method, url, formatted_body
                )))
            }
            403 => {
                error!(
                    "PERMISSION DENIED for {} {}\n\
                     Status: 403 Forbidden\n\
                     Content-Type: {}\n\
                     Response Body:\n{}\n\
                     \n\
                     The authenticated user does not have permission to access this resource.",
                    method, url, content_type, formatted_body
                );
                Err(OneLoginError::PermissionDenied(format!(
                    "Permission denied for {} {}\nStatus: 403\nResponse: {}",
                    method, url, formatted_body
                )))
            }
            404 => {
                error!(
                    "RESOURCE NOT FOUND for {} {}\n\
                     Status: 404 Not Found\n\
                     Content-Type: {}\n\
                     Response Body:\n{}\n\
                     \n\
                     The requested resource does not exist.",
                    method, url, content_type, formatted_body
                );
                Err(OneLoginError::NotFound(format!(
                    "Resource not found for {} {}\nStatus: 404\nResponse: {}",
                    method, url, formatted_body
                )))
            }
            429 => {
                error!(
                    "RATE LIMIT EXCEEDED for {} {}\n\
                     Status: 429 Too Many Requests\n\
                     Content-Type: {}\n\
                     Response Body:\n{}\n\
                     \n\
                     You have exceeded the API rate limit. Please wait before retrying.",
                    method, url, content_type, formatted_body
                );
                Err(OneLoginError::RateLimitExceeded)
            }
            400 => {
                error!(
                    "BAD REQUEST for {} {}\n\
                     Status: 400 Bad Request\n\
                     Content-Type: {}\n\
                     Response Body:\n{}\n\
                     \n\
                     The request was malformed or contains invalid parameters.",
                    method, url, content_type, formatted_body
                );
                Err(OneLoginError::ApiRequestFailed(format!(
                    "Bad request for {} {}\nStatus: 400\nResponse: {}",
                    method, url, formatted_body
                )))
            }
            500..=599 => {
                error!(
                    "SERVER ERROR for {} {}\n\
                     Status: {} (Server Error)\n\
                     Content-Type: {}\n\
                     Response Body:\n{}\n\
                     \n\
                     The OneLogin API server encountered an error. This is typically a temporary issue.",
                    method, url, status, content_type, formatted_body
                );
                Err(OneLoginError::ApiRequestFailed(format!(
                    "Server error for {} {}\nStatus: {}\nResponse: {}",
                    method, url, status, formatted_body
                )))
            }
            _ => {
                error!(
                    "API REQUEST FAILED for {} {}\n\
                     Status: {}\n\
                     Content-Type: {}\n\
                     Response Body:\n{}\n\
                     \n\
                     An unexpected error occurred.",
                    method, url, status, content_type, formatted_body
                );
                Err(OneLoginError::ApiRequestFailed(format!(
                    "Request failed for {} {}\nStatus: {}\nResponse: {}",
                    method, url, status, formatted_body
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
