use crate::command::{CommandInvocation, Namespace};

pub fn parse_line(input: &str) -> Result<CommandInvocation, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty command".into());
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return Err("empty command".into());
    }

    let ns = match parts[0] {
        "system" => Namespace::System,
        "module" => Namespace::Module,
        "plugin" => Namespace::Plugin,
        "security" => Namespace::Security,
        "data" => Namespace::Data,
        "ai" => Namespace::Ai,
        "bio" => Namespace::Bio,
        "cloud" => Namespace::Cloud,
        "session" => Namespace::Session,
        "audit" => Namespace::Audit,
        "upgrade" => Namespace::Upgrade,
        "integrator" => Namespace::Integrator,
        "custom" => Namespace::Custom,
        "phone" | "tel" => Namespace::Telephony,
        other => {
            return Err(format!("unknown namespace '{other}'"));
        }
    };

    if parts.len() < 2 {
        return Err("missing action verb".into());
    }

    let action = parts[1].to_string();
    let mut target: Option<String> = None;
    let mut flags: Vec<String> = Vec::new();

    for p in parts.iter().skip(2) {
        if p.starts_with('-') {
            flags.push((*p).to_string());
        } else if target.is_none() {
            target = Some((*p).to_string());
        } else {
            flags.push((*p).to_string());
        }
    }

    Ok(CommandInvocation::new(ns, action, target, flags, trimmed))
}
