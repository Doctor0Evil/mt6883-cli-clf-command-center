use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GdbAsset {
    pub id: String,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub checksum: String,
}
