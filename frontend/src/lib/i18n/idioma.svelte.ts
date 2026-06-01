import {
  type ClaveTraduccion,
  type Idioma,
  traducir,
} from "./traducciones";

const CLAVE_STORAGE = "diffplayerqc-v2-idioma";

function cargarIdioma(): Idioma {
  if (typeof localStorage === "undefined") {
    return "es";
  }
  const raw = localStorage.getItem(CLAVE_STORAGE);
  return raw === "en" ? "en" : "es";
}

function guardarIdioma(idioma: Idioma) {
  if (typeof localStorage === "undefined") return;
  localStorage.setItem(CLAVE_STORAGE, idioma);
}

class IdiomaStore {
  idioma = $state<Idioma>(cargarIdioma());

  /** Texto traducido para la clave dada (reactivo en plantillas Svelte). */
  t(clave: ClaveTraduccion): string {
    return traducir(this.idioma, clave);
  }

  establecer(idioma: Idioma) {
    this.idioma = idioma;
    guardarIdioma(idioma);
    if (typeof document !== "undefined") {
      document.documentElement.lang = idioma;
    }
  }

  alternar() {
    this.establecer(this.idioma === "es" ? "en" : "es");
  }
}

export const idiomaStore = new IdiomaStore();

if (typeof document !== "undefined") {
  document.documentElement.lang = idiomaStore.idioma;
}
