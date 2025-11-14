use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::policies::*;
use std::sync::Arc;
use tracing::instrument;

pub struct PoliciesApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl PoliciesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_policies(&self) -> Result<Vec<Policy>> {
        self.client.get("/policies").await
    }

    #[instrument(skip(self))]
    pub async fn get_policy(&self, policy_id: &str) -> Result<Policy> {
        self.client
            .get(&format!("/policies/{}", policy_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn create_policy(&self, request: CreatePolicyRequest) -> Result<Policy> {
        self.client.post("/policies", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_policy(
        &self,
        policy_id: &str,
        request: UpdatePolicyRequest,
    ) -> Result<Policy> {
        self.client
            .put(&format!("/policies/{}", policy_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_policy(&self, policy_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/policies/{}", policy_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn assign_to_user(&self, policy_id: &str, user_id: i64) -> Result<()> {
        self.client
            .post(
                &format!("/policies/{}/users/{}", policy_id, user_id),
                None::<&()>,
            )
            .await
    }
}
