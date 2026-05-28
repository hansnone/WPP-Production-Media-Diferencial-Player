import { defineConfig } from "@playwright/test";

/** Config mínima M0; los flujos E2E se activan en M2. */
export default defineConfig({
  testDir: "e2e",
  use: { headless: true },
});
