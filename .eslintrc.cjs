// https://typescript-eslint.io/linting/typed-linting/monorepos

/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  // This tells ESLint to load the config from the package `eslint-config-custom`
  extends: ["custom"],

  parser: "@typescript-eslint/parser",
  parserOptions: {
    tsconfigRootDir: __dirname,
    // project: [
    //   "./tsconfig.json",
    //   "./apps/*/tsconfig.json",
    //   "./packages/*/tsconfig.json",
    // ],
  },
  settings: {
    next: {
      rootDir: "apps/web-nextjs",
    },
  },
};
