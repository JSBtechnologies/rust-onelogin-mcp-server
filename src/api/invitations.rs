use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::invitations::*;
use std::sync::Arc;
use tracing::instrument;

pub struct InvitationsApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl InvitationsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self, request))]
    pub async fn generate_invite_link(
        &self,
        request: GenerateInviteLinkRequest,
    ) -> Result<Invitation> {
        self.client
            .post("/invitations/generate", Some(&request))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn send_invite_link(&self, request: SendInviteLinkRequest) -> Result<Invitation> {
        self.client.post("/invitations/send", Some(&request)).await
    }

    #[instrument(skip(self))]
    pub async fn get_invitation(&self, invitation_id: &str) -> Result<Invitation> {
        self.client
            .get(&format!("/invitations/{}", invitation_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn cancel_invitation(&self, invitation_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/invitations/{}", invitation_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn list_pending_invitations(&self) -> Result<Vec<Invitation>> {
        self.client.get("/invitations?status=pending").await
    }
}
