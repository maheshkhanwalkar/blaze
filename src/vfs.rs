use crate::command::CommandExecutionError;
use crate::file::find_dir;
use std::fs;

pub fn vfs_already_initialized() -> bool {
    find_dir(&String::from(".blaze"), true)
}

pub fn vfs_init() -> Result<(), CommandExecutionError> {
    fs::create_dir(".blaze")
        .and_then(|()| fs::create_dir(".blaze/objects"))
        .and_then(|()| fs::create_dir(".blaze/commits"))
        .and_then(|()| fs::create_dir(".blaze/partitions"))
        .or_else(|_| {
            Err(CommandExecutionError {
                message: String::from("failed to create .blaze"),
            })
        })
}
