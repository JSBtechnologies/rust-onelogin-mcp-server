use serde::{Deserialize, Serialize};

/// Response from generate_invite_link - contains the activation URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteLinkResponse {
    pub invite_link: String,
}

/// Response from send_invite_link - confirmation that email was sent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInviteResponse {
    pub message: String,
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
