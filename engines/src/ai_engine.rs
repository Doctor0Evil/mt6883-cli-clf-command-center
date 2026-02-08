use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub id: String,
    pub dataset: String,
    pub started_at: String,
    pub status: String,
    pub notes: String,
}

pub fn analyze_dataset(dataset: &str) -> AnalysisReport {
    AnalysisReport {
        id: Uuid::new_v4().to_string(),
        dataset: dataset.to_string(),
        started_at: Utc::now().to_rfc3339(),
        status: "running".into(),
        notes: "analysis pipeline initialized".into(),
    }
}
