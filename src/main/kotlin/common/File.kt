package com.revtekk.blaze.common

import java.io.File
import java.io.FileNotFoundException

fun readLines(path: String): List<String> {
    try {
        return File(path).readLines()
    } catch (_: FileNotFoundException) {
        throw CommandExecutionException("file not found: $path")
    }
}
