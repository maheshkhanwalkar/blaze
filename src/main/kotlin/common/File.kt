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

fun getFullPath(relativePath: String): String {
    return File(relativePath).canonicalPath
}

fun directoryExists(path: String): Boolean {
    return File(path).exists()
}

fun createDirectory(path: String) {
    if (!File(path).mkdirs()) {
        throw CommandExecutionException("directory $path could not be created")
    }
}
