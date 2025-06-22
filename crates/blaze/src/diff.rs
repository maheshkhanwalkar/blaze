use crate::command::Command;
use crate::file::read_lines;
use crate::vfs::{vfs_set_cwd, RootType};
use anyhow::Result;
use catalyst::diff::SegmentType::{Diff, Equal};
use catalyst::diff::{diff, DiffLine, DiffSegment, DiffType, FileDiff};
use std::cmp::{max, min};
use std::collections::HashSet;

pub struct DiffCommand {
    pub first: String,
    pub second: String,
}

impl Command for DiffCommand {
    fn execute(&self) -> Result<()> {
        let first_lines = read_lines(&self.first)?;
        let second_lines = read_lines(&self.second)?;
        let file_diff = diff(&first_lines, &second_lines);
        print_diff(file_diff);
        Ok(())
    }

    fn set_vfs_root(&self) -> Result<()> {
        vfs_set_cwd(RootType::None)
    }
}

// ASCII escape codes for diff color
const RED_COLOR: &str = "\u{001B}[31m";
const BLUE_COLOR: &str = "\u{001B}[34m";
const GREEN_COLOR: &str = "\u{001B}[32m";
const RESET_COLOR: &str = "\u{001B}[0m";

pub fn print_diff(file_diff: FileDiff) {
    // No diff
    if file_diff.segments.len() == 1 && file_diff.segments[0].segment_type == Equal {
        return;
    }

    let mut processed: HashSet<usize> = HashSet::new();

    for (i, segment) in file_diff.segments.iter().enumerate() {
        if segment.segment_type == Diff {
            let prev_segment = get(&file_diff.segments, i as i64 - 1);

            if prev_segment.is_some() && prev_segment.unwrap().segment_type == Equal {
                let equal_lines = safe_sub_list(
                    &prev_segment.unwrap().lines,
                    prev_segment.unwrap().lines.len(),
                    -4,
                );
                print_equal_lines(&equal_lines, &mut processed);
            }

            println!(
                "{BLUE_COLOR}@ line_no:{line_no}{RESET_COLOR}",
                line_no = segment.lines[0].line_no
            );

            for line in &segment.lines {
                match line.diff_type {
                    DiffType::Insert => println!("{GREEN_COLOR}+ {l}{RESET_COLOR}", l = line.line),
                    DiffType::Delete => println!("{RED_COLOR}- {l}{RESET_COLOR}", l = line.line),
                    DiffType::Equal => {}
                }
            }
        } else {
            let prev_segment = get(&file_diff.segments, i as i64 - 1);

            if prev_segment.is_some() && prev_segment.unwrap().segment_type == Diff {
                let equal_lines = safe_sub_list(&segment.lines, 0, 4);
                print_equal_lines(&equal_lines, &mut processed);
            }
        }
    }
}

fn print_equal_lines(equal_lines: &Vec<DiffLine>, processed: &mut HashSet<usize>) {
    /*
     * There's an edge case where the same equal (non-diff) line could be printed multiple times,
     * so this logic below tracks if we've ever seen a line before and filters it out.
     *
     * Edge case:
     *    + added line
     *    <same1>
     *    <same2>
     *    + another added line
     *
     * In the case above, <same1> and <same2> are both a predecessor and a successor of a diff line,
     * so they would end up being included within the context window twice and hence get printed twice.
     * Therefore, we have the filtration here to prevent this scenario.
     */
    let n_processed: Vec<usize> = equal_lines
        .iter()
        .filter(|it| !processed.contains(&it.line_no))
        .map(|it| {
            println!("  {line}", line = it.line);
            it.line_no
        })
        .collect();

    for line_no in n_processed {
        processed.insert(line_no);
    }
}

fn safe_sub_list(list: &Vec<DiffLine>, anchor: usize, line_count: i64) -> Vec<DiffLine> {
    let from_index: i64 = if line_count < 0 {
        anchor as i64 + line_count
    } else {
        anchor as i64
    };

    let to_index: i64 = if line_count < 0 {
        anchor as i64
    } else {
        anchor as i64 + line_count
    };

    list[max(0, from_index) as usize..min(list.len(), to_index as usize)].to_vec()
}

fn get(segments: &Vec<DiffSegment>, index: i64) -> Option<&DiffSegment> {
    if index < 0 {
        None
    } else {
        segments.get(index as usize)
    }
}
