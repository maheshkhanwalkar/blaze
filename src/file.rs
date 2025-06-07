use crate::command::CommandExecutionError;
use std::fs::File;
use std::io::read_to_string;

pub fn read_lines(path: &String) -> Result<Vec<String>, CommandExecutionError> {
    let file = File::open(path);

    if file.is_err() {
        return Err(CommandExecutionError {
            message: format!("file not found {path}").to_string(),
        });
    }

    let lines: Vec<String> = read_to_string(file.unwrap())
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    Ok(lines)
}
