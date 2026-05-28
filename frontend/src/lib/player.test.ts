import { describe, expect, it } from "vitest";
import { formatearPts } from "./player";

describe("formatearPts", () => {
  it("formatea minutos y segundos", () => {
    expect(formatearPts(65.5)).toMatch(/1:05/);
  });
});
