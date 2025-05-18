package com.revtekk.blaze.diff

import java.io.File

/**
 * Compares the contents of two files line by line and prints the differences.
 * Lines that appear in only one file are highlighted with colors to indicate
 * additions or deletions. Common lines are displayed without color.
 *
 * @param first the first file to compare
 * @param second the second file to compare
 */
fun diff(first: File, second: File) {
    val firstLines = first.readLines()
    val secondLines = second.readLines()

    val unique = mutableMapOf<String, ULong>()
    var counter = 0UL

    for (line in firstLines) {
        if (line !in unique) {
            unique[line] = counter++
        }
    }
    for (line in secondLines) {
        if (line !in unique) {
            unique[line] = counter++
        }
    }

    val firstTransformed = firstLines.map { unique[it]!! }
    val secondTransformed = secondLines.map { unique[it]!! }

    val result = longestCommonSubsequence(firstTransformed, secondTransformed)

    var i = 0
    var j = 0
    var k = 0

    // ASCII escape codes for diff color
    val redColor = "\u001B[31m"
    val blueColor = "\u001B[34m"
    val greenColor = "\u001B[32m"
    val resetColor = "\u001B[0m"

    // 'first' and 'second' are identical
    if (result.lcs.size == firstLines.size && result.lcs.size == secondLines.size) {
        return
    }

    while (i < firstLines.size || j < secondLines.size) {
        /*
         * The 'first' file is considered the original state, while the 'second'
         * file is the new state.
         *
         * We treat any line in the 'first' list that isn't a part of the LCS as a
         * removal and any line in the 'second' as an addition. We keep processing
         * elements in 'first' and 'second' in order until we hit a "break point"
         *
         * The break point is when we reach an element of the LCS -- which we just
         * print as-is, since there's no diff there. Then, we repeat the process
         * until we hit the next break point or reach the end of the file.
         */
        var first = true
        while (canContinue(k, i, result.firstPos, firstLines.size)) {
            if (first) {
                println("$blueColor@ line_no: ${i+1}$resetColor")
                first = false
            }
            println("$redColor- ${firstLines[i]}$resetColor")
            i++
        }
        while (canContinue(k, j, result.secondPos, secondLines.size)) {
            if (first) {
                /*
                 * we use 'i' here because the positions are anchored against the numbering of the
                 * original file.
                 */
                println("$blueColor@ line_no: ${i+1}$resetColor")
                first = false
            }
            println("$greenColor+ ${secondLines[j]}$resetColor")
            j++
        }

        i++
        j++
        k++
    }
}

private fun canContinue(k: Int, currPos: Int, positions: List<Int>, linesLen: Int): Boolean {
    return (k < positions.size && currPos < positions[k]) || (k >= positions.size && currPos < linesLen)
}
