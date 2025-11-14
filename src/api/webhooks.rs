use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::webhooks::*;
use std::sync::Arc;
use tracing::instrument;

pub struct WebhooksApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl WebhooksApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_webhook_events(&self, filter: Option<String>) -> Result<Vec<WebhookEvent>> {
        let mut path = "/webhooks/events".to_string();
        if let Some(f) = filter {
            path.push_str(&format!("?filter={}", f));
        }
        self.client.get(&path).await
    }

    #[instrument(skip(verification))]
    pub fn verify_signature(verification: WebhookSignatureVerification) -> bool {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(verification.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(verification.payload.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        let expected = hex::encode(code_bytes);
        expected == verification.signature
    }
}
