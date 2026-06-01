import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { SnapshotReproduccion } from "./player";

/** Tipo de hallazgo QC (alineado con Rust). */
export type TipoEventoQc = "manual" | "video" | "audio";

export interface NotaQc {
  id: number;
  texto: string;
  pts_secs: number;
  creado_unix_ms: number;
}

export interface EventoQc {
  id: number;
  tipo: TipoEventoQc;
  pts_secs: number;
  titulo: string;
  descripcion?: string | null;
  notas: NotaQc[];
  creado_unix_ms: number;
}

export interface RegistroEventosQc {
  clave_proyecto: string;
  eventos: EventoQc[];
  siguiente_id: number;
}

export type FiltroTipoEvento = TipoEventoQc | "todos";

const CLAVE_LS_PREFIX = "diffplayerqc-v2-eventos-";

export function claveLocalStorage(rutaA?: string | null, rutaB?: string | null): string {
  const a = (rutaA ?? "").trim();
  const b = (rutaB ?? "").trim();
  if (!a && !b) return `${CLAVE_LS_PREFIX}sin-proyecto`;
  return `${CLAVE_LS_PREFIX}${a}::${b}`;
}

export function cargarRegistroLocal(clave: string): RegistroEventosQc {
  try {
    const raw = localStorage.getItem(clave);
    if (!raw) {
      return { clave_proyecto: clave, eventos: [], siguiente_id: 1 };
    }
    return JSON.parse(raw) as RegistroEventosQc;
  } catch {
    return { clave_proyecto: clave, eventos: [], siguiente_id: 1 };
  }
}

export function guardarRegistroLocal(clave: string, reg: RegistroEventosQc): void {
  localStorage.setItem(clave, JSON.stringify(reg));
}

export async function actualizarProyectoEventos(
  rutaA?: string | null,
  rutaB?: string | null,
): Promise<RegistroEventosQc | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }
  return invoke<RegistroEventosQc>("actualizar_proyecto_eventos", {
    rutaA: rutaA ?? null,
    rutaB: rutaB ?? null,
  });
}

export async function listarEventos(
  filtroTipo?: TipoEventoQc | null,
): Promise<EventoQc[]> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return [];
  }
  return invoke<EventoQc[]>("listar_eventos", {
    filtroTipo: filtroTipo ?? null,
  });
}

export async function crearEvento(
  tipo: TipoEventoQc,
  ptsSecs: number,
  titulo: string,
  descripcion?: string,
): Promise<EventoQc | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }
  return invoke<EventoQc>("crear_evento", {
    tipo,
    ptsSecs,
    titulo,
    descripcion: descripcion ?? null,
  });
}

export async function crearNota(
  eventoId: number,
  texto: string,
  ptsSecs: number,
): Promise<EventoQc | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }
  return invoke<EventoQc>("crear_nota", {
    eventoId,
    texto,
    ptsSecs,
  });
}

export async function eliminarEvento(id: number): Promise<boolean> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return false;
  }
  return invoke<boolean>("eliminar_evento", { id });
}

export async function seekAEvento(id: number): Promise<SnapshotReproduccion | null> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }
  return invoke<SnapshotReproduccion>("seek_a_evento", { id });
}

export function escucharEventosQc(
  callback: (reg: RegistroEventosQc) => void,
): Promise<UnlistenFn> {
  return listen<RegistroEventosQc>("eventos-qc-actualizados", (ev) => {
    callback(ev.payload);
  });
}
