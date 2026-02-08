pub mod command;
pub mod parser;
pub mod registry;
pub mod router;

pub use command::{CommandInvocation, CommandResult, Namespace};
pub use parser::parse_line;
pub use registry::dispatch;
pub use router::init_registry;
