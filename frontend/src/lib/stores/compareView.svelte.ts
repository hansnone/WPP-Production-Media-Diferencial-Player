import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { CompareMode, DiffMode } from "../compare";
import { establecerVistaCompare, type VistaCompare } from "../viewport";

/** Vista de comparación compartida entre panel derecho, canvas y overlay wgpu. */
class CompareViewStore {
  /** Overlay wgpu: reproducción fluida (RGBA directo, sin JPEG). Canvas = fallback. */
  overlayGpuActiva = $state(true);

  /** SideBySide muestra A|B; diff en el panel derecho solo si se elige explícitamente. */
  vista = $state<VistaCompare>({
    modo: "SplitScreen",
    diff_mode: "None",
    split_pos: 0.5,
    amplifier: 5,
    zoom: 1,
    pan_u: 0,
    pan_v: 0,
    split_horizontal: false,
  });

  /** Solo vídeo A (cortina a la derecha). */
  soloA() {
    this.vista.modo = "SplitScreen";
    this.vista.split_pos = 1.0;
    void this.aplicar();
  }

  /** Solo vídeo B. */
  soloB() {
    this.vista.modo = "SplitScreen";
    this.vista.split_pos = 0.0;
    void this.aplicar();
  }

  /** Cortina al 50 % (o alterna desde solo A/B). */
  cortina() {
    this.vista.modo = "SplitScreen";
    const sp = this.vista.split_pos;
    if (sp > 0.95 || sp < 0.05) {
      this.vista.split_pos = 0.5;
    }
    void this.aplicar();
  }

  setModo(m: CompareMode) {
    this.vista.modo = m;
    if (m === "SideBySide") {
      this.vista.diff_mode = "None";
    } else if (m === "AbsDiff" || m === "Heatmap") {
      if (this.vista.diff_mode === "None") {
        this.vista.diff_mode = "AbsLinear";
      }
    }
    void this.aplicar();
  }

  setDiff(d: DiffMode) {
    this.vista.diff_mode = d;
    void this.aplicar();
  }

  setSplitPos(pos: number) {
    this.vista.split_pos = Math.min(1, Math.max(0, pos));
  }

  async aplicar() {
    if ("__TAURI_INTERNALS__" in window) {
      await establecerVistaCompare(this.vista).catch(() => undefined);
    }
  }

  /** true si la overlay wgpu está activa y operativa (desactivada por defecto). */
  gpuListo = $state(false);

  /** Escucha `viewport-gpu` (solo si `overlayGpuActiva`). */
  async iniciarEscuchaGpu(): Promise<UnlistenFn | undefined> {
    if (!("__TAURI_INTERNALS__" in window)) {
      return undefined;
    }
    return listen<{ listo: boolean }>("viewport-gpu", (ev) => {
      this.gpuListo = ev.payload.listo;
      if (ev.payload.listo) {
        window.dispatchEvent(new CustomEvent("diffplayerqc-sync-viewport"));
      }
    });
  }
}

export const compareViewStore = new CompareViewStore();
