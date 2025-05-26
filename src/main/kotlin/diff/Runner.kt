package com.revtekk.blaze.diff

import com.revtekk.blaze.common.Command
import com.revtekk.blaze.common.CommandArgumentValidationException
import com.revtekk.blaze.common.readLines
import java.io.File

/**
 * Command to compute the difference between two files
 *
 * @property args first file path, second file path
 * @throws CommandArgumentValidationException if insufficient arguments are provided to run the command
 *
 * Usage: diff <first_file> <second_file>
 */
class DiffCommand: Command {
    constructor(args: List<String>) : super(args) {
        if (args.size != 2) {
            throw CommandArgumentValidationException(
                "diff: insufficient arguments specified, expected 2, got ${args.size}")
        }
    }

    override fun execute() {
        val first = File(args[0])
        val second = File(args[1])

        val firstLines = readLines(first.path)
        val secondLines = readLines(second.path)

        val fileDiff = diff(firstLines, secondLines)
        printDiff(fileDiff)
    }
}
