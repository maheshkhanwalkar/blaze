package com.revtekk.blaze

import com.revtekk.blaze.diff.runDiff
import kotlin.system.exitProcess

/**
 * Blaze entry point.
 */
fun main(args: Array<String>) {
    if (args.isEmpty()) {
        println("Usage: blaze <command> <args>")
        return
    }

    val command = args[0]
    if (command == "diff") {
        runDiff(args.drop(1))
    } else {
        println("Unknown command: $command")
        exitProcess(1)
    }
}
