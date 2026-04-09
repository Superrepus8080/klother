package com.klother.db

import io.vertx.core.Vertx
import io.vertx.pgclient.PgConnectOptions
import io.vertx.pgclient.PgPool
import io.vertx.sqlclient.PoolOptions
import io.vertx.core.json.JsonObject
import mu.KotlinLogging
import org.flywaydb.core.Flyway

private val log = KotlinLogging.logger {}

class Database(private val cfg: JsonObject) {

    private val host     = cfg.getString("host",     "localhost")
    private val port     = cfg.getInteger("port",    5432)
    private val name     = cfg.getString("name",     "klother")
    private val user     = cfg.getString("user",     "klother")
    private val password = cfg.getString("password", "klother")

    /** Run Flyway migrations. Call from a blocking context (vertx.executeBlocking). */
    fun migrate() {
        log.info { "Running database migrations against $host:$port/$name" }
        Flyway.configure()
            .dataSource("jdbc:postgresql://$host:$port/$name", user, password)
            .locations("classpath:db/migration")
            .load()
            .migrate()
        log.info { "Migrations complete" }
    }

    /** Create a reactive PostgreSQL connection pool. */
    fun createPool(vertx: Vertx): PgPool {
        val connectOptions = PgConnectOptions()
            .setHost(host)
            .setPort(port)
            .setDatabase(name)
            .setUser(user)
            .setPassword(password)

        val poolOptions = PoolOptions().setMaxSize(10)

        return PgPool.pool(vertx, connectOptions, poolOptions)
    }
}
