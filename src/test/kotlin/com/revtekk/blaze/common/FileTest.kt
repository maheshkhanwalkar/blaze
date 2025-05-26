package com.revtekk.blaze.common

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import kotlin.io.path.createTempFile

/**
 * Unit tests for the `readLines` function in the `FileKt` class.
 *
 * This class tests the behavior of the `readLines` function, which reads all lines from a file
 * given a valid file path. The function throws a `CommandExecutionException` if the file is not found.
 */
class FileTest {

    @Test
    fun `readLines should correctly return all lines from a non-empty file`() {
        val tempFile = createTempFile().toFile()
        tempFile.writeText("line1\nline2\nline3")

        val result = readLines(tempFile.absolutePath)

        assertEquals(listOf("line1", "line2", "line3"), result)
        tempFile.delete()
    }

    @Test
    fun `readLines should return an empty list for an empty file`() {
        val tempFile = createTempFile().toFile()
        tempFile.writeText("")

        val result = readLines(tempFile.absolutePath)

        assertEquals(emptyList<String>(), result)
        tempFile.delete()
    }

    @Test
    fun `readLines should throw CommandExecutionException when file is not found`() {
        val invalidPath = "nonexistent_file.txt"

        val exception = assertThrows<CommandExecutionException> {
            readLines(invalidPath)
        }

        assertEquals("file not found: $invalidPath", exception.message)
    }
}
