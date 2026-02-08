use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub dna_profile_hash: Option<String>,
    pub mfa_enabled: bool,
    pub created_at: DateTime<Utc>,
}
