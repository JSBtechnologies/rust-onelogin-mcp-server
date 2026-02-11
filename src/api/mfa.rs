use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::mfa::*;
use std::sync::Arc;
use tracing::instrument;

pub struct MfaApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl MfaApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    // Get available authentication factors for a user
    #[instrument(skip(self))]
    pub async fn get_available_factors(&self, user_id: i64) -> Result<Vec<MfaFactor>> {
        // OneLogin API v2 returns direct array
        self.client
            .get(&format!("/api/2/mfa/users/{}/factors", user_id))
            .await
    }

    // Get enrolled MFA devices for a user
    #[instrument(skip(self))]
    pub async fn list_factors(&self, user_id: i64) -> Result<Vec<MfaDevice>> {
        // OneLogin API v2 returns direct array
        self.client
            .get(&format!("/api/2/mfa/users/{}/devices", user_id))
            .await
    }

    // Enroll a new MFA factor
    #[instrument(skip(self, request))]
    pub async fn enroll_factor(
        &self,
        user_id: i64,
        factor_id: i64,
        request: EnrollMfaRequest,
    ) -> Result<MfaDevice> {
        // OneLogin API v2 returns direct object
        self.client
            .post(&format!("/api/2/mfa/users/{}/factors/{}", user_id, factor_id), Some(&request))
            .await
    }

    // Verify MFA factor enrollment (OTP)
    #[instrument(skip(self, request))]
    pub async fn verify_enrollment(
        &self,
        user_id: i64,
        factor_id: i64,
        request: MfaVerification,
    ) -> Result<MfaVerificationResponse> {
        // OneLogin API v2 returns direct object
        self.client
            .put(&format!("/api/2/mfa/users/{}/factors/{}/verify", user_id, factor_id), Some(&request))
            .await
    }

    // Activate an enrolled MFA device
    #[instrument(skip(self, request))]
    pub async fn activate_factor(
        &self,
        user_id: i64,
        device_id: i64,
        request: Option<serde_json::Value>,
    ) -> Result<MfaVerificationResponse> {
        // OneLogin API v2 returns direct object
        self.client
            .post(&format!("/api/2/mfa/users/{}/devices/{}", user_id, device_id), request.as_ref())
            .await
    }

    // Verify MFA factor (OTP)
    #[instrument(skip(self, verification))]
    pub async fn verify_factor(
        &self,
        user_id: i64,
        device_id: i64,
        verification: MfaVerification,
    ) -> Result<MfaVerificationResponse> {
        // OneLogin API v2 returns direct object
        self.client
            .put(&format!("/api/2/mfa/users/{}/devices/{}/verify", user_id, device_id), Some(&verification))
            .await
    }

    // Remove/delete an MFA device
    #[instrument(skip(self))]
    pub async fn remove_factor(&self, user_id: i64, device_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/api/2/mfa/users/{}/devices/{}", user_id, device_id))
            .await
    }

    /// Generate a temporary MFA token for helpdesk scenarios
    /// This allows a support agent to generate a one-time MFA bypass token for a user
    #[instrument(skip(self, request))]
    pub async fn generate_mfa_token(
        &self,
        user_id: i64,
        request: GenerateMfaTokenRequest,
    ) -> Result<MfaToken> {
        self.client
            .post(&format!("/api/2/mfa/users/{}/mfa_token", user_id), Some(&request))
            .await
    }

    /// Verify an MFA token
    /// Check if a temporary MFA token is still valid
    #[instrument(skip(self, request))]
    pub async fn verify_mfa_token(
        &self,
        user_id: i64,
        request: VerifyMfaTokenRequest,
    ) -> Result<VerifyMfaTokenResponse> {
        self.client
            .post(&format!("/api/2/mfa/users/{}/mfa_token/verify", user_id), Some(&request))
            .await
    }
}
