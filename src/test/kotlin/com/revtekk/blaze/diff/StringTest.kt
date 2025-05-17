package com.revtekk.blaze.diff

import kotlin.test.Test
import kotlin.test.assertEquals

class StringTest {
    @Test
    fun `test longest common subsequence when strings share partial overlap`() {
        val first = "abcde"
        val second = "ace"
        val result = longestCommonSubsequence(first, second)
        assertEquals("ace", result, "LCS should be 'ace' when input strings are 'abcde' and 'ace'")
    }

    @Test
    fun `test longest common subsequence when strings are identical`() {
        val first = "abcdef"
        val second = "abcdef"
        val result = longestCommonSubsequence(first, second)
        assertEquals("abcdef", result, "LCS should be 'abcdef' when both input strings are identical")
    }

    @Test
    fun `test longest common subsequence when strings have no common characters`() {
        val first = "abc"
        val second = "xyz"
        val result = longestCommonSubsequence(first, second)
        assertEquals("", result, "LCS should be an empty string when there are no common characters")
    }

    @Test
    fun `test longest common subsequence when one string is empty`() {
        val first = "abcde"
        val second = ""
        val result = longestCommonSubsequence(first, second)
        assertEquals("", result, "LCS should be an empty string when one of the input strings is empty")
    }

    @Test
    fun `test longest common subsequence when both strings are empty`() {
        val first = ""
        val second = ""
        val result = longestCommonSubsequence(first, second)
        assertEquals("", result, "LCS should be an empty string when both input strings are empty")
    }

    @Test
    fun `test longest common subsequence when there are repeated characters`() {
        val first = "aabbcc"
        val second = "ababc"
        val result = longestCommonSubsequence(first, second)
        assertEquals("abbc", result, "LCS should be 'abbc' for the input strings 'aabbcc' and 'ababc'")
    }

    @Test
    fun `test longest common subsequence when strings have complex overlaps`() {
        val first = "AGGTAB"
        val second = "GXTXAYB"
        val result = longestCommonSubsequence(first, second)
        assertEquals("GTAB", result, "LCS should be 'GTAB' for input strings 'AGGTAB' and 'GXTXAYB'")
    }
}
