use crate::file::{contains_dir, find_dir, get_parent_dir};
use crate::vfs::Partition::Root;
use anyhow::{Context, Result};
use std::cmp::PartialEq;
use std::env::current_dir;
use std::fs;
use std::process::exit;

const BLAZE_REPOSITORY_DIR: &str = ".blaze";

#[derive(Debug, PartialEq)]
pub enum Partition {
    Root,
    Local,
}

pub fn vfs_already_initialized() -> bool {
    find_dir(&*current_dir().unwrap(), BLAZE_REPOSITORY_DIR, true).is_some()
}

pub fn vfs_set_cwd(partition: Partition) -> Result<()> {
    let cwd = current_dir()?;

    let Some(vfs_root) = find_dir(cwd.as_path(), BLAZE_REPOSITORY_DIR, true) else {
        eprintln!("not a blaze repository: {}", cwd.display());
        exit(1);
    };

    std::env::set_current_dir(vfs_root.as_path())?;
    let root_dir = format!("{BLAZE_REPOSITORY_DIR}/db/root");

    if partition != Root || contains_dir(vfs_root.as_path(), root_dir.as_str()) {
        return Ok(());
    }

    loop {
        let parent = get_parent_dir()?;
        let blaze_dir =
            find_dir(parent.as_path(), BLAZE_REPOSITORY_DIR, true).ok_or_else(|| {
                anyhow::anyhow!("could not find root repository, likely something is corrupted")
            })?;

        std::env::set_current_dir(blaze_dir.as_path())?;
        if contains_dir(blaze_dir.as_path(), root_dir.as_str()) {
            break Ok(());
        }
    }
}

pub fn vfs_init() -> Result<()> {
    fs::create_dir(BLAZE_REPOSITORY_DIR)
        .and_then(|()| fs::create_dir(fmt_sub("db")))
        .and_then(|()| fs::create_dir(fmt_sub("db/global")))
        .and_then(|()| fs::create_dir(fmt_sub("db/root")))
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
