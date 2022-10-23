/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "src/main.rs",
    "src/components/*.rs",
    "src/templates/*.rs"
  ],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
