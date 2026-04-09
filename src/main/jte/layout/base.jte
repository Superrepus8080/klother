@import gg.jte.Content
@param String pageTitle
@param String activePage
@param Content content

<!DOCTYPE html>
<html lang="en" class="h-full">
<head>
    <meta charset="UTF-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>${pageTitle}</title>

    <%-- Tailwind CSS (CDN for dev; replace with /static/css/output.css in prod) --%>
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            theme: {
                extend: {
                    colors: {
                        brand: {
                            DEFAULT: '#1a1a2e',
                            accent:  '#e94560',
                            muted:   '#f5f0eb',
                        }
                    },
                    fontFamily: {
                        sans: ['Inter', 'system-ui', 'sans-serif'],
                    }
                }
            }
        }
    </script>

    <%-- Inter font --%>
    <link rel="preconnect" href="https://fonts.googleapis.com"/>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet"/>

    <%-- HTMX — enables SPA-like interactions without React --%>
    <script src="https://unpkg.com/htmx.org@1.9.12" defer></script>

    <%-- Alpine.js — lightweight reactive state (camera, form steps, modals) --%>
    <script src="https://unpkg.com/alpinejs@3.14.1/dist/cdn.min.js" defer></script>
</head>

<body class="h-full bg-brand-muted font-sans text-brand antialiased"
      hx-boost="true">

<%-- ── Navigation ─────────────────────────────────────────────────────────── --%>
<nav class="bg-brand shadow-md">
    <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="flex h-16 items-center justify-between">

            <%-- Logo --%>
            <a href="/" class="flex items-center gap-2 text-white font-bold text-xl tracking-tight">
                <span class="text-brand-accent">K</span>lother
            </a>

            <%-- Nav links --%>
            <div class="hidden md:flex items-center gap-8">
                @template.layout.navLink(href = "/shop",    label = "Shop",    active = activePage.equals("shop"))
                @template.layout.navLink(href = "/measure", label = "My Size", active = activePage.equals("measure"))
                @template.layout.navLink(href = "/account", label = "Account", active = activePage.equals("account"))
            </div>

            <%-- Mobile menu button --%>
            <button class="md:hidden text-white" x-data @click="$dispatch('toggle-menu')">
                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                </svg>
            </button>
        </div>
    </div>
</nav>

<%-- ── Page content ──────────────────────────────────────────────────────── --%>
<main class="min-h-[calc(100vh-4rem)]">
    ${content}
</main>

<%-- ── Footer ────────────────────────────────────────────────────────────── --%>
<footer class="bg-brand text-white/60 text-sm py-8 mt-16">
    <div class="mx-auto max-w-7xl px-4 flex flex-col md:flex-row justify-between gap-4">
        <p>© 2026 Klother. All rights reserved.</p>
        <div class="flex gap-6">
            <a href="/privacy" class="hover:text-white transition">Privacy</a>
            <a href="/terms"   class="hover:text-white transition">Terms</a>
        </div>
    </div>
</footer>

</body>
</html>
