package com.revtekk.blaze

import com.revtekk.blaze.diff.diff
import java.io.File

/**
 * Blaze entry point.
 */
fun main(args: Array<String>) {
    val first = File(args[0])
    val second = File(args[1])

    diff(first, second)
}
