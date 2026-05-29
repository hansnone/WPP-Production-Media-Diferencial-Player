import {
  alternarPlay,
  abrirDialogo,
  escucharTicks,
  obtenerEstado,
  seek,
  stepAdelante,
  stepAtras,
  type SnapshotReproduccion,
} from "../player";

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
    this.snap = await obtenerEstado();
    await escucharTicks((s) => {
      this.snap = s;
    });
    this.inicializado = true;
  }

  async abrir(canal: "a" | "b") {
    try {
      const s = await abrirDialogo(canal);
      if (s) this.snap = s;
    } catch (err) {
      console.error("Error al abrir vídeo:", err);
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
}

export const playerStore = new PlayerStore();
