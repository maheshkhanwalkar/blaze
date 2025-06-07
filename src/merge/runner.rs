use crate::command::{Command, CommandExecutionError};
use crate::file::read_lines;
use crate::merge::{merge, MergeSegmentType};

pub struct MergeCommand {
    pub args: Vec<String>,
}

impl Command for MergeCommand {
    fn execute(&self) -> Result<(), CommandExecutionError> {
        if self.args.len() != 3 {
            return Err(CommandExecutionError {
                message: format!(
                    "merge: insufficient arguments specified, expected 3, got {size}",
                    size = self.args.len()
                ),
            });
        }

        /*
         * original => the common ancestor of v1 and v2
         * v1 => modification of original
         * v2 => modification of original
         *
         * objective: merge v1, v2 using the original as a guide (aka 3-way merge)
         */
        let original = &self.args[0];
        let v1 = &self.args[1];
        let v2 = &self.args[2];

        let original_lines = match read_lines(original) {
            Ok(lines) => lines,
            Err(e) => return Err(e),
        };
        let v1_lines = match read_lines(v1) {
            Ok(lines) => lines,
            Err(e) => return Err(e),
        };
        let v2_lines = match read_lines(v2) {
            Ok(lines) => lines,
            Err(e) => return Err(e),
        };

        let result = merge(&original_lines, &v1_lines, &v2_lines);

        for segment in result.segments {
            match segment.segment_type {
                MergeSegmentType::Lines => {
                    println!("{}", segment.lines.join("\n"));
                }
                MergeSegmentType::Conflict => {
                    println!("--- @blaze:mg_conflict:v1 ---");
                    println!("{}", segment.v1_changes.join("\n"));
                    println!("--- @blaze:mg_conflict:v2 ---");
                    println!("{}", segment.v2_changes.join("\n"));
                    println!("--- @blaze:mg_conflict:end ---");
                }
            }
        }

        Ok(())
    }
}
