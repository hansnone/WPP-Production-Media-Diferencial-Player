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
import { formaOndaStore } from "./formaOnda.svelte";
import { recientesStore, type CanalReciente } from "./recientes.svelte";
import { scopesStore } from "./scopes.svelte";

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
    this.snap = await obtenerEstado();
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
        formaOndaStore.esperarEscaneo(canal);
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
      formaOndaStore.esperarEscaneo(canal);
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
}

export const playerStore = new PlayerStore();
