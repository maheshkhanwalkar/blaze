package com.revtekk.blaze.diff

import java.io.File
import kotlin.system.exitProcess

fun runDiff(args: List<String>) {
    if (args.size != 2) {
        println("Usage: blaze diff <file-1> <file-2>")
        exitProcess(1)
    }

    val first = args[0]
    val second = args[1]
    val fileDiff = diff(File(first), File(second))

    // TODO print out the diff
    println(fileDiff)
}
