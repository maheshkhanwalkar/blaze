use crate::command::Command;
use crate::file::read_lines;
use anyhow::Result;
use catalyst::merge::{merge, MergeSegment};

pub struct MergeCommand {
    pub original: String,
    pub v1: String,
    pub v2: String,
}

impl Command for MergeCommand {
    fn execute(&self) -> Result<()> {
        /*
         * original => the common ancestor of v1 and v2
         * v1 => modification of original
         * v2 => modification of original
         *
         * objective: merge v1, v2 using the original as a guide (aka 3-way merge)
         */
        let original_lines = read_lines(&self.original)?;
        let v1_lines = read_lines(&self.v1)?;
        let v2_lines = read_lines(&self.v2)?;

        let result = merge(&original_lines, &v1_lines, &v2_lines);

        for segment in result.segments {
            match segment {
                MergeSegment::Lines { lines } => {
                    println!("{}", lines.join("\n"));
                }
                MergeSegment::Conflict {
                    v1_changes,
                    v2_changes,
                } => {
                    println!("--- @blaze:mg_conflict:v1 ---");
                    println!("{}", v1_changes.join("\n"));
                    println!("--- @blaze:mg_conflict:v2 ---");
                    println!("{}", v2_changes.join("\n"));
                    println!("--- @blaze:mg_conflict:end ---");
                }
            }
        }

        Ok(())
    }
}
