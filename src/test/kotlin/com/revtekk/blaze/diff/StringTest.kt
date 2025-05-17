package com.revtekk.blaze.diff

import kotlin.test.Test
import kotlin.test.assertEquals

class StringTest {
    @Test
    fun `test longest common subsequence when strings share partial overlap`() {
        val first = "abcde".toList()
        val second = "ace".toList()
        val result = longestCommonSubsequence(first, second).asString()
        assertEquals("ace", result, "LCS should be 'ace' when input strings are 'abcde' and 'ace'")
    }

    @Test
    fun `test longest common subsequence when strings are identical`() {
        val first = "abcdef".toList()
        val second = "abcdef".toList()
        val result = longestCommonSubsequence(first, second).asString()
        assertEquals("abcdef", result, "LCS should be 'abcdef' when both input strings are identical")
    }

    @Test
    fun `test longest common subsequence when strings have no common characters`() {
        val first = "abc".toList()
        val second = "xyz".toList()
        val result = longestCommonSubsequence(first, second).asString()
        assertEquals("", result, "LCS should be an empty string when there are no common characters")
    }

    @Test
    fun `test longest common subsequence when one string is empty`() {
        val first = "abcde".toList()
        val second = "".toList()
        val result = longestCommonSubsequence(first, second).asString()
        assertEquals("", result, "LCS should be an empty string when one of the input strings is empty")
    }

    @Test
    fun `test longest common subsequence when both strings are empty`() {
        val first = "".toList()
        val second = "".toList()
        val result = longestCommonSubsequence(first, second).asString()
        assertEquals("", result, "LCS should be an empty string when both input strings are empty")
    }

    @Test
    fun `test longest common subsequence when there are repeated characters`() {
        val first = "aabbcc".toList()
        val second = "ababc".toList()
        val result = longestCommonSubsequence(first, second).asString()
        assertEquals("abbc", result, "LCS should be 'abbc' for the input strings 'aabbcc' and 'ababc'")
    }

    @Test
    fun `test longest common subsequence when strings have complex overlaps`() {
        val first = "AGGTAB".toList()
        val second = "GXTXAYB".toList()
        val result = longestCommonSubsequence(first, second).asString()
        assertEquals("GTAB", result, "LCS should be 'GTAB' for input strings 'AGGTAB' and 'GXTXAYB'")
    }

    private fun List<Char>.asString(): String = this.joinToString("")
}
