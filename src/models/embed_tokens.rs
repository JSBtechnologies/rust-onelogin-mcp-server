use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedToken {
    pub token: String,
    pub expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateEmbedTokenRequest {
    pub email: String,
    pub session_duration: Option<i32>,
    pub return_to_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddableApp {
    pub id: i64,
    pub name: String,
    pub icon_url: Option<String>,
}
