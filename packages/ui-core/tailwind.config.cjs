/** @type {import('tailwindcss').Config} */
module.exports = {
  presets: [require("tailwind-config")],
  content: [
    // app content
    `src/**/*.{js,ts,jsx,tsx}`,
    // packages
    "../../packages/ui-core/button/*.{ts,tsx}",
  ],
};
