use crate::command::CommandExecutionError;
use crate::file::find_dir;
use std::fs;

const BLAZE_REPOSITORY_DIR: &str = ".blaze";

pub fn vfs_already_initialized() -> bool {
    find_dir(BLAZE_REPOSITORY_DIR, true)
}

pub fn vfs_init() -> Result<(), CommandExecutionError> {
    fs::create_dir(BLAZE_REPOSITORY_DIR)
        .and_then(|()| fs::create_dir(fmt("objects")))
        .and_then(|()| fs::create_dir(fmt("commits")))
        .and_then(|()| fs::create_dir(fmt("partitions")))
        .or_else(|_| {
            Err(CommandExecutionError {
                message: String::from("failed to create .blaze"),
            })
        })
}

fn fmt(sub_dir: &str) -> String {
    format!("{}/{}", BLAZE_REPOSITORY_DIR, sub_dir)
}
