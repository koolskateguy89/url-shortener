const path = require("path");

/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  extends: ["custom-solid"],
  parserOptions: {
    project: path.join(__dirname, "tsconfig.json"),
  },
};
