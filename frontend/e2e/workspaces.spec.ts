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
    await panelIzq.getByRole("button", { name: /Plegar panel|Collapse panel/i }).click();
    await expect(page.getByTestId("panel-tab-izquierdo")).toBeVisible();
  });

  test("panel modos compare en workspace Compare", async ({ page }) => {
    await page.getByTestId("workspace-tab-compare").click();
    await expect(page.getByTestId("workspace-compare")).toBeAttached();
    await expect(page.getByTestId("compare-mode-panel")).toBeVisible();
    await expect(page.getByTestId("metricas-panel")).toBeVisible();
  });

  test("workspace Inspect muestra panel de scopes", async ({ page }) => {
    await page.getByTestId("workspace-tab-inspect").click();
    await expect(page.getByTestId("workspace-inspect")).toBeVisible();
    await expect(page.getByTestId("inspect-scopes-panel")).toBeVisible();
    await expect(page.getByTestId("scope-histograma")).toBeVisible();
    await expect(page.getByTestId("scope-vectoscopio")).toBeVisible();
    await expect(page.getByTestId("scope-monitor-luma")).toBeVisible();
  });

  test("workspace Report muestra panel de eventos QC", async ({ page }) => {
    await page.getByTestId("workspace-tab-report").click();
    await expect(page.getByTestId("workspace-report")).toBeVisible();
    await expect(page.getByTestId("eventos-qc-panel")).toBeVisible();
    await expect(page.getByTestId("eventos-btn-marcar")).toBeVisible();
  });

  test("workspace Audio muestra loudness y waveforms", async ({ page }) => {
    await page.getByTestId("workspace-tab-audio").click();
    await expect(page.getByTestId("workspace-audio")).toBeVisible();
    await expect(page.getByTestId("audio-loudness")).toBeVisible();
    await expect(page.getByTestId("waveform-canvas-a")).toBeAttached();
    await expect(page.getByTestId("waveform-canvas-b")).toBeAttached();
    await expect(page.getByTestId("waveform-canvas-diff")).toBeAttached();
  });
});
