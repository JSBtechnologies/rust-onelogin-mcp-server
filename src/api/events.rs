use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::events::*;
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
        let mut path = "/events".to_string();
        if let Some(p) = params {
            if let Ok(query) = serde_qs::to_string(&p) {
                path.push('?');
                path.push_str(&query);
            }
        }
        self.client.get(&path).await
    }

    #[instrument(skip(self))]
    pub async fn get_event(&self, event_id: i64) -> Result<Event> {
        self.client.get(&format!("/events/{}", event_id)).await
    }

    #[instrument(skip(self, request))]
    pub async fn create_event(&self, request: CreateEventRequest) -> Result<Event> {
        self.client.post("/events", Some(&request)).await
    }
}
