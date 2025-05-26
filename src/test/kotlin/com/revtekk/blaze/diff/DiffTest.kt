package com.revtekk.blaze.diff

import java.io.File
import kotlin.test.Test
import kotlin.test.assertEquals

class DiffTest {

    @Test
    fun `test diff with identical files`() {
        val file1 = createTempFile(content = "line1\nline2\nline3")
        val file2 = createTempFile(content = "line1\nline2\nline3")

        val result = diff(file1.readLines(), file2.readLines())

        val expectedSegments = listOf(
            DiffSegment(
                SegmentType.EQUAL, listOf(
                    DiffLine(DiffType.EQUAL, 1, "line1"),
                    DiffLine(DiffType.EQUAL, 2, "line2"),
                    DiffLine(DiffType.EQUAL, 3, "line3")
                )
            )
        )
        assertEquals(FileDiff(expectedSegments), result)
    }

    @Test
    fun `test diff with completely different files`() {
        val file1 = createTempFile(content = "lineA\nlineB\nlineC")
        val file2 = createTempFile(content = "lineX\nlineY\nlineZ")

        val result = diff(file1.readLines(), file2.readLines())

        val expectedSegments = listOf(
            DiffSegment(
                SegmentType.DIFF, listOf(
                    DiffLine(DiffType.DELETE, 1, "lineA"),
                    DiffLine(DiffType.DELETE, 2, "lineB"),
                    DiffLine(DiffType.DELETE, 3, "lineC"),
                    DiffLine(DiffType.INSERT, 1, "lineX"),
                    DiffLine(DiffType.INSERT, 1, "lineY"),
                    DiffLine(DiffType.INSERT, 1, "lineZ")
                )
            )
        )
        assertEquals(FileDiff(expectedSegments), result)
    }

    @Test
    fun `test diff with additions to second file`() {
        val file1 = createTempFile(content = "line1\nline2")
        val file2 = createTempFile(content = "line1\nline2\nline3")

        val result = diff(file1.readLines(), file2.readLines())

        val expectedSegments = listOf(
            DiffSegment(
                SegmentType.EQUAL, listOf(
                    DiffLine(DiffType.EQUAL, 1, "line1"),
                    DiffLine(DiffType.EQUAL, 2, "line2")
                )
            ),
            DiffSegment(
                SegmentType.DIFF, listOf(
                    DiffLine(DiffType.INSERT, 3, "line3")
                )
            )
        )
        assertEquals(FileDiff(expectedSegments), result)
    }

    @Test
    fun `test diff with deletions from first file`() {
        val file1 = createTempFile(content = "line1\nline2\nline3")
        val file2 = createTempFile(content = "line1\nline2")

        val result = diff(file1.readLines(), file2.readLines())

        val expectedSegments = listOf(
            DiffSegment(
                SegmentType.EQUAL, listOf(
                    DiffLine(DiffType.EQUAL, 1, "line1"),
                    DiffLine(DiffType.EQUAL, 2, "line2")
                )
            ),
            DiffSegment(
                SegmentType.DIFF, listOf(
                    DiffLine(DiffType.DELETE, 3, "line3")
                )
            )
        )
        assertEquals(FileDiff(expectedSegments), result)
    }

    @Test
    fun `test diff with complex changes`() {
        val file1 = createTempFile(content = "line1\nline2\nline3\nline4")
        val file2 = createTempFile(content = "line1\nlineA\nline3\nlineB")

        val result = diff(file1.readLines(), file2.readLines())

        val expectedSegments = listOf(
            DiffSegment(
                SegmentType.EQUAL, listOf(
                    DiffLine(DiffType.EQUAL, 1, "line1")
                )
            ),
            DiffSegment(
                SegmentType.DIFF, listOf(
                    DiffLine(DiffType.DELETE, 2, "line2"),
                    DiffLine(DiffType.INSERT, 2, "lineA")
                )
            ),
            DiffSegment(
                SegmentType.EQUAL, listOf(
                    DiffLine(DiffType.EQUAL, 3, "line3")
                )
            ),
            DiffSegment(
                SegmentType.DIFF, listOf(
                    DiffLine(DiffType.DELETE, 4, "line4"),
                    DiffLine(DiffType.INSERT, 4, "lineB")
                )
            )
        )
        assertEquals(FileDiff(expectedSegments), result)
    }

    private fun createTempFile(content: String): File {
        val tempFile = kotlin.io.path.createTempFile().toFile()
        tempFile.writeText(content)
        return tempFile
    }
}
