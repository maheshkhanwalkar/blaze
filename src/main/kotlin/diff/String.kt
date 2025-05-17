package com.revtekk.blaze.diff

/**
 * Computes the longest common subsequence between two input strings.
 *
 * @param first the first input string
 * @param second the second input string
 * @return the longest common subsequence shared between the two input strings
 */
fun longestCommonSubsequence(first: String, second: String): String {
    val T = Array(first.length) { IntArray(second.length) }
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
     * To construct an LCS string, we need to traverse the back pointer
     * graph until we reach a (terminal) sink node.
     *
     * We only append a character when traversing a diagonal edge, because
     * the edge (i,j) -> (i-1, j-1) means that first[i] = second[j], so it
     * must be a part of the LCS.
     */
    var node = first.length - 1 to second.length - 1
    val result = StringBuilder()

    while (!isSink(back, node)) {
        val prev = back[node]!!

        if (isEdgeDiagonal(node, prev)) {
            result.append(first[node.first])
        }

        node = prev
    }

    return result.reverse().toString()
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
