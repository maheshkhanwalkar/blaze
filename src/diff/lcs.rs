use array2d::Array2D;
use std::collections::HashMap;

/// Represents a subsequence found as the result of a longest common subsequence (LCS) computation.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the subsequence.
///
/// # Fields
///
/// * `lcs` - The list of elements that are part of the longest common subsequence.
/// * `first_pos` - The list of indices in the first list that are part of the subsequence.
/// * `second_pos` - The list of indices in the second list that are part of the subsequence.
pub struct Subsequence<T> {
    pub lcs: Vec<T>,
    pub first_pos: Vec<i32>,
    pub second_pos: Vec<i32>,
}

/// Computes the longest common subsequence between two input lists.
///
/// # Arguments
///
/// * `first` - The first input list
/// * `second` - The second input list
///
/// # Returns
///
/// The longest common subsequence shared between the two input lists
pub fn longest_common_subsequence<T: Eq + Clone>(first: Vec<T>, second: Vec<T>) -> Subsequence<T> {
    #[allow(non_snake_case)]
    let mut T = Array2D::filled_with(0, first.len(), second.len());
    let mut back: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    /*
     * LCS algorithm boils down to the following recursive definition:
     *
     * T[i, j] = T[i-1, j-1] + 1            first[i] =  second[j]
     *           max{T[i-1, j], T[i, j-1]   first[i] != second[j]
     *
     * The loop below proceeds in the ordering such that we have already
     * computed T[i-1, j-1], T[i-1, j], and T[i, j-1] before computing
     * T[i, j]; hence, we do not need to recurse.
     *
     * To make reconstructing the actual LCS easier, we keep track of the
     * back pointers via the `back` map.
     */
    for i in 0..first.len() as i32 {
        for j in 0..second.len() as i32 {
            // The use of get() allows us to simplify the logic
            if first[i as usize] == second[j as usize] {
                T[(i as usize, j as usize)] = get(&T, i - 1, j - 1) + 1;
                back.insert((i, j), (i - 1, j - 1));
            } else {
                let up = get(&T, i - 1, j);
                let left = get(&T, i, j - 1);

                T[(i as usize, j as usize)] = if up > left {
                    back.insert((i, j), (i - 1, j));
                    up
                } else {
                    back.insert((i, j), (i, j - 1));
                    left
                }
            }
        }
    }

    /*
     * To construct an LCS, we need to traverse the back pointer
     * graph until we reach a (terminal) sink node.
     *
     * We only append an element when traversing a diagonal edge, because
     * the edge (i,j) -> (i-1, j-1) means that first[i] = second[j], so it
     * must be a part of the LCS.
     */
    let mut node = (first.len() as i32 - 1, second.len() as i32 - 1);

    let mut lcs: Vec<T> = Vec::new();
    let mut first_pos = Vec::new();
    let mut second_pos = Vec::new();

    while !is_sink(&back, node) {
        let prev = back[&node];

        if is_edge_diagonal(node, prev) {
            lcs.insert(0, first.get(node.0 as usize).unwrap().clone());
            first_pos.insert(0, node.0);
            second_pos.insert(0, node.1);
        }

        node = prev;
    }

    Subsequence {
        lcs,
        first_pos,
        second_pos,
    }
}

/// Retrieves the value from a 2D array at the specified position. If the indices are negative,
/// it returns 0.
///
/// # Arguments
///
/// * `T` - The 2D array of integers
/// * `i` - The row index
/// * `j` - The column index
///
/// # Returns
///
/// The value at the specified position in the array, or 0 if the indices are negative
#[allow(non_snake_case)]
fn get(T: &Array2D<i32>, i: i32, j: i32) -> i32 {
    if i < 0 || j < 0 {
        0
    } else {
        T[(i as usize, j as usize)]
    }
}

/// Checks if the given node is a sink -- that is, it does not have any
/// further edges in the graph.
///
/// # Arguments
///
/// * `graph` - edges of the graph
/// * `node` - node in the graph
///
/// # Returns
///
/// `true` if the node is a sink, `false` otherwise
fn is_sink(graph: &HashMap<(i32, i32), (i32, i32)>, node: (i32, i32)) -> bool {
    !graph.contains_key(&node)
}

/// Determines if the edge between the source and sink positions is diagonal.
///
/// # Arguments
///
/// * `source` - The source position represented as a pair of integers (row, column)
/// * `sink` - The sink position represented as a pair of integers (row, column)
///
/// # Returns
///
/// `true` if the edge between the positions is diagonal, `false` otherwise
fn is_edge_diagonal(source: (i32, i32), sink: (i32, i32)) -> bool {
    /*
     * sink <- source [not diagonal]
     * -------------------------------
     * sink
     * |
     * source         [not diagonal]
     * -------------------------------
     * sink
     *     \
     *      source    [diagonal!]
     * -------------------------------
     */
    source.0 - 1 == sink.0 && source.1 - 1 == sink.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_sequences() {
        let result = longest_common_subsequence(Vec::<i32>::new(), Vec::<i32>::new());
        assert_eq!(result.lcs.len(), 0);
        assert_eq!(result.first_pos.len(), 0);
        assert_eq!(result.second_pos.len(), 0);
    }

    #[test]
    fn test_common_elements() {
        let first = vec![1, 2, 3, 4];
        let second = vec![2, 3, 5];
        let result = longest_common_subsequence(first, second);
        assert_eq!(result.lcs, vec![2, 3]);
        assert_eq!(result.first_pos, vec![1, 2]);
        assert_eq!(result.second_pos, vec![0, 1]);
    }

    #[test]
    fn test_no_common_elements() {
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];
        let result = longest_common_subsequence(first, second);
        assert_eq!(result.lcs.len(), 0);
        assert_eq!(result.first_pos.len(), 0);
        assert_eq!(result.second_pos.len(), 0);
    }

    #[test]
    fn test_identical_sequences() {
        let first = vec![1, 2, 3];
        let second = vec![1, 2, 3];
        let result = longest_common_subsequence(first, second);
        assert_eq!(result.lcs, vec![1, 2, 3]);
        assert_eq!(result.first_pos, vec![0, 1, 2]);
        assert_eq!(result.second_pos, vec![0, 1, 2]);
    }

    #[test]
    fn test_different_lengths() {
        let first = vec![1, 2, 3, 4, 5];
        let second = vec![2, 4, 5];
        let result = longest_common_subsequence(first, second);
        assert_eq!(result.lcs, vec![2, 4, 5]);
        assert_eq!(result.first_pos, vec![1, 3, 4]);
        assert_eq!(result.second_pos, vec![0, 1, 2]);
    }
}
