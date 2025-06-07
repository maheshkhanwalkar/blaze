use crate::command::CommandExecutionError;
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

pub fn vfs_already_initialized() -> bool {
    let mut current_dir = if let Ok(dir) = current_dir() {
        dir
    } else {
        return false;
    };

    while !current_dir.join(".blaze").exists() {
        current_dir = if let Some(parent) = current_dir.parent() {
            PathBuf::from(parent)
        } else {
            return false;
        };
    }

    true
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
