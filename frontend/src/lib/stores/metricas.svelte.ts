import {
  escucharMetricasLista,
  escucharMetricasProgreso,
  exportarMetricasCsv,
  obtenerMetricasVideo,
  type SerieMetricasVideo,
} from "../metricas";
import { seek } from "../player";

class MetricasStore {
  serie = $state<SerieMetricasVideo | null>(null);
  escaneando = $state(false);
  progreso = $state(0);
  private escuchaLista = false;

  async registrarEscucha(): Promise<void> {
    if (!("__TAURI_INTERNALS__" in window) || this.escuchaLista) {
      return;
    }
    await escucharMetricasLista((ev) => {
      this.serie = ev.serie;
      this.escaneando = false;
      this.progreso = 1;
    });
    await escucharMetricasProgreso((ev) => {
      this.escaneando = true;
      this.progreso = ev.fraccion;
    });
    this.escuchaLista = true;
    await this.refrescar();
  }

  async refrescar(): Promise<void> {
    if (!("__TAURI_INTERNALS__" in window)) {
      return;
    }
    const datos = await obtenerMetricasVideo();
    if (datos !== null) {
      this.serie = datos;
      this.escaneando = false;
    }
  }

  marcarEscaneando() {
    this.serie = null;
    this.escaneando = true;
    this.progreso = 0;
  }

  async descargarCsv(): Promise<void> {
    const csv = await exportarMetricasCsv();
    const blob = new Blob([csv], { type: "text/csv;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `diffplayerqc-metricas-${Date.now()}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  }

  /** Salta al siguiente punto con SSIM bajo (timeline QC). */
  async siguienteCaida(ptsActual: number): Promise<void> {
    const serie = this.serie;
    if (!serie) return;
    const umbral = serie.umbral_ssim_bajo;
    const siguiente = serie.puntos.find((p) => p.pts > ptsActual + 0.001 && p.ssim < umbral);
    if (siguiente === undefined) return;
    await seek(siguiente.pts);
  }

  async anteriorCaida(ptsActual: number): Promise<void> {
    const serie = this.serie;
    if (!serie) return;
    const umbral = serie.umbral_ssim_bajo;
    let candidato: (typeof serie.puntos)[0] | undefined;
    for (const p of serie.puntos) {
      if (p.pts < ptsActual - 0.001 && p.ssim < umbral) {
        candidato = p;
      }
    }
    if (candidato === undefined) return;
    await seek(candidato.pts);
  }

  async descargarJson(): Promise<void> {
    if (!this.serie) return;
    const json = JSON.stringify(this.serie, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `diffplayerqc-metricas-${Date.now()}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }
}

export const metricasStore = new MetricasStore();
