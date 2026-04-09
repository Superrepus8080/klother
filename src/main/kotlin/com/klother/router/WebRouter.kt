package com.klother.router

import com.klother.model.Gender
import com.klother.service.SizeMapper
import com.klother.service.UserService
import io.vertx.ext.web.Router
import io.vertx.ext.web.RoutingContext
import io.vertx.ext.web.templ.jte.JteTemplateEngine
import io.vertx.kotlin.coroutines.coAwait
import io.vertx.kotlin.coroutines.coroutineHandler
import mu.KotlinLogging

private val log = KotlinLogging.logger {}

/**
 * Server-side rendered routes using JTE templates + Tailwind CSS.
 *
 * Each handler puts values into RoutingContext data then hands off to the
 * JTE engine, which compiles the template to Kotlin/Java bytecode — giving
 * you type-checked, fast server-side rendering with no runtime string parsing.
 */
class WebRouter(
    private val engine:      JteTemplateEngine,
    private val userService: UserService,
    private val sizeMapper:  SizeMapper,
) {

    fun attach(router: Router) {
        router.get("/").coroutineHandler(::home)
        router.get("/measure").coroutineHandler(::measurePage)
        router.get("/results").coroutineHandler(::resultsPage)
        router.get("/shop").coroutineHandler(::shopPage)
        router.get("/account").coroutineHandler(::accountPage)
        router.get("/account/register").coroutineHandler(::registerPage)
    }

    // ── Pages ─────────────────────────────────────────────────────────────────

    private suspend fun home(ctx: RoutingContext) {
        ctx.data()["pageTitle"]   = "Klother — Fashion in Your Size"
        ctx.data()["activePage"]  = "home"
        ctx.data()["categories"]  = listOf("Tops", "Bottoms", "Dresses", "Outerwear", "Shoes")
        render(ctx, "pages/home.jte")
    }

    private suspend fun measurePage(ctx: RoutingContext) {
        ctx.data()["pageTitle"]  = "Get Your Size — Klother"
        ctx.data()["activePage"] = "measure"
        render(ctx, "pages/measure.jte")
    }

    private suspend fun resultsPage(ctx: RoutingContext) {
        val session = ctx.session()
        val profile = session.get<io.vertx.core.json.JsonObject>("sizeProfile")
        val measurements = session.get<io.vertx.core.json.JsonObject>("measurements")

        if (profile == null) {
            ctx.redirect("/measure")
            return
        }

        ctx.data()["pageTitle"]    = "Your Size Profile — Klother"
        ctx.data()["activePage"]   = "measure"
        ctx.data()["sizeProfile"]  = profile
        ctx.data()["measurements"] = measurements
        render(ctx, "pages/results.jte")
    }

    private suspend fun shopPage(ctx: RoutingContext) {
        ctx.data()["pageTitle"]  = "Shop — Klother"
        ctx.data()["activePage"] = "shop"

        // Read size filter from session or query param
        val sizeFilter = ctx.queryParam("size").firstOrNull()
            ?: ctx.session().get<String>("recommendedTopsSize")

        ctx.data()["sizeFilter"] = sizeFilter
        // TODO: load paginated products from DB filtered by sizeFilter
        ctx.data()["products"] = emptyList<Any>()
        render(ctx, "pages/shop.jte")
    }

    private suspend fun accountPage(ctx: RoutingContext) {
        ctx.data()["pageTitle"]  = "My Account — Klother"
        ctx.data()["activePage"] = "account"
        // TODO: load user from session / JWT
        render(ctx, "pages/account.jte")
    }

    private suspend fun registerPage(ctx: RoutingContext) {
        ctx.data()["pageTitle"]  = "Create Account — Klother"
        ctx.data()["activePage"] = "account"
        render(ctx, "pages/register.jte")
    }

    // ── Render helper ─────────────────────────────────────────────────────────

    private suspend fun render(ctx: RoutingContext, template: String) {
        try {
            val buf = engine.render(ctx.data(), template).coAwait()
            ctx.response()
                .putHeader("Content-Type", "text/html; charset=UTF-8")
                .end(buf)
        } catch (e: Exception) {
            log.error(e) { "Template render failed: $template" }
            ctx.fail(500, e)
        }
    }
}
