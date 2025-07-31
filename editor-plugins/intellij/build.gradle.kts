plugins {
    id("java")
    id("org.jetbrains.intellij") version "1.17.2"
    id("org.jetbrains.kotlin.jvm") version "1.9.22"
}

group = "ai.fugue"
version = "0.1.0"

repositories {
    mavenCentral()
}

// Configure Gradle IntelliJ Plugin
intellij {
    version.set("2023.3.4")
    type.set("IC") // Target IDE Platform

    plugins.set(listOf(
        "java",
        "kotlin",
        "yaml",
        "git4idea",
        "terminal"
    ))
}

dependencies {
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
    implementation("org.jetbrains.kotlin:kotlin-reflect")
    implementation("com.fasterxml.jackson.core:jackson-databind:2.15.3")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-yaml:2.15.3")
    implementation("org.yaml:snakeyaml:2.2")
    implementation("com.google.guava:guava:32.1.3-jre")
    implementation("org.apache.commons:commons-lang3:3.14.0")
    implementation("org.apache.commons:commons-io:2.15.1")
    implementation("com.google.code.gson:gson:2.10.1")
    
    // Testing dependencies
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.10.1")
    testImplementation("org.junit.jupiter:junit-jupiter-engine:5.10.1")
    testImplementation("org.mockito:mockito-core:5.8.0")
    testImplementation("org.mockito:mockito-junit-jupiter:5.8.0")
    testImplementation("org.assertj:assertj-core:3.24.2")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")
}

tasks {
    // Set the JVM compatibility versions
    withType<JavaCompile> {
        sourceCompatibility = "17"
        targetCompatibility = "17"
    }
    withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
        kotlinOptions.jvmTarget = "17"
    }

    patchPluginXml {
        sinceBuild.set("231")
        untilBuild.set("241.*")
    }

    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }

    publishPlugin {
        token.set(System.getenv("PUBLISH_TOKEN"))
    }

    test {
        useJUnitPlatform()
        testLogging {
            events("passed", "skipped", "failed")
        }
    }

    // Custom task for running integration tests
    register<Test>("integrationTest") {
        description = "Runs integration tests"
        group = "verification"
        
        useJUnitPlatform {
            includeTags("integration")
        }
        
        testClassesDirs = sourceSets["test"].output.classesDirs
        classpath = sourceSets["test"].runtimeClasspath
        
        shouldRunAfter("test")
    }

    // Custom task for running E2E tests
    register<Test>("e2eTest") {
        description = "Runs end-to-end tests"
        group = "verification"
        
        useJUnitPlatform {
            includeTags("e2e")
        }
        
        testClassesDirs = sourceSets["test"].output.classesDirs
        classpath = sourceSets["test"].runtimeClasspath
        
        shouldRunAfter("integrationTest")
    }

    // Custom task for running all tests
    register("allTests") {
        description = "Runs all tests (unit, integration, e2e)"
        group = "verification"
        
        dependsOn("test", "integrationTest", "e2eTest")
    }

    // Custom task for building the plugin
    register("buildPlugin") {
        description = "Builds the Rhema plugin"
        group = "build"
        
        dependsOn("buildPluginZip")
    }

    // Custom task for running the plugin in a sandbox
    register("runIde") {
        description = "Runs the plugin in a sandbox IDE"
        group = "development"
        
        dependsOn("buildPlugin")
    }

    // Custom task for debugging the plugin
    register("runIdeForUi") {
        description = "Runs the plugin in a sandbox IDE for UI testing"
        group = "development"
        
        dependsOn("buildPlugin")
    }
}

// Configure source sets
sourceSets {
    main {
        java {
            srcDirs("src/main/java")
        }
        kotlin {
            srcDirs("src/main/kotlin")
        }
        resources {
            srcDirs("src/main/resources")
        }
    }
    test {
        java {
            srcDirs("src/test/java")
        }
        kotlin {
            srcDirs("src/test/kotlin")
        }
        resources {
            srcDirs("src/test/resources")
        }
    }
}

// Configure test tasks
tasks.withType<Test> {
    useJUnitPlatform()
    
    // Set system properties for tests
    systemProperty("rhema.test.mode", "true")
    systemProperty("rhema.debug", "false")
    
    // Configure test logging
    testLogging {
        events("passed", "skipped", "failed")
        exceptionFormat = org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL
        showStandardStreams = true
    }
    
    // Configure test reports
    reports {
        html.required.set(true)
        junitXml.required.set(true)
    }
}

// Configure Kotlin
kotlin {
    jvmToolchain(17)
}

// Configure Java
java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

// Configure IntelliJ plugin
intellij {
    version.set("2023.3.4")
    type.set("IC")
    
    plugins.set(listOf(
        "java",
        "kotlin",
        "yaml",
        "git4idea",
        "terminal",
        "properties",
        "markdown",
        "json"
    ))
    
    // Plugin dependencies
    pluginName.set("rhema")
    description.set("Git-Based Agent Context Protocol integration for IntelliJ IDEs")
    version.set("0.1.0")
    vendor.set("Fugue AI")
    website.set("https://github.com/fugue-ai/rhema")
    issueTracker.set("https://github.com/fugue-ai/rhema/issues")
    changeNotes.set("""
        Initial release of Rhema plugin for IntelliJ IDEs.
        
        Features:
        - Context-aware IntelliSense and autocomplete
        - Integrated command palette and shortcuts
        - Real-time context validation and feedback
        - Visual context exploration and navigation
        - Integrated debugging and troubleshooting
        - Advanced refactoring and code generation
        - Git integration and version control
        - Performance profiling and optimization
        - Custom language support and syntax highlighting
    """.trimIndent())
    
    // Plugin configuration
    updateSinceUntilBuild.set(false)
    sameSinceUntilBuild.set(false)
    
    // Plugin signing
    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }
    
    // Plugin publishing
    publishPlugin {
        token.set(System.getenv("PUBLISH_TOKEN"))
        channels.set(listOf("stable"))
    }
} 