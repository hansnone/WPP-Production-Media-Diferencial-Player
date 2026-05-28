import { describe, expect, it } from "vitest";
import { etiquetaWorkspace } from "./workspaces";

describe("etiquetaWorkspace", () => {
  it("resuelve compare", () => {
    expect(etiquetaWorkspace("compare")).toBe("Compare");
  });

  it("devuelve el id si es desconocido", () => {
    expect(etiquetaWorkspace("otro")).toBe("otro");
  });
});
