use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::connectors::*;
use std::sync::Arc;
use tracing::instrument;

pub struct ConnectorsApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl ConnectorsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_connectors(&self) -> Result<Vec<Connector>> {
        self.client.get("/api/2/connectors").await
    }

    /// Get a specific connector by ID
    #[instrument(skip(self))]
    pub async fn get_connector(&self, connector_id: i64) -> Result<Connector> {
        self.client
            .get(&format!("/api/2/connectors/{}", connector_id))
            .await
    }
}
