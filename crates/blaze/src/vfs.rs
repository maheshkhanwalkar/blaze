use crate::command::CommandExecutionError;
use crate::file::find_dir;
use std::fs;
use std::process::exit;

const BLAZE_REPOSITORY_DIR: &str = ".blaze";

pub fn vfs_already_initialized() -> bool {
    find_dir(BLAZE_REPOSITORY_DIR, true).is_some()
}

pub fn vfs_set_cwd() {
    let Some(vfs_root) = find_dir(BLAZE_REPOSITORY_DIR, true) else {
        let cwd = std::env::current_dir().unwrap();
        eprintln!("not a blaze repository: {}", cwd.display());
        exit(1);
    };

    std::env::set_current_dir(vfs_root).unwrap();
}

pub fn vfs_init() -> Result<(), CommandExecutionError> {
    fs::create_dir(BLAZE_REPOSITORY_DIR)
        .and_then(|()| fs::create_dir(fmt_sub("db")))
        .and_then(|()| fs::create_dir(fmt_sub("db/global")))
        .and_then(|()| fs::create_dir(fmt_sub("db/partitions")))
        .or_else(|_| {
            Err(CommandExecutionError {
                message: String::from("failed to create .blaze"),
            })
        })
}

fn fmt_sub(sub_dir: &str) -> String {
    format!("{}/{}", BLAZE_REPOSITORY_DIR, sub_dir)
}
