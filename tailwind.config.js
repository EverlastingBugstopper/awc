/** @type {import('tailwindcss').Config} */
const path = require("path");
const BASE = path.resolve(__dirname, "crates/awc-server");
module.exports = {
  content: [
    path.resolve(BASE, "ui/template.html"),
    path.resolve(BASE, "ui/index.ts"),
  ],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
};
