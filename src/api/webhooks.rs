use crate::models::webhooks::WebhookSignatureVerification;
use tracing::instrument;

/// Webhook utilities for verifying incoming webhook payloads.
///
/// Note: Webhook configuration (create, update, delete) is done through
/// the OneLogin Admin UI at Developers > Webhooks, not via API.
pub struct WebhooksApi;

impl WebhooksApi {
    pub fn new() -> Self {
        Self
    }

    /// Verify the HMAC signature of an incoming webhook payload.
    ///
    /// When OneLogin sends events to your webhook endpoint, it includes
    /// an X-OneLogin-Signature header containing an HMAC-SHA256 signature.
    /// Use this function to verify the payload authenticity.
    ///
    /// # Arguments
    /// * `verification` - Contains the signature, payload, and your webhook secret
    ///
    /// # Returns
    /// * `true` if the signature is valid
    /// * `false` if the signature doesn't match
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

impl Default for WebhooksApi {
    fn default() -> Self {
        Self::new()
    }
}
