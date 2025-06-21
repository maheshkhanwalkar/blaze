use crate::file::find_dir;
use anyhow::{Context, Result};
use std::fs;
use std::process::exit;

const BLAZE_REPOSITORY_DIR: &str = ".blaze";

pub fn vfs_already_initialized() -> bool {
    find_dir(BLAZE_REPOSITORY_DIR, true).is_some()
}

pub fn vfs_set_cwd() -> Result<()> {
    let Some(vfs_root) = find_dir(BLAZE_REPOSITORY_DIR, true) else {
        let cwd = std::env::current_dir()?;
        eprintln!("not a blaze repository: {}", cwd.display());
        exit(1);
    };

    std::env::set_current_dir(vfs_root)?;
    Ok(())
}

pub fn vfs_init() -> Result<()> {
    fs::create_dir(BLAZE_REPOSITORY_DIR)
        .and_then(|()| fs::create_dir(fmt_sub("db")))
        .and_then(|()| fs::create_dir(fmt_sub("db/global")))
        .with_context(|| "failed to create .blaze")
}

pub fn vfs_create_partition(path: &str) -> Result<()> {
    fs::create_dir(format!("{}/{}", path, BLAZE_REPOSITORY_DIR))
        .and_then(|()| fs::create_dir(fmt_partition_sub(path, "db")))
        .with_context(|| format!("failed to create partition: {}", path))
}

fn fmt_sub(sub_dir: &str) -> String {
    format!("{}/{}", BLAZE_REPOSITORY_DIR, sub_dir)
}

fn fmt_partition_sub(dir: &str, sub_dir: &str) -> String {
    format!("{}/{}/{}", dir, BLAZE_REPOSITORY_DIR, sub_dir)
}
