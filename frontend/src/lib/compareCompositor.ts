import type { CompareMode, DiffMode } from "./compare";
import type { VistaCompare } from "./viewport";

const AMARILLO = "#ffff00";
const GROSOR_LINEA = 3;

export interface RectVideo {
  x: number;
  y: number;
  w: number;
  h: number;
}

function indiceDiffMode(m: DiffMode): number {
  const map: Record<DiffMode, number> = {
    LegacyAbs: 0,
    AbsLinear: 1,
    AbsSqrt: 2,
    SignedDiverging: 3,
    None: 4,
  };
  return map[m];
}

function diferenciaRgb(
  a: [number, number, number],
  b: [number, number, number],
  diffMode: DiffMode,
  amp: number,
): [number, number, number] {
  const d0 = Math.abs(a[0] - b[0]);
  const d1 = Math.abs(a[1] - b[1]);
  const d2 = Math.abs(a[2] - b[2]);
  const idx = indiceDiffMode(diffMode);

  if (idx === 0) {
    return [
      Math.min(255, d0 * 2 * 255),
      Math.min(255, d1 * 2 * 255),
      Math.min(255, d2 * 2 * 255),
    ];
  }
  if (idx === 1) {
    return [
      Math.min(255, d0 * amp * 255),
      Math.min(255, d1 * amp * 255),
      Math.min(255, d2 * amp * 255),
    ];
  }
  if (idx === 2) {
    return [
      Math.sqrt(Math.min(1, d0 * amp)) * 255,
      Math.sqrt(Math.min(1, d1 * amp)) * 255,
      Math.sqrt(Math.min(1, d2 * amp)) * 255,
    ];
  }
  // SignedDiverging
  const mag = (c: number) => Math.sqrt(Math.min(1, c * amp));
  const sign = (da: number, db: number) => (da >= db ? 1 : -1);
  const m0 = mag(d0);
  const m1 = mag(d1);
  const m2 = mag(d2);
  const s0 = sign(a[0], b[0]);
  const s1 = sign(a[1], b[1]);
  const s2 = sign(a[2], b[2]);
  return [
    (0.5 + s0 * m0 * 0.5) * 255,
    (0.5 + s1 * m1 * 0.5) * 255,
    (0.5 + s2 * m2 * 0.5) * 255,
  ];
}

function colorHeatmap(t: number): [number, number, number] {
  const c = Math.min(1, Math.max(0, t));
  const r = c < 0.4 ? 0 : c < 0.8 ? ((c - 0.4) / 0.4) * 255 : 255;
  const g =
    c < 0.5
      ? (c / 0.5) * 255
      : c < 1
        ? (1 - (c - 0.5) / 0.5) * 200
        : 0;
  const b = c < 0.25 ? (c / 0.25) * 180 : c < 0.6 ? (1 - (c - 0.25) / 0.35) * 180 : 0;
  return [r, g, b];
}

/** Área letterbox del vídeo dentro del canvas. */
export function calcularRectVideo(
  cw: number,
  ch: number,
  vidW: number,
  vidH: number,
): RectVideo {
  if (vidW < 1 || vidH < 1) {
    return { x: 0, y: 0, w: cw, h: ch };
  }
  const escala = Math.min(cw / vidW, ch / vidH);
  const w = vidW * escala;
  const h = vidH * escala;
  return { x: (cw - w) / 2, y: (ch - h) / 2, w, h };
}

/** Dibuja un canvas offscreen con letterbox dentro de la región (evita estirar al hueco). */
function dibujarCanvasEnRegion(
  ctx: CanvasRenderingContext2D,
  fuente: HTMLCanvasElement,
  region: RectVideo,
) {
  const imgAspect = fuente.width / Math.max(1, fuente.height);
  const regAspect = region.w / Math.max(1, region.h);
  let dw: number;
  let dh: number;
  let dx: number;
  let dy: number;
  if (regAspect > imgAspect) {
    dh = region.h;
    dw = dh * imgAspect;
    dx = region.x + (region.w - dw) / 2;
    dy = region.y;
  } else {
    dw = region.w;
    dh = dw / imgAspect;
    dx = region.x;
    dy = region.y + (region.h - dh) / 2;
  }
  ctx.drawImage(fuente, dx, dy, dw, dh);
}

/** Ancho/alto de una fuente dibujable en canvas. */
function dimensionesFuente(img: CanvasImageSource): { w: number; h: number } {
  if (img instanceof HTMLImageElement) {
    return { w: img.naturalWidth, h: img.naturalHeight };
  }
  if (img instanceof HTMLVideoElement) {
    return { w: img.videoWidth, h: img.videoHeight };
  }
  if (img instanceof ImageBitmap) {
    return { w: img.width, h: img.height };
  }
  const c = img as HTMLCanvasElement;
  return { w: c.width, h: c.height };
}

/** Vídeo con letterbox dentro de un panel (mitad de cortina, lado a lado, etc.). */
function dibujarLetterboxEnRegion(
  ctx: CanvasRenderingContext2D,
  img: CanvasImageSource,
  region: RectVideo,
) {
  const { w: iw, h: ih } = dimensionesFuente(img);
  const imgAspect = iw / Math.max(1, ih);
  const regAspect = region.w / Math.max(1, region.h);
  let dw: number;
  let dh: number;
  let dx: number;
  let dy: number;
  if (regAspect > imgAspect) {
    dh = region.h;
    dw = dh * imgAspect;
    dx = region.x + (region.w - dw) / 2;
    dy = region.y;
  } else {
    dw = region.w;
    dh = dw / imgAspect;
    dx = region.x;
    dy = region.y + (region.h - dh) / 2;
  }
  ctx.drawImage(img, dx, dy, dw, dh);
}

function dibujarImagenEnRect(
  ctx: CanvasRenderingContext2D,
  img: CanvasImageSource,
  rect: RectVideo,
) {
  dibujarLetterboxEnRegion(ctx, img, rect);
}

function dibujarLineaCortina(
  ctx: CanvasRenderingContext2D,
  rect: RectVideo,
  splitPos: number,
  horizontal: boolean,
) {
  ctx.strokeStyle = AMARILLO;
  ctx.lineWidth = GROSOR_LINEA;
  ctx.beginPath();
  if (horizontal) {
    const y = rect.y + rect.h * splitPos;
    ctx.moveTo(rect.x, y);
    ctx.lineTo(rect.x + rect.w, y);
  } else {
    const x = rect.x + rect.w * splitPos;
    ctx.moveTo(x, rect.y);
    ctx.lineTo(x, rect.y + rect.h);
  }
  ctx.stroke();
}

/** Offscreen con letterbox negro (misma geometría que un panel SideBySide). */
function canvasConLetterbox(
  img: CanvasImageSource,
  ancho: number,
  alto: number,
): HTMLCanvasElement {
  const c = document.createElement("canvas");
  c.width = ancho;
  c.height = alto;
  const cx = c.getContext("2d");
  if (!cx) return c;
  cx.fillStyle = "#000";
  cx.fillRect(0, 0, ancho, alto);
  dibujarLetterboxEnRegion(cx, img, { x: 0, y: 0, w: ancho, h: alto });
  return c;
}

/** Diff pixel a pixel tras alinear A y B con letterbox (evita estirar el panel derecho). */
function calcularDiffOffscreen(
  imgA: CanvasImageSource,
  imgB: CanvasImageSource,
  tw: number,
  th: number,
  diffMode: DiffMode,
  amp: number,
): HTMLCanvasElement {
  const ca = canvasConLetterbox(imgA, tw, th);
  const cb = canvasConLetterbox(imgB, tw, th);
  const off = document.createElement("canvas");
  off.width = tw;
  off.height = th;
  const octx = off.getContext("2d");
  if (!octx) return off;

  const ctxA = ca.getContext("2d");
  const ctxB = cb.getContext("2d");
  if (!ctxA || !ctxB) return off;

  const dataA = ctxA.getImageData(0, 0, tw, th);
  const dataB = ctxB.getImageData(0, 0, tw, th);
  const out = octx.createImageData(tw, th);
  for (let i = 0; i < dataA.data.length; i += 4) {
    const caRgb: [number, number, number] = [
      dataA.data[i]! / 255,
      dataA.data[i + 1]! / 255,
      dataA.data[i + 2]! / 255,
    ];
    const cbRgb: [number, number, number] = [
      dataB.data[i]! / 255,
      dataB.data[i + 1]! / 255,
      dataB.data[i + 2]! / 255,
    ];
    const [r, g, b] = diferenciaRgb(caRgb, cbRgb, diffMode, amp);
    out.data[i] = r;
    out.data[i + 1] = g;
    out.data[i + 2] = b;
    out.data[i + 3] = 255;
  }
  octx.putImageData(out, 0, 0);
  return off;
}

function pintarDiffPixels(
  ctx: CanvasRenderingContext2D,
  rect: RectVideo,
  imgA: CanvasImageSource,
  imgB: CanvasImageSource,
  vista: VistaCompare,
  modo: "AbsDiff" | "Heatmap",
) {
  const tw = Math.min(640, Math.max(32, Math.round(rect.w)));
  const th = Math.min(360, Math.max(32, Math.round(rect.h)));
  const off = document.createElement("canvas");
  off.width = tw;
  off.height = th;
  const octx = off.getContext("2d");
  if (!octx) return;

  octx.drawImage(imgA, 0, 0, tw, th);
  const dataA = octx.getImageData(0, 0, tw, th);
  octx.clearRect(0, 0, tw, th);
  octx.drawImage(imgB, 0, 0, tw, th);
  const dataB = octx.getImageData(0, 0, tw, th);
  const out = octx.createImageData(tw, th);
  const amp = vista.amplifier;
  const diffMode = vista.diff_mode;
  const horizontal = vista.split_horizontal;
  const sp = vista.split_pos;
  const lineHalf = 2 / tw;

  for (let y = 0; y < th; y++) {
    for (let x = 0; x < tw; x++) {
      const u = x / tw;
      const v = y / th;
      const onLeft = horizontal ? v < sp : u < sp;
      const inLine = horizontal
        ? Math.abs(v - sp) < lineHalf
        : Math.abs(u - sp) < lineHalf;

      const i = (y * tw + x) * 4;
      let r: number;
      let g: number;
      let b: number;

      if (inLine) {
        r = 255;
        g = 255;
        b = 0;
      } else if (onLeft) {
        r = dataA.data[i]!;
        g = dataA.data[i + 1]!;
        b = dataA.data[i + 2]!;
      } else if (modo === "Heatmap") {
        const da = dataA.data[i]! / 255;
        const db = dataB.data[i]! / 255;
        const dg = dataA.data[i + 1]! / 255;
        const eg = dataB.data[i + 1]! / 255;
        const dr = dataA.data[i + 2]! / 255;
        const er = dataB.data[i + 2]! / 255;
        const intensity =
          (Math.abs(da - db) * 0.2126 + Math.abs(dg - eg) * 0.7152 + Math.abs(dr - er) * 0.0722) *
          amp;
        [r, g, b] = colorHeatmap(intensity);
      } else {
        const ca: [number, number, number] = [
          dataA.data[i]! / 255,
          dataA.data[i + 1]! / 255,
          dataA.data[i + 2]! / 255,
        ];
        const cb: [number, number, number] = [
          dataB.data[i]! / 255,
          dataB.data[i + 1]! / 255,
          dataB.data[i + 2]! / 255,
        ];
        [r, g, b] = diferenciaRgb(ca, cb, diffMode, amp);
      }

      out.data[i] = r;
      out.data[i + 1] = g;
      out.data[i + 2] = b;
      out.data[i + 3] = 255;
    }
  }

  octx.putImageData(out, 0, 0);
  ctx.drawImage(off, rect.x, rect.y, rect.w, rect.h);
}

/**
 * Pinta la comparación A/B en el canvas (equivalente a compare.wgsl en DOM).
 */
export function pintarComparacion(
  ctx: CanvasRenderingContext2D,
  cw: number,
  ch: number,
  imgA: CanvasImageSource | null,
  imgB: CanvasImageSource | null,
  vista: VistaCompare,
  vidW: number,
  vidH: number,
): RectVideo {
  ctx.fillStyle = "#000";
  ctx.fillRect(0, 0, cw, ch);

  const rect = calcularRectVideo(cw, ch, vidW, vidH);
  const imgFallback = imgA ?? imgB;
  if (!imgFallback) {
    return rect;
  }

  const a = imgA ?? imgFallback;
  const b = imgB ?? imgFallback;
  const modo: CompareMode = vista.modo;
  const sp = vista.split_pos;
  const horizontal = vista.split_horizontal;

  if (modo === "SideBySide") {
    const mitad = cw / 2;
    const rectA: RectVideo = { x: 0, y: 0, w: mitad, h: ch };
    const rectB: RectVideo = { x: mitad, y: 0, w: mitad, h: ch };
    dibujarLetterboxEnRegion(ctx, a, rectA);
    if (vista.diff_mode === "None" || !imgB) {
      dibujarLetterboxEnRegion(ctx, b, rectB);
    } else {
      const regAspect = rectB.w / Math.max(1, rectB.h);
      const tw = 320;
      const th = Math.max(32, Math.round(tw / regAspect));
      const off = calcularDiffOffscreen(a, b, tw, th, vista.diff_mode, vista.amplifier);
      dibujarCanvasEnRegion(ctx, off, rectB);
    }
    ctx.strokeStyle = AMARILLO;
    ctx.lineWidth = GROSOR_LINEA;
    ctx.beginPath();
    ctx.moveTo(mitad, 0);
    ctx.lineTo(mitad, ch);
    ctx.stroke();
    return { x: 0, y: 0, w: cw, h: ch };
  }

  if (modo === "AbsDiff" || modo === "Heatmap") {
    if (imgA && imgB) {
      pintarDiffPixels(ctx, rect, a, b, vista, modo);
    } else {
      dibujarImagenEnRect(ctx, imgFallback, rect);
    }
    return rect;
  }

  // SplitScreen (cortina): un solo encuadre; clip izquierda=A, derecha=B
  if (horizontal) {
    const yCorte = rect.y + rect.h * sp;
    ctx.save();
    ctx.beginPath();
    ctx.rect(rect.x, rect.y, rect.w, rect.h * sp);
    ctx.clip();
    dibujarLetterboxEnRegion(ctx, a, rect);
    ctx.restore();
    ctx.save();
    ctx.beginPath();
    ctx.rect(rect.x, yCorte, rect.w, rect.h * (1 - sp));
    ctx.clip();
    dibujarLetterboxEnRegion(ctx, b, rect);
    ctx.restore();
  } else {
    const xCorte = rect.x + rect.w * sp;
    ctx.save();
    ctx.beginPath();
    ctx.rect(rect.x, rect.y, rect.w * sp, rect.h);
    ctx.clip();
    dibujarLetterboxEnRegion(ctx, a, rect);
    ctx.restore();
    ctx.save();
    ctx.beginPath();
    ctx.rect(xCorte, rect.y, rect.w * (1 - sp), rect.h);
    ctx.clip();
    dibujarLetterboxEnRegion(ctx, b, rect);
    ctx.restore();
  }

  if (imgA && imgB && sp > 0.02 && sp < 0.98) {
    dibujarLineaCortina(ctx, rect, sp, horizontal);
  }

  return rect;
}

/** Convierte coordenadas de puntero a split_pos [0,1] en el rect del vídeo. */
export function splitDesdePuntero(
  clientX: number,
  clientY: number,
  canvas: HTMLCanvasElement,
  rect: RectVideo,
  horizontal: boolean,
): number {
  const br = canvas.getBoundingClientRect();
  const sx = ((clientX - br.left) / br.width) * canvas.width;
  const sy = ((clientY - br.top) / br.height) * canvas.height;
  if (horizontal) {
    return Math.min(1, Math.max(0, (sy - rect.y) / rect.h));
  }
  return Math.min(1, Math.max(0, (sx - rect.x) / rect.w));
}
