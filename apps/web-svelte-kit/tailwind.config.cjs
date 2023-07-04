/** @type {import('tailwindcss').Config} */
const config = require('ui/tailwind.config.cjs');

/** @type {import('tailwindcss').Config} */
module.exports = {
	presets: [config],
	content: [
		...config.content,
		// app content
		'./src/**/*.{html,js,svelte,ts}'
	],
	darkMode: ['class'],

	theme: {
		extend: {}
	},

	plugins: []
};
