import type { WorkspaceId } from "../stores/layout.svelte";

export interface ComandoPaleta {
  id: string;
  etiqueta: string;
  grupo: string;
  atajo?: string;
  ejecutar: () => void | Promise<void>;
}

export function filtrarComandos(
  comandos: ComandoPaleta[],
  consulta: string,
): ComandoPaleta[] {
  const q = consulta.trim().toLowerCase();
  if (!q) return comandos;
  return comandos.filter(
    (c) =>
      c.etiqueta.toLowerCase().includes(q) ||
      c.grupo.toLowerCase().includes(q) ||
      c.id.includes(q),
  );
}

export type FabricaComandos = {
  abrirA: () => Promise<void>;
  abrirB: () => Promise<void>;
  playPausa: () => Promise<void>;
  irWorkspace: (id: WorkspaceId) => void;
  togglePanelIzq: () => void;
  togglePanelDer: () => void;
};

export function crearComandos(f: FabricaComandos): ComandoPaleta[] {
  return [
    {
      id: "file.open-a",
      etiqueta: "Abrir fuente A",
      grupo: "Archivo",
      ejecutar: f.abrirA,
    },
    {
      id: "file.open-b",
      etiqueta: "Abrir fuente B",
      grupo: "Archivo",
      ejecutar: f.abrirB,
    },
    {
      id: "transport.play",
      etiqueta: "Reproducir / Pausa",
      grupo: "Transporte",
      atajo: "Space",
      ejecutar: f.playPausa,
    },
    {
      id: "view.compare",
      etiqueta: "Workspace Compare",
      grupo: "Vista",
      atajo: "Shift+1",
      ejecutar: () => f.irWorkspace("compare"),
    },
    {
      id: "view.inspect",
      etiqueta: "Workspace Inspect",
      grupo: "Vista",
      atajo: "Shift+2",
      ejecutar: () => f.irWorkspace("inspect"),
    },
    {
      id: "view.audio",
      etiqueta: "Workspace Audio",
      grupo: "Vista",
      atajo: "Shift+3",
      ejecutar: () => f.irWorkspace("audio"),
    },
    {
      id: "view.report",
      etiqueta: "Workspace Report",
      grupo: "Vista",
      atajo: "Shift+4",
      ejecutar: () => f.irWorkspace("report"),
    },
    {
      id: "view.export",
      etiqueta: "Workspace Export",
      grupo: "Vista",
      atajo: "Shift+5",
      ejecutar: () => f.irWorkspace("export"),
    },
    {
      id: "panel.toggle-left",
      etiqueta: "Alternar panel izquierdo",
      grupo: "Paneles",
      ejecutar: f.togglePanelIzq,
    },
    {
      id: "panel.toggle-right",
      etiqueta: "Alternar panel derecho",
      grupo: "Paneles",
      ejecutar: f.togglePanelDer,
    },
  ];
}
