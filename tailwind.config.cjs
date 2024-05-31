/** @type {import('tailwindcss').Config} */
// To use the tailwind CLI to minify: https://tailwindcss.com/blog/standalone-cli
const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ['./templates/*.html'],
  theme: {
    extend: {
      fontFamily: {
        'sans': ['"Chakra Petch"', ...defaultTheme.fontFamily.sans]
      }
    },
  },
  plugins: [],
}

