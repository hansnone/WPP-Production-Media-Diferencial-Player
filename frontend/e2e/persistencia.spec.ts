import { test, expect } from "@playwright/test";

test.describe("M6 — Persistencia", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("layout guardado en localStorage", async ({ page }) => {
    const layout = await page.evaluate(() =>
      localStorage.getItem("diffplayerqc-v2-layout"),
    );
    expect(layout).toBeTruthy();
    const parsed = JSON.parse(layout!) as { workspaceActivo: string };
    expect(parsed.workspaceActivo).toBeTruthy();
  });

  test("cambiar idioma a English persiste", async ({ page }) => {
    await page.getByTestId("menu-ver").locator("summary").click();
    await page.getByTestId("menu-idioma-en").click();
    await expect(page.getByTestId("menu-archivo").locator("summary")).toHaveText(
      "File",
    );
    const idioma = await page.evaluate(() =>
      localStorage.getItem("diffplayerqc-v2-idioma"),
    );
    expect(idioma).toBe("en");
  });

  test("menu recientes visible (vacío al inicio)", async ({ page }) => {
    await page.getByTestId("menu-archivo").locator("summary").click();
    await expect(page.getByTestId("menu-sin-recientes")).toBeVisible();
  });
});
