import { test, expect } from "@playwright/test";

test.describe("M2 — UI shell", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("shell principal visible", async ({ page }) => {
    await expect(page.getByTestId("app-shell")).toBeVisible();
    await expect(page.getByTestId("menubar")).toBeVisible();
    await expect(page.getByTestId("toolbar")).toBeVisible();
    await expect(page.getByTestId("workspace-tabs")).toBeVisible();
  });

  test("cambiar cada workspace", async ({ page }) => {
    const ids = ["compare", "inspect", "audio", "report", "export"] as const;
    for (const id of ids) {
      await page.getByTestId(`workspace-tab-${id}`).click();
      await expect(page.getByTestId(`workspace-${id}`)).toBeVisible();
    }
  });

  test("command palette abre con Ctrl+K", async ({ page }) => {
    const mod = process.platform === "darwin" ? "Meta" : "Control";
    await page.keyboard.press(`${mod}+KeyK`);
    await expect(page.getByTestId("command-palette")).toBeVisible();
    await expect(page.getByTestId("command-palette-input")).toBeFocused();
  });

  test("paneles colapsables en Compare", async ({ page }) => {
    await page.getByTestId("workspace-tab-compare").click();
    const panelIzq = page.getByTestId("panel-izquierdo");
    await expect(panelIzq).toBeVisible();
    await panelIzq.getByRole("button", { name: "Plegar panel" }).click();
    await expect(page.getByTestId("panel-tab-izquierdo")).toBeVisible();
  });
});
