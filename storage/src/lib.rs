use domain-core::{AuditEvent, GdbAsset, Module, SessionSnapshot, User};
use once_cell::sync::Lazy;
use parking_lot::RwLock;

pub mod db_assets;
pub mod db_audit;
pub mod db_modules;
pub mod db_sessions;
pub mod db_users;

pub static MEM_MODULES: Lazy<RwLock<Vec<Module>>> =
    Lazy::new(|| RwLock::new(Vec::new()));
pub static MEM_GDB: Lazy<RwLock<Vec<GdbAsset>>> =
    Lazy::new(|| RwLock::new(Vec::new()));
pub static MEM_AUDIT: Lazy<RwLock<Vec<AuditEvent>>> =
    Lazy::new(|| RwLock::new(Vec::new()));
pub static MEM_USERS: Lazy<RwLock<Vec<User>>> =
    Lazy::new(|| RwLock::new(Vec::new()));
pub static MEM_SESSIONS: Lazy<RwLock<Vec<SessionSnapshot>>> =
    Lazy::new(|| RwLock::new(Vec::new()));

pub use db_assets::*;
pub use db_audit::*;
pub use db_modules::*;
pub use db_sessions::*;
pub use db_users::*;
