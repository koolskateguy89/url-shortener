/** @type {import('tailwindcss').Config} */
const config = require("ui/tailwind.config.cjs");

/** @type {import('tailwindcss').Config} */
module.exports = {
  presets: [config],
  content: [
    ...config.content,
    // app content
    "src/app/**/*.{ts,tsx}",
  ],
  darkMode: ["class"],

  theme: {
    extend: {},
  },

  plugins: [],
};
