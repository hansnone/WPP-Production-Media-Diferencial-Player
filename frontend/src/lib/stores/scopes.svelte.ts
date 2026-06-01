import { escucharScopes, obtenerScopes, type ScopesFrame } from "../scopes";

class ScopesStore {
  actual = $state<ScopesFrame | null>(null);
  private escuchaLista = false;

  async registrarEscucha(): Promise<void> {
    if (!("__TAURI_INTERNALS__" in window) || this.escuchaLista) {
      return;
    }
    await escucharScopes((frame) => {
      this.actual = frame;
    });
    this.escuchaLista = true;
    this.actual = await obtenerScopes();
  }

  async refrescar(): Promise<void> {
    if (!("__TAURI_INTERNALS__" in window)) {
      return;
    }
    this.actual = await obtenerScopes();
  }
}

export const scopesStore = new ScopesStore();
