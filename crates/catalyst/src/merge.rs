use crate::diff::{DiffLine, DiffType, FileDiff, SegmentType, diff};
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum MergeSegment {
    Conflict {
        v1_changes: Vec<String>,
        v2_changes: Vec<String>,
    },
    Lines {
        lines: Vec<String>,
    },
}

pub struct MergeResult {
    pub segments: Vec<MergeSegment>,
}

#[derive(Clone, PartialEq)]
enum LineModificationType {
    /// Inserting a net-new line into the original file.
    Insert,

    /// Removing a line from the original file.
    Delete,

    /// Replacing a line from the original file with a new line.
    /// This is a special case of INSERT followed by DELETE.
    Replace,
}

#[derive(Clone)]
struct LineModification {
    mod_type: LineModificationType,
    line_no: usize,
    line: String,
}

/// Merges changes from two sets of lines (`v1_lines` and `v2_lines`) based on an original set of lines (`original_lines`).
/// Identifies conflicts where modifications have been made to the same line across both sets.
///
/// # Arguments
///
/// * `original_lines` - The list of original lines that serve as the base for comparison
/// * `v1_lines` - The list of lines representing the first modified version
/// * `v2_lines` - The list of lines representing the second modified version
///
/// # Returns
///
/// A `MergeResult` instance containing the merged lines with any conflicts recorded
pub fn merge(
    original_lines: &Vec<String>,
    v1_lines: &Vec<String>,
    v2_lines: &Vec<String>,
) -> MergeResult {
    let v1_diff = diff(original_lines, v1_lines);
    let v2_diff = diff(original_lines, v2_lines);

    let v1_categories = categorise(&v1_diff);
    let v2_categories = categorise(&v2_diff);

    let mut segments: Vec<MergeSegment> = Vec::new();
    let mut curr_lines: Vec<String> = Vec::new();

    for line_no in 0..=original_lines.len() {
        let v1_lines = v1_categories.get(&(line_no + 1));
        let v2_lines = v2_categories.get(&(line_no + 1));

        if v1_lines.is_some() && v2_lines.is_some() {
            let v1_types: Vec<LineModificationType> = v1_lines
                .unwrap()
                .iter()
                .map(|l| l.mod_type.clone())
                .collect();
            let v2_types: Vec<LineModificationType> = v2_lines
                .unwrap()
                .iter()
                .map(|l| l.mod_type.clone())
                .collect();

            /*
             * Conflict conditions.
             *
             * Inserts conflict with each other, due to a lack of a good reconciliation strategy.
             * It is definitely possible to auto-resolve via some combination, e.g. accept both and
             * just concatenate, but that likely causes more problems than it solves.
             *
             * Replaces directly conflict each other, as they replace the same line in the original
             * to two different lines.
             *
             */
            if (v1_types.contains(&LineModificationType::Insert)
                && v2_types.contains(&LineModificationType::Insert))
                || (v1_types.contains(&LineModificationType::Replace)
                    && v2_types.contains(&LineModificationType::Replace))
            {
                collect(&mut curr_lines, &mut segments);

                let no_replace = !v1_lines
                    .unwrap()
                    .iter()
                    .any(|l| matches!(l.mod_type, LineModificationType::Replace))
                    && !v2_lines
                        .unwrap()
                        .iter()
                        .any(|l| matches!(l.mod_type, LineModificationType::Replace));

                let v1_filtered: Vec<String> = v1_lines
                    .unwrap()
                    .iter()
                    .filter(|l| {
                        matches!(
                            l.mod_type,
                            LineModificationType::Insert | LineModificationType::Replace
                        )
                    })
                    .map(|l| l.line.clone())
                    .collect();

                let v2_filtered: Vec<String> = v2_lines
                    .unwrap()
                    .iter()
                    .filter(|l| {
                        matches!(
                            l.mod_type,
                            LineModificationType::Insert | LineModificationType::Replace
                        )
                    })
                    .map(|l| l.line.clone())
                    .collect();

                segments.push(MergeSegment::Conflict {
                    v1_changes: v1_filtered,
                    v2_changes: v2_filtered,
                });

                if no_replace {
                    add_original_line(line_no, &mut curr_lines, original_lines);
                }
            } else {
                /*
                 * It cannot be the case that both v1Lines and v2Lines contain the same type, as that
                 * would be a merge conflict -- but the code below is written like this, so we
                 * don't need to do a bunch of if-else(s) based on whether the INSERT or REPLACE is
                 * in v1Lines or v2Lines.
                 */
                let mut inserts: Vec<&LineModification> = v1_lines
                    .unwrap()
                    .iter()
                    .filter(|l| matches!(l.mod_type, LineModificationType::Insert))
                    .collect();
                inserts.extend(
                    v2_lines
                        .unwrap()
                        .iter()
                        .filter(|l| matches!(l.mod_type, LineModificationType::Insert)),
                );

                let mut replacements: Vec<&LineModification> = v1_lines
                    .unwrap()
                    .iter()
                    .filter(|l| matches!(l.mod_type, LineModificationType::Replace))
                    .collect();
                replacements.extend(
                    v2_lines
                        .unwrap()
                        .iter()
                        .filter(|l| matches!(l.mod_type, LineModificationType::Replace)),
                );

                /*
                 * Handle REPLACE first, then INSERT to preserve correct ordering -- except for
                 * the first line where we need to do the INSERT(s) first, since they are actually
                 * inserting *before* the first line.
                 */
                if line_no == 0 {
                    for insert in inserts {
                        curr_lines.push(insert.line.clone());
                    }
                    for replacement in replacements {
                        curr_lines.push(replacement.line.clone());
                    }
                } else {
                    for replacement in replacements {
                        curr_lines.push(replacement.line.clone());
                    }
                    for insert in inserts {
                        curr_lines.push(insert.line.clone());
                    }
                }
            }
        } else if v1_lines.is_some() || v2_lines.is_some() {
            let lines = v1_lines.or(v2_lines).unwrap();
            append(&mut curr_lines, lines);

            /*
             * When handling an INSERT line, the original line should still be
             * retained as well.
             */
            if !lines.is_empty() && matches!(lines[0].mod_type, LineModificationType::Insert) {
                add_original_line(line_no, &mut curr_lines, original_lines);
            }
        } else {
            add_original_line(line_no, &mut curr_lines, original_lines);
        }
    }

    collect(&mut curr_lines, &mut segments);
    MergeResult { segments }
}

fn categorise(diff: &FileDiff) -> HashMap<usize, Vec<LineModification>> {
    let mut result = HashMap::new();

    for seg in &diff.segments {
        if seg.segment_type == SegmentType::Equal {
            continue;
        }

        // All lines in this diff are INSERT(s)
        if !seg
            .lines
            .iter()
            .any(|line| line.diff_type == DiffType::Delete)
        {
            for line in &seg.lines {
                add_to_map(
                    &mut result,
                    line.line_no,
                    LineModification {
                        mod_type: LineModificationType::Insert,
                        line_no: line.line_no,
                        line: line.line.clone(),
                    },
                );
            }
            continue;
        }

        // All lines in this diff are DELETE(s)
        if !seg
            .lines
            .iter()
            .any(|line| line.diff_type == DiffType::Insert)
        {
            for line in &seg.lines {
                add_to_map(
                    &mut result,
                    line.line_no,
                    LineModification {
                        mod_type: LineModificationType::Delete,
                        line_no: line.line_no,
                        line: line.line.clone(),
                    },
                );
            }
            continue;
        }

        /*
         * Now comes the complicated part -- we've got both INSERT and DELETE types in this diff
         * segment, so that means that some of these will coalesce into a REPLACE type.
         *
         * We need to pair up the DELETE and INSERT lines to form a REPLACE line, and anything that
         * remains unpaired will be treated as a separate INSERT or DELETE line.
         */
        let boundary = find_insert_position(&seg.lines);
        let mut del_pos = 0;
        let mut ins_pos = boundary;

        while del_pos < boundary && ins_pos < seg.lines.len() {
            let line_no = seg.lines[del_pos].line_no;
            let line = seg.lines[ins_pos].line.clone();

            add_to_map(
                &mut result,
                line_no,
                LineModification {
                    mod_type: LineModificationType::Replace,
                    line_no,
                    line,
                },
            );
            del_pos += 1;
            ins_pos += 1;
        }

        while ins_pos < seg.lines.len() {
            let diff_line = &seg.lines[ins_pos];
            add_to_map(
                &mut result,
                diff_line.line_no,
                LineModification {
                    mod_type: LineModificationType::Insert,
                    line_no: diff_line.line_no,
                    line: diff_line.line.clone(),
                },
            );
            ins_pos += 1;
        }

        while del_pos < boundary {
            let diff_line = &seg.lines[del_pos];
            add_to_map(
                &mut result,
                diff_line.line_no,
                LineModification {
                    mod_type: LineModificationType::Delete,
                    line_no: diff_line.line_no,
                    line: diff_line.line.clone(),
                },
            );
            del_pos += 1;
        }
    }

    result
}

fn find_insert_position(lines: &Vec<DiffLine>) -> usize {
    let mut i = 0;

    while i < lines.len() && lines[i].diff_type != DiffType::Insert {
        i += 1;
    }

    i
}

fn add_to_map(
    map: &mut HashMap<usize, Vec<LineModification>>,
    line_no: usize,
    line: LineModification,
) {
    map.entry(line_no)
        .and_modify(|v| v.push(line.clone()))
        .or_insert_with(|| vec![line]);
}

fn append(curr_lines: &mut Vec<String>, lines: &Vec<LineModification>) {
    lines
        .iter()
        .filter(|m| {
            matches!(
                m.mod_type,
                LineModificationType::Insert | LineModificationType::Replace
            )
        })
        .for_each(|m| curr_lines.push(m.line.clone()));
}

fn add_original_line(line_no: usize, curr_lines: &mut Vec<String>, original_lines: &Vec<String>) {
    if line_no < original_lines.len() {
        curr_lines.push(original_lines[line_no].clone());
    }
}

fn collect(curr_lines: &mut Vec<String>, segments: &mut Vec<MergeSegment>) {
    if !curr_lines.is_empty() {
        segments.push(MergeSegment::Lines {
            lines: curr_lines.clone(),
        });
        curr_lines.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Lines;

    #[test]
    fn test_merge_with_no_differences() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v1 = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v2 = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        assert_eq!(1, result.segments.len());
        let segment = &result.segments[0];

        match segment {
            MergeSegment::Lines { lines } => {
                assert_eq!(original, *lines);
            }
            _ => panic!("Expected MergeSegment::Lines"),
        }
    }

    #[test]
    fn test_merge_with_non_conflicting_changes() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v1 = vec![
            "line1".to_string(),
            "v1-line2".to_string(),
            "line3".to_string(),
        ];
        let v2 = vec![
            "line1".to_string(),
            "line2".to_string(),
            "v2-line3".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        assert_eq!(1, result.segments.len());
        let segment = &result.segments[0];
        match segment {
            MergeSegment::Lines { lines } => {
                assert_eq!(
                    vec![
                        "line1".to_string(),
                        "v1-line2".to_string(),
                        "v2-line3".to_string()
                    ],
                    *lines
                );
            }
            _ => panic!("Expected MergeSegment::Lines"),
        }
    }

    #[test]
    fn test_merge_with_conflicting_changes() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v1 = vec![
            "line1".to_string(),
            "v1-line2".to_string(),
            "line3".to_string(),
        ];
        let v2 = vec![
            "line1".to_string(),
            "v2-line2".to_string(),
            "line3".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        assert_eq!(3, result.segments.len());
        match &result.segments[0] {
            MergeSegment::Lines { .. } => (),
            _ => panic!("Expected MergeSegment::Lines"),
        }
        match &result.segments[1] {
            MergeSegment::Conflict {
                v1_changes,
                v2_changes,
            } => {
                assert_eq!(vec!["v1-line2".to_string()], *v1_changes);
                assert_eq!(vec!["v2-line2".to_string()], *v2_changes);
            }
            _ => panic!("Expected MergeSegment::Conflict"),
        }
        match &result.segments[2] {
            MergeSegment::Lines { .. } => (),
            _ => panic!("Expected MergeSegment::Lines"),
        }
    }

    #[test]
    fn test_merge_with_multiple_conflicts() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v1 = vec![
            "line1".to_string(),
            "v1-line2".to_string(),
            "v1-line3".to_string(),
        ];
        let v2 = vec![
            "line1".to_string(),
            "v2-line2".to_string(),
            "v2-line3".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        assert_eq!(3, result.segments.len());
        match &result.segments[0] {
            MergeSegment::Lines { .. } => (),
            _ => panic!("Expected MergeSegment::Lines"),
        }
        match &result.segments[1] {
            MergeSegment::Conflict {
                v1_changes,
                v2_changes,
            } => {
                assert_eq!(vec!["v1-line2".to_string()], *v1_changes);
                assert_eq!(vec!["v2-line2".to_string()], *v2_changes);
            }
            _ => panic!("Expected MergeSegment::Conflict"),
        }
        match &result.segments[2] {
            MergeSegment::Conflict {
                v1_changes,
                v2_changes,
            } => {
                assert_eq!(vec!["v1-line3".to_string()], *v1_changes);
                assert_eq!(vec!["v2-line3".to_string()], *v2_changes);
            }
            _ => panic!("Expected MergeSegment::Conflict"),
        }
    }

    #[test]
    fn test_merge_with_insertions_in_both_versions() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v1 = vec![
            "line1".to_string(),
            "line2".to_string(),
            "v1-line".to_string(),
            "line3".to_string(),
        ];
        let v2 = vec![
            "line1".to_string(),
            "v2-line".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        assert_eq!(1, result.segments.len());
        match &result.segments[0] {
            MergeSegment::Lines { lines } => {
                assert_eq!(
                    vec![
                        "line1".to_string(),
                        "v2-line".to_string(),
                        "line2".to_string(),
                        "v1-line".to_string(),
                        "line3".to_string()
                    ],
                    *lines
                );
            }
            _ => panic!("Expected MergeSegment::Lines"),
        }
    }

    #[test]
    fn test_merge_with_deletions_in_both_versions() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
            "line4".to_string(),
        ];
        let v1 = vec![
            "line1".to_string(),
            "line3".to_string(),
            "line4".to_string(),
        ];
        let v2 = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line4".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        assert_eq!(1, result.segments.len());
        match &result.segments[0] {
            MergeSegment::Lines { lines } => {
                assert_eq!(vec!["line1".to_string(), "line4".to_string()], *lines);
            }
            _ => panic!("Expected MergeSegment::Lines"),
        }
    }

    #[test]
    fn test_merge_with_interleaved_changes() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
            "line4".to_string(),
        ];
        let v1 = vec![
            "line1".to_string(),
            "v1-line2".to_string(),
            "line3".to_string(),
            "v1-line4".to_string(),
        ];
        let v2 = vec![
            "v2-line1".to_string(),
            "line2".to_string(),
            "v2-line3".to_string(),
            "line4".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        match &result.segments[0] {
            MergeSegment::Lines { lines } => {
                assert_eq!(
                    vec![
                        "v2-line1".to_string(),
                        "v1-line2".to_string(),
                        "v2-line3".to_string(),
                        "v1-line4".to_string()
                    ],
                    *lines
                );
            }
            _ => panic!("Expected MergeSegment::Lines"),
        }
    }

    #[test]
    fn test_merge_with_prepended_changes_and_replacement() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v1 = vec![
            "v1-line1".to_string(),
            "v1-line1-2".to_string(),
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v2 = vec![
            "v2-line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        match &result.segments[0] {
            MergeSegment::Lines { lines } => {
                assert_eq!(
                    vec![
                        "v1-line1".to_string(),
                        "v1-line1-2".to_string(),
                        "v2-line1".to_string(),
                        "line2".to_string(),
                        "line3".to_string()
                    ],
                    *lines
                );
            }
            _ => panic!("Expected MergeSegment::Lines"),
        }
    }

    #[test]
    fn test_merge_with_replacements_and_deletes() {
        let original = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let v1 = vec!["line1".to_string(), "line3".to_string()];
        let v2 = vec![
            "line1".to_string(),
            "v2-line2".to_string(),
            "v2-line2-2".to_string(),
            "line3".to_string(),
        ];

        let result = merge(&original, &v1, &v2);

        match &result.segments[0] {
            MergeSegment::Lines { lines } => {
                assert_eq!(
                    vec![
                        "line1".to_string(),
                        "v2-line2".to_string(),
                        "v2-line2-2".to_string(),
                        "line3".to_string()
                    ],
                    *lines
                );
            }
            _ => panic!("Expected MergeSegment::Lines"),
        }
    }
}
