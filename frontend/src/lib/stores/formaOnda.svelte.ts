import {
  escucharFormaOnda,
  obtenerFormaOnda,
  type FormaOnda,
  type FormaOndaEvento,
} from "../formaOnda";

class FormaOndaStore {
  formaA = $state<FormaOnda | null>(null);
  formaB = $state<FormaOnda | null>(null);
  escaneandoA = $state(false);
  escaneandoB = $state(false);

  private escuchaLista = false;
  private timersEspera = new Map<"a" | "b", ReturnType<typeof setTimeout>>();

  /** Registrar listener lo antes posible (AppShell), no solo al entrar en Audio. */
  async registrarEscucha(): Promise<void> {
    if (!("__TAURI_INTERNALS__" in window) || this.escuchaLista) {
      return;
    }
    await escucharFormaOnda((ev) => this.aplicarEvento(ev));
    this.escuchaLista = true;
    await this.refrescarDesdeBackend();
  }

  private aplicarEvento(ev: FormaOndaEvento) {
    this.cancelarEspera(ev.canal);
    if (ev.canal === "a") {
      this.formaA = ev;
      this.escaneandoA = false;
    } else {
      this.formaB = ev;
      this.escaneandoB = false;
    }
  }

  /** Sincroniza con la caché del motor (por si el evento se emitió antes del listener). */
  async refrescarDesdeBackend(): Promise<void> {
    if (!("__TAURI_INTERNALS__" in window)) {
      return;
    }
    const [a, b] = await Promise.all([obtenerFormaOnda("a"), obtenerFormaOnda("b")]);
    if (a !== null) {
      this.formaA = a;
      this.escaneandoA = false;
    }
    if (b !== null) {
      this.formaB = b;
      this.escaneandoB = false;
    }
  }

  /** Invalida caché al abrir; el backend escanea en paralelo. */
  marcarEscaneando(canal: "a" | "b") {
    if (canal === "a") {
      this.formaA = null;
      this.escaneandoA = true;
    } else {
      this.formaB = null;
      this.escaneandoB = true;
    }
  }

  /**
   * Respaldo por polling: el evento Tauri puede perderse si el usuario
   * abre archivos antes de montar el listener.
   */
  esperarEscaneo(canal: "a" | "b") {
    this.cancelarEspera(canal);
    const inicio = Date.now();
    const intervaloMs = 250;
    const timeoutMs = 60_000;

    const tick = async () => {
      const datos = await obtenerFormaOnda(canal);
      if (datos !== null) {
        if (canal === "a") {
          this.formaA = datos;
          this.escaneandoA = false;
        } else {
          this.formaB = datos;
          this.escaneandoB = false;
        }
        this.cancelarEspera(canal);
        return;
      }
      if (Date.now() - inicio >= timeoutMs) {
        if (canal === "a") {
          this.escaneandoA = false;
        } else {
          this.escaneandoB = false;
        }
        this.cancelarEspera(canal);
        return;
      }
      const id = setTimeout(() => void tick(), intervaloMs);
      this.timersEspera.set(canal, id);
    };

    void tick();
  }

  private cancelarEspera(canal: "a" | "b") {
    const id = this.timersEspera.get(canal);
    if (id !== undefined) {
      clearTimeout(id);
      this.timersEspera.delete(canal);
    }
  }
}

export const formaOndaStore = new FormaOndaStore();
