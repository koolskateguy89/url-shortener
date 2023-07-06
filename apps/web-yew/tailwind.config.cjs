/** @type {import('tailwindcss').Config} */
module.exports = {
  presets: [require("tailwind-config")],
  content: ["./src/**/*.rs", "./index.html"],
  // darkMode: ["class"],

  theme: {
    extend: {},
  },

  plugins: [],
};
