/** @type {import('tailwindcss').Config} */
module.exports = {
    // Scan JTE templates for class names
    content: [
        "./src/main/jte/**/*.jte",
        "./src/main/resources/static/**/*.js",
    ],
    theme: {
        extend: {
            colors: {
                brand: {
                    DEFAULT: '#1a1a2e',
                    accent:  '#e94560',
                    muted:   '#f5f0eb',
                },
            },
            fontFamily: {
                sans: ['Inter', 'system-ui', 'sans-serif'],
            },
            borderRadius: {
                '2xl': '1rem',
                '3xl': '1.5rem',
            },
        },
    },
    plugins: [],
}
