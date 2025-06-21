use anyhow::Result;

/// Trait that represents a generic command that can be executed.
pub trait Command {
    fn execute(&self) -> Result<()>;
}
