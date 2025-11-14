use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitation {
    pub id: String,
    pub email: String,
    pub status: String,
    pub created_at: String,
    pub expires_at: String,
    pub invite_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateInviteLinkRequest {
    pub email: String,
    pub custom_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendInviteLinkRequest {
    pub email: String,
    pub personal_email: Option<String>,
    pub custom_message: Option<String>,
}
