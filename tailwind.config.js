/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'], theme: {
        extend: {
            fontFamily: {
                mono: ['Switzer', 'monospace'],
                display: ['Switzer', 'sans-serif'],
            },
            colors: {
                transparent: 'transparent',
                current: 'currentColor',
                primary: '#FFB703',
                secondary: '#8ECAE6',
                accent: '#023047',
                'mm-blue': {
                    DEFAULT: '#53666D',
                    50: '#B3C0C5',
                    100: '#A7B6BC',
                    200: '#90A3AA',
                    300: '#799099',
                    400: '#657C84',
                    500: '#53666D',
                    600: '#3B484D',
                    700: '#222A2D',
                    800: '#0A0D0D',
                    900: '#000000',
                    950: '#000000'
                }, 'mm-green': {
                    DEFAULT: '#497059',
                    50: '#A9C7B6',
                    100: '#9DBFAB',
                    200: '#84AF96',
                    300: '#6C9F81',
                    400: '#59896D',
                    500: '#497059',
                    600: '#334E3E',
                    700: '#1D2C23',
                    800: '#070A08',
                    900: '#000000',
                    950: '#000000'
                }
            },
        },
    }, plugins: [],
}