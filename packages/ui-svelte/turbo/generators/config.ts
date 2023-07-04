import type { PlopTypes } from '@turbo/gen';

// Learn more about Turborepo Generators at https://turbo.build/repo/docs/core-concepts/monorepos/code-generation

export default function generator(plop: PlopTypes.NodePlopAPI): void {
	// A simple generator to add a new Svelte component to the internal UI library
	plop.setGenerator('svelte-component', {
		description: 'Adds a new svelte component',
		prompts: [
			{
				type: 'input',
				name: 'name',
				message: 'What is the name of the component?'
			}
		],
		actions: [
			{
				type: 'add',
				path: 'src/{{dashCase name}}/index.ts',
				templateFile: 'templates/component.hbs'
			},
			{
				type: 'add',
				path: 'src/{{dashCase name}}/{{pascalCase name}}.svelte',
				templateFile: 'templates/component.svelte.hbs'
			},
			{
				type: 'append',
				path: 'index.ts',
				pattern: /(\/\/ component exports)/g,
				template: 'export * from "./src/{{dashCase name}}";'
			}
		]
	});
}
