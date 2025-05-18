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

    val revLookup = unique.entries.associate { it.value to it.key }

    val firstTransformed = firstLines.map { unique[it]!! }
    val secondTransformed = secondLines.map { unique[it]!! }

    val result = longestCommonSubsequence(firstTransformed, secondTransformed)

    var i = 0
    var j = 0

    // ASCII escape codes for diff color
    val redColor = "\u001B[31m"
    val resetColor = "\u001B[0m"
    val greenColor = "\u001B[32m"

    for (k in 0 until result.lcs.size) {
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
        while (i < result.firstPos[k]) {
            println("${redColor}- ${firstLines[i++]}${resetColor}")
        }
        while (j < result.secondPos[k]) {
            println("${greenColor}+ ${secondLines[j++]}${resetColor}")
        }
        println("  ${revLookup[result.lcs[k]]}")
        i++
        j++
    }
}
