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
                primary: '#9000d4',
                secondary: '#b3b0fd',
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
                },
                'mm-orange': {
                    DEFAULT: '#E38300',
                    50: '#FFD59C',
                    100: '#FFCC87',
                    200: '#FFBB5E',
                    300: '#FFAA36',
                    400: '#FF990D',
                    500: '#E38300',
                    600: '#AB6300',
                    700: '#734200',
                    800: '#3B2200',
                    900: '#030200',
                    950: '#000000'
                },
                'mm-pink': {
                    DEFAULT: '#8B5C5D',
                    50: '#D9C6C6',
                    100: '#D1B9BA',
                    200: '#C1A1A1',
                    300: '#B08889',
                    400: '#A07071',
                    500: '#8B5C5D',
                    600: '#694646',
                    700: '#472F30',
                    800: '#261919',
                    900: '#040303',
                    950: '#000000'
                },
            },
        },
    }, plugins: [],
}