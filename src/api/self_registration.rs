use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::self_registration::*;
use std::sync::Arc;
use tracing::instrument;

pub struct SelfRegistrationApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl SelfRegistrationApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// List all self-registration profiles
    #[instrument(skip(self))]
    pub async fn list_profiles(&self) -> Result<Vec<SelfRegistrationProfile>> {
        let response: SelfRegistrationProfilesResponse =
            self.client.get("/api/2/self_registration_profiles").await?;
        Ok(response.self_registration_profiles)
    }

    /// Get a specific self-registration profile
    #[instrument(skip(self))]
    pub async fn get_profile(&self, profile_id: i64) -> Result<SelfRegistrationProfile> {
        self.client
            .get(&format!("/api/2/self_registration_profiles/{}", profile_id))
            .await
    }

    /// Create a new self-registration profile
    #[instrument(skip(self, request))]
    pub async fn create_profile(
        &self,
        request: CreateSelfRegistrationProfileRequest,
    ) -> Result<SelfRegistrationProfile> {
        self.client
            .post("/api/2/self_registration_profiles", Some(&request))
            .await
    }

    /// Update an existing self-registration profile
    #[instrument(skip(self, request))]
    pub async fn update_profile(
        &self,
        profile_id: i64,
        request: UpdateSelfRegistrationProfileRequest,
    ) -> Result<SelfRegistrationProfile> {
        self.client
            .put(
                &format!("/api/2/self_registration_profiles/{}", profile_id),
                Some(&request),
            )
            .await
    }

    /// Delete a self-registration profile
    #[instrument(skip(self))]
    pub async fn delete_profile(&self, profile_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/api/2/self_registration_profiles/{}", profile_id))
            .await
    }

    /// List pending registrations for a profile
    #[instrument(skip(self))]
    pub async fn list_registrations(&self, profile_id: i64) -> Result<Vec<Registration>> {
        self.client
            .get(&format!(
                "/api/2/self_registration_profiles/{}/registrations",
                profile_id
            ))
            .await
    }

    /// Approve or reject a registration
    #[instrument(skip(self, request))]
    pub async fn approve_registration(
        &self,
        profile_id: i64,
        registration_id: i64,
        request: ApproveRegistrationRequest,
    ) -> Result<Registration> {
        self.client
            .put(
                &format!(
                    "/api/2/self_registration_profiles/{}/registrations/{}",
                    profile_id, registration_id
                ),
                Some(&request),
            )
            .await
    }
}
