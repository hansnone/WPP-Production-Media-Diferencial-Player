import { describe, expect, it } from "vitest";
import { filtrarComandos, type ComandoPaleta } from "./commands";

const MOCK: ComandoPaleta[] = [
  { id: "a", etiqueta: "Abrir A", grupo: "Archivo", ejecutar: () => {} },
  { id: "b", etiqueta: "Compare", grupo: "Vista", ejecutar: () => {} },
];

describe("filtrarComandos", () => {
  it("sin consulta devuelve todo", () => {
    expect(filtrarComandos(MOCK, "")).toHaveLength(2);
  });

  it("filtra por etiqueta", () => {
    expect(filtrarComandos(MOCK, "abrir")).toHaveLength(1);
  });
});
