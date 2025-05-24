package com.revtekk.blaze.diff

import java.io.File

enum class DiffType {
    INSERT, DELETE, EQUAL
}

enum class SegmentType {
    DIFF, EQUAL
}

data class DiffLine(val type: DiffType, val lineNo: Int, val line: String)
data class DiffSegment(val type: SegmentType, val lines: List<DiffLine>)
data class FileDiff(val segments: List<DiffSegment>)

private enum class ProcessingState {
    NONE, IN_DIFF, IN_EQUAL
}

/**
 * Compares the contents of two files line by line
 *
 * @param first the first file to compare
 * @param second the second file to compare
 * @return the diff
 */
fun diff(first: File, second: File): FileDiff {
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

    // ASCII escape codes for diff color
    val redColor = "\u001B[31m"
    val blueColor = "\u001B[34m"
    val greenColor = "\u001B[32m"
    val resetColor = "\u001B[0m"

    // 'first' and 'second' are identical -- so there's no diff
    if (result.lcs.size == firstLines.size && result.lcs.size == secondLines.size) {
        val lines = firstLines.mapIndexed { idx, line -> DiffLine(DiffType.EQUAL, idx+1, line) }
        return FileDiff(listOf(DiffSegment(SegmentType.EQUAL, lines)))
    }

    val segments = mutableListOf<DiffSegment>()
    val lines = mutableListOf<DiffLine>()

    var state = ProcessingState.NONE

    for (k in 0 until result.lcs.size) {
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
        while (i < result.firstPos[k]) {
            if (state == ProcessingState.NONE) {
                state = ProcessingState.IN_DIFF
            }
            else if (state == ProcessingState.IN_EQUAL) {
                // Collect the pending items into a segment
                state = collect(lines, state, segments)
            }

            lines.add(DiffLine(DiffType.DELETE, i+1, firstLines[i]))
            i++
        }

        while (j < result.secondPos[k]) {
            if (state == ProcessingState.NONE) {
                state = ProcessingState.IN_DIFF
            }
            else if (state == ProcessingState.IN_EQUAL) {
                state = collect(lines, state, segments)
            }

            /*
             * we use 'i' here because the positions are anchored against the numbering of the
             * original file.
             */
            lines.add(DiffLine(DiffType.INSERT, i+1, secondLines[j]))
            j++
        }

        if (state == ProcessingState.IN_DIFF) {
            state = collect(lines, state, segments)
        }

        val lineNo = result.firstPos[k]
        lines.add(DiffLine(DiffType.EQUAL, lineNo + 1, firstLines[lineNo]))
        i++
        j++
    }

    /*
     * Collect the remaining lines to finish out the segment. Handle the edge case where there
     * are still lines after the last LCS element and collect those as well.
     */
    state = collect(lines, state, segments)

    while (i < firstLines.size) {
        lines.add(DiffLine(DiffType.DELETE, i+1, firstLines[i]))
        i++
    }

    while (j < secondLines.size) {
        lines.add(DiffLine(DiffType.INSERT, i+1, secondLines[j]))
        j++
    }

    collect(lines, state, segments)
    return FileDiff(segments)
}

private fun collect(lines: MutableList<DiffLine>, state: ProcessingState, segments: MutableList<DiffSegment>): ProcessingState {
    val type = when (state) {
        ProcessingState.NONE -> throw IllegalStateException("Cannot collect from NONE state")
        ProcessingState.IN_DIFF -> SegmentType.DIFF
        ProcessingState.IN_EQUAL -> SegmentType.EQUAL
    }

    // Avoid adding an empty segment
    if (lines.isNotEmpty()) {
        segments.add(DiffSegment(type, lines.toList()))
        lines.clear()
    }

    return when(state) {
        ProcessingState.NONE -> throw IllegalStateException("Cannot collect from NONE state")
        ProcessingState.IN_DIFF -> ProcessingState.IN_EQUAL
        ProcessingState.IN_EQUAL -> ProcessingState.IN_DIFF
    }
}

private fun allEmpty(additions: List<DiffLine>, subtractions: List<DiffLine>, common: List<DiffLine>) =
    additions.isEmpty() && subtractions.isEmpty() && common.isEmpty()

private fun canContinue(k: Int, currPos: Int, positions: List<Int>, linesLen: Int): Boolean {
    return (k < positions.size && currPos < positions[k]) || (k >= positions.size && currPos < linesLen)
}
