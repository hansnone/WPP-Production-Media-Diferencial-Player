/** Idiomas soportados en v2 (M6). */
export type Idioma = "es" | "en";

export type ClaveTraduccion =
  | "menu.archivo"
  | "menu.ver"
  | "menu.ayuda"
  | "menu.abrirA"
  | "menu.abrirB"
  | "menu.recientes"
  | "menu.sinRecientes"
  | "menu.paleta"
  | "menu.idioma"
  | "menu.idiomaEs"
  | "menu.idiomaEn"
  | "menu.resetLayout"
  | "menu.limpiarRecientes"
  | "menu.ayudaVersion"
  | "toolbar.abrirA"
  | "toolbar.abrirB"
  | "toolbar.play"
  | "toolbar.frameAtras"
  | "toolbar.frameAdelante"
  | "toolbar.muteAOn"
  | "toolbar.muteAOff"
  | "toolbar.muteBOn"
  | "toolbar.muteBOff"
  | "toolbar.seek"
  | "toolbar.paleta"
  | "panel.fuentes"
  | "panel.histograma"
  | "panel.loudness"
  | "panel.diffAudio"
  | "panel.plegar"
  | "panel.expandir"
  | "workspace.compare"
  | "workspace.inspect"
  | "workspace.audio"
  | "workspace.report"
  | "workspace.export"
  | "palette.titulo"
  | "palette.placeholder"
  | "palette.grupo.archivo"
  | "palette.grupo.transporte"
  | "palette.grupo.vista"
  | "palette.grupo.paneles"
  | "palette.grupo.preferencias"
  | "palette.abrirA"
  | "palette.abrirB"
  | "palette.play"
  | "palette.compare"
  | "palette.inspect"
  | "palette.audio"
  | "palette.report"
  | "palette.export"
  | "palette.panelIzq"
  | "palette.panelDer"
  | "palette.idiomaEs"
  | "palette.idiomaEn"
  | "palette.resetLayout"
  | "aviso.navegador"
  | "fuentes.sinArchivo"
  | "fuentes.abrir"
  | "scopes.vacio"
  | "scopes.histograma"
  | "scopes.vectoscopio"
  | "scopes.monitorLuma"
  | "audio.escaneandoA"
  | "audio.escaneandoB"
  | "placeholder.report"
  | "placeholder.export";

const ES: Record<ClaveTraduccion, string> = {
  "menu.archivo": "Archivo",
  "menu.ver": "Ver",
  "menu.ayuda": "Ayuda",
  "menu.abrirA": "Abrir A…",
  "menu.abrirB": "Abrir B…",
  "menu.recientes": "Recientes",
  "menu.sinRecientes": "Sin archivos recientes",
  "menu.paleta": "Paleta de comandos…",
  "menu.idioma": "Idioma",
  "menu.idiomaEs": "Español",
  "menu.idiomaEn": "English",
  "menu.resetLayout": "Restablecer layout del workspace",
  "menu.limpiarRecientes": "Vaciar recientes",
  "menu.ayudaVersion": "v2 — DiffPlayerQC",
  "toolbar.abrirA": "Abrir A",
  "toolbar.abrirB": "Abrir B",
  "toolbar.play": "Play / Pausa",
  "toolbar.frameAtras": "Frame atrás",
  "toolbar.frameAdelante": "Frame adelante",
  "toolbar.muteAOn": "Activar audio A",
  "toolbar.muteAOff": "Silenciar A",
  "toolbar.muteBOn": "Activar audio B",
  "toolbar.muteBOff": "Silenciar B",
  "toolbar.seek": "Seek",
  "toolbar.paleta": "Paleta (Ctrl+K)",
  "panel.fuentes": "Fuentes",
  "panel.histograma": "Histograma",
  "panel.loudness": "Loudness",
  "panel.diffAudio": "Diff / Audio",
  "panel.plegar": "Plegar panel",
  "panel.expandir": "Expandir",
  "workspace.compare": "Compare",
  "workspace.inspect": "Inspect",
  "workspace.audio": "Audio",
  "workspace.report": "Report",
  "workspace.export": "Export",
  "palette.titulo": "Comandos",
  "palette.placeholder": "Buscar comando…",
  "palette.grupo.archivo": "Archivo",
  "palette.grupo.transporte": "Transporte",
  "palette.grupo.vista": "Vista",
  "palette.grupo.paneles": "Paneles",
  "palette.grupo.preferencias": "Preferencias",
  "palette.abrirA": "Abrir fuente A",
  "palette.abrirB": "Abrir fuente B",
  "palette.play": "Reproducir / Pausa",
  "palette.compare": "Workspace Compare",
  "palette.inspect": "Workspace Inspect",
  "palette.audio": "Workspace Audio",
  "palette.report": "Workspace Report",
  "palette.export": "Workspace Export",
  "palette.panelIzq": "Alternar panel izquierdo",
  "palette.panelDer": "Alternar panel derecho",
  "palette.idiomaEs": "Idioma: Español",
  "palette.idiomaEn": "Idioma: English",
  "palette.resetLayout": "Restablecer layout",
  "aviso.navegador": "Modo navegador: ejecuta cargo tauri dev para IPC completo.",
  "fuentes.sinArchivo": "Sin archivo",
  "fuentes.abrir": "Abrir",
  "scopes.vacio": "Abre un vídeo y reproduce o haz seek para ver scopes.",
  "scopes.histograma": "Histograma RGB",
  "scopes.vectoscopio": "Vectoscopio",
  "scopes.monitorLuma": "Monitor luma",
  "audio.escaneandoA": "Escaneando audio A…",
  "audio.escaneandoB": "Escaneando audio B…",
  "placeholder.report": "Vista PDF — disponible en hitos posteriores.",
  "placeholder.export": "Formulario de exportación — disponible en hitos posteriores.",
};

const EN: Record<ClaveTraduccion, string> = {
  "menu.archivo": "File",
  "menu.ver": "View",
  "menu.ayuda": "Help",
  "menu.abrirA": "Open A…",
  "menu.abrirB": "Open B…",
  "menu.recientes": "Recent",
  "menu.sinRecientes": "No recent files",
  "menu.paleta": "Command palette…",
  "menu.idioma": "Language",
  "menu.idiomaEs": "Español",
  "menu.idiomaEn": "English",
  "menu.resetLayout": "Reset workspace layout",
  "menu.limpiarRecientes": "Clear recent files",
  "menu.ayudaVersion": "v2 — DiffPlayerQC",
  "toolbar.abrirA": "Open A",
  "toolbar.abrirB": "Open B",
  "toolbar.play": "Play / Pause",
  "toolbar.frameAtras": "Previous frame",
  "toolbar.frameAdelante": "Next frame",
  "toolbar.muteAOn": "Unmute A",
  "toolbar.muteAOff": "Mute A",
  "toolbar.muteBOn": "Unmute B",
  "toolbar.muteBOff": "Mute B",
  "toolbar.seek": "Seek",
  "toolbar.paleta": "Palette (Ctrl+K)",
  "panel.fuentes": "Sources",
  "panel.histograma": "Histogram",
  "panel.loudness": "Loudness",
  "panel.diffAudio": "Diff / Audio",
  "panel.plegar": "Collapse panel",
  "panel.expandir": "Expand",
  "workspace.compare": "Compare",
  "workspace.inspect": "Inspect",
  "workspace.audio": "Audio",
  "workspace.report": "Report",
  "workspace.export": "Export",
  "palette.titulo": "Commands",
  "palette.placeholder": "Search command…",
  "palette.grupo.archivo": "File",
  "palette.grupo.transporte": "Transport",
  "palette.grupo.vista": "View",
  "palette.grupo.paneles": "Panels",
  "palette.grupo.preferencias": "Preferences",
  "palette.abrirA": "Open source A",
  "palette.abrirB": "Open source B",
  "palette.play": "Play / Pause",
  "palette.compare": "Compare workspace",
  "palette.inspect": "Inspect workspace",
  "palette.audio": "Audio workspace",
  "palette.report": "Report workspace",
  "palette.export": "Export workspace",
  "palette.panelIzq": "Toggle left panel",
  "palette.panelDer": "Toggle right panel",
  "palette.idiomaEs": "Language: Español",
  "palette.idiomaEn": "Language: English",
  "palette.resetLayout": "Reset layout",
  "aviso.navegador": "Browser mode: run cargo tauri dev for full IPC.",
  "fuentes.sinArchivo": "No file",
  "fuentes.abrir": "Open",
  "scopes.vacio": "Open a video and play or seek to see scopes.",
  "scopes.histograma": "RGB histogram",
  "scopes.vectoscopio": "Vectorscope",
  "scopes.monitorLuma": "Luma monitor",
  "audio.escaneandoA": "Scanning audio A…",
  "audio.escaneandoB": "Scanning audio B…",
  "placeholder.report": "PDF view — coming in a later milestone.",
  "placeholder.export": "Export form — coming in a later milestone.",
};

export const TRADUCCIONES: Record<Idioma, Record<ClaveTraduccion, string>> = {
  es: ES,
  en: EN,
};

export function traducir(idioma: Idioma, clave: ClaveTraduccion): string {
  return TRADUCCIONES[idioma][clave] ?? clave;
}
