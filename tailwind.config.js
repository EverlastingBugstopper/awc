/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/ui/template.html", "./src/ui/index.ts"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
