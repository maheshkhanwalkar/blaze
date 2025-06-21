use anyhow::{Context, Result};
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::read_to_string;
use std::path::{Path, PathBuf};

pub fn read_lines(path: &String) -> Result<Vec<String>> {
    let file = File::open(path).with_context(|| format!("file not found {path}"))?;
    let lines: Vec<String> = read_to_string(file)?.lines().map(String::from).collect();
    Ok(lines)
}

/// Find a directory in the current directory or by traversing up the directory tree.
pub fn find_dir(start_path: &Path, name: &str, traverse: bool) -> Option<PathBuf> {
    let mut current_dir = start_path;

    while !current_dir.join(name).exists() {
        if !traverse {
            return None;
        }

        current_dir = if let Some(parent) = current_dir.parent() {
            parent
        } else {
            return None;
        };
    }

    Some(PathBuf::from(current_dir))
}

/// Check if a directory is empty.
pub fn is_dir_empty(name: &str) -> Result<bool> {
    let dir_itr = fs::read_dir(name).with_context(|| format!("could not open {name}"))?;
    Ok(dir_itr.peekable().peek().is_none())
}

pub fn contains_dir(start_path: &Path, name: &str) -> bool {
    let path = start_path.join(name);
    path.exists() && path.is_dir()
}

pub fn get_parent_dir() -> Result<PathBuf> {
    let curr = current_dir()?;
    let parent = curr
        .parent()
        .ok_or_else(|| anyhow::anyhow!("could not get parent dir"))?;
    Ok(PathBuf::from(parent))
}
