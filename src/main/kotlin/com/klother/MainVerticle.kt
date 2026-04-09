package com.klother

import com.klother.db.Database
import com.klother.router.ApiRouter
import com.klother.router.WebRouter
import com.klother.service.MeasurementClient
import com.klother.service.SizeMapper
import com.klother.service.UserService
import io.vertx.core.http.HttpServerOptions
import io.vertx.ext.web.Router
import io.vertx.ext.web.handler.BodyHandler
import io.vertx.ext.web.handler.SessionHandler
import io.vertx.ext.web.handler.StaticHandler
import io.vertx.ext.web.sstore.LocalSessionStore
import io.vertx.ext.web.templ.jte.JteTemplateEngine
import io.vertx.kotlin.coroutines.CoroutineVerticle
import io.vertx.kotlin.coroutines.coAwait
import mu.KotlinLogging
import java.nio.file.Path

private val log = KotlinLogging.logger {}

class MainVerticle : CoroutineVerticle() {

    override suspend fun start() {
        // ── Config ────────────────────────────────────────────────────────────
        val cfg = config.getJsonObject("app") ?: io.vertx.core.json.JsonObject()
        val port             = cfg.getInteger("port", 8080)
        val measureServiceUrl = cfg.getString("measureServiceUrl", "http://localhost:9090")
        val devMode          = cfg.getBoolean("devMode", true)

        // ── Database migrations ───────────────────────────────────────────────
        val db = Database(cfg.getJsonObject("db") ?: io.vertx.core.json.JsonObject())
        vertx.executeBlocking { db.migrate() }.coAwait()
        val pgPool = db.createPool(vertx)

        // ── Services ──────────────────────────────────────────────────────────
        val measureClient = MeasurementClient(vertx, measureServiceUrl)
        val sizeMapper    = SizeMapper()
        val userService   = UserService(pgPool)

        // ── JTE template engine ───────────────────────────────────────────────
        // In devMode: resolves templates from disk (hot-reload).
        // In production: uses precompiled classes from jte-classes/.
        val templateEngine = if (devMode) {
            JteTemplateEngine.create(vertx, Path.of("src/main/jte"))
        } else {
            JteTemplateEngine.create(vertx)
        }

        // ── Root router ───────────────────────────────────────────────────────
        val router = Router.router(vertx)

        // Global body handler (max 20 MB — for photo uploads)
        router.route().handler(BodyHandler.create().setBodyLimit(20 * 1024 * 1024))

        // Session store (in-memory for now; swap for Redis in production)
        router.route().handler(SessionHandler.create(LocalSessionStore.create(vertx)))

        // Static assets (Tailwind output, images, JS)
        router.route("/static/*").handler(StaticHandler.create("static"))

        // ── Sub-routers ───────────────────────────────────────────────────────
        WebRouter(templateEngine, userService, sizeMapper).attach(router)
        ApiRouter(measureClient, sizeMapper, userService).attach(router)

        // ── HTTP server ───────────────────────────────────────────────────────
        vertx.createHttpServer(HttpServerOptions().setPort(port))
            .requestHandler(router)
            .listen()
            .coAwait()

        log.info { "HTTP server listening on port $port (devMode=$devMode)" }
    }
}
