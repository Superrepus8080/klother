@param String pageTitle
@param String activePage
@param String sizeFilter
@param java.util.List<?> products

@template.layout.base(
    pageTitle  = pageTitle,
    activePage = activePage,
    content    = @`

    <div class="mx-auto max-w-7xl px-4 py-12">

        <%-- Header --%>
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
            <div>
                <h1 class="text-2xl font-bold">Shop</h1>
                @if(sizeFilter != null)
                <p class="text-sm text-brand/60 mt-1">
                    Showing items in your size:
                    <span class="font-semibold text-brand-accent">${sizeFilter}</span>
                    — <a href="/shop" class="underline hover:text-brand">show all</a>
                </p>
                @else
                <p class="text-sm text-brand/60 mt-1">
                    <a href="/measure" class="text-brand-accent font-medium hover:underline">
                        Get your size profile
                    </a>
                    to see only items that fit you.
                </p>
                @endif
            </div>

            <%-- Size filter chips --%>
            <div class="flex gap-2 flex-wrap">
                @for(String s : new String[]{"XS","S","M","L","XL","XXL"})
                <a href="/shop?size=${s}"
                   class="px-3 py-1.5 rounded-full text-xs font-semibold border transition
                          ${s.equals(sizeFilter)
                            ? "bg-brand-accent text-white border-brand-accent"
                            : "border-brand/20 text-brand hover:border-brand-accent hover:text-brand-accent"}">
                    ${s}
                </a>
                @endfor
            </div>
        </div>

        <%-- Product grid — populated by DB query --%>
        @if(products.isEmpty())
        <div class="text-center py-24 text-brand/40">
            <p class="text-5xl mb-4">👗</p>
            <p class="text-lg font-medium">Products coming soon</p>
            <p class="text-sm mt-1">We're stocking up — check back shortly.</p>
        </div>
        @else
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-6">
            <%-- Product cards rendered here --%>
        </div>
        @endif
    </div>

    `)
