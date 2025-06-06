mod lcs;
mod printer;
pub mod runner;

use crate::diff::lcs::longest_common_subsequence;
use crate::diff::DiffType::Insert;
use crate::diff::ProcessingState::{InDiff, InEqual};
use std::collections::HashMap;

#[derive(Clone)]
pub enum DiffType {
    Insert,
    Delete,
    Equal,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SegmentType {
    Diff,
    Equal,
}

#[derive(Clone)]
pub struct DiffLine {
    diff_type: DiffType,
    line_no: usize,
    line: String,
}

pub struct DiffSegment {
    segment_type: SegmentType,
    lines: Vec<DiffLine>,
}

pub struct FileDiff {
    segments: Vec<DiffSegment>,
}

#[derive(PartialEq, Eq)]
enum ProcessingState {
    None,
    InDiff,
    InEqual,
}

/// Compares the contents of two files line by line
///
/// # Arguments
///
/// * `first_lines` - the lines of the first file to compare
/// * `second_lines` - the lines of the second file to compare
///
/// # Returns
///
/// The diff between the files
pub fn diff(first_lines: Vec<String>, second_lines: Vec<String>) -> FileDiff {
    let mut unique: HashMap<String, usize> = HashMap::new();
    let mut counter: usize = 0;

    if first_lines.is_empty() && second_lines.is_empty() {
        return FileDiff { segments: vec![] };
    }

    for line in &first_lines {
        if !unique.contains_key(line) {
            unique.insert(line.clone(), counter);
            counter += 1;
        }
    }
    for line in &second_lines {
        if !unique.contains_key(line) {
            unique.insert(line.clone(), counter);
            counter += 1;
        }
    }

    let first_transformed = first_lines
        .iter()
        .map(|it| unique.get(it).unwrap())
        .collect();
    let second_transformed = second_lines
        .iter()
        .map(|it| unique.get(it).unwrap())
        .collect();

    let result = longest_common_subsequence(first_transformed, second_transformed);

    let mut i: usize = 0;
    let mut j: usize = 0;

    // 'first' and 'second' are identical -- so there's no diff
    if result.lcs.len() == first_lines.len() && result.lcs.len() == second_lines.len() {
        let lines = first_lines
            .iter()
            .enumerate()
            .map(|(idx, line)| DiffLine {
                diff_type: DiffType::Equal,
                line_no: idx + 1,
                line: line.clone(),
            })
            .collect();
        return FileDiff {
            segments: vec![DiffSegment {
                segment_type: SegmentType::Equal,
                lines,
            }],
        };
    }

    let mut segments: Vec<DiffSegment> = Vec::new();
    let mut lines: Vec<DiffLine> = Vec::new();

    let mut state = ProcessingState::None;

    for k in 0..result.lcs.len() {
        /*
         * The 'first' file is considered the original state, while the 'second'
         * file is the new state.
         *
         * We treat any line in the 'first' list that isn't a part of the LCS as a
         * removal and any line in the 'second' as an addition. We keep processing
         * elements in 'first' and 'second' in order until we reach an element of
         * the LCS -- which marks the end of the current DIFF segment and the start
         * of a new EQUAL segment.
         */
        let i_start = i;
        while i < result.first_pos[k] as usize {
            if state == ProcessingState::None {
                state = InDiff;
            } else if state == InEqual {
                // Collect the pending items into a segment
                state = collect(&mut lines, state, &mut segments);
            }

            lines.push(DiffLine {
                diff_type: DiffType::Delete,
                line_no: i + 1,
                line: first_lines[i].clone(),
            });
            i += 1;
        }

        while j < result.second_pos[k] as usize {
            if state == ProcessingState::None {
                state = InDiff;
            } else if state == InEqual {
                state = collect(&mut lines, state, &mut segments);
            }

            /*
             * we use 'i_start' here because the positions are anchored against the numbering of the
             * original file, and it would be the position prior to any deletions.
             */
            lines.push(DiffLine {
                diff_type: Insert,
                line_no: i_start + 1,
                line: second_lines[j].clone(),
            });
            j += 1;
        }

        if state == ProcessingState::None {
            state = InEqual;
        } else if state == InDiff {
            state = collect(&mut lines, state, &mut segments);
        }

        let line_no = result.first_pos[k];
        lines.push(DiffLine {
            diff_type: DiffType::Equal,
            line_no: line_no as usize + 1,
            line: first_lines[line_no as usize].clone(),
        });
        i += 1;
        j += 1;
    }

    /*
     * Collect the remaining lines to finish out the segment. Handle the edge case where there
     * are still lines after the last LCS element and collect those as well.
     */
    if state != ProcessingState::None {
        state = collect(&mut lines, state, &mut segments);
    } else {
        // This happens when we have two files that are completely different (empty LCS)
        state = InDiff;
    }

    let i_start = i;
    while i < first_lines.len() {
        lines.push(DiffLine {
            diff_type: DiffType::Delete,
            line_no: i + 1,
            line: first_lines[i].clone(),
        });
        i += 1;
    }

    while j < second_lines.len() {
        lines.push(DiffLine {
            diff_type: Insert,
            line_no: i_start + 1,
            line: second_lines[j].clone(),
        });
        j += 1;
    }

    collect(&mut lines, state, &mut segments);
    FileDiff { segments }
}

fn collect(
    lines: &mut Vec<DiffLine>,
    state: ProcessingState,
    segments: &mut Vec<DiffSegment>,
) -> ProcessingState {
    let segment_type = match state {
        ProcessingState::None => panic!("Cannot collect from NONE state"),
        InDiff => SegmentType::Diff,
        InEqual => SegmentType::Equal,
    };

    // Avoid adding an empty segment
    if !lines.is_empty() {
        segments.push(DiffSegment {
            segment_type,
            lines: lines.clone(),
        });
        lines.clear();
    }

    match state {
        ProcessingState::None => panic!("Cannot collect from NONE state"),
        InDiff => InEqual,
        InEqual => InDiff,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_files() {
        let first = vec!["line1".to_string(), "line2".to_string()];
        let second = first.clone();
        let result = diff(first, second);
        assert_eq!(result.segments.len(), 1);
        assert_eq!(result.segments[0].segment_type, SegmentType::Equal);
    }

    #[test]
    fn test_completely_different_files() {
        let first = vec!["line1".to_string()];
        let second = vec!["line2".to_string()];
        let result = diff(first, second);
        assert_eq!(result.segments.len(), 1);
        assert_eq!(result.segments[0].segment_type, SegmentType::Diff);
    }

    #[test]
    fn test_partial_diff() {
        let first = vec!["line1".to_string(), "line2".to_string()];
        let second = vec!["line1".to_string(), "line3".to_string()];
        let result = diff(first, second);
        assert_eq!(result.segments.len(), 2);
        assert_eq!(result.segments[0].segment_type, SegmentType::Equal);
        assert_eq!(result.segments[1].segment_type, SegmentType::Diff);
    }

    #[test]
    fn test_alternating_segments() {
        let first = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        let second = vec!["a".to_string(), "x".to_string(), "c".to_string(), "y".to_string()];
        let result = diff(first, second);
        assert_eq!(result.segments.len(), 4);
        assert_eq!(result.segments[0].segment_type, SegmentType::Equal);
        assert_eq!(result.segments[1].segment_type, SegmentType::Diff);
        assert_eq!(result.segments[2].segment_type, SegmentType::Equal);
        assert_eq!(result.segments[3].segment_type, SegmentType::Diff);
    }

    #[test]
    fn test_changes_at_start() {
        let first = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let second = vec!["x".to_string(), "y".to_string(), "c".to_string()];
        let result = diff(first, second);
        assert_eq!(result.segments.len(), 2);
        assert_eq!(result.segments[0].segment_type, SegmentType::Diff);
        assert_eq!(result.segments[1].segment_type, SegmentType::Equal);
    }

    #[test]
    fn test_changes_at_end() {
        let first = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let second = vec!["a".to_string(), "x".to_string(), "y".to_string()];
        let result = diff(first, second);
        assert_eq!(result.segments.len(), 2);
        assert_eq!(result.segments[0].segment_type, SegmentType::Equal);
        assert_eq!(result.segments[1].segment_type, SegmentType::Diff);
    }

    #[test]
    fn test_empty_files() {
        let first: Vec<String> = vec![];
        let second: Vec<String> = vec![];
        let result = diff(first, second);
        assert_eq!(result.segments.len(), 0);
    }
}
