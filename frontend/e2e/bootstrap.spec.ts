import { test, expect } from "@playwright/test";

test.skip("shell M0 — se habilita en M2", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { name: "DiffPlayerQC v2" })).toBeVisible();
});
