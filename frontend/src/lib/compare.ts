/** Modos alineados con `diffplayerqc_core` y `compare.wgsl`. */
export type CompareMode =
  | "SplitScreen"
  | "AbsDiff"
  | "Heatmap"
  | "SideBySide";

export type DiffMode =
  | "LegacyAbs"
  | "AbsLinear"
  | "AbsSqrt"
  | "SignedDiverging"
  | "None";

export const MODOS_COMPARACION: CompareMode[] = [
  "SplitScreen",
  "AbsDiff",
  "Heatmap",
  "SideBySide",
];

export const MODOS_DIFF: DiffMode[] = [
  "LegacyAbs",
  "AbsLinear",
  "AbsSqrt",
  "SignedDiverging",
  "None",
];
