use crate::file::{contains_dir, find_dir, get_parent_dir};
use crate::vfs::RootType::Partition;
use anyhow::{Context, Result};
use std::cmp::PartialEq;
use std::env::current_dir;
use std::fs;
use std::process::exit;

const BLAZE_REPOSITORY_DIR: &str = ".blaze";

#[derive(Debug, PartialEq)]
pub enum RootType {
    Repository,
    Partition,
    None,
}

/// Returns true if the VFS has already been initialized.
pub fn vfs_already_initialized() -> bool {
    find_dir(current_dir().unwrap().as_path(), BLAZE_REPOSITORY_DIR, true).is_some()
}

/// Sets the current working directory to the appropriate root directory based on the
/// provided `RootType`.
///
/// # Arguments
///
/// * `root_type` - An enum of type `RootType` that indicates whether to locate the root
///   repository directory or the nearest (by walking up the directory tree) partition.
///
/// # Returns
///
/// Returns a `Result` wrapped in `()` to indicate success. If the repository root cannot
/// be found, or any underlying OS-related operations (e.g., `set_current_dir`) fail,
/// the function will return an error.
pub fn vfs_set_cwd(root_type: RootType) -> Result<()> {
    if root_type == RootType::None {
        return Ok(());
    }

    let cwd = current_dir()?;

    let Some(vfs_root) = find_dir(cwd.as_path(), BLAZE_REPOSITORY_DIR, true) else {
        eprintln!("not a blaze repository: {}", cwd.display());
        exit(1);
    };

    std::env::set_current_dir(vfs_root.as_path())?;
    let root_dir = format!("{BLAZE_REPOSITORY_DIR}/db/root");

    if root_type == Partition || contains_dir(vfs_root.as_path(), root_dir.as_str()) {
        return Ok(());
    }

    /*
     * We are currently in a child partition, so we need to go up to the root partition.
     * It's possible for child partitions to be nested, so we need to loop until we find the root
     * or hit an error.
     */
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

/// Initializes the VFS, creating the necessary metadata structure.
pub fn vfs_init() -> Result<()> {
    fs::create_dir(BLAZE_REPOSITORY_DIR)
        .and_then(|()| fs::create_dir(fmt_sub("db")))
        .and_then(|()| fs::create_dir(fmt_sub("db/global")))
        .and_then(|()| fs::create_dir(fmt_sub("db/root")))
        .with_context(|| "failed to create .blaze")
}

/// Creates a new partition in the VFS.
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
