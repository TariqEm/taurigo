import { fileURLToPath, URL } from "node:url";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// Library-mode config — packages/ui has no dev server of its own; this exists so
// tooling (shadcn CLI's framework detection, vitest, a future build step) has a
// single source of truth for the React/JSX pipeline.
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@taurigo/ui": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});
