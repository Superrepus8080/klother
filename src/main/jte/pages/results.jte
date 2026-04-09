@import io.vertx.core.json.JsonObject
@param String pageTitle
@param String activePage
@param JsonObject sizeProfile
@param JsonObject measurements

@template.layout.base(
    pageTitle  = pageTitle,
    activePage = activePage,
    content    = @`

    <div class="mx-auto max-w-2xl px-4 py-16">

        <%-- Header --%>
        <div class="text-center mb-12">
            <div class="inline-flex items-center gap-2 bg-green-100 text-green-700
                        text-sm font-semibold px-4 py-2 rounded-full mb-4">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0
                          00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0
                          001.414 0l4-4z" clip-rule="evenodd"/>
                </svg>
                Size profile created
            </div>
            <h1 class="text-3xl font-bold">Your Size Profile</h1>
            <p class="text-brand/60 mt-2">Based on your body measurements — always shown first when shopping.</p>
        </div>

        <%-- Size cards --%>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-10">

            @if(sizeProfile != null && sizeProfile.getJsonObject("tops") != null)
            !{var tops = sizeProfile.getJsonObject("tops");}
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-brand/5 text-center">
                <p class="text-xs text-brand/40 uppercase tracking-widest font-semibold mb-1">Tops</p>
                <p class="text-5xl font-bold text-brand-accent mb-2">${tops.getString("recommended_size", "—")}</p>
                <p class="text-xs text-brand/50">
                    Also fits:
                    @for(Object alt : tops.getJsonArray("also_fits", new io.vertx.core.json.JsonArray()))
                    <span class="font-medium">${alt}</span>
                    @endfor
                </p>
                <div class="mt-3 h-1.5 rounded-full bg-brand/10 overflow-hidden">
                    <div class="h-full bg-brand-accent rounded-full"
                         style="width: ${(int)(tops.getFloat("confidence", 0.9f) * 100)}%"></div>
                </div>
                <p class="text-xs text-brand/40 mt-1">${(int)(tops.getFloat("confidence", 0.9f) * 100)}% confidence</p>
            </div>
            @endif

            @if(sizeProfile != null && sizeProfile.getJsonObject("bottoms") != null)
            !{var bottoms = sizeProfile.getJsonObject("bottoms");}
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-brand/5 text-center">
                <p class="text-xs text-brand/40 uppercase tracking-widest font-semibold mb-1">Bottoms</p>
                <p class="text-5xl font-bold text-brand-accent mb-2">${bottoms.getString("recommended_size", "—")}</p>
                <p class="text-xs text-brand/50">
                    Also fits:
                    @for(Object alt : bottoms.getJsonArray("also_fits", new io.vertx.core.json.JsonArray()))
                    <span class="font-medium">${alt}</span>
                    @endfor
                </p>
                <div class="mt-3 h-1.5 rounded-full bg-brand/10 overflow-hidden">
                    <div class="h-full bg-brand-accent rounded-full"
                         style="width: ${(int)(bottoms.getFloat("confidence", 0.9f) * 100)}%"></div>
                </div>
                <p class="text-xs text-brand/40 mt-1">${(int)(bottoms.getFloat("confidence", 0.9f) * 100)}% confidence</p>
            </div>
            @endif

            @if(sizeProfile != null && sizeProfile.getJsonObject("dresses") != null)
            !{var dresses = sizeProfile.getJsonObject("dresses");}
            <div class="bg-white rounded-2xl p-6 shadow-sm border border-brand/5 text-center">
                <p class="text-xs text-brand/40 uppercase tracking-widest font-semibold mb-1">Dresses</p>
                <p class="text-5xl font-bold text-brand-accent mb-2">${dresses.getString("recommended_size", "—")}</p>
                <p class="text-xs text-brand/50">
                    Also fits:
                    @for(Object alt : dresses.getJsonArray("also_fits", new io.vertx.core.json.JsonArray()))
                    <span class="font-medium">${alt}</span>
                    @endfor
                </p>
                <div class="mt-3 h-1.5 rounded-full bg-brand/10 overflow-hidden">
                    <div class="h-full bg-brand-accent rounded-full"
                         style="width: ${(int)(dresses.getFloat("confidence", 0.9f) * 100)}%"></div>
                </div>
                <p class="text-xs text-brand/40 mt-1">${(int)(dresses.getFloat("confidence", 0.9f) * 100)}% confidence</p>
            </div>
            @endif
        </div>

        <%-- Raw measurements breakdown --%>
        @if(measurements != null)
        <div class="bg-white rounded-2xl p-6 shadow-sm border border-brand/5 mb-8">
            <h2 class="font-semibold mb-4">Your measurements (cm)</h2>
            <div class="grid grid-cols-3 gap-4 text-center text-sm">
                <div>
                    <p class="text-2xl font-bold">${measurements.getFloat("chest_cm", 0f)}</p>
                    <p class="text-brand/50 text-xs">Chest</p>
                </div>
                <div>
                    <p class="text-2xl font-bold">${measurements.getFloat("waist_cm", 0f)}</p>
                    <p class="text-brand/50 text-xs">Waist</p>
                </div>
                <div>
                    <p class="text-2xl font-bold">${measurements.getFloat("hip_cm", 0f)}</p>
                    <p class="text-brand/50 text-xs">Hip</p>
                </div>
                <div>
                    <p class="text-2xl font-bold">${measurements.getFloat("shoulder_width_cm", 0f)}</p>
                    <p class="text-brand/50 text-xs">Shoulders</p>
                </div>
                <div>
                    <p class="text-2xl font-bold">${measurements.getFloat("inseam_cm", 0f)}</p>
                    <p class="text-brand/50 text-xs">Inseam</p>
                </div>
                <div>
                    <p class="text-2xl font-bold">${measurements.getFloat("height_cm", 0f)}</p>
                    <p class="text-brand/50 text-xs">Height</p>
                </div>
            </div>
        </div>
        @endif

        <%-- CTAs --%>
        <div class="flex flex-col sm:flex-row gap-3">
            <a href="/shop"
               class="flex-1 text-center rounded-full bg-brand-accent py-3 font-semibold text-white
                      hover:bg-brand-accent/90 transition">
                Shop in My Size →
            </a>
            <a href="/measure"
               class="flex-1 text-center rounded-full border border-brand/20 py-3 font-semibold text-brand
                      hover:bg-brand/5 transition">
                Retake Photos
            </a>
        </div>
    </div>

    `)
