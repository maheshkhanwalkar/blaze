use crate::command::{Command, CommandExecutionError};
use crate::diff::diff;
use crate::diff::printer::print_diff;
use crate::file::read_lines;

pub struct DiffCommand {
    pub first: String,
    pub second: String,
}

impl Command for DiffCommand {
    fn execute(&self) -> Result<(), CommandExecutionError> {
        let first_lines = match read_lines(&self.first) {
            Ok(lines) => lines,
            Err(err) => return Err(err),
        };
        let second_lines = match read_lines(&self.second) {
            Ok(lines) => lines,
            Err(err) => return Err(err),
        };

        let file_diff = diff(&first_lines, &second_lines);
        print_diff(file_diff);
        Ok(())
    }
}
