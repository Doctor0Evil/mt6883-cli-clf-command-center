use crate::MEM_GDB;
use chrono::Utc;
use domain-core::GdbAsset;
use uuid::Uuid;

pub fn crawl_and_ingest(target: &str) -> Vec<GdbAsset> {
    let mut mem = MEM_GDB.write();
    let asset = GdbAsset {
        id: format!("gdb_{}", Uuid::new_v4()),
        source: target.into(),
        created_at: Utc::now(),
        checksum: "checksum-placeholder".into(),
    };
    mem.push(asset.clone());
    vec![asset]
}
