/** @type {import('tailwindcss').Config} */
const path = require("path");
const BASE = path.resolve(__dirname, "awc-web");
module.exports = {
  content: [
    path.resolve(BASE, "src/browser/template.html"),
    path.resolve(BASE, "src/server/main.rs"),
  ],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["night"]
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
};
