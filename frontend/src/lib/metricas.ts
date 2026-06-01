import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface PuntoMetrica {
  pts: number;
  ssim: number;
  ms_ssim: number;
  psnr: number;
  mse: number;
  vmaf?: number | null;
}

export interface SerieMetricasVideo {
  puntos: PuntoMetrica[];
  duracion_secs: number;
  muestras_por_segundo: number;
  umbral_ssim_bajo: number;
  vmaf_integrado?: number | null;
  vmaf_disponible_en_sistema: boolean;
}

export interface MetricasProgresoEvento {
  fraccion: number;
}

export interface MetricasVideoEvento {
  serie: SerieMetricasVideo;
}

export async function obtenerMetricasVideo(): Promise<SerieMetricasVideo | null> {
  return invoke("obtener_metricas_video");
}

export async function exportarMetricasCsv(): Promise<string> {
  return invoke("exportar_metricas_csv");
}

export function escucharMetricasLista(
  callback: (ev: MetricasVideoEvento) => void,
): Promise<UnlistenFn> {
  return listen<MetricasVideoEvento>("metricas-lista", (e) => {
    callback(e.payload);
  });
}

export function escucharMetricasProgreso(
  callback: (ev: MetricasProgresoEvento) => void,
): Promise<UnlistenFn> {
  return listen<MetricasProgresoEvento>("metricas-progreso", (e) => {
    callback(e.payload);
  });
}
