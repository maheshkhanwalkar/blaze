package com.revtekk.blaze.merge

import com.revtekk.blaze.common.Command
import com.revtekk.blaze.common.readLines

class MergeCommand: Command {
    constructor(args: List<String>) : super(args) {
        if (args.size != 3) {
            throw IllegalArgumentException("merge: insufficient arguments specified, expected 3, got ${args.size}")
        }
    }

    override fun execute() {
        /*
         * original => the common ancestor of v1 and v2
         * v1 => modification of original
         * v2 => modification of original
         *
         * objective: merge v1, v2 using the original as a guide (aka 3-way merge)
         */
        val original = args[0]
        val v1 = args[1]
        val v2 = args[2]

        val originalLines = readLines(original)
        val v1Lines = readLines(v1)
        val v2Lines = readLines(v2)

        val result = merge(originalLines, v1Lines, v2Lines)
        result.segments.forEach {
            // FIXME once merge conflict detection is implemented, need to handle this
            if (it is MergeLines) {
                println(it.lines.joinToString("\n"))
            }
        }
    }
}
