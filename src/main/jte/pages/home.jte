@import java.util.List
@param String pageTitle
@param String activePage
@param List<String> categories

@template.layout.base(
    pageTitle  = pageTitle,
    activePage = activePage,
    content    = @`

    <%-- ── Hero ────────────────────────────────────────────────────────────── --%>
    <section class="bg-brand text-white py-24 px-4 text-center">
        <div class="mx-auto max-w-3xl">
            <p class="text-brand-accent text-sm font-semibold uppercase tracking-widest mb-3">
                Fashion That Fits You
            </p>
            <h1 class="text-5xl font-bold leading-tight mb-6">
                Shop Clothes Perfectly <br/> Matched to Your Body
            </h1>
            <p class="text-white/70 text-lg mb-10 max-w-xl mx-auto">
                Upload two photos, enter your height, and we'll build your size profile —
                then show you only the clothes that fit. Accurate to ±2 cm.
            </p>
            <div class="flex flex-col sm:flex-row gap-4 justify-center">
                <a href="/measure"
                   class="rounded-full bg-brand-accent px-8 py-3 font-semibold hover:bg-brand-accent/90 transition">
                    Get My Size →
                </a>
                <a href="/shop"
                   class="rounded-full border border-white/30 px-8 py-3 font-semibold hover:bg-white/10 transition">
                    Browse All
                </a>
            </div>
        </div>
    </section>

    <%-- ── How it works ──────────────────────────────────────────────────────── --%>
    <section class="py-20 px-4">
        <div class="mx-auto max-w-5xl">
            <h2 class="text-3xl font-bold text-center mb-14">How It Works</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-10 text-center">

                <div class="flex flex-col items-center gap-4">
                    <div class="w-16 h-16 rounded-full bg-brand-accent/10 text-brand-accent
                                flex items-center justify-center text-2xl font-bold">1</div>
                    <h3 class="text-lg font-semibold">Upload 2 Photos</h3>
                    <p class="text-brand/60 text-sm leading-relaxed">
                        Stand in front of a wall. Take one front photo, one side photo.
                        Wear fitted clothes for best results.
                    </p>
                </div>

                <div class="flex flex-col items-center gap-4">
                    <div class="w-16 h-16 rounded-full bg-brand-accent/10 text-brand-accent
                                flex items-center justify-center text-2xl font-bold">2</div>
                    <h3 class="text-lg font-semibold">AI Reads Your Body</h3>
                    <p class="text-brand/60 text-sm leading-relaxed">
                        Our computer vision model detects 33 body landmarks and calculates
                        your chest, waist, hip, and inseam — in seconds.
                    </p>
                </div>

                <div class="flex flex-col items-center gap-4">
                    <div class="w-16 h-16 rounded-full bg-brand-accent/10 text-brand-accent
                                flex items-center justify-center text-2xl font-bold">3</div>
                    <h3 class="text-lg font-semibold">Shop in Your Size</h3>
                    <p class="text-brand/60 text-sm leading-relaxed">
                        We filter the entire catalog to show only things that fit.
                        Every product shows your personal fit score.
                    </p>
                </div>
            </div>
        </div>
    </section>

    <%-- ── Categories ──────────────────────────────────────────────────────── --%>
    <section class="py-16 px-4 bg-white">
        <div class="mx-auto max-w-5xl">
            <h2 class="text-2xl font-bold mb-8">Shop by Category</h2>
            <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-4">
                @for(String cat : categories)
                <a href="/shop?category=${cat.toLowerCase()}"
                   class="rounded-2xl bg-brand-muted hover:bg-brand-accent/10 border border-brand/10
                          py-8 text-center font-medium text-brand hover:text-brand-accent transition group">
                    ${cat}
                </a>
                @endfor
            </div>
        </div>
    </section>

    `)
