import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

/** Estado de reproducción emitido en `playback-tick` (alineado con Rust). */
export interface SnapshotReproduccion {
  pts_actual: number;
  reproduciendo: boolean;
  duracion_a: number;
  duracion_b: number;
  ruta_a: string | null;
  ruta_b: string | null;
  fps: number;
  nivel_audio_a: number;
  nivel_audio_b: number;
}

export type Canal = "a" | "b";

export async function obtenerEstado(): Promise<SnapshotReproduccion> {
  return invoke("obtener_estado");
}

export async function abrirDialogo(canal: Canal): Promise<SnapshotReproduccion | null> {
  return invoke("abrir_dialogo", { canal });
}

export async function abrirVideo(canal: Canal, ruta: string): Promise<SnapshotReproduccion> {
  return invoke("abrir_video", { canal, ruta });
}

export async function alternarPlay(): Promise<SnapshotReproduccion> {
  return invoke("alternar_play");
}

export async function seek(pts: number): Promise<SnapshotReproduccion> {
  return invoke("seek", { pts });
}

export async function stepAdelante(): Promise<SnapshotReproduccion> {
  return invoke("step_adelante");
}

export async function stepAtras(): Promise<SnapshotReproduccion> {
  return invoke("step_atras");
}

export function escucharTicks(
  callback: (snap: SnapshotReproduccion) => void,
): Promise<UnlistenFn> {
  return listen<SnapshotReproduccion>("playback-tick", (ev) => {
    callback(ev.payload);
  });
}

export function formatearPts(segundos: number): string {
  const s = Math.max(0, segundos);
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = Math.floor(s % 60);
  const fracc = Math.floor((s % 1) * 100);
  if (h > 0) {
    return `${h}:${String(m).padStart(2, "0")}:${String(sec).padStart(2, "0")}.${String(fracc).padStart(2, "0")}`;
  }
  return `${m}:${String(sec).padStart(2, "0")}.${String(fracc).padStart(2, "0")}`;
}
