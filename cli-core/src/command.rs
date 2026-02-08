use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Namespace {
    System,
    Module,
    Plugin,
    Security,
    Data,
    Ai,
    Bio,
    Cloud,
    Session,
    Audit,
    Upgrade,
    Integrator,
    Custom,
    Telephony,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInvocation {
    pub namespace: Namespace,
    pub action: String,
    pub target: Option<String>,
    pub flags: Vec<String>,
    pub raw: String,
}

impl CommandInvocation {
    pub fn new(
        namespace: Namespace,
        action: impl Into<String>,
        target: Option<String>,
        flags: Vec<String>,
        raw: impl Into<String>,
    ) -> Self {
        Self {
            namespace,
            action: action.into(),
            target,
            flags,
            raw: raw.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub ok: bool,
    pub message: String,
    pub payload: Option<serde_json::Value>,
}

impl CommandResult {
    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            ok: true,
            message: message.into(),
            payload: None,
        }
    }

    pub fn ok_with(message: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            ok: true,
            message: message.into(),
            payload: Some(payload),
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            message: message.into(),
            payload: None,
        }
    }
}
