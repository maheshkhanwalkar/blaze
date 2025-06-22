use anyhow::Result;

/// Trait that represents a generic command that can be executed.
pub trait Command {
    /// Executes the command.
    fn execute(&self) -> Result<()>;
    /// Set the required VFS root. Some commands need to be anchored against a
    /// repository, partition, or can be run from anywhere.
    fn set_vfs_root(&self) -> Result<()>;
}
