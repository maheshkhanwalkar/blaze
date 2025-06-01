package com.revtekk.blaze.merge

import com.revtekk.blaze.diff.DiffLine
import com.revtekk.blaze.diff.DiffSegment
import com.revtekk.blaze.diff.DiffType
import com.revtekk.blaze.diff.FileDiff
import com.revtekk.blaze.diff.SegmentType
import com.revtekk.blaze.diff.diff

interface MergeSegment

data class MergeConflict(private val v1Changes: DiffSegment, private val v2Changes: DiffSegment): MergeSegment
data class MergeLines(val lines: List<String>): MergeSegment

data class MergeResult(val segments: List<MergeSegment>)

private enum class LineModificationType {
    /**
     * Inserting a net-new line into the original file.
     */
    INSERT,
    /**
     * Removing a line from the original file.
     */
    DELETE,

    /**
     * Replacing a line from the original file with a new line.
     * This is a special case of INSERT followed by DELETE.
     */
    REPLACE
}

private data class LineModification(val type: LineModificationType, val lineNo: Int, val line: String)

/**
 * Merges changes from two sets of lines (`v1Lines` and `v2Lines`) based on an original set of lines (`originalLines`).
 * Identifies conflicts where modifications have been made to the same line across both sets.
 *
 * @param originalLines the list of original lines that serve as the base for comparison
 * @param v1Lines the list of lines representing the first modified version
 * @param v2Lines the list of lines representing the second modified version
 * @return a MergeResult instance containing the merged lines with any conflicts recorded
 */
fun merge(originalLines: List<String>, v1Lines: List<String>, v2Lines: List<String>): MergeResult {
    val v1Diff = diff(originalLines, v1Lines)
    val v2Diff = diff(originalLines, v2Lines)

    val v1Categories = categorise(v1Diff)
    val v2Categories = categorise(v2Diff)

    val segments = mutableListOf<MergeSegment>()
    val currLines = mutableListOf<String>()

    for (lineNo in 0..originalLines.size) {
        val v1Lines = v1Categories[lineNo + 1]
        val v2Lines = v2Categories[lineNo + 1]

        if (v1Lines != null && v2Lines != null) {
            val v1Types = v1Lines.map { it.type }.toSet()
            val v2Types = v2Lines.map { it.type }.toSet()

            if ((LineModificationType.INSERT in v1Types && LineModificationType.INSERT in v2Types) ||
                (LineModificationType.REPLACE in v1Types && LineModificationType.REPLACE in v2Types) ||
                (LineModificationType.REPLACE in v1Types && LineModificationType.DELETE in v2Types) ||
                (LineModificationType.DELETE in v1Types && LineModificationType.REPLACE in v2Types)
            ) {
               TODO("Handle conflict")
            } else {
                /*
                 * It cannot be the case that both v1Lines and v2Lines contain the same type, as that
                 * would be a merge conflict -- but the code below is written like this, so we
                 * don't need to do a much of if-else(s) based on whether the INSERT or REPLACE is
                 * in v1Lines or v2Lines.
                 */
                val inserts = v1Lines.filter { it.type == LineModificationType.INSERT } +
                        v2Lines.filter { it.type == LineModificationType.INSERT }
                val replacements = v1Lines.filter { it.type == LineModificationType.REPLACE } +
                        v2Lines.filter { it.type == LineModificationType.REPLACE }

                // Handle REPLACE first, then INSERT to preserve correct ordering
                replacements.forEach { currLines.add(it.line) }
                inserts.forEach { currLines.add(it.line) }
            }
        } else if (v1Lines != null || v2Lines != null) {
            val lines = v1Lines ?: v2Lines!!

            lines.forEach {
                when(it.type) {
                    LineModificationType.INSERT -> currLines.add(it.line)
                    LineModificationType.REPLACE -> currLines.add(it.line)
                    LineModificationType.DELETE -> {}
                }
            }

            /**
             * When handling an INSERT line, the original line should still be
             * retained as well.
             */
            if (lines.getOrNull(0)?.type == LineModificationType.INSERT) {
                if (lineNo < originalLines.size) {
                    currLines.add(originalLines[lineNo])
                }
            }
        } else {
            if (lineNo < originalLines.size) {
                currLines.add(originalLines[lineNo])
            }
        }
    }

    if (currLines.isNotEmpty()) {
        segments.add(MergeLines(currLines.toList()))
    }

    return MergeResult(segments)
}

private fun categorise(diff: FileDiff): Map<Int, List<LineModification>> {
    val result = mutableMapOf<Int, MutableList<LineModification>>()

    for (seg in diff.segments) {
        if (seg.type == SegmentType.EQUAL) {
            continue
        }

        // All lines in this diff are INSERT(s)
        if (seg.lines.none { it.type == DiffType.DELETE }) {
            seg.lines.forEach { addToMap(result, it.lineNo, LineModification(LineModificationType.INSERT, it.lineNo, it.line)) }
            continue
        }

        // All lines in this diff are DELETE(s)
        if (seg.lines.none { it.type == DiffType.INSERT }) {
            seg.lines.forEach { addToMap(result, it.lineNo, LineModification(LineModificationType.DELETE, it.lineNo, it.line)) }
            continue
        }

        /**
         * Now comes the complicated part -- we've got both INSERT and DELETE types in this diff
         * segment, so that means that some of these will coalesce into a REPLACE type.
         *
         * We need to pair up the DELETE and INSERT lines to form a REPLACE line, and anything that
         * remains unpaired will be treated as a separate INSERT or DELETE line.
         */
        val boundary = findInsertPosition(seg.lines)
        var delPos = 0
        var insPos = boundary

        while (delPos < boundary && insPos < seg.lines.size) {
            val lineNo = seg.lines[delPos].lineNo
            val line = seg.lines[insPos].line

            addToMap(result, lineNo, LineModification(LineModificationType.REPLACE, lineNo, line))
            delPos++
            insPos++
        }

        while (insPos < seg.lines.size) {
            val diffLine = seg.lines[delPos]
            addToMap(result, diffLine.lineNo, LineModification(LineModificationType.INSERT, diffLine.lineNo, diffLine.line))
            insPos++
        }

        while (delPos < boundary) {
            val diffLine = seg.lines[delPos]
            addToMap(result, diffLine.lineNo, LineModification(LineModificationType.DELETE, diffLine.lineNo, diffLine.line))
            delPos++
        }
    }

    return result
}

private fun findInsertPosition(lines: List<DiffLine>): Int {
    var i = 0
    while (i < lines.size && lines[i].type != DiffType.INSERT) {
        i++
    }
    return i
}

private fun addToMap(map: MutableMap<Int, MutableList<LineModification>>, lineNo: Int, line: LineModification) {
    if (lineNo in map) {
        map[lineNo]!!.add(line)
    } else {
        map[lineNo] = mutableListOf(line)
    }
}
