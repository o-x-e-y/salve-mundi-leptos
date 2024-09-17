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

                'darker': '#1a1a1a',
                // 'darker': '#141414',
                'txt-dark': '#eddded',
                // 'selected-bg-darkmode': '#230046',
                // 'forminput-bg-darkmode': '#282828',
                // 'card-bg-darkmode': '#434343',

                'sm-blue': '#3490dc',
                'sm-indigo': '#6574cd',
                'sm-purple': '#9561e2',
                'sm-pink': '#f66d9b',
                'sm-red': '#e3342f',
                'sm-orange': '#f6993f',
                'sm-yellow': '#ffed4a',
                'sm-green': '#38c172',
                'sm-teal': '#4dc0b5',
                'sm-cyan': '#6cb2eb',
                'sm-primary': '#663265',
                'sm-light': '#864485',
                'sm-link': '#BA5EB8',
                'sm-dark': '#39203B',
                'sm-boxShadow': 'rgba(102, 50, 101, 0.6)',
                'sm-boxShadowWhite': 'rgba(255, 255, 255, 0.24)',

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
                        '--tw-prose-body': "#eee",
                        '--tw-prose-headings': "#eee",
                        '--tw-prose-lead': "#eee",
                        '--tw-prose-links': "#eee",
                        '--tw-prose-bold': "#eee",
                        '--tw-prose-counters': "#eee",
                        '--tw-prose-bullets': "#eee",
                        '--tw-prose-hr': "#ddd",
                        '--tw-prose-quotes': "#eee",
                        '--tw-prose-quote-borders': "#ddd",
                        '--tw-prose-captions': "#ddd",
                        '--tw-prose-code': "#eee",
                        '--tw-prose-pre-bg': "#1a1a1a",
                        '--tw-prose-th-borders': "#ddd",
                        '--tw-prose-td-borders': "#eee",
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
