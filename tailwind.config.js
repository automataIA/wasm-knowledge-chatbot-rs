/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{rs,html}",
        "./index.html"
    ],
    darkMode: 'class',
    theme: {
        extend: {},
    },
    plugins: [require('daisyui')],
    daisyui: {
        themes: [
            "light",
            {
                black: {
                    "color-scheme": "dark",
                    primary: "oklch(40% 0.15 270)",
                    "primary-content": "oklch(98% 0 0)",
                    secondary: "oklch(35% 0.12 180)",
                    "secondary-content": "oklch(98% 0 0)",
                    accent: "oklch(50% 0.2 45)",
                    "accent-content": "oklch(10% 0 0)",
                    neutral: "oklch(12% 0 0)",
                    "neutral-content": "oklch(95% 0 0)",
                    "base-100": "oklch(0% 0 0)",
                    "base-200": "oklch(8% 0 0)",
                    "base-300": "oklch(15% 0 0)",
                    "base-content": "oklch(95% 0 0)",
                    info: "oklch(45% 0.15 230)",
                    "info-content": "oklch(95% 0 0)",
                    success: "oklch(45% 0.12 140)",
                    "success-content": "oklch(95% 0 0)",
                    warning: "oklch(60% 0.15 70)",
                    "warning-content": "oklch(15% 0 0)",
                    error: "oklch(50% 0.18 20)",
                    "error-content": "oklch(95% 0 0)",
                }
            }
        ],
        darkTheme: "black",
        base: true,
        styled: true,
        utils: true,
    },
}
