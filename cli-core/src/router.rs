use crate::command::{CommandInvocation, CommandResult, Namespace};
use crate::registry::GLOBAL_REGISTRY;
use domain-core::modules;
use domain-core::sessions;
use engines::{ai_engine, bio_engine, cloud_sync, data_lake, integrator_chat};
use security::{crypto, mfa};
use storage::{db_audit, db_assets, db_modules, db_sessions, db_users};
use telephony::{phone_profile, signaling};

pub fn init_registry() {
    let mut reg = GLOBAL_REGISTRY.write();

    // system
    reg.register(Namespace::System, "crawl".into(), system_crawl);
    reg.register(Namespace::System, "status".into(), system_status);

    // module
    reg.register(Namespace::Module, "avail".into(), module_avail);
    reg.register(Namespace::Module, "load".into(), module_load);

    // plugin
    reg.register(Namespace::Plugin, "register".into(), plugin_register);

    // security
    reg.register(Namespace::Security, "audit".into(), security_audit);
    reg.register(Namespace::Security, "mfa".into(), security_mfa);

    // data
    reg.register(Namespace::Data, "ingest".into(), data_ingest);

    // ai
    reg.register(Namespace::Ai, "analyze".into(), ai_analyze);

    // bio
    reg.register(Namespace::Bio, "enroll".into(), bio_enroll);

    // cloud
    reg.register(Namespace::Cloud, "sync".into(), cloud_sync_cmd);

    // session
    reg.register(Namespace::Session, "rollback".into(), session_rollback);
    reg.register(Namespace::Session, "save".into(), session_save);

    // integrator
    reg.register(Namespace::Integrator, "chat".into(), integrator_chat_cmd);

    // telephony
    reg.register(Namespace::Telephony, "profile".into(), tel_profile);
    reg.register(Namespace::Telephony, "dial".into(), tel_dial);
}

fn system_crawl(cmd: &CommandInvocation) -> CommandResult {
    let target = cmd
        .flags
        .iter()
        .find(|f| f.starts_with("--target="))
        .map(|f| f.trim_start_matches("--target=").to_string())
        .unwrap_or_else(|| ".".into());

    let assets = db_assets::crawl_and_ingest(&target);
    CommandResult::ok_with(
        format!("system crawl complete for {}", target),
        serde_json::json!({ "ingested": assets }),
    )
}

fn system_status(_: &CommandInvocation) -> CommandResult {
    let modules = db_modules::list_modules();
    let sessions = db_sessions::latest_snapshot_id();
    let stats = serde_json::json!({
        "modules_loaded": modules.len(),
        "latest_snapshot": sessions,
    });
    CommandResult::ok_with("system status", stats)
}

fn module_avail(_: &CommandInvocation) -> CommandResult {
    let mods = db_modules::list_modules();
    CommandResult::ok_with("available modules", serde_json::to_value(mods).unwrap())
}

fn module_load(cmd: &CommandInvocation) -> CommandResult {
    let name = match &cmd.target {
        Some(t) => t,
        None => return CommandResult::err("module name required"),
    };
    if modules::load_module(name) {
        CommandResult::ok(format!("module '{}' loaded", name))
    } else {
        CommandResult::err(format!("failed to load module '{}'", name))
    }
}

fn plugin_register(cmd: &CommandInvocation) -> CommandResult {
    let path = match &cmd.target {
        Some(t) => t,
        None => return CommandResult::err("plugin binary path required"),
    };
    let plugin = db_modules::register_plugin(path);
    CommandResult::ok_with(
        format!("plugin registered from {}", path),
        serde_json::to_value(plugin).unwrap(),
    )
}

fn security_audit(_: &CommandInvocation) -> CommandResult {
    let events = db_audit::all_events();
    CommandResult::ok_with("security audit", serde_json::to_value(events).unwrap())
}

fn security_mfa(cmd: &CommandInvocation) -> CommandResult {
    if cmd.flags.contains(&"--enroll".into()) {
        if let Some(user) = &cmd.target {
            if mfa::enroll_user(user) {
                return CommandResult::ok(format!("MFA enrolled for {}", user));
            }
        }
        return CommandResult::err("MFA enroll failed, user required");
    }
    if cmd.flags.contains(&"--verify".into()) {
        if let Some(user) = &cmd.target {
            if mfa::verify_user(user) {
                return CommandResult::ok(format!("MFA verified for {}", user));
            }
        }
        return CommandResult::err("MFA verify failed, user required");
    }
    CommandResult::err("MFA requires --enroll or --verify")
}

fn data_ingest(cmd: &CommandInvocation) -> CommandResult {
    let source = match &cmd.target {
        Some(t) => t,
        None => return CommandResult::err("data source required"),
    };
    let gdb_id = data_lake::ingest_source(source);
    CommandResult::ok_with(
        format!("data ingested from {}", source),
        serde_json::json!({ "gdb_id": gdb_id }),
    )
}

fn ai_analyze(cmd: &CommandInvocation) -> CommandResult {
    let dataset = match &cmd.target {
        Some(t) => t,
        None => return CommandResult::err("dataset path or gdb id required"),
    };
    let report = ai_engine::analyze_dataset(dataset);
    CommandResult::ok_with("ai analysis started", serde_json::to_value(report).unwrap())
}

fn bio_enroll(cmd: &CommandInvocation) -> CommandResult {
    let user = match &cmd.target {
        Some(t) => t,
        None => return CommandResult::err("user id required"),
    };
    if bio_engine::enroll_biometric(user) {
        CommandResult::ok(format!("biometric enrolled for {}", user))
    } else {
        CommandResult::err("biometric enrollment failed")
    }
}

fn cloud_sync_cmd(cmd: &CommandInvocation) -> CommandResult {
    let snapshot = cmd.flags.contains(&"--snapshot".into());
    if cloud_sync::sync_state(snapshot) {
        CommandResult::ok("cloud sync complete")
    } else {
        CommandResult::err("cloud sync failed")
    }
}

fn session_save(cmd: &CommandInvocation) -> CommandResult {
    let now = cmd.flags.contains(&"--now".into());
    if !now {
        return CommandResult::err("--now flag required");
    }
    let id = sessions::snapshot_now();
    CommandResult::ok_with("session saved", serde_json::json!({ "snapshot_id": id }))
}

fn session_rollback(cmd: &CommandInvocation) -> CommandResult {
    let snap = match &cmd.target {
        Some(t) => t,
        None => return CommandResult::err("snapshot id required"),
    };
    if sessions::rollback_to(snap) {
        CommandResult::ok(format!("rolled back to {}", snap))
    } else {
        CommandResult::err("rollback failed")
    }
}

fn integrator_chat_cmd(cmd: &CommandInvocation) -> CommandResult {
    if cmd.flags.contains(&"--enable".into()) {
        if integrator_chat::enable() {
            return CommandResult::ok("AI-Chat integrator enabled");
        }
        return CommandResult::err("failed to enable integrator");
    }
    CommandResult::err("integrator chat requires --enable")
}

fn tel_profile(cmd: &CommandInvocation) -> CommandResult {
    if cmd.flags.contains(&"--list".into()) {
        let profiles = phone_profile::list_profiles();
        return CommandResult::ok_with(
            "phone profiles",
            serde_json::to_value(profiles).unwrap(),
        );
    }
    CommandResult::err("phone profile requires --list")
}

fn tel_dial(cmd: &CommandInvocation) -> CommandResult {
    let target = match &cmd.target {
        Some(t) => t,
        None => return CommandResult::err("phone number required"),
    };
    let session = signaling::dial_number(target);
    CommandResult::ok_with("dial initiated", serde_json::to_value(session).unwrap())
}
