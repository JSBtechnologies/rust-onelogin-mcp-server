use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::trusted_idps::*;
use std::sync::Arc;
use tracing::instrument;

pub struct TrustedIdpsApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl TrustedIdpsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// List all trusted identity providers
    #[instrument(skip(self))]
    pub async fn list_trusted_idps(&self) -> Result<Vec<TrustedIdp>> {
        self.client.get("/api/2/trusted_idps").await
    }

    /// Get a specific trusted IDP by ID
    #[instrument(skip(self))]
    pub async fn get_trusted_idp(&self, idp_id: i64) -> Result<TrustedIdp> {
        self.client
            .get(&format!("/api/2/trusted_idps/{}", idp_id))
            .await
    }

    /// Create a new trusted IDP
    #[instrument(skip(self, request))]
    pub async fn create_trusted_idp(&self, request: CreateTrustedIdpRequest) -> Result<TrustedIdp> {
        self.client
            .post("/api/2/trusted_idps", Some(&request))
            .await
    }

    /// Update a trusted IDP
    #[instrument(skip(self, request))]
    pub async fn update_trusted_idp(
        &self,
        idp_id: i64,
        request: UpdateTrustedIdpRequest,
    ) -> Result<TrustedIdp> {
        self.client
            .put(&format!("/api/2/trusted_idps/{}", idp_id), Some(&request))
            .await
    }

    /// Delete a trusted IDP
    #[instrument(skip(self))]
    pub async fn delete_trusted_idp(&self, idp_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/api/2/trusted_idps/{}", idp_id))
            .await
    }

    /// Get SAML metadata for a trusted IDP
    #[instrument(skip(self))]
    pub async fn get_trusted_idp_metadata(&self, idp_id: i64) -> Result<String> {
        self.client
            .get(&format!("/api/2/trusted_idps/{}/metadata", idp_id))
            .await
    }

    /// Update SAML metadata for a trusted IDP
    #[instrument(skip(self, request))]
    pub async fn update_trusted_idp_metadata(
        &self,
        idp_id: i64,
        request: UpdateTrustedIdpMetadataRequest,
    ) -> Result<()> {
        self.client
            .put(
                &format!("/api/2/trusted_idps/{}/metadata", idp_id),
                Some(&request),
            )
            .await
    }

    /// Get issuer URL for a trusted IDP
    #[instrument(skip(self))]
    pub async fn get_trusted_idp_issuer(&self, idp_id: i64) -> Result<TrustedIdpIssuer> {
        self.client
            .get(&format!("/api/2/trusted_idps/{}/issuer", idp_id))
            .await
    }
}
