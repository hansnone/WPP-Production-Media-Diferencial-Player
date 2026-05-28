import globals from "globals";
import svelte from "eslint-plugin-svelte";
import js from "@eslint/js";

export default [
  js.configs.recommended,
  ...svelte.configs["flat/recommended"],
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.node },
    },
  },
  {
    ignores: ["dist/", "node_modules/"],
  },
];
