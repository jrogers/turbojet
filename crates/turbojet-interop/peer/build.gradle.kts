// A QuickFIX/J session driven over stdin/stdout by turbojet-interop's tests.

plugins {
    java
    id("com.gradleup.shadow") version "9.6.1"
}

repositories {
    mavenCentral()
}

// Any JDK from 21 builds it.
tasks.withType<JavaCompile> {
    options.release = 21
}

val quickfixj = "3.0.2"

dependencies {
    implementation("org.quickfixj:quickfixj-core:$quickfixj")
    // The data dictionaries (FIX42.xml, FIX43.xml, FIX44.xml, FIXT11.xml, FIX50SP2.xml) and message
    // classes.
    implementation("org.quickfixj:quickfixj-messages-fix42:$quickfixj")
    implementation("org.quickfixj:quickfixj-messages-fix43:$quickfixj")
    implementation("org.quickfixj:quickfixj-messages-fix44:$quickfixj")
    implementation("org.quickfixj:quickfixj-messages-fixt11:$quickfixj")
    implementation("org.quickfixj:quickfixj-messages-fix50sp2:$quickfixj")
    runtimeOnly("org.slf4j:slf4j-simple:2.0.18")
}

tasks.shadowJar {
    archiveFileName = "turbojet-interop-peer-all.jar"
    manifest {
        attributes["Main-Class"] = "dev.turbojet.interop.Peer"
    }
    mergeServiceFiles()
}
