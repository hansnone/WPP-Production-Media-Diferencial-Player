import { layoutStore, type WorkspaceId } from "./stores/layout.svelte";
import { playerStore } from "./stores/player.svelte";

const WORKSPACES: WorkspaceId[] = [
  "compare",
  "inspect",
  "audio",
  "report",
  "export",
];

export type AtajosHandlers = {
  abrirPaleta: () => void;
};

export function manejarAtajoGlobal(e: KeyboardEvent, h: AtajosHandlers): boolean {
  const mod = e.metaKey || e.ctrlKey;

  if (mod && e.key.toLowerCase() === "k") {
    e.preventDefault();
    h.abrirPaleta();
    return true;
  }

  if (e.shiftKey && e.code.startsWith("Digit")) {
    const n = Number(e.code.replace("Digit", ""));
    if (n >= 1 && n <= 5) {
      e.preventDefault();
      layoutStore.cambiarWorkspace(WORKSPACES[n - 1]);
      return true;
    }
  }

  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
    return false;
  }

  if (e.code === "Space") {
    e.preventDefault();
    void playerStore.playPausa();
    return true;
  }
  if (e.code === "ArrowRight") {
    void playerStore.stepFwd();
    return true;
  }
  if (e.code === "ArrowLeft") {
    void playerStore.stepBck();
    return true;
  }

  return false;
}
