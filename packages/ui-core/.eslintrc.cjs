const path = require("path");

/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  extends: ["custom-ts"],
  parserOptions: {
    project: path.join(__dirname, "tsconfig.json"),
  },
};
