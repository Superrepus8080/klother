package com.klother

import io.vertx.core.DeploymentOptions
import io.vertx.core.Vertx
import io.vertx.core.VertxOptions
import io.vertx.core.json.JsonObject
import mu.KotlinLogging

private val log = KotlinLogging.logger {}

fun main() {
    val vertx = Vertx.vertx(
        VertxOptions()
            .setWorkerPoolSize(20)
            .setEventLoopPoolSize(Runtime.getRuntime().availableProcessors())
    )

    // devMode=true  → JTE compiles templates from src/main/jte at runtime (for local dev with ./gradlew run)
    // devMode=false → JTE uses precompiled classes bundled in the fat JAR (for java -jar)
    val devMode = System.getProperty("klother.devMode", "false").toBoolean()
    val config = JsonObject().put("app", JsonObject().put("devMode", devMode))

    vertx.deployVerticle(MainVerticle(), DeploymentOptions().setConfig(config)) { ar ->
        if (ar.succeeded()) {
            log.info { "Klother started  →  http://localhost:8080  (devMode=$devMode)" }
        } else {
            log.error(ar.cause()) { "Failed to start Klother" }
            vertx.close()
        }
    }
}
