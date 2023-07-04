/** @type {import("prettier").Config} */
module.exports = {
	useTabs: true,
	singleQuote: true,
	trailingComma: 'none',
	printWidth: 100,
	// FIXME?: tailwind plugin isn't doing anything
	plugins: ['prettier-plugin-svelte', 'prettier-plugin-tailwindcss'],
	pluginSearchDirs: ['.'],
	overrides: [{ files: '*.svelte', options: { parser: 'svelte' } }]
};
