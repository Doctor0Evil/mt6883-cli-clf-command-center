use crate::command::{CommandInvocation, CommandResult, Namespace};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

pub type CommandHandler = fn(&CommandInvocation) -> CommandResult;

#[derive(Default)]
pub struct Registry {
    handlers: HashMap<(Namespace, String), CommandHandler>,
}

impl Registry {
    pub fn register(
        &mut self,
        namespace: Namespace,
        action: impl Into<String>,
        handler: CommandHandler,
    ) {
        self.handlers
            .insert((namespace, action.into()), handler);
    }

    pub fn dispatch(&self, cmd: &CommandInvocation) -> CommandResult {
        let key = (cmd.namespace.clone(), cmd.action.clone());
        if let Some(handler) = self.handlers.get(&key) {
            handler(cmd)
        } else {
            CommandResult::err(format!(
                "no handler for {:?} {}",
                cmd.namespace, cmd.action
            ))
        }
    }
}

pub static GLOBAL_REGISTRY: Lazy<RwLock<Registry>> =
    Lazy::new(|| RwLock::new(Registry::default()));

pub fn dispatch(cmd: &CommandInvocation) -> CommandResult {
    GLOBAL_REGISTRY.read().dispatch(cmd)
}
