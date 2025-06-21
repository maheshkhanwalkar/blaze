use anyhow::Result;
use std::fmt;
use std::fmt::Formatter;

/// Error to indicate a failure during the execution of a command.
#[derive(Debug, Clone)]
pub struct CommandExecutionError {
    pub(crate) message: String,
}

impl fmt::Display for CommandExecutionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Trait that represents a generic command that can be executed.
pub trait Command {
    fn execute(&self) -> Result<()>;
}
