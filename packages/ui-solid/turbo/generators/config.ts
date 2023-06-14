import type { PlopTypes } from "@turbo/gen";

// Learn more about Turborepo Generators at https://turbo.build/repo/docs/core-concepts/monorepos/code-generation

export default function generator(plop: PlopTypes.NodePlopAPI): void {
  // A simple generator to add a new Solid component to the internal UI library
  plop.setGenerator("solid-component", {
    description: "Adds a new solid component",
    prompts: [
      {
        type: "input",
        name: "name",
        message: "What is the name of the component?",
      },
    ],
    actions: [
      {
        type: "add",
        path: "src/{{dashCase name}}.tsx",
        templateFile: "templates/component.hbs",
      },
      {
        type: "append",
        path: "index.ts",
        pattern: /(\/\/ component exports)/g,
        template: 'export * from "./src/{{dashCase name}}";',
      },
    ],
  });
}
