use thiserror::Error;

#[derive(Error, Debug)]
pub enum OneLoginError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("API request failed: {0}")]
    ApiRequestFailed(String),

    #[error("Invalid response from API: {0}")]
    InvalidResponse(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Circuit breaker open: {0}")]
    CircuitBreakerOpen(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, OneLoginError>;

impl OneLoginError {
    pub fn is_retriable(&self) -> bool {
        matches!(
            self,
            OneLoginError::RateLimitExceeded
                | OneLoginError::ApiRequestFailed(_)
                | OneLoginError::HttpClientError(_)
        )
    }

    pub fn status_code(&self) -> u16 {
        match self {
            OneLoginError::NotFound(_) => 404,
            OneLoginError::PermissionDenied(_) => 403,
            OneLoginError::AuthenticationFailed(_) => 401,
            OneLoginError::InvalidInput(_) => 400,
            OneLoginError::RateLimitExceeded => 429,
            OneLoginError::CircuitBreakerOpen(_) => 503,
            _ => 500,
        }
    }
}
