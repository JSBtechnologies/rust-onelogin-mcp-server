use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::directories::*;
use std::sync::Arc;
use tracing::instrument;

pub struct DirectoriesApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl DirectoriesApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_connectors(&self) -> Result<Vec<DirectoryConnector>> {
        self.client.get("/directories").await
    }

    #[instrument(skip(self))]
    pub async fn get_connector(&self, connector_id: &str) -> Result<DirectoryConnector> {
        self.client
            .get(&format!("/directories/{}", connector_id))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn create_connector(
        &self,
        request: CreateDirectoryConnectorRequest,
    ) -> Result<DirectoryConnector> {
        self.client.post("/directories", Some(&request)).await
    }

    #[instrument(skip(self, request))]
    pub async fn update_connector(
        &self,
        connector_id: &str,
        request: UpdateDirectoryConnectorRequest,
    ) -> Result<DirectoryConnector> {
        self.client
            .put(&format!("/directories/{}", connector_id), Some(&request))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_connector(&self, connector_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/directories/{}", connector_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn sync_directory(&self, connector_id: &str) -> Result<SyncStatus> {
        self.client
            .post(&format!("/directories/{}/sync", connector_id), None::<&()>)
            .await
    }

    #[instrument(skip(self))]
    pub async fn get_sync_status(&self, connector_id: &str) -> Result<SyncStatus> {
        self.client
            .get(&format!("/directories/{}/sync/status", connector_id))
            .await
    }
}
