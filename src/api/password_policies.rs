use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::password_policies::*;
use std::sync::Arc;
use tracing::instrument;

pub struct PasswordPoliciesApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl PasswordPoliciesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// List all password policies
    #[instrument(skip(self))]
    pub async fn list_password_policies(&self) -> Result<Vec<PasswordPolicy>> {
        self.client.get("/api/2/password_policies").await
    }

    /// Get a specific password policy by ID
    #[instrument(skip(self))]
    pub async fn get_password_policy(&self, policy_id: i64) -> Result<PasswordPolicy> {
        self.client
            .get(&format!("/api/2/password_policies/{}", policy_id))
            .await
    }

    /// Create a new password policy
    #[instrument(skip(self, request))]
    pub async fn create_password_policy(
        &self,
        request: CreatePasswordPolicyRequest,
    ) -> Result<PasswordPolicy> {
        self.client
            .post("/api/2/password_policies", Some(&request))
            .await
    }

    /// Update an existing password policy
    #[instrument(skip(self, request))]
    pub async fn update_password_policy(
        &self,
        policy_id: i64,
        request: UpdatePasswordPolicyRequest,
    ) -> Result<PasswordPolicy> {
        self.client
            .put(
                &format!("/api/2/password_policies/{}", policy_id),
                Some(&request),
            )
            .await
    }
}
