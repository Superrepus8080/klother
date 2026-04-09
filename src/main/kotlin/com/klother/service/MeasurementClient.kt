package com.klother.service

import com.klother.model.BodyMeasurements
import com.klother.model.MeasurementRequest
import io.vertx.core.Vertx
import io.vertx.core.buffer.Buffer
import io.vertx.ext.web.client.WebClient
import io.vertx.ext.web.client.WebClientOptions
import io.vertx.ext.web.multipart.MultipartForm
import io.vertx.kotlin.coroutines.coAwait
import mu.KotlinLogging

private val log = KotlinLogging.logger {}

/**
 * HTTP client that calls the Rust measurement service.
 *
 * The Rust service (Axum) runs at [baseUrl] and exposes:
 *   POST /measure
 *     multipart fields: front_photo (binary), side_photo (binary),
 *                       height_cm (float), gender (string)
 *   → JSON: { chest_cm, waist_cm, hip_cm, shoulder_width_cm, inseam_cm }
 *
 * For production at scale, upgrade to gRPC (tonic ↔ vertx-grpc-client)
 * and stream the photo bytes rather than buffering them.
 */
class MeasurementClient(vertx: Vertx, private val baseUrl: String) {

    private val client = WebClient.create(
        vertx,
        WebClientOptions()
            .setMaxPoolSize(20)
            .setConnectTimeout(5_000)
            .setIdleTimeout(30)           // seconds
            .setKeepAlive(true)
    )

    suspend fun measure(req: MeasurementRequest): BodyMeasurements {
        log.debug { "Calling Rust measure service at $baseUrl (height=${req.heightCm}cm)" }

        val form = MultipartForm.create()
            .binaryFileUpload(
                "front_photo",
                "front.jpg",
                Buffer.buffer(req.frontPhotoBytes),
                "image/jpeg"
            )
            .binaryFileUpload(
                "side_photo",
                "side.jpg",
                Buffer.buffer(req.sidePhotoBytes),
                "image/jpeg"
            )
            .attribute("height_cm", req.heightCm.toString())
            .attribute("gender",    req.gender.name.lowercase())

        val response = client.postAbs("$baseUrl/measure")
            .sendMultipartForm(form)
            .coAwait()

        if (response.statusCode() != 200) {
            val body = response.bodyAsString()
            log.error { "Measure service error ${response.statusCode()}: $body" }
            throw MeasurementException("Measurement service returned ${response.statusCode()}: $body")
        }

        val json = response.bodyAsJsonObject()
        return BodyMeasurements.fromJson(json)
    }
}

class MeasurementException(message: String) : RuntimeException(message)
