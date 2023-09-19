/** @type {import('tailwindcss').Config} */
module.exports = {
  presets: [require("tailwind-config")],
  content: [
    // packages
    "../../packages/ui-core/button/*.{ts,tsx}",
  ],
};
