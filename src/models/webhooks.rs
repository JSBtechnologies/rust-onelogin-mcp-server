use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub event_type: String,
    pub created_at: String,
    pub payload: serde_json::Value,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookSignatureVerification {
    pub signature: String,
    pub payload: String,
    pub secret: String,
}
