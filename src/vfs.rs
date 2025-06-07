use crate::command::CommandExecutionError;
use std::fs;

pub fn vfs_already_initialized() -> bool {
    // FIXME: it's actually more complicated than this -- since we can execute this
    //  command in any directory, we would need to recurse upwards
    fs::metadata(".blaze").is_ok()
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
