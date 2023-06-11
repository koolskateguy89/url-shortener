const path = require("path");

/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  plugins: ["@tanstack/query"],
  extends: ["custom-next", "plugin:@tanstack/eslint-plugin-query/recommended"],
  parserOptions: {
    project: path.join(__dirname, "tsconfig.json"),
  },
  rules: {
    "@next/next/no-html-link-for-pages": "off",
  },
};
