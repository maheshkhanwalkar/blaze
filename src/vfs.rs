mod commit;

use crate::command::CommandExecutionError;
use crate::file::find_dir;
use std::fs;

const BLAZE_REPOSITORY_DIR: &str = ".blaze";

pub fn vfs_already_initialized() -> bool {
    find_dir(BLAZE_REPOSITORY_DIR, true)
}

pub fn vfs_init() -> Result<(), CommandExecutionError> {
    fs::create_dir(BLAZE_REPOSITORY_DIR)
        .and_then(|()| fs::create_dir(fmt_sub("atoms")))
        .and_then(|()| fs::create_dir(fmt_sub("commits")))
        .and_then(|()| fs::create_dir(fmt_sub("partitions")))
        .or_else(|_| {
            Err(CommandExecutionError {
                message: String::from("failed to create .blaze"),
            })
        })
}

fn fmt_sub(sub_dir: &str) -> String {
    format!("{}/{}", BLAZE_REPOSITORY_DIR, sub_dir)
}

fn fmt_file(sub_dir: &str, file_name: &str) -> String {
    format!("{}/{}/{}", BLAZE_REPOSITORY_DIR, sub_dir, file_name)
}
