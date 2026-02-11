use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::certificates::*;
use std::sync::Arc;
use tracing::instrument;

pub struct CertificatesApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl CertificatesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// List all certificates
    #[instrument(skip(self))]
    pub async fn list_certificates(&self) -> Result<Vec<Certificate>> {
        self.client.get("/api/2/certificates").await
    }

    /// Get a specific certificate by ID
    #[instrument(skip(self))]
    pub async fn get_certificate(&self, cert_id: i64) -> Result<Certificate> {
        self.client
            .get(&format!("/api/2/certificates/{}", cert_id))
            .await
    }

    /// Generate a new certificate
    #[instrument(skip(self, request))]
    pub async fn generate_certificate(
        &self,
        request: GenerateCertificateRequest,
    ) -> Result<Certificate> {
        self.client
            .post("/api/2/certificates", Some(&request))
            .await
    }

    /// Renew an existing certificate
    #[instrument(skip(self))]
    pub async fn renew_certificate(&self, cert_id: i64) -> Result<Certificate> {
        self.client
            .put(
                &format!("/api/2/certificates/{}/renew", cert_id),
                None::<&()>,
            )
            .await
    }
}
