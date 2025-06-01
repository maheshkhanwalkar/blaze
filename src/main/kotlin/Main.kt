package com.revtekk.blaze

import com.revtekk.blaze.common.Command
import com.revtekk.blaze.common.CommandArgumentValidationException
import com.revtekk.blaze.common.CommandExecutionException
import com.revtekk.blaze.diff.DiffCommand
import com.revtekk.blaze.merge.MergeCommand
import kotlin.system.exitProcess

/**
 * Blaze entry point.
 */
fun main(args: Array<String>) {
    if (args.isEmpty()) {
        println("Usage: blaze <command> <args>")
        return
    }

    try {
        val command = getCommand(args)
        command.execute()
    } catch (e: Exception) {
        when(e) {
            is CommandArgumentValidationException -> error(e)
            is CommandExecutionException -> error(e)
            else -> fatalError(e)
        }
    }
}

private fun getCommand(args: Array<String>): Command {
    val command = args[0]
    val remArgs = args.drop(1)

    return when(command) {
        "diff"  -> DiffCommand(remArgs)
        "merge" -> MergeCommand(remArgs)
        else -> throw CommandArgumentValidationException("unknown command: $command")
    }
}

private fun error(e: Exception) {
    println(e.message)
    exitProcess(1)
}

private fun fatalError(e: Exception) {
    println("fatal error, report to blaze dev team: $e")
    exitProcess(1)
}
