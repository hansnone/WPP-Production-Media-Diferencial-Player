import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { Canal } from "./player";

/** Forma de onda precomputada (alineada con Rust `FormaOnda`). */
export interface FormaOnda {
  picos: number[];
  duracion_secs: number;
  lufs_integrado: number;
  picos_por_segundo: number;
}

/** Evento emitido al terminar el escaneo offline. */
export interface FormaOndaEvento extends FormaOnda {
  canal: Canal;
}

export async function obtenerFormaOnda(canal: Canal): Promise<FormaOnda | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }
  return invoke<FormaOnda | null>("obtener_forma_onda", { canal });
}

export function escucharFormaOnda(
  callback: (ev: FormaOndaEvento) => void,
): Promise<UnlistenFn> {
  return listen<FormaOndaEvento>("forma-onda-lista", (ev) => {
    callback(ev.payload);
  });
}

/** Formatea LUFS para la UI; `-Infinity` → em dash. */
export function formatearLufs(lufs: number): string {
  if (!Number.isFinite(lufs)) {
    return "—";
  }
  return `${lufs.toFixed(1)} LUFS`;
}

/**
 * Picos de diferencia |A − B| re-muestreados al mismo número de buckets.
 * Usa el máximo de longitudes para no perder detalle del clip más largo.
 */
export function calcularPicosDiff(a: FormaOnda | null, b: FormaOnda | null): number[] {
  if (!a?.picos.length || !b?.picos.length) {
    return [];
  }

  const len = Math.max(a.picos.length, b.picos.length);
  const diff: number[] = [];
  for (let i = 0; i < len; i += 1) {
    const pa = a.picos[Math.min(i, a.picos.length - 1)] ?? 0;
    const pb = b.picos[Math.min(i, b.picos.length - 1)] ?? 0;
    diff.push(Math.abs(pa - pb));
  }
  return diff;
}
