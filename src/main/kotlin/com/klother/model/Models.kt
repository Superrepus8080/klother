package com.klother.model

import io.vertx.core.json.JsonObject

// ── Body measurements (all in cm) ────────────────────────────────────────────

data class BodyMeasurements(
    val heightCm:         Float,
    val chestCm:          Float,
    val waistCm:          Float,
    val hipCm:            Float,
    val shoulderWidthCm:  Float,
    val inseamCm:         Float,
) {
    fun toJson(): JsonObject = JsonObject()
        .put("height_cm",          heightCm)
        .put("chest_cm",           chestCm)
        .put("waist_cm",           waistCm)
        .put("hip_cm",             hipCm)
        .put("shoulder_width_cm",  shoulderWidthCm)
        .put("inseam_cm",          inseamCm)

    companion object {
        fun fromJson(j: JsonObject) = BodyMeasurements(
            heightCm         = j.getFloat("height_cm"),
            chestCm          = j.getFloat("chest_cm"),
            waistCm          = j.getFloat("waist_cm"),
            hipCm            = j.getFloat("hip_cm"),
            shoulderWidthCm  = j.getFloat("shoulder_width_cm"),
            inseamCm         = j.getFloat("inseam_cm"),
        )
    }
}

// ── Size recommendation ───────────────────────────────────────────────────────

enum class SizeLabel { XS, S, M, L, XL, XXL }

data class SizeRecommendation(
    val primary:    SizeLabel,
    val alsoFits:   List<SizeLabel>,
    val confidence: Float,            // 0.0 – 1.0
) {
    fun toJson(): JsonObject = JsonObject()
        .put("recommended_size", primary.name)
        .put("also_fits",        alsoFits.map { it.name })
        .put("confidence",       confidence)
}

data class SizeProfile(
    val tops:    SizeRecommendation,
    val bottoms: SizeRecommendation,
    val dresses: SizeRecommendation?,
) {
    fun toJson(): JsonObject = JsonObject()
        .put("tops",    tops.toJson())
        .put("bottoms", bottoms.toJson())
        .put("dresses", dresses?.toJson())
}

// ── User ──────────────────────────────────────────────────────────────────────

data class User(
    val id:          Long,
    val email:       String,
    val displayName: String,
    val gender:      Gender,
    val measurements: BodyMeasurements?,
    val sizeProfile:  SizeProfile?,
)

enum class Gender { WOMEN, MEN, NON_BINARY }

// ── Product ───────────────────────────────────────────────────────────────────

data class Product(
    val id:          Long,
    val name:        String,
    val brand:       String,
    val category:    ProductCategory,
    val priceGbp:    Double,
    val imageUrl:    String,
    val sizes:       List<SizeLabel>,
    val description: String,
)

enum class ProductCategory { TOPS, BOTTOMS, DRESSES, OUTERWEAR, SHOES, ACCESSORIES }

// ── Measurement request (from the upload form) ────────────────────────────────

data class MeasurementRequest(
    val heightCm:       Float,
    val gender:         Gender,
    val frontPhotoBytes: ByteArray,
    val sidePhotoBytes:  ByteArray,
)
