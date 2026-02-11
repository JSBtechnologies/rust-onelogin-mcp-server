use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::events::*;
use crate::models::users::LockUserResponse;
use crate::models::{ApiResponse, PaginatedResponse};
use std::sync::Arc;
use tracing::instrument;

pub struct EventsApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl EventsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_events(&self, params: Option<EventQueryParams>) -> Result<Vec<Event>> {
        let mut path = "/api/1/events".to_string();
        if let Some(p) = params {
            if let Ok(query) = serde_qs::to_string(&p) {
                path.push('?');
                path.push_str(&query);
            }
        }
        // OneLogin API v1 returns events wrapped in a pagination envelope
        let response: PaginatedResponse<Event> = self.client.get(&path).await?;
        Ok(response.data)
    }

    #[instrument(skip(self))]
    pub async fn get_event(&self, event_id: i64) -> Result<Event> {
        // OneLogin API v1 returns event wrapped in response envelope
        let response: ApiResponse<Event> = self.client.get(&format!("/api/1/events/{}", event_id)).await?;
        Ok(response.data)
    }

    #[instrument(skip(self, request))]
    pub async fn create_event(&self, request: CreateEventRequest) -> Result<()> {
        // OneLogin API v1 returns status response, not event data
        let _response: LockUserResponse = self.client.post("/api/1/events", Some(&request)).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn list_event_types(&self) -> Result<Vec<EventType>> {
        // OneLogin API v1 returns event types wrapped in response envelope
        let response: ApiResponse<Vec<EventType>> = self.client.get("/api/1/events/types").await?;
        Ok(response.data)
    }
}
