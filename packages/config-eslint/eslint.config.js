import js from "@eslint/js";
import prettier from "eslint-config-prettier";
import react from "eslint-plugin-react";
import reactHooks from "eslint-plugin-react-hooks";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config(
  {
    ignores: [
      "**/dist/**",
      "**/build/**",
      "**/target/**",
      "**/src-tauri/gen/**",
      "**/.turbo/**",
      "**/node_modules/**",
    ],
  },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  react.configs.flat.recommended,
  react.configs.flat["jsx-runtime"],
  {
    plugins: {
      "react-hooks": reactHooks,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      // False positives on the common "sync state from a subscription/listener
      // in an effect" pattern (matchMedia listeners, embla-carousel callbacks,
      // etc.) — flags shadcn's own generated components (use-mobile.ts,
      // carousel.tsx), not just app code.
      "react-hooks/set-state-in-effect": "off",
    },
  },
  {
    rules: {
      // Redundant under TypeScript — prop types are checked by the compiler,
      // not PropTypes, and this rule doesn't understand TS types (false
      // positives on shadcn's generated calendar.tsx, e.g.).
      "react/prop-types": "off",
    },
  },
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    settings: {
      react: { version: "detect" },
    },
  },
  prettier,
);
