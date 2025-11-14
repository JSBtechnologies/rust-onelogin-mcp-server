use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::sessions::*;
use std::sync::Arc;
use tracing::instrument;

pub struct SessionsApi {
    client: Arc<HttpClient>,
    cache: Arc<CacheManager>,
}

impl SessionsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    #[instrument(skip(self))]
    pub async fn list_sessions(&self, params: Option<SessionQueryParams>) -> Result<Vec<Session>> {
        let mut path = "/sessions".to_string();
        if let Some(p) = params {
            if let Ok(query) = serde_qs::to_string(&p) {
                path.push('?');
                path.push_str(&query);
            }
        }
        self.client.get(&path).await
    }

    #[instrument(skip(self))]
    pub async fn get_session(&self, session_id: i64) -> Result<Session> {
        self.client
            .get(&format!("/sessions/{}", session_id))
            .await
    }

    #[instrument(skip(self))]
    pub async fn delete_session(&self, session_id: i64) -> Result<()> {
        self.client
            .delete(&format!("/sessions/{}", session_id))
            .await
    }
}
