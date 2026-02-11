use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
}
