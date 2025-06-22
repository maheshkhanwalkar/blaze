use crate::command::Command;
use crate::vfs::{vfs_already_initialized, vfs_init, vfs_set_cwd, RootType};
use anyhow::Result;

pub struct InitCommand;

impl Command for InitCommand {
    fn execute(&self) -> Result<()> {
        let curr_directory = std::env::current_dir()?;

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

    fn set_vfs_root(&self) -> Result<()> {
        vfs_set_cwd(RootType::None)
    }
}
