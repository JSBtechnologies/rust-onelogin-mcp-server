use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::device_trust::*;
use std::sync::Arc;
use tracing::instrument;

pub struct DeviceTrustApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl DeviceTrustApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// List all trusted devices
    #[instrument(skip(self))]
    pub async fn list_devices(&self, query: DeviceQuery) -> Result<Vec<Device>> {
        let mut path = "/api/2/devices".to_string();
        let mut params = vec![];

        if let Some(user_id) = query.user_id {
            params.push(format!("user_id={}", user_id));
        }
        if let Some(device_type) = query.device_type {
            params.push(format!("device_type={}", device_type));
        }
        if let Some(limit) = query.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(page) = query.page {
            params.push(format!("page={}", page));
        }

        if !params.is_empty() {
            path.push_str(&format!("?{}", params.join("&")));
        }

        self.client.get(&path).await
    }

    /// Get a specific device by ID
    #[instrument(skip(self))]
    pub async fn get_device(&self, device_id: &str) -> Result<Device> {
        self.client
            .get(&format!("/api/2/devices/{}", device_id))
            .await
    }

    /// Register a new trusted device
    #[instrument(skip(self, request))]
    pub async fn register_device(&self, request: RegisterDeviceRequest) -> Result<Device> {
        self.client.post("/api/2/devices", Some(&request)).await
    }

    /// Update a trusted device
    #[instrument(skip(self, request))]
    pub async fn update_device(
        &self,
        device_id: &str,
        request: UpdateDeviceRequest,
    ) -> Result<Device> {
        self.client
            .put(&format!("/api/2/devices/{}", device_id), Some(&request))
            .await
    }

    /// Delete a trusted device
    #[instrument(skip(self))]
    pub async fn delete_device(&self, device_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/api/2/devices/{}", device_id))
            .await
    }
}
