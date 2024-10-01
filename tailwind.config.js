/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: [
            "index.html",
            "./src/*.rs"
        ],
    },
    theme: {
        extend: {
            keyframes: {
                fadein: {
                    '0%': { opacity: 0 },
                    '100%': { opacity: 1 },
                },
            },
            animation: {
                'fadein-1': 'fadein 1s ease-in-out 0.25s forwards',
                'fadein-2': 'fadein 1s ease-in-out 1.5s forwards',
                'fadein-3': 'fadein 1s ease-in-out 2.75s forwards',
            },
            backgroundImage: {
                'search-icon': 'url(/public/images/search.svg)',
            },
            backgroundPosition: {
                'right-margin': 'calc(100% - 0.5rem) 0.25rem',
            },
            backgroundSize: {
                '6': '1.5rem',
            },
            colors: {
                'bg-lightmode': '#f8fafc',
                'bg-darkmode': '#111111',

                'grey-highlight': '#222222',
                'txt-dark': '#ffeefe',
                'lighter': '#ffddfe',
                // 'selected-bg-darkmode': '#230046',
                // 'forminput-bg-darkmode': '#282828',
                // 'card-bg-darkmode': '#434343',

                'blue': '#3490dc',
                'indigo': '#6574cd',
                'purple': '#9561e2',
                'pink': '#f66d9b',
                'red': '#e3342f',
                'orange': '#f6993f',
                'yellow': '#ffed4a',
                'green': '#38c172',
                'teal': '#4dc0b5',
                'cyan': '#6cb2eb',
                'primary': '#663265',
                'light': '#864485',
                'link': '#BA5EB8',
                'dark': '#39203B',
                'boxShadow': 'rgba(102, 50, 101, 0.6)',
                'boxShadowWhite': 'rgba(255, 255, 255, 0.24)',

                // 'header': '#242424',
                // 'darker': '#1a1a1a',
                // 'hovered': '#ffffff10',
                // 'txt': "#eee",
                // 'ccc': "#ccc",
                // 'pink-f': '#b4014b',
                // 'red-f': '#d53e4f',
                // 'orange-f': '#f46d43',
                // 'light-orange-f': '#fdae61',
                // 'yellow-f': '#fee08b',
                // 'light-green-f': '#e6f598',
                // 'green-f': '#abdda4',
                // 'cyan-f': '#66c2a5',
                // 'blue-f': '#3288bd',
                // 'purple-f': '#6b5ab8',
            },
            gridTemplateColumns: {
                'homepage': '1fr 2fr',
                'metadata': '1fr 4fr',
                'analyzer': '4fr 1fr',
            },
            typography: ({ theme }) => ({
                posts: {
                    css: {
                        '--tw-prose-body': "#feeefe",
                        '--tw-prose-headings': "#feeefe",
                        '--tw-prose-lead': "#feeefe",
                        '--tw-prose-links': "#feeefe",
                        '--tw-prose-bold': "#feeefe",
                        '--tw-prose-counters': "#feeefe",
                        '--tw-prose-bullets': "#feeefe",
                        '--tw-prose-hr': "#ddd",
                        '--tw-prose-quotes': "#feeefe",
                        '--tw-prose-quote-borders': "#ddd",
                        '--tw-prose-captions': "#ddd",
                        '--tw-prose-code': "#feeefe",
                        '--tw-prose-pre-bg': "#1a1a1a",
                        '--tw-prose-th-borders': "#ddd",
                        '--tw-prose-td-borders': "#feeefe",
                        // '--tw-prose-invert-body': theme('colors.pink[200]'),
                        // '--tw-prose-invert-headings': theme('colors.white'),
                        // '--tw-prose-invert-lead': "#ddd",
                        // '--tw-prose-invert-links': theme('colors.white'),
                        // '--tw-prose-invert-bold': theme('colors.white'),
                        // '--tw-prose-invert-counters': theme('colors.pink[400]'),
                        // '--tw-prose-invert-bullets': theme('colors.pink[600]'),
                        // '--tw-prose-invert-hr': "#ddd",
                        // '--tw-prose-invert-quotes': theme('colors.pink[100]'),
                        // '--tw-prose-invert-quote-borders': "#ddd",
                        // '--tw-prose-invert-captions': theme('colors.pink[400]'),
                        // '--tw-prose-invert-code': theme('colors.white'),
                        // '--tw-prose-invert-pre-code': "#ddd",
                        // '--tw-prose-invert-pre-bg': 'rgb(0 0 0 / 50%)',
                        // '--tw-prose-invert-th-borders': theme('colors.pink[600]'),
                        // '--tw-prose-invert-td-borders': "#ddd",
                    },
                },
                DEFAULT: {
                    css: {
                        maxWidth: 'min(70ch, 95%)',
                        minWidth: '50%',
                    }
                }
            }),
        },
    },
    plugins: [
        require("@tailwindcss/typography"),
        function ({ addUtilities }) {
            const newUtilities = {
                '.container-inline-size': {
                    'container-type': 'inline-size',
                },
                '.br': {
                    'border': '1px solid red'
                },
            };
            addUtilities(newUtilities);
        },
    ],
}
