use crate::MEM_MODULES;
use chrono::Utc;
use domain-core::{Module, ModuleStatus, Plugin};
use uuid::Uuid;

pub fn list_modules() -> Vec<Module> {
    MEM_MODULES.read().clone()
}

pub fn register_plugin(path: &str) -> Plugin {
    Plugin {
        id: Uuid::new_v4().to_string(),
        name: infer_name(path),
        version: "1.0.0".into(),
        binary_path: path.into(),
        description: "auto-registered plugin".into(),
        registered_at: Utc::now(),
    }
}

fn infer_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string()
}

pub fn seed() {
    let mut modules = MEM_MODULES.write();
    if !modules.is_empty() {
        return;
    }
    modules.push(Module {
        id: Uuid::new_v4().to_string(),
        name: "DNA_MFA_Module".into(),
        version: "1.0.0".into(),
        module_type: "Security".into(),
        binary_location: "/modules/dna_mfa.bin".into(),
        description: "DNA-based multi-factor authentication module".into(),
        status: ModuleStatus::Available,
        registered_at: Utc::now(),
    });
}
