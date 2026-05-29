import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
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

const FILTROS_VIDEO = [
  {
    name: "Vídeo",
    extensions: ["mp4", "mov", "mkv", "mxf", "avi", "webm"],
  },
];

export async function obtenerEstado(): Promise<SnapshotReproduccion> {
  return invoke("obtener_estado");
}

/**
 * Abre el diálogo nativo de archivos y carga el vídeo en el canal indicado.
 * El plugin `dialog` en el frontend evita bloqueos del hilo principal en macOS.
 */
export async function abrirDialogo(canal: Canal): Promise<SnapshotReproduccion | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }

  // La overlay wgpu puede quedarse encima y robar el foco del NSOpenPanel en macOS.
  await invoke("ocultar_viewport").catch(() => undefined);
  await getCurrentWindow().setFocus().catch(() => undefined);

  const ruta = await open({
    multiple: false,
    title: canal === "a" ? "Abrir vídeo A" : "Abrir vídeo B",
    filters: FILTROS_VIDEO,
  });

  if (!ruta || Array.isArray(ruta)) {
    return null;
  }

  const snap = await invoke<SnapshotReproduccion>("abrir_video", { canal, ruta });
  avisarResyncViewport();
  return snap;
}

export async function abrirVideo(canal: Canal, ruta: string): Promise<SnapshotReproduccion> {
  const snap = await invoke<SnapshotReproduccion>("abrir_video", { canal, ruta });
  avisarResyncViewport();
  return snap;
}

function avisarResyncViewport() {
  window.dispatchEvent(new CustomEvent("diffplayerqc-sync-viewport"));
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
