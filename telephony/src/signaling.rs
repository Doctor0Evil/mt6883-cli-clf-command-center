use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Serialize)]
pub struct CallSession {
    pub id: String,
    pub number: String,
    pub started_at: String,
    pub state: String,
}

pub fn dial_number(number: &str) -> CallSession {
    CallSession {
        id: Uuid::new_v4().to_string(),
        number: number.into(),
        started_at: Utc::now().to_rfc3339(),
        state: "dialing".into(),
    }
}
