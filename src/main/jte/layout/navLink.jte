@param String href
@param String label
@param boolean active

<a href="${href}"
   class="text-sm font-medium transition-colors ${active ? "text-brand-accent border-b-2 border-brand-accent pb-0.5" : "text-white/70 hover:text-white"}">
    ${label}
</a>
