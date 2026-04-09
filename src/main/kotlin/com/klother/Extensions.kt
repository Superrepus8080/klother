package com.klother

import io.vertx.ext.web.Route
import io.vertx.ext.web.RoutingContext
import io.vertx.kotlin.coroutines.dispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

/**
 * Attach a suspend handler to a Vert.x route.
 *
 * Vert.x 4.x ships CoroutineRouterSupport (coHandler/coFailureHandler) which
 * requires the caller to implement CoroutineScope. This standalone extension
 * is equivalent but self-contained: it derives the dispatcher from the
 * RoutingContext's Vertx instance so no external scope is needed.
 */
fun Route.coroutineHandler(fn: suspend (RoutingContext) -> Unit): Route =
    handler { ctx ->
        CoroutineScope(ctx.vertx().dispatcher()).launch {
            try {
                fn(ctx)
            } catch (e: Exception) {
                ctx.fail(e)
            }
        }
    }
