use crate::command::{Command, CommandExecutionError};
use crate::diff::diff;
use crate::diff::printer::print_diff;
use crate::file::read_lines;

pub struct DiffCommand {
    pub args: Vec<String>,
}

impl Command for DiffCommand {
    fn execute(&self) -> Result<(), CommandExecutionError> {
        if self.args.len() != 2 {
            return Err(CommandExecutionError {
                message: format!(
                    "diff: insufficient arguments specified, expected 2, got {len}",
                    len = self.args.len()
                ),
            });
        }

        let first = &self.args[0];
        let second = &self.args[1];

        let first_lines = match read_lines(first) {
            Ok(lines) => lines,
            Err(err) => return Err(err),
        };
        let second_lines = match read_lines(second) {
            Ok(lines) => lines,
            Err(err) => return Err(err),
        };

        let file_diff = diff(first_lines, second_lines);
        print_diff(file_diff);
        Ok(())
    }
}
