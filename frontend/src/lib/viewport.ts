import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow, type WebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { CompareMode, DiffMode } from "./compare";

/** Rectángulo en píxeles físicos de pantalla (escritorio). */
export interface RectViewport {
  x: number;
  y: number;
  width: number;
  height: number;
  fisico?: boolean;
}

export interface VistaCompare {
  modo: CompareMode;
  diff_mode: DiffMode;
  split_pos: number;
  amplifier: number;
  zoom: number;
  pan_u: number;
  pan_v: number;
  split_horizontal: boolean;
}

interface MarcoVentana {
  escala: number;
  innerX: number;
  innerY: number;
  outerX: number;
  outerY: number;
  chromeSup: number;
  chromeIzq: number;
}

interface OrigenPantalla {
  ox: number;
  oy: number;
}

async function leerMarcoVentana(ventana: WebviewWindow): Promise<MarcoVentana> {
  const escala = await ventana.scaleFactor();
  const inner = await ventana.innerPosition();
  const outer = await ventana.outerPosition();
  const innerSize = await ventana.innerSize();
  const outerSize = await ventana.outerSize();
  const chromeSup = Math.max(0, outerSize.height - innerSize.height);
  const chromeIzq = Math.max(0, Math.round((outerSize.width - innerSize.width) / 2));
  return {
    escala,
    innerX: inner.x,
    innerY: inner.y,
    outerX: outer.x,
    outerY: outer.y,
    chromeSup,
    chromeIzq,
  };
}

function elegirOrigenPantalla(marco: MarcoVentana): OrigenPantalla {
  if (typeof window.screenX === "number" && typeof window.screenY === "number") {
    return {
      ox: window.screenX * marco.escala,
      oy: window.screenY * marco.escala,
    };
  }
  if (marco.chromeSup > 0) {
    return {
      ox: marco.outerX + marco.chromeIzq,
      oy: marco.outerY + marco.chromeSup,
    };
  }
  return { ox: marco.innerX, oy: marco.innerY };
}

function pantallaY(marco: MarcoVentana, origen: OrigenPantalla, logicoY: number): number {
  const desdeOrigen = origen.oy + logicoY * marco.escala;
  const candidatos = [marco.innerY + logicoY * marco.escala, desdeOrigen];
  if (marco.chromeSup > 0) {
    candidatos.push(marco.outerY + marco.chromeSup + logicoY * marco.escala);
  }
  if (typeof window.screenY === "number") {
    candidatos.push((window.screenY + logicoY) * marco.escala);
  }
  return Math.max(...candidatos);
}

function pantallaX(marco: MarcoVentana, origen: OrigenPantalla, logicoX: number): number {
  const desdeOrigen = origen.ox + logicoX * marco.escala;
  const candidatos = [marco.innerX + logicoX * marco.escala, desdeOrigen];
  if (marco.chromeIzq > 0) {
    candidatos.push(marco.outerX + marco.chromeIzq + logicoX * marco.escala);
  }
  if (typeof window.screenX === "number") {
    candidatos.push((window.screenX + logicoX) * marco.escala);
  }
  return Math.max(...candidatos);
}

/**
 * Convierte `#canvas-slot` a rectángulo de pantalla en píxeles físicos.
 * Ventana overlay independiente (sin parent): coords de escritorio.
 */
export async function rectViewportDesdeElemento(elemento: HTMLElement): Promise<RectViewport> {
  const ventana = getCurrentWebviewWindow();
  const marco = await leerMarcoVentana(ventana);
  const origen = elegirOrigenPantalla(marco);
  const r = elemento.getBoundingClientRect();

  let logIzq = r.left;
  let logSup = r.top;
  let logDer = r.right;
  let logInf = r.bottom;

  const toolbar = document.querySelector('[data-testid="toolbar"]');
  if (toolbar) {
    logSup = Math.max(logSup, toolbar.getBoundingClientRect().bottom);
  }

  const grid = document.querySelector('[data-testid="workspace-grid"]');
  if (grid) {
    logSup = Math.max(logSup, grid.getBoundingClientRect().top);
  }

  const zona = document.querySelector('[data-testid="workspace-main"]');
  if (zona) {
    const zr = zona.getBoundingClientRect();
    logIzq = Math.max(logIzq, zr.left);
    logSup = Math.max(logSup, zr.top);
    logDer = Math.min(logDer, zr.right);
    logInf = Math.min(logInf, zr.bottom);
  }

  const anchoLog = Math.max(1, logDer - logIzq);
  const altoLog = Math.max(1, logInf - logSup);

  return {
    x: Math.round(pantallaX(marco, origen, logIzq)),
    y: Math.round(pantallaY(marco, origen, logSup)),
    width: Math.max(8, Math.round(anchoLog * marco.escala)),
    height: Math.max(8, Math.round(altoLog * marco.escala)),
    fisico: true,
  };
}

export async function sincronizarViewport(rect: RectViewport): Promise<void> {
  return invoke("sincronizar_viewport", { rect });
}

export async function ocultarViewport(): Promise<void> {
  return invoke("ocultar_viewport");
}

export async function establecerVistaCompare(vista: VistaCompare): Promise<void> {
  return invoke("establecer_vista_compare", { vista });
}
