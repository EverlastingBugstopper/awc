/** @type {import('tailwindcss').Config} */
const path = require("path");
const BASE = path.resolve(__dirname, "awc-web");
module.exports = {
  content: [
    path.resolve(BASE, "src/browser/template.html"),
    path.resolve(BASE, "src/browser/index.ts"),
  ],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
};
