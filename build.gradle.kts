plugins {
    kotlin("jvm") version "2.1.20"
    id("org.graalvm.buildtools.native") version "0.10.6"
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(21))
    }
}

graalvmNative {
    toolchainDetection.set(true)
    binaries {
        named("main") {
            imageName.set("blaze")
            mainClass.set("com.revtekk.blaze.MainKt")
        }
        named("test") {
            buildArgs.add("-O0")
        }
    }
    binaries.all {
        resources.autodetect()
    }
}

group = "com.revtekk"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}