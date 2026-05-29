import { invoke } from "@tauri-apps/api/core";
import type { CompareMode, DiffMode } from "./compare";

export interface RectViewport {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface VistaCompare {
  modo: CompareMode;
  diff_mode: DiffMode;
  split_pos: number;
  amplifier: number;
  zoom: number;
  pan_u: number;
  pan_v: number;
  split_horizontal: boolean;
}

export async function sincronizarViewport(rect: RectViewport): Promise<void> {
  return invoke("sincronizar_viewport", { rect });
}

export async function ocultarViewport(): Promise<void> {
  return invoke("ocultar_viewport");
}

export async function establecerVistaCompare(vista: VistaCompare): Promise<void> {
  return invoke("establecer_vista_compare", { vista });
}
