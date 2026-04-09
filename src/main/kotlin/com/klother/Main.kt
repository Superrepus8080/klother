package com.klother

import io.vertx.core.Vertx
import io.vertx.core.VertxOptions
import mu.KotlinLogging

private val log = KotlinLogging.logger {}

fun main() {
    val vertx = Vertx.vertx(
        VertxOptions()
            .setWorkerPoolSize(20)
            .setEventLoopPoolSize(Runtime.getRuntime().availableProcessors())
    )

    vertx.deployVerticle(MainVerticle()) { ar ->
        if (ar.succeeded()) {
            log.info { "Klother started  →  http://localhost:8080" }
        } else {
            log.error(ar.cause()) { "Failed to start Klother" }
            vertx.close()
        }
    }
}
