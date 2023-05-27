// TODO: make this an actual base config with default rules, or at least a config with default TS rules

/** @type {import("eslint").Linter.Config} */
module.exports = {
  extends: ["turbo", "prettier"],
  // rules: {},
  // parserOptions: {
  //   babelOptions: {
  //     presets: [require.resolve("next/babel")],
  //   },
  // },
};
