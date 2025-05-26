package com.revtekk.blaze

import com.revtekk.blaze.common.Command
import com.revtekk.blaze.common.CommandArgumentValidationException
import com.revtekk.blaze.diff.DiffCommand
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
    } catch (e: CommandArgumentValidationException) {
        error(e)
    } catch (e: Exception) {
        fatalError(e)
    }
}

private fun getCommand(args: Array<String>): Command {
    val command = args[0]
    val remArgs = args.drop(1)

    when(command) {
        "diff" -> return DiffCommand(remArgs)
        else -> throw IllegalArgumentException("Unknown command: $command")
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
