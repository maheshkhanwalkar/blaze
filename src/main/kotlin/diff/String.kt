package com.revtekk.blaze.diff

/**
 * Represents a subsequence found as the result of a longest common subsequence (LCS) computation.
 *
 * @param T The type of elements in the subsequence.
 * @property lcs The list of elements that are part of the longest common subsequence.
 * @property firstPos The list of indices in the first list that are part of the subsequence.
 * @property secondPos The list of indices in the second list that are part of the subsequence.
 */
data class Subsequence<T>(val lcs: List<T>, val firstPos: List<Int>, val secondPos: List<Int>)

/**
 * Computes the longest common subsequence between two input lists.
 *
 * @param first the first input list
 * @param second the second input list
 * @return the longest common subsequence shared between the two input lists
 */
fun <T> longestCommonSubsequence(first: List<T>, second: List<T>): Subsequence<T> {
    val T = Array(first.size) { IntArray(second.size) }
    val back = mutableMapOf<Pair<Int, Int>, Pair<Int, Int>>()

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
    for (i in first.indices) {
        for (j in second.indices) {
            // The use of get() allows us to simplify the logic
            if (first[i] == second[j]) {
                T[i][j] = get(T, i - 1, j - 1) + 1
                back[i to j] = i - 1 to j - 1
            } else {
                val up = get(T, i - 1, j)
                val left = get(T, i, j - 1)

                T[i][j] = if (up > left) {
                    back[i to j] = i - 1 to j
                    up
                } else {
                    back[i to j] = i to j - 1
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
    var node = first.size - 1 to second.size - 1

    val lcs = mutableListOf<T>()
    val firstPos = mutableListOf<Int>()
    val secondPos = mutableListOf<Int>()

    while (!isSink(back, node)) {
        val prev = back[node]!!

        if (isEdgeDiagonal(node, prev)) {
            lcs.addFirst(first[node.first])
            firstPos.addFirst(node.first)
            secondPos.addFirst(node.second)
        }

        node = prev
    }

    return Subsequence(lcs, firstPos, secondPos)
}

/**
 * Retrieves the value from a 2D array at the specified position. If the indices are negative,
 * it returns 0.
 *
 * @param T the 2D array of integers
 * @param i the row index
 * @param j the column index
 * @return the value at the specified position in the array, or 0 if the indices are negative
 */
private fun get(T: Array<IntArray>, i: Int, j: Int): Int {
    return if (i < 0 || j < 0) {
        0
    } else {
        T[i][j]
    }
}

/**
 * Checks if the given node is a sink -- that is, it does not have any
 * further edges in the graph.
 *
 * @param graph edges of the graph
 * @param node node in the graph
 *
 * @return true if the node is a sink, false otherwise
 */
private fun isSink(graph: Map<Pair<Int, Int>, Pair<Int, Int>>, node: Pair<Int, Int>): Boolean {
    return node !in graph
}

/**
 * Determines if the edge between the source and sink positions is diagonal.
 *
 * @param source The source position represented as a pair of integers (row, column).
 * @param sink The sink position represented as a pair of integers (row, column).
 * @return True if the edge between the positions is diagonal, false otherwise.
 */
private fun isEdgeDiagonal(source: Pair<Int, Int>, sink: Pair<Int, Int>): Boolean {
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
    return source.first - 1 == sink.first && source.second - 1 == sink.second
}
