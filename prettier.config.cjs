/** @typedef  {{}} SortImportsConfig*/
/** @typedef  {import("@ianvs/prettier-plugin-sort-imports").PluginConfig} SortImportsConfig*/
/** @typedef  {import("prettier").Config} PrettierConfig*/
/** @typedef  {{ tailwindConfig: string }} TailwindConfig*/

/** @type { PrettierConfig | SortImportsConfig | TailwindConfig } */
module.exports = {
  plugins: [
    // "@ianvs/prettier-plugin-sort-imports",
    "prettier-plugin-tailwindcss",
    // require.resolve("prettier-plugin-tailwindcss"),
    // require.resolve("@ianvs/prettier-plugin-sort-imports"),
  ],
  // tailwindConfig: "./packages/config/tailwind",
  // importOrder: [
  //   "^(react/(.*)$)|^(react$)|^(react-native(.*)$)",
  //   "^(next/(.*)$)|^(next$)",
  //   "^(expo(.*)$)|^(expo$)",
  //   "<THIRD_PARTY_MODULES>",
  //   "",
  //   "^@acme/(.*)$",
  //   "",
  //   "^~/utils/(.*)$",
  //   "^~/components/(.*)$",
  //   "^~/styles/(.*)$",
  //   "^~/(.*)$",
  //   "^[./]",
  // ],
  // importOrderSeparation: false,
  // importOrderSortSpecifiers: true,
  // importOrderBuiltinModulesToTop: true,
  // importOrderParserPlugins: ["typescript", "jsx", "decorators-legacy"],
  // importOrderMergeDuplicateImports: true,
  // importOrderCombineTypeAndValueImports: true,
};
