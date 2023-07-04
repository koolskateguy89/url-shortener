const path = require("path");

/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  extends: ["custom", "plugin:react/recommended", "plugin:react/jsx-runtime"],
  parserOptions: {
    project: path.join(__dirname, "tsconfig.json"),
  },
  settings: {
    react: {
      version: "detect",
    },
  },
};
