import {
  actualizarProyectoEventos,
  cargarRegistroLocal,
  claveLocalStorage,
  crearEvento,
  crearNota,
  eliminarEvento,
  escucharEventosQc,
  guardarRegistroLocal,
  listarEventos,
  type EventoQc,
  type FiltroTipoEvento,
  type RegistroEventosQc,
  type TipoEventoQc,
} from "../eventosQc";

class EventosQcStore {
  eventos = $state<EventoQc[]>([]);
  filtro = $state<FiltroTipoEvento>("todos");
  claveProyecto = $state("sin-proyecto");
  eventoSeleccionadoId = $state<number | null>(null);

  private escuchaLista = false;
  private claveLs = claveLocalStorage();

  async registrarEscucha(): Promise<void> {
    if (!("__TAURI_INTERNALS__" in window)) {
      return;
    }
    if (this.escuchaLista) {
      return;
    }
    await escucharEventosQc((reg) => this.aplicarRegistro(reg));
    this.escuchaLista = true;
  }

  private aplicarRegistro(reg: RegistroEventosQc) {
    this.claveProyecto = reg.clave_proyecto;
    this.eventos = [...reg.eventos].sort((a, b) => a.pts_secs - b.pts_secs);
  }

  /** Sincroniza proyecto con rutas A/B del reproductor. */
  async sincronizarProyecto(rutaA?: string | null, rutaB?: string | null): Promise<void> {
    if ("__TAURI_INTERNALS__" in window) {
      const reg = await actualizarProyectoEventos(rutaA, rutaB);
      if (reg) {
        this.aplicarRegistro(reg);
      }
      await this.refrescarLista();
      return;
    }

    this.claveLs = claveLocalStorage(rutaA, rutaB);
    const reg = cargarRegistroLocal(this.claveLs);
    this.aplicarRegistro(reg);
  }

  establecerFiltro(f: FiltroTipoEvento) {
    this.filtro = f;
    void this.refrescarLista();
  }

  eventosFiltrados(): EventoQc[] {
    if (this.filtro === "todos") {
      return this.eventos;
    }
    return this.eventos.filter((e) => e.tipo === this.filtro);
  }

  async refrescarLista(): Promise<void> {
    if ("__TAURI_INTERNALS__" in window) {
      const filtro = this.filtro === "todos" ? null : this.filtro;
      this.eventos = await listarEventos(filtro);
      return;
    }
    const reg = cargarRegistroLocal(this.claveLs);
    this.aplicarRegistro(reg);
    if (this.filtro !== "todos") {
      this.eventos = this.eventos.filter((e) => e.tipo === this.filtro);
    }
  }

  async marcarEnPlayhead(
    ptsSecs: number,
    titulo: string,
    tipo: TipoEventoQc = "manual",
  ): Promise<void> {
    if ("__TAURI_INTERNALS__" in window) {
      await crearEvento(tipo, ptsSecs, titulo, undefined);
      await this.refrescarLista();
      return;
    }
    const reg = cargarRegistroLocal(this.claveLs);
    const id = reg.siguiente_id;
    reg.siguiente_id += 1;
    reg.eventos.push({
      id,
      tipo,
      pts_secs: ptsSecs,
      titulo,
      notas: [],
      creado_unix_ms: Date.now(),
    });
    reg.eventos.sort((a, b) => a.pts_secs - b.pts_secs);
    guardarRegistroLocal(this.claveLs, reg);
    this.aplicarRegistro(reg);
  }

  async anadirNota(eventoId: number, texto: string, ptsSecs: number): Promise<void> {
    if ("__TAURI_INTERNALS__" in window) {
      await crearNota(eventoId, texto, ptsSecs);
      await this.refrescarLista();
      return;
    }
    const reg = cargarRegistroLocal(this.claveLs);
    const ev = reg.eventos.find((e) => e.id === eventoId);
    if (!ev) return;
    const notaId = reg.siguiente_id;
    reg.siguiente_id += 1;
    ev.notas.push({
      id: notaId,
      texto,
      pts_secs: ptsSecs,
      creado_unix_ms: Date.now(),
    });
    guardarRegistroLocal(this.claveLs, reg);
    this.aplicarRegistro(reg);
  }

  async borrar(id: number): Promise<void> {
    if ("__TAURI_INTERNALS__" in window) {
      await eliminarEvento(id);
      await this.refrescarLista();
    } else {
      const reg = cargarRegistroLocal(this.claveLs);
      reg.eventos = reg.eventos.filter((e) => e.id !== id);
      guardarRegistroLocal(this.claveLs, reg);
      this.aplicarRegistro(reg);
    }
    if (this.eventoSeleccionadoId === id) {
      this.eventoSeleccionadoId = null;
    }
  }

  seleccionar(id: number | null) {
    this.eventoSeleccionadoId = id;
  }

  ptsDeEvento(id: number): number | null {
    return this.eventos.find((e) => e.id === id)?.pts_secs ?? null;
  }
}

export const eventosQcStore = new EventosQcStore();
