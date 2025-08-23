/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{rs,html}",
        "./index.html"
    ],
    safelist: [
        "swap",
        "swap-rotate",
        "swap-on",
        "swap-off",
        "theme-controller",
    ],
    darkMode: 'class',
    theme: {
        extend: {},
    },
    plugins: [require('daisyui')],
    daisyui: {
        themes: [
            "light",
            "business"
        ],
        darkTheme: "business",
        base: true,
        styled: true,
        utils: true,
    },
}
