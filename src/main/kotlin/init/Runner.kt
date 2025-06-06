package com.revtekk.blaze.init

import com.revtekk.blaze.common.Command
import com.revtekk.blaze.common.createDirectory
import com.revtekk.blaze.common.directoryExists
import com.revtekk.blaze.common.getFullPath

class InitCommand: Command(emptyList()) {
    override fun execute() {
        val configPath = ".blaze"
        val fullPath = getFullPath(configPath)

        if (directoryExists(fullPath)) {
            println("blaze repository already initialized in $fullPath, no action performed.")
            return
        }

        createDirectory(fullPath)
        println("blaze repository initialized at $fullPath")
    }
}
