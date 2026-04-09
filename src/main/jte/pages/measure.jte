@param String pageTitle
@param String activePage

@template.layout.base(
    pageTitle  = pageTitle,
    activePage = activePage,
    content    = @`

    <div class="mx-auto max-w-2xl px-4 py-16"
         x-data="measureFlow()"
         x-cloak>

        <%-- ── Step indicator ──────────────────────────────────────────────── --%>
        <div class="flex items-center justify-center gap-2 mb-12">
            <template x-for="(s, i) in steps" :key="i">
                <div class="flex items-center gap-2">
                    <div class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-semibold transition-all"
                         :class="i <= currentStep
                                 ? 'bg-brand-accent text-white'
                                 : 'bg-brand/10 text-brand/40'">
                        <span x-text="i + 1"></span>
                    </div>
                    <span class="text-xs hidden sm:block"
                          :class="i <= currentStep ? 'text-brand font-medium' : 'text-brand/40'"
                          x-text="s"></span>
                    <div x-show="i < steps.length - 1"
                         class="w-8 h-px bg-brand/20 mx-1"></div>
                </div>
            </template>
        </div>

        <%-- ── Step 0: Info ─────────────────────────────────────────────────── --%>
        <div x-show="currentStep === 0" x-transition>
            <h1 class="text-3xl font-bold mb-2">Let's find your perfect fit</h1>
            <p class="text-brand/60 mb-8">Takes about 60 seconds. We'll never store your photos.</p>

            <div class="bg-white rounded-2xl p-6 shadow-sm border border-brand/5 mb-8 space-y-4">
                <h2 class="font-semibold text-brand">For the best results:</h2>
                <ul class="space-y-2 text-sm text-brand/70">
                    <li class="flex gap-2">
                        <span class="text-green-500">✓</span>
                        Wear a fitted top and leggings or tight trousers
                    </li>
                    <li class="flex gap-2">
                        <span class="text-green-500">✓</span>
                        Stand against a plain, well-lit wall
                    </li>
                    <li class="flex gap-2">
                        <span class="text-green-500">✓</span>
                        You'll take 2 photos: facing the camera, then side-on
                    </li>
                    <li class="flex gap-2">
                        <span class="text-green-500">✓</span>
                        Your photos are deleted immediately after processing
                    </li>
                </ul>
            </div>

            <%-- Height + gender inputs --%>
            <div class="grid grid-cols-2 gap-4 mb-8">
                <div>
                    <label class="block text-sm font-medium mb-1">Your height (cm)</label>
                    <input type="number" min="130" max="220"
                           x-model="heightCm"
                           placeholder="e.g. 168"
                           class="w-full rounded-xl border border-brand/20 px-4 py-3 text-brand
                                  focus:outline-none focus:ring-2 focus:ring-brand-accent/50"/>
                </div>
                <div>
                    <label class="block text-sm font-medium mb-1">Sizing preference</label>
                    <select x-model="gender"
                            class="w-full rounded-xl border border-brand/20 px-4 py-3 text-brand
                                   focus:outline-none focus:ring-2 focus:ring-brand-accent/50">
                        <option value="WOMEN">Women's sizing</option>
                        <option value="MEN">Men's sizing</option>
                        <option value="NON_BINARY">Unisex / Non-binary</option>
                    </select>
                </div>
            </div>

            <button @click="nextStep()"
                    :disabled="!heightCm || heightCm < 130 || heightCm > 220"
                    class="w-full rounded-full bg-brand-accent py-3 font-semibold text-white
                           hover:bg-brand-accent/90 transition disabled:opacity-40 disabled:cursor-not-allowed">
                I'm ready — continue →
            </button>
        </div>

        <%-- ── Step 1: Front photo ──────────────────────────────────────────── --%>
        <div x-show="currentStep === 1" x-transition>
            <h2 class="text-2xl font-bold mb-2">Photo 1: Face the camera</h2>
            <p class="text-brand/60 mb-6">Stand straight, arms slightly out (A-pose), full body visible.</p>

            <div class="relative rounded-2xl overflow-hidden bg-brand/5 border-2 border-dashed border-brand/20
                        mb-6 flex items-center justify-center min-h-64"
                 :class="frontPreview ? 'border-green-400 bg-green-50' : ''">

                <template x-if="!frontPreview">
                    <label class="cursor-pointer flex flex-col items-center gap-3 py-12 px-6 text-center">
                        <div class="w-16 h-16 rounded-full bg-brand/10 flex items-center justify-center">
                            <svg class="w-7 h-7 text-brand/40" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                      d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                      d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                        <span class="text-sm text-brand/50">Tap to upload or take a photo</span>
                        <input type="file" accept="image/*" capture="user" class="hidden"
                               @change="onFrontPhoto($event)"/>
                    </label>
                </template>

                <template x-if="frontPreview">
                    <div class="relative w-full">
                        <img :src="frontPreview" class="w-full object-cover max-h-80 rounded-2xl"/>
                        <button @click="frontPreview = null; frontFile = null"
                                class="absolute top-2 right-2 w-8 h-8 rounded-full bg-black/50 text-white
                                       flex items-center justify-center text-xs hover:bg-black/70">✕</button>
                        <div class="absolute bottom-3 left-1/2 -translate-x-1/2 bg-green-500 text-white
                                    text-xs font-semibold px-3 py-1 rounded-full">
                            ✓ Front photo ready
                        </div>
                    </div>
                </template>
            </div>

            <div class="flex gap-3">
                <button @click="prevStep()"
                        class="flex-1 rounded-full border border-brand/20 py-3 font-semibold text-brand
                               hover:bg-brand/5 transition">
                    ← Back
                </button>
                <button @click="nextStep()"
                        :disabled="!frontFile"
                        class="flex-1 rounded-full bg-brand-accent py-3 font-semibold text-white
                               hover:bg-brand-accent/90 transition disabled:opacity-40 disabled:cursor-not-allowed">
                    Next →
                </button>
            </div>
        </div>

        <%-- ── Step 2: Side photo ───────────────────────────────────────────── --%>
        <div x-show="currentStep === 2" x-transition>
            <h2 class="text-2xl font-bold mb-2">Photo 2: Turn sideways</h2>
            <p class="text-brand/60 mb-6">Turn 90° to your right. Stand straight, full body visible.</p>

            <div class="relative rounded-2xl overflow-hidden bg-brand/5 border-2 border-dashed border-brand/20
                        mb-6 flex items-center justify-center min-h-64"
                 :class="sidePreview ? 'border-green-400 bg-green-50' : ''">

                <template x-if="!sidePreview">
                    <label class="cursor-pointer flex flex-col items-center gap-3 py-12 px-6 text-center">
                        <div class="w-16 h-16 rounded-full bg-brand/10 flex items-center justify-center">
                            <svg class="w-7 h-7 text-brand/40" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                      d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
                                      d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                        <span class="text-sm text-brand/50">Tap to upload or take a photo</span>
                        <input type="file" accept="image/*" capture="environment" class="hidden"
                               @change="onSidePhoto($event)"/>
                    </label>
                </template>

                <template x-if="sidePreview">
                    <div class="relative w-full">
                        <img :src="sidePreview" class="w-full object-cover max-h-80 rounded-2xl"/>
                        <button @click="sidePreview = null; sideFile = null"
                                class="absolute top-2 right-2 w-8 h-8 rounded-full bg-black/50 text-white
                                       flex items-center justify-center text-xs hover:bg-black/70">✕</button>
                        <div class="absolute bottom-3 left-1/2 -translate-x-1/2 bg-green-500 text-white
                                    text-xs font-semibold px-3 py-1 rounded-full">
                            ✓ Side photo ready
                        </div>
                    </div>
                </template>
            </div>

            <div class="flex gap-3">
                <button @click="prevStep()"
                        class="flex-1 rounded-full border border-brand/20 py-3 font-semibold text-brand
                               hover:bg-brand/5 transition">
                    ← Back
                </button>
                <button @click="submit()"
                        :disabled="!sideFile || loading"
                        class="flex-1 rounded-full bg-brand-accent py-3 font-semibold text-white
                               hover:bg-brand-accent/90 transition disabled:opacity-40 disabled:cursor-not-allowed">
                    <span x-show="!loading">Analyse my size →</span>
                    <span x-show="loading" class="flex items-center justify-center gap-2">
                        <svg class="animate-spin h-4 w-4" viewBox="0 0 24 24" fill="none">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
                            <path class="opacity-75" fill="currentColor"
                                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
                        </svg>
                        Analysing...
                    </span>
                </button>
            </div>

            <%-- Error display --%>
            <p x-show="error" x-text="error"
               class="mt-4 text-center text-sm text-red-500 bg-red-50 rounded-xl py-2 px-4"></p>
        </div>

    </div>

    <script>
        function measureFlow() {
            return {
                steps: ['Setup', 'Front photo', 'Side photo'],
                currentStep: 0,
                heightCm: '',
                gender: 'WOMEN',
                frontFile: null,
                frontPreview: null,
                sideFile: null,
                sidePreview: null,
                loading: false,
                error: '',

                nextStep() { if (this.currentStep < this.steps.length - 1) this.currentStep++ },
                prevStep() { if (this.currentStep > 0) this.currentStep-- },

                onFrontPhoto(e) {
                    const file = e.target.files[0];
                    if (!file) return;
                    this.frontFile = file;
                    this.frontPreview = URL.createObjectURL(file);
                },

                onSidePhoto(e) {
                    const file = e.target.files[0];
                    if (!file) return;
                    this.sideFile = file;
                    this.sidePreview = URL.createObjectURL(file);
                },

                async submit() {
                    this.loading = true;
                    this.error = '';
                    try {
                        const form = new FormData();
                        form.append('front_photo', this.frontFile);
                        form.append('side_photo',  this.sideFile);
                        form.append('height_cm',   this.heightCm);
                        form.append('gender',      this.gender);

                        const res = await fetch('/api/measure', { method: 'POST', body: form });
                        if (res.redirected) { window.location.href = res.url; return; }

                        // HTMX redirect via header
                        const redirect = res.headers.get('HX-Redirect');
                        if (redirect) { window.location.href = redirect; return; }

                        if (!res.ok) {
                            const body = await res.json();
                            this.error = body.error || 'Something went wrong. Please try again.';
                        }
                    } catch (e) {
                        this.error = 'Network error. Please check your connection.';
                    } finally {
                        this.loading = false;
                    }
                }
            }
        }
    </script>

    `)
