/** @type {import('tailwindcss').Config} */
const config = require("ui-core/tailwind.config.cjs");

/** @type {import('tailwindcss').Config} */
module.exports = {
  presets: [config],
  content: [
    ...config.content,
    // packages
    "../../packages/ui-react/src/**/*.{js,ts,jsx,tsx}",
  ],
};
