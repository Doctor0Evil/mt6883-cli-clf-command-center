use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthzFlowState {
    pub id: String,
    pub provider: String,
    pub redirect_uri: String,
    pub created_at: String,
}

pub fn register_flow(provider: &str, redirect_uri: &str) -> AuthzFlowState {
    AuthzFlowState {
        id: uuid::Uuid::new_v4().to_string(),
        provider: provider.into(),
        redirect_uri: redirect_uri.into(),
        created_at: chrono::Utc::now().to_rfc3339(),
    }
}
