use crate::MEM_AUDIT;
use domain-core::AuditEvent;

pub fn all_events() -> Vec<AuditEvent> {
    MEM_AUDIT.read().clone()
}
