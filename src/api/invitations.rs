use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::{OneLoginError, Result};
use crate::models::invitations::*;
use crate::models::{ApiResponse, StatusOnlyResponse};
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
    ) -> Result<InviteLinkResponse> {
        // Note: invitations use API v1
        // Response format: {"data":["https://...reset?token=..."],"status":{...}}
        let response: ApiResponse<Vec<String>> = self
            .client
            .post("/api/1/invites/get_invite_link", Some(&request))
            .await?;

        let link = response.data.into_iter().next().ok_or_else(|| {
            OneLoginError::InvalidResponse(
                "No invite link returned in API response data".to_string(),
            )
        })?;

        Ok(InviteLinkResponse { invite_link: link })
    }

    #[instrument(skip(self, request))]
    pub async fn send_invite_link(
        &self,
        request: SendInviteLinkRequest,
    ) -> Result<SendInviteResponse> {
        // Note: invitations use API v1
        // Response format: {"status":{"type":"success","code":200,"message":"Email sent successfully to ...","error":false}}
        let response: StatusOnlyResponse = self
            .client
            .post("/api/1/invites/send_invite_link", Some(&request))
            .await?;

        let message = response
            .status
            .message
            .unwrap_or_else(|| "Invite email sent successfully".to_string());

        Ok(SendInviteResponse { message })
    }
}
