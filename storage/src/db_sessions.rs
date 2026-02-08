use crate::MEM_SESSIONS;
use chrono::Utc;
use domain-core::SessionSnapshot;
use uuid::Uuid;

pub fn latest_snapshot_id() -> Option<String> {
    MEM_SESSIONS
        .read()
        .last()
        .map(|s| s.id.clone())
}

pub fn create_snapshot(description: &str) -> String {
    let mut mem = MEM_SESSIONS.write();
    let snap = SessionSnapshot {
        id: format!("snap_{}", Uuid::new_v4()),
        created_at: Utc::now(),
        description: description.into(),
    };
    let id = snap.id.clone();
    mem.push(snap);
    id
}
