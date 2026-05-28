/** Etiquetas de workspace alineadas con `core::WorkspaceLayout`. */
const ETIQUETAS: Record<string, string> = {
  compare: "Compare",
  inspect: "Inspect",
  audio: "Audio",
  report: "Report",
  export: "Export",
};

export function etiquetaWorkspace(id: string): string {
  return ETIQUETAS[id] ?? id;
}
