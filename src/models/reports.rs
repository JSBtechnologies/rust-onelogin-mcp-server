use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A report definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ReportParameter>>,
}

/// A parameter for a report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub param_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A running or completed report job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Value>,
}

/// Request to run a report
#[derive(Debug, Serialize, Deserialize)]
pub struct RunReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}
