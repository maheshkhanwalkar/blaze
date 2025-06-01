package com.revtekk.blaze.merge

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class MergeTest {

    @Test
    fun `test merge with no differences`() {
        val original = listOf("line1", "line2", "line3")
        val v1 = listOf("line1", "line2", "line3")
        val v2 = listOf("line1", "line2", "line3")

        val result = merge(original, v1, v2)

        assertEquals(1, result.segments.size)
        val segment = result.segments[0]
        assertTrue(segment is MergeLines)
        assertEquals(original, segment.lines)
    }

    @Test
    fun `test merge with non-conflicting changes`() {
        val original = listOf("line1", "line2", "line3")
        val v1 = listOf("line1", "v1-line2", "line3")
        val v2 = listOf("line1", "line2", "v2-line3")

        val result = merge(original, v1, v2)

        assertEquals(1, result.segments.size)
        val segment = result.segments[0]
        assertTrue(segment is MergeLines)
        assertEquals(listOf("line1", "v1-line2", "v2-line3"), segment.lines)
    }

    @Test
    fun `test merge with conflicting changes`() {
        val original = listOf("line1", "line2", "line3")
        val v1 = listOf("line1", "v1-line2", "line3")
        val v2 = listOf("line1", "v2-line2", "line3")

        val result = merge(original, v1, v2)

        assertEquals(3, result.segments.size)
        assertTrue(result.segments[0] is MergeLines)
        assertTrue(result.segments[1] is MergeConflict)
        assertTrue(result.segments[2] is MergeLines)

        val conflict = result.segments[1] as MergeConflict
        assertEquals(listOf("v1-line2"), conflict.v1Changes)
        assertEquals(listOf("v2-line2"), conflict.v2Changes)
    }

    @Test
    fun `test merge with multiple conflicts`() {
        val original = listOf("line1", "line2", "line3")
        val v1 = listOf("line1", "v1-line2", "v1-line3")
        val v2 = listOf("line1", "v2-line2", "v2-line3")

        val result = merge(original, v1, v2)

        assertEquals(3, result.segments.size)
        assertTrue(result.segments[0] is MergeLines)
        assertTrue(result.segments[1] is MergeConflict)
        assertTrue(result.segments[2] is MergeConflict)

        val conflict1 = result.segments[1] as MergeConflict
        assertEquals(listOf("v1-line2"), conflict1.v1Changes)
        assertEquals(listOf("v2-line2"), conflict1.v2Changes)

        val conflict2 = result.segments[2] as MergeConflict
        assertEquals(listOf("v1-line3"), conflict2.v1Changes)
        assertEquals(listOf("v2-line3"), conflict2.v2Changes)
    }

    @Test
    fun `test merge with insertions in both versions`() {
        val original = listOf("line1", "line2", "line3")
        val v1 = listOf("line1", "line2", "v1-line", "line3")
        val v2 = listOf("line1", "v2-line", "line2", "line3")

        val result = merge(original, v1, v2)

        assertEquals(1, result.segments.size)
        val segment = result.segments[0]
        assertTrue(segment is MergeLines)
        assertEquals(listOf("line1", "v2-line", "line2", "v1-line", "line3"), segment.lines)
    }

    @Test
    fun `test merge with deletions in both versions`() {
        val original = listOf("line1", "line2", "line3", "line4")
        val v1 = listOf("line1", "line3", "line4")
        val v2 = listOf("line1", "line2", "line4")

        val result = merge(original, v1, v2)

        assertEquals(1, result.segments.size)
        val segment = result.segments[0]
        assertTrue(segment is MergeLines)
        assertEquals(listOf("line1", "line4"), segment.lines)
    }

    @Test
    fun `test merge with interleaved changes`() {
        val original = listOf("line1", "line2", "line3", "line4")
        val v1 = listOf("line1", "v1-line2", "line3", "v1-line4")
        val v2 = listOf("v2-line1", "line2", "v2-line3", "line4")

        val result = merge(original, v1, v2)

        val segment = result.segments[0]
        assertTrue(segment is MergeLines)
        assertEquals(listOf("v2-line1", "v1-line2", "v2-line3", "v1-line4"), segment.lines)
    }
}
