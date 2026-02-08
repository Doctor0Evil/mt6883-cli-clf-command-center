use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleStatus {
    Available,
    Loaded,
    Error,
    UpgradePending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub version: String,
    pub module_type: String,
    pub binary_location: String,
    pub description: String,
    pub status: ModuleStatus,
    pub registered_at: DateTime<Utc>,
}

pub fn load_module(name: &str) -> bool {
    // hook into storage::db_modules in real implementation
    let _ = name;
    true
}
