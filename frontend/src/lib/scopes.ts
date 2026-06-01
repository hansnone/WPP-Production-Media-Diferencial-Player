import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

/** Scopes del fotograma actual (alineado con Rust `ScopesFrame`). */
export interface ScopesFrame {
  pts: number;
  canal: string;
  histograma_r: number[];
  histograma_g: number[];
  histograma_b: number[];
  vectoscopio: number[];
  monitor_luma: number[];
}

export async function obtenerScopes(): Promise<ScopesFrame | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }
  return invoke<ScopesFrame | null>("obtener_scopes");
}

export function escucharScopes(
  callback: (frame: ScopesFrame) => void,
): Promise<UnlistenFn> {
  return listen<ScopesFrame>("scopes-actualizados", (ev) => {
    callback(ev.payload);
  });
}

/** Normaliza un histograma a 0..1 para dibujar. */
export function normalizarHistograma(bins: number[]): number[] {
  const max = bins.reduce((m, v) => Math.max(m, v), 0);
  if (max <= 0) return bins.map(() => 0);
  return bins.map((v) => v / max);
}
