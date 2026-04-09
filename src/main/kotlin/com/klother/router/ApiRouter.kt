package com.klother.router

import com.klother.model.Gender
import com.klother.model.MeasurementRequest
import com.klother.service.MeasurementClient
import com.klother.service.MeasurementException
import com.klother.service.SizeMapper
import com.klother.service.UserService
import com.klother.coroutineHandler
import io.vertx.core.json.JsonObject
import io.vertx.ext.web.Router
import io.vertx.ext.web.RoutingContext
import io.vertx.kotlin.coroutines.coAwait
import mu.KotlinLogging

private val log = KotlinLogging.logger {}

/**
 * JSON API routes consumed by HTMX fragments and Alpine.js components.
 *
 * POST /api/measure  — accepts multipart photo upload, calls Rust service,
 *                       stores result in session, returns JSON + HTMX redirect.
 */
class ApiRouter(
    private val measureClient: MeasurementClient,
    private val sizeMapper:    SizeMapper,
    private val userService:   UserService,
) {

    fun attach(router: Router) {
        router.post("/api/measure").coroutineHandler(::handleMeasure)
        router.get("/api/health").handler  { ctx ->
            ctx.response()
                .putHeader("Content-Type", "application/json")
                .end(JsonObject().put("status", "ok").encode())
        }
    }

    // ── POST /api/measure ─────────────────────────────────────────────────────

    private suspend fun handleMeasure(ctx: RoutingContext) {
        val form      = ctx.request().formAttributes()
        val fileUploads = ctx.fileUploads()

        // Validate required fields
        val heightCm = form.get("height_cm")?.toFloatOrNull()
            ?: return badRequest(ctx, "height_cm is required and must be a number")

        val gender = try {
            Gender.valueOf((form.get("gender") ?: "WOMEN").uppercase())
        } catch (e: IllegalArgumentException) {
            return badRequest(ctx, "gender must be WOMEN, MEN, or NON_BINARY")
        }

        if (heightCm < 130f || heightCm > 220f) {
            return badRequest(ctx, "height_cm must be between 130 and 220")
        }

        val frontUpload = fileUploads.find { it.name() == "front_photo" }
            ?: return badRequest(ctx, "front_photo is required")
        val sideUpload  = fileUploads.find { it.name() == "side_photo" }
            ?: return badRequest(ctx, "side_photo is required")

        val frontBytes = ctx.vertx().fileSystem()
            .readFile(frontUpload.uploadedFileName()).coAwait().bytes
        val sideBytes  = ctx.vertx().fileSystem()
            .readFile(sideUpload.uploadedFileName()).coAwait().bytes

        // ── Call Rust measurement service ─────────────────────────────────────
        val measurements = try {
            measureClient.measure(
                MeasurementRequest(heightCm, gender, frontBytes, sideBytes)
            )
        } catch (e: MeasurementException) {
            log.warn { "Measurement failed: ${e.message}" }
            ctx.response()
                .setStatusCode(422)
                .putHeader("Content-Type", "application/json")
                .end(JsonObject().put("error", e.message).encode())
            return
        }

        // ── Map to size profile ────────────────────────────────────────────────
        val sizeProfile = sizeMapper.buildProfile(measurements, gender)

        // ── Persist to session (and optionally to DB if user is logged in) ─────
        ctx.session().put("sizeProfile",  sizeProfile.toJson())
        ctx.session().put("measurements", measurements.toJson())
        ctx.session().put("recommendedTopsSize", sizeProfile.tops.primary.name)

        // If authenticated, save to user record
        ctx.session().get<Long>("userId")?.let { userId ->
            userService.saveMeasurements(userId, measurements)
        }

        // ── Respond ────────────────────────────────────────────────────────────
        // HTMX expects HX-Redirect header to navigate the full page.
        ctx.response()
            .putHeader("Content-Type", "application/json")
            .putHeader("HX-Redirect", "/results")
            .setStatusCode(200)
            .end(JsonObject()
                .put("measurements", measurements.toJson())
                .put("size_profile", sizeProfile.toJson())
                .encode()
            )
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    private fun badRequest(ctx: RoutingContext, msg: String) {
        // Assign Future to local var so the function body evaluates to Unit,
        // not Future<Void> (which would break callers using `return badRequest(...)`)
        val ignored = ctx.response()
            .setStatusCode(400)
            .putHeader("Content-Type", "application/json")
            .end(JsonObject().put("error", msg).encode())
    }
}
