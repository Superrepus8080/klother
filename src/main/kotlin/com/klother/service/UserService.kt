package com.klother.service

import com.klother.model.BodyMeasurements
import com.klother.model.Gender
import com.klother.model.SizeProfile
import com.klother.model.User
import io.vertx.pgclient.PgPool
import io.vertx.sqlclient.Tuple
import io.vertx.kotlin.coroutines.coAwait
import mu.KotlinLogging

private val log = KotlinLogging.logger {}

class UserService(private val pool: PgPool) {

    suspend fun findById(id: Long): User? {
        val rows = pool.preparedQuery(
            """
            SELECT id, email, display_name, gender,
                   chest_cm, waist_cm, hip_cm, shoulder_width_cm, inseam_cm, height_cm
            FROM users WHERE id = $1
            """
        ).execute(Tuple.of(id)).coAwait()

        return rows.firstOrNull()?.let { row ->
            val hasMeasurements = row.getFloat("chest_cm") != null
            User(
                id          = row.getLong("id"),
                email       = row.getString("email"),
                displayName = row.getString("display_name"),
                gender      = Gender.valueOf(row.getString("gender").uppercase()),
                measurements = if (hasMeasurements) BodyMeasurements(
                    heightCm        = row.getFloat("height_cm"),
                    chestCm         = row.getFloat("chest_cm"),
                    waistCm         = row.getFloat("waist_cm"),
                    hipCm           = row.getFloat("hip_cm"),
                    shoulderWidthCm = row.getFloat("shoulder_width_cm"),
                    inseamCm        = row.getFloat("inseam_cm"),
                ) else null,
                sizeProfile = null,   // loaded separately if needed
            )
        }
    }

    suspend fun saveMeasurements(userId: Long, m: BodyMeasurements) {
        pool.preparedQuery(
            """
            UPDATE users
            SET chest_cm = $2, waist_cm = $3, hip_cm = $4,
                shoulder_width_cm = $5, inseam_cm = $6, height_cm = $7,
                measurements_updated_at = NOW()
            WHERE id = $1
            """
        ).execute(
            Tuple.of(userId, m.chestCm, m.waistCm, m.hipCm, m.shoulderWidthCm, m.inseamCm, m.heightCm)
        ).coAwait()

        log.info { "Saved measurements for user $userId" }
    }

    suspend fun createUser(email: String, displayName: String, gender: Gender): Long {
        val rows = pool.preparedQuery(
            "INSERT INTO users (email, display_name, gender) VALUES ($1, $2, $3) RETURNING id"
        ).execute(Tuple.of(email, displayName, gender.name.lowercase())).coAwait()
        return rows.first().getLong("id")
    }
}
