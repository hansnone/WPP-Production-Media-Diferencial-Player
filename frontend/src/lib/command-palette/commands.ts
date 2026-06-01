import type { ClaveTraduccion } from "../i18n/traducciones";
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
  t: (clave: ClaveTraduccion) => string;
  abrirA: () => Promise<void>;
  abrirB: () => Promise<void>;
  playPausa: () => Promise<void>;
  irWorkspace: (id: WorkspaceId) => void;
  togglePanelIzq: () => void;
  togglePanelDer: () => void;
  idiomaEs: () => void;
  idiomaEn: () => void;
  resetLayout: () => void;
};

export function crearComandos(f: FabricaComandos): ComandoPaleta[] {
  return [
    {
      id: "file.open-a",
      etiqueta: f.t("palette.abrirA"),
      grupo: f.t("palette.grupo.archivo"),
      ejecutar: f.abrirA,
    },
    {
      id: "file.open-b",
      etiqueta: f.t("palette.abrirB"),
      grupo: f.t("palette.grupo.archivo"),
      ejecutar: f.abrirB,
    },
    {
      id: "transport.play",
      etiqueta: f.t("palette.play"),
      grupo: f.t("palette.grupo.transporte"),
      atajo: "Space",
      ejecutar: f.playPausa,
    },
    {
      id: "view.compare",
      etiqueta: f.t("palette.compare"),
      grupo: f.t("palette.grupo.vista"),
      atajo: "Shift+1",
      ejecutar: () => f.irWorkspace("compare"),
    },
    {
      id: "view.inspect",
      etiqueta: f.t("palette.inspect"),
      grupo: f.t("palette.grupo.vista"),
      atajo: "Shift+2",
      ejecutar: () => f.irWorkspace("inspect"),
    },
    {
      id: "view.audio",
      etiqueta: f.t("palette.audio"),
      grupo: f.t("palette.grupo.vista"),
      atajo: "Shift+3",
      ejecutar: () => f.irWorkspace("audio"),
    },
    {
      id: "view.report",
      etiqueta: f.t("palette.report"),
      grupo: f.t("palette.grupo.vista"),
      atajo: "Shift+4",
      ejecutar: () => f.irWorkspace("report"),
    },
    {
      id: "view.export",
      etiqueta: f.t("palette.export"),
      grupo: f.t("palette.grupo.vista"),
      atajo: "Shift+5",
      ejecutar: () => f.irWorkspace("export"),
    },
    {
      id: "panel.toggle-left",
      etiqueta: f.t("palette.panelIzq"),
      grupo: f.t("palette.grupo.paneles"),
      ejecutar: f.togglePanelIzq,
    },
    {
      id: "panel.toggle-right",
      etiqueta: f.t("palette.panelDer"),
      grupo: f.t("palette.grupo.paneles"),
      ejecutar: f.togglePanelDer,
    },
    {
      id: "prefs.lang-es",
      etiqueta: f.t("palette.idiomaEs"),
      grupo: f.t("palette.grupo.preferencias"),
      ejecutar: f.idiomaEs,
    },
    {
      id: "prefs.lang-en",
      etiqueta: f.t("palette.idiomaEn"),
      grupo: f.t("palette.grupo.preferencias"),
      ejecutar: f.idiomaEn,
    },
    {
      id: "prefs.reset-layout",
      etiqueta: f.t("palette.resetLayout"),
      grupo: f.t("palette.grupo.preferencias"),
      ejecutar: f.resetLayout,
    },
  ];
}
