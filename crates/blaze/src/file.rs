use crate::command::CommandExecutionError;
use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::read_to_string;
use std::path::PathBuf;

pub fn read_lines(path: &String) -> Result<Vec<String>, CommandExecutionError> {
    let file = File::open(path);

    if file.is_err() {
        return Err(CommandExecutionError {
            message: format!("file not found {path}"),
        });
    }

    let lines: Vec<String> = read_to_string(file.unwrap())
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    Ok(lines)
}

/// Find a directory in the current directory or by traversing up the directory tree.
pub fn find_dir(name: &str, traverse: bool) -> Option<String> {
    let mut current_dir = if let Ok(dir) = current_dir() {
        dir
    } else {
        return None;
    };

    while !current_dir.join(name).exists() {
        if !traverse {
            return None;
        }

        current_dir = if let Some(parent) = current_dir.parent() {
            PathBuf::from(parent)
        } else {
            return None;
        };
    }

    Some(current_dir.display().to_string())
}

/// Check if a directory is empty.
pub fn is_dir_empty(name: &str) -> Result<bool, CommandExecutionError> {
    let Ok(dir_itr) = fs::read_dir(name) else {
        return Err(CommandExecutionError {
            message: format!("could not open {name}"),
        });
    };

    Ok(dir_itr.peekable().peek().is_none())
}
