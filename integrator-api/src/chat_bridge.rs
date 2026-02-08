use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub user_agent: String,
    pub created_at: String,
}

pub fn start_session(agent: &str) -> ChatSession {
    ChatSession {
        id: uuid::Uuid::new_v4().to_string(),
        user_agent: agent.into(),
        created_at: chrono::Utc::now().to_rfc3339(),
    }
}
