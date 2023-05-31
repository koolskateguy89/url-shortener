/** @type {import('tailwindcss').Config} */
module.exports = {
  presets: [require("tailwind-config")],
  // darkMode: ["class"],
  content: [
    "./src/**/*.rs",
    "./index.html",
    "./src/**/*.html",
    "./src/**/*.css",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
