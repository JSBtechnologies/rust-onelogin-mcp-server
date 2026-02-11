use crate::core::cache::CacheManager;
use crate::core::client::HttpClient;
use crate::core::error::Result;
use crate::models::reports::*;
use std::sync::Arc;
use tracing::instrument;

pub struct ReportsApi {
    client: Arc<HttpClient>,
    #[allow(dead_code)]
    cache: Arc<CacheManager>,
}

impl ReportsApi {
    pub fn new(client: Arc<HttpClient>, cache: Arc<CacheManager>) -> Self {
        Self { client, cache }
    }

    /// List all available reports
    #[instrument(skip(self))]
    pub async fn list_reports(&self) -> Result<Vec<Report>> {
        self.client.get("/api/2/reports").await
    }

    /// Get a specific report by ID
    #[instrument(skip(self))]
    pub async fn get_report(&self, report_id: i64) -> Result<Report> {
        self.client
            .get(&format!("/api/2/reports/{}", report_id))
            .await
    }

    /// Run a report synchronously and return results
    #[instrument(skip(self, request))]
    pub async fn run_report(&self, report_id: i64, request: Option<RunReportRequest>) -> Result<ReportJob> {
        self.client
            .post(&format!("/api/2/reports/{}/run", report_id), request.as_ref())
            .await
    }

    /// Get results from a report job
    #[instrument(skip(self))]
    pub async fn get_report_results(&self, report_id: i64, job_id: &str) -> Result<ReportJob> {
        self.client
            .get(&format!("/api/2/reports/{}/results/{}", report_id, job_id))
            .await
    }
}
