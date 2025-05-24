package com.revtekk.blaze.diff

import kotlin.math.max
import kotlin.math.min

// ASCII escape codes for diff color
private const val RED_COLOR = "\u001B[31m"
private const val BLUE_COLOR = "\u001B[34m"
private const val GREEN_COLOR = "\u001B[32m"
private const val RESET_COLOR = "\u001B[0m"

fun printDiff(fileDiff: FileDiff) {
    // No diff
    if (fileDiff.segments.size == 1 && fileDiff.segments[0].type == SegmentType.EQUAL) {
        return
    }

    val processed = mutableSetOf<Int>()

    for ((i, segment) in fileDiff.segments.withIndex()) {
        if (segment.type == SegmentType.DIFF) {
            val prevSegment = fileDiff.segments.getOrNull(i - 1)

            if (prevSegment != null && prevSegment.type == SegmentType.EQUAL) {
                val equalLines = safeSubList(prevSegment.lines, prevSegment.lines.size, -4)
                printEqualLines(equalLines, processed)
            }

            println("$BLUE_COLOR@ line_no:${segment.lines[0].lineNo}$RESET_COLOR")

            segment.lines.forEach { line ->
                when(line.type) {
                    DiffType.INSERT -> println("$GREEN_COLOR+ ${line.line}$RESET_COLOR")
                    DiffType.DELETE -> println("$RED_COLOR- ${line.line}$RESET_COLOR")
                    DiffType.EQUAL -> {}
                }
            }
        } else {
            val prevSegment = fileDiff.segments.getOrNull(i - 1)

            if (prevSegment != null && prevSegment.type == SegmentType.DIFF) {
                val equalLines = safeSubList(segment.lines, 0, 4)
                printEqualLines(equalLines, processed)
            }
        }
    }
}

private fun printEqualLines(equalLines: List<DiffLine>, processed: MutableSet<Int>) {
    /**
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
    val nProcessed = equalLines.filter {
        it.lineNo !in processed
    }.map {
        println("  ${it.line}")
        it.lineNo
    }
    processed.addAll(nProcessed)
}

private fun safeSubList(list: List<DiffLine>, anchor: Int, lineCount: Int): List<DiffLine> {
    val fromIndex = if (lineCount < 0) {
        anchor + lineCount
    } else {
        anchor
    }

    val toIndex = if (lineCount < 0) {
        anchor
    } else {
        anchor + lineCount
    }

    return list.subList(max(0, fromIndex), min(list.size, toIndex))
}
