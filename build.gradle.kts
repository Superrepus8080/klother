import com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar

plugins {
    kotlin("jvm") version "1.9.25"
    application
    id("com.github.johnrengelman.shadow") version "8.1.1"
    id("gg.jte.gradle") version "3.1.12"
}

group = "com.klother"
version = "1.0.0-SNAPSHOT"

val vertxVersion = "4.5.11"
val jteVersion   = "3.1.12"
val mainVerticleName = "com.klother.MainVerticle"

repositories {
    mavenCentral()
}

dependencies {
    // ── Vert.x core ──────────────────────────────────────────────────────────
    implementation(platform("io.vertx:vertx-stack-depchain:$vertxVersion"))
    implementation("io.vertx:vertx-web")
    implementation("io.vertx:vertx-config")
    implementation("io.vertx:vertx-config-yaml")
    implementation("io.vertx:vertx-auth-jwt")
    implementation("io.vertx:vertx-web-client")   // calls Rust measure service

    // ── Kotlin ────────────────────────────────────────────────────────────────
    implementation("io.vertx:vertx-lang-kotlin")
    implementation("io.vertx:vertx-lang-kotlin-coroutines")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")

    // ── Database ──────────────────────────────────────────────────────────────
    implementation("io.vertx:vertx-pg-client")
    implementation("io.vertx:vertx-sql-client-templates")
    implementation("org.flywaydb:flyway-core:10.15.2")
    implementation("org.flywaydb:flyway-database-postgresql:10.15.2")
    implementation("org.postgresql:postgresql:42.7.3")

    // ── JTE templates ─────────────────────────────────────────────────────────
    implementation("io.vertx:vertx-web-templ-jte:$vertxVersion")
    implementation("gg.jte:jte:$jteVersion")
    implementation("gg.jte:jte-kotlin:$jteVersion")

    // ── Logging ───────────────────────────────────────────────────────────────
    implementation("ch.qos.logback:logback-classic:1.5.6")
    implementation("io.github.microutils:kotlin-logging-jvm:3.0.5")

    // ── Jackson (Vert.x uses Jackson under the hood) ──────────────────────────
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.17.2")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-yaml:2.17.2")

    // ── Testing ───────────────────────────────────────────────────────────────
    testImplementation("io.vertx:vertx-junit5")
    testImplementation("org.junit.jupiter:junit-jupiter:5.11.0")
    testImplementation(kotlin("test"))
}

// ── JTE precompilation ────────────────────────────────────────────────────────
jte {
    sourceDirectory.set(file("src/main/jte").toPath())
    targetDirectory.set(file("jte-classes").toPath())
    contentType.set(gg.jte.ContentType.Html)
    generate()
}

// ── Application entrypoint ────────────────────────────────────────────────────
application {
    mainClass.set("com.klother.MainKt")
}

// ── Shadow fat-JAR ────────────────────────────────────────────────────────────
tasks.withType<ShadowJar> {
    archiveClassifier.set("fat")
    manifest { attributes["Main-Class"] = "com.klother.MainKt" }
    mergeServiceFiles()
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions {
        jvmTarget = "21"
        freeCompilerArgs = listOf("-Xjsr305=strict")
    }
}

tasks.test {
    useJUnitPlatform()
}

java {
    toolchain { languageVersion.set(JavaLanguageVersion.of(21)) }
}
