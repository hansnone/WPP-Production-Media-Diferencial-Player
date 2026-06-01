/** Archivos abiertos recientemente (M6), persistidos en localStorage. */

export type CanalReciente = "a" | "b";

export interface EntradaReciente {
  ruta: string;
  nombre: string;
  canal: CanalReciente;
  ultimoUso: number;
}

const CLAVE_STORAGE = "diffplayerqc-v2-recientes";
const MAX_ENTRADAS = 12;

function nombreDesdeRuta(ruta: string): string {
  const partes = ruta.replace(/\\/g, "/").split("/");
  return partes[partes.length - 1] || ruta;
}

function cargar(): EntradaReciente[] {
  if (typeof localStorage === "undefined") {
    return [];
  }
  try {
    const raw = localStorage.getItem(CLAVE_STORAGE);
    if (!raw) return [];
    const parsed = JSON.parse(raw) as EntradaReciente[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function guardar(lista: EntradaReciente[]) {
  if (typeof localStorage === "undefined") return;
  localStorage.setItem(CLAVE_STORAGE, JSON.stringify(lista));
}

class RecientesStore {
  entradas = $state<EntradaReciente[]>(cargar());

  registrar(ruta: string, canal: CanalReciente) {
    const nombre = nombreDesdeRuta(ruta);
    const filtradas = this.entradas.filter((e) => e.ruta !== ruta);
    const nueva: EntradaReciente = {
      ruta,
      nombre,
      canal,
      ultimoUso: Date.now(),
    };
    this.entradas = [nueva, ...filtradas].slice(0, MAX_ENTRADAS);
    guardar(this.entradas);
  }

  vaciar() {
    this.entradas = [];
    guardar(this.entradas);
  }
}

export const recientesStore = new RecientesStore();
