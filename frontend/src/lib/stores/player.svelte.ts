import {
  alternarPlay,
  abrirDialogo,
  abrirVideo,
  alternarMuteAudio,
  escucharTicks,
  obtenerEstado,
  seek,
  stepAdelante,
  stepAtras,
  type SnapshotReproduccion,
} from "../player";
import { compareViewStore } from "./compareView.svelte";
import { formaOndaStore } from "./formaOnda.svelte";
import { recientesStore, type CanalReciente } from "./recientes.svelte";
import { scopesStore } from "./scopes.svelte";
import { seekAEvento } from "../eventosQc";
import { metricasStore } from "./metricas.svelte";
import { eventosQcStore } from "./eventosQc.svelte";

class PlayerStore {
  snap = $state<SnapshotReproduccion | null>(null);
  enTauri = $state(false);
  inicializado = $state(false);

  async iniciar() {
    this.enTauri = "__TAURI_INTERNALS__" in window;
    if (!this.enTauri) {
      this.inicializado = true;
      return;
    }
    await formaOndaStore.registrarEscucha();
    await scopesStore.registrarEscucha();
    await metricasStore.registrarEscucha();
    await eventosQcStore.registrarEscucha();
    await compareViewStore.iniciarEscuchaGpu();
    this.snap = await obtenerEstado();
    await eventosQcStore.sincronizarProyecto(this.snap?.ruta_a, this.snap?.ruta_b);
    await escucharTicks((s) => {
      this.snap = s;
    });
    this.inicializado = true;
  }

  async abrir(canal: "a" | "b") {
    try {
      formaOndaStore.marcarEscaneando(canal);
      const s = await abrirDialogo(canal);
      if (s) {
        this.snap = s;
        const ruta = canal === "a" ? s.ruta_a : s.ruta_b;
        if (ruta) recientesStore.registrar(ruta, canal);
        void eventosQcStore.sincronizarProyecto(s.ruta_a, s.ruta_b);
        formaOndaStore.esperarEscaneo(canal);
        if (s.ruta_a && s.ruta_b) {
          metricasStore.marcarEscaneando();
        }
      }
    } catch (err) {
      console.error("Error al abrir vídeo:", err);
    }
  }

  /** Abre una ruta del historial en el canal indicado (menú Recientes). */
  async abrirReciente(ruta: string, canal: CanalReciente) {
    if (!("__TAURI_INTERNALS__" in window)) {
      return;
    }
    try {
      formaOndaStore.marcarEscaneando(canal);
      this.snap = await abrirVideo(canal, ruta);
      recientesStore.registrar(ruta, canal);
      void eventosQcStore.sincronizarProyecto(this.snap?.ruta_a, this.snap?.ruta_b);
      formaOndaStore.esperarEscaneo(canal);
      if (this.snap?.ruta_a && this.snap?.ruta_b) {
        metricasStore.marcarEscaneando();
      }
    } catch (err) {
      console.error("Error al abrir reciente:", err);
    }
  }

  async playPausa() {
    this.snap = await alternarPlay();
  }

  async seekPts(pts: number) {
    this.snap = await seek(pts);
  }

  async stepFwd() {
    this.snap = await stepAdelante();
  }

  async stepBck() {
    this.snap = await stepAtras();
  }

  async alternarMute(canal: "a" | "b") {
    this.snap = await alternarMuteAudio(canal);
  }

  /** Seek al PTS del evento QC y resalta la fila en listados (M10). */
  async seekAEventoQc(id: number) {
    eventosQcStore.seleccionar(id);
    const pts = eventosQcStore.ptsDeEvento(id);
    if ("__TAURI_INTERNALS__" in window) {
      const snap = await seekAEvento(id);
      if (snap) {
        this.snap = snap;
      }
    } else if (pts !== null) {
      await this.seekPts(pts);
    }
  }
}

export const playerStore = new PlayerStore();
