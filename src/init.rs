use crate::command::{Command, CommandExecutionError};
use crate::vfs::{vfs_already_initialized, vfs_init};

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self) -> Result<(), CommandExecutionError> {
        let curr_directory = std::env::current_dir().unwrap();

        if vfs_already_initialized() {
            println!(
                "blaze repository already initialized in {}/.blaze",
                curr_directory.display()
            );
            return Ok(());
        }

        vfs_init()?;
        println!(
            "blaze repository initialized in {}/.blaze",
            curr_directory.display()
        );
        Ok(())
    }
}
