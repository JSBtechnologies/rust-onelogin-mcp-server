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

    #[instrument(skip(self))]
    pub async fn list_factors(&self, user_id: i64) -> Result<Vec<MfaDevice>> {
        self.client
            .get(&format!("/users/{}/otp_devices", user_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn enroll_factor(
        &self,
        user_id: i64,
        request: EnrollMfaRequest,
    ) -> Result<MfaDevice> {
        self.client
            .post(&format!("/users/{}/otp_devices", user_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn remove_factor(&self, user_id: i64, device_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/users/{}/otp_devices/{}", user_id, device_id))
            .await
    }

    #[instrument(skip(self, verification))]
    pub async fn verify_factor(
        &self,
        user_id: i64,
        verification: MfaVerification,
    ) -> Result<MfaVerificationResponse> {
        self.client
            .post(
                &format!("/users/{}/otp_devices/verify", user_id),
                Some(&verification),
            )
            .await
    }
}
