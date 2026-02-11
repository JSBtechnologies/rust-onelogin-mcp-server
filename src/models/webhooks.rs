use serde::{Deserialize, Serialize};

/// Data for verifying webhook signatures.
///
/// When OneLogin sends events to your webhook endpoint, it includes
/// an X-OneLogin-Signature header. Use this struct with
/// `WebhooksApi::verify_signature()` to validate the payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookSignatureVerification {
    /// The signature from the X-OneLogin-Signature header
    pub signature: String,
    /// The raw JSON payload body
    pub payload: String,
    /// Your webhook secret (configured in OneLogin Admin UI)
    pub secret: String,
}
