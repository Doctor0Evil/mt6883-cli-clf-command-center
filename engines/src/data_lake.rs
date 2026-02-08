use chrono::Utc;
use uuid::Uuid;

pub fn ingest_source(source: &str) -> String {
    let _ = source;
    format!("gdb_{}_{}", Utc::now().format("%Y%m%d_%H%M%S"), Uuid::new_v4())
}
