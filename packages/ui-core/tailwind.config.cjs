// TODO?: merge tailwind-config package into here, idt it
// gets imported into any other packages

/** @type {import('tailwindcss').Config} */
module.exports = {
  presets: [require("tailwind-config")],
  content: [
    // packages
    "../../packages/ui-core/button/*.{ts,tsx}",
  ],
};
