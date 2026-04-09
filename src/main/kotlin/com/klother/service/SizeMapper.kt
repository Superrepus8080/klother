package com.klother.service

import com.klother.model.BodyMeasurements
import com.klother.model.Gender
import com.klother.model.ProductCategory
import com.klother.model.SizeLabel
import com.klother.model.SizeProfile
import com.klother.model.SizeRecommendation

/**
 * Maps raw body measurements (cm) to clothing size recommendations.
 *
 * Charts use ISO 8559 as a baseline; each brand's chart should override
 * these defaults via the [BrandSizeChart] extension point.
 *
 * Chest, waist, hip values are circumferences in cm.
 */
class SizeMapper {

    // ── Size chart: (chestMin, chestMax, waistMin, waistMax, hipMin, hipMax) ─

    private val womenTops = mapOf(
        SizeLabel.XS  to Bounds(chest = 76f..82f,  waist = 58f..63f,  hip = null),
        SizeLabel.S   to Bounds(chest = 82f..88f,  waist = 63f..68f,  hip = null),
        SizeLabel.M   to Bounds(chest = 88f..94f,  waist = 68f..73f,  hip = null),
        SizeLabel.L   to Bounds(chest = 94f..100f, waist = 73f..79f,  hip = null),
        SizeLabel.XL  to Bounds(chest = 100f..106f,waist = 79f..85f,  hip = null),
        SizeLabel.XXL to Bounds(chest = 106f..114f,waist = 85f..92f,  hip = null),
    )

    private val menTops = mapOf(
        SizeLabel.XS  to Bounds(chest = 82f..88f,  waist = null, hip = null),
        SizeLabel.S   to Bounds(chest = 88f..94f,  waist = null, hip = null),
        SizeLabel.M   to Bounds(chest = 94f..100f, waist = null, hip = null),
        SizeLabel.L   to Bounds(chest = 100f..107f,waist = null, hip = null),
        SizeLabel.XL  to Bounds(chest = 107f..114f,waist = null, hip = null),
        SizeLabel.XXL to Bounds(chest = 114f..122f,waist = null, hip = null),
    )

    private val womenBottoms = mapOf(
        SizeLabel.XS  to Bounds(chest = null, waist = 58f..62f, hip = 84f..88f),
        SizeLabel.S   to Bounds(chest = null, waist = 62f..66f, hip = 88f..92f),
        SizeLabel.M   to Bounds(chest = null, waist = 66f..70f, hip = 92f..96f),
        SizeLabel.L   to Bounds(chest = null, waist = 70f..75f, hip = 96f..101f),
        SizeLabel.XL  to Bounds(chest = null, waist = 75f..80f, hip = 101f..106f),
        SizeLabel.XXL to Bounds(chest = null, waist = 80f..86f, hip = 106f..112f),
    )

    private val menBottoms = mapOf(
        SizeLabel.XS  to Bounds(chest = null, waist = 68f..72f, hip = null),
        SizeLabel.S   to Bounds(chest = null, waist = 72f..76f, hip = null),
        SizeLabel.M   to Bounds(chest = null, waist = 76f..81f, hip = null),
        SizeLabel.L   to Bounds(chest = null, waist = 81f..86f, hip = null),
        SizeLabel.XL  to Bounds(chest = null, waist = 86f..92f, hip = null),
        SizeLabel.XXL to Bounds(chest = null, waist = 92f..98f, hip = null),
    )

    // ── Public API ────────────────────────────────────────────────────────────

    fun buildProfile(m: BodyMeasurements, gender: Gender): SizeProfile {
        val topsChart    = if (gender == Gender.WOMEN) womenTops    else menTops
        val bottomsChart = if (gender == Gender.WOMEN) womenBottoms else menBottoms

        return SizeProfile(
            tops    = recommend(m, topsChart,    ProductCategory.TOPS),
            bottoms = recommend(m, bottomsChart, ProductCategory.BOTTOMS),
            dresses = if (gender == Gender.WOMEN) recommend(m, womenTops, ProductCategory.DRESSES) else null,
        )
    }

    // ── Internal ──────────────────────────────────────────────────────────────

    private fun recommend(
        m:      BodyMeasurements,
        chart:  Map<SizeLabel, Bounds>,
        cat:    ProductCategory,
    ): SizeRecommendation {
        val matched = chart.entries.filter { (_, bounds) ->
            val chestOk = bounds.chest?.contains(m.chestCm) ?: true
            val waistOk = bounds.waist?.contains(m.waistCm) ?: true
            val hipOk   = bounds.hip?.contains(m.hipCm)     ?: true
            val relevant = when (cat) {
                ProductCategory.TOPS, ProductCategory.DRESSES -> chestOk && waistOk
                ProductCategory.BOTTOMS                        -> waistOk && hipOk
                else                                           -> chestOk
            }
            relevant
        }.map { it.key }

        val sizeOrder = SizeLabel.entries
        val primary   = matched.firstOrNull() ?: closestSize(m, chart, cat)
        val idx       = sizeOrder.indexOf(primary)

        val alsoFits  = listOfNotNull(
            sizeOrder.getOrNull(idx - 1),
            sizeOrder.getOrNull(idx + 1),
        ).filter { it != primary }

        // Confidence: 1 exact match = high, >1 = ambiguous, 0 = estimated
        val confidence = when (matched.size) {
            1    -> 0.95f
            0    -> 0.75f
            else -> 0.82f
        }

        return SizeRecommendation(primary, alsoFits, confidence)
    }

    /** Fallback: find the size whose chest bound midpoint is nearest to the measurement. */
    private fun closestSize(m: BodyMeasurements, chart: Map<SizeLabel, Bounds>, cat: ProductCategory): SizeLabel {
        return chart.minByOrNull { (_, b) ->
            val ref = when (cat) {
                ProductCategory.BOTTOMS -> b.waist?.midpoint() ?: Float.MAX_VALUE
                else                   -> b.chest?.midpoint() ?: Float.MAX_VALUE
            }
            kotlin.math.abs(ref - m.chestCm)
        }?.key ?: SizeLabel.M
    }

    private data class Bounds(
        val chest: ClosedFloatingPointRange<Float>?,
        val waist: ClosedFloatingPointRange<Float>?,
        val hip:   ClosedFloatingPointRange<Float>?,
    )

    private fun ClosedFloatingPointRange<Float>.midpoint() = (start + endInclusive) / 2f
}
