<script lang="ts">
  /**
   * Canvas de forma de onda con playhead y click-to-seek.
   * Dibuja picos simétricos alrededor del eje central (estilo DAW).
   */
  import { playerStore } from "../stores/player.svelte";

  interface Props {
    picos: number[];
    duracionSecs: number;
    color: string;
    etiqueta: string;
    testId?: string;
    /** Si true, no emite seek al hacer click (p. ej. franja diff). */
    soloLectura?: boolean;
    /** LUFS momentáneo por bucket (-70..0) para franja inferior (M9). */
    lufsBuckets?: number[];
  }

  let {
    picos,
    duracionSecs,
    color,
    etiqueta,
    testId = "waveform-canvas",
    soloLectura = false,
    lufsBuckets = [],
  }: Props = $props();

  let canvas: HTMLCanvasElement | undefined = $state();

  const ptsActual = $derived(playerStore.snap?.pts_actual ?? 0);
  const duracion = $derived(
    duracionSecs > 0
      ? duracionSecs
      : Math.max(playerStore.snap?.duracion_a ?? 0, playerStore.snap?.duracion_b ?? 0),
  );

  /** El canvas no entiende `var(--token)`; resolvemos contra `:root`. */
  function resolverColor(css: string): string {
    const m = css.match(/^var\((--[^,)]+)\)$/);
    if (!m) return css;
    const val = getComputedStyle(document.documentElement).getPropertyValue(m[1]).trim();
    return val || "#888888";
  }

  function pintar() {
    const el = canvas;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    const w = Math.max(1, Math.floor(rect.width));
    const h = Math.max(1, Math.floor(rect.height));
    if (w !== el.width || h !== el.height) {
      el.width = w;
      el.height = h;
    }

    const ctx = el.getContext("2d");
    if (!ctx) return;

    ctx.clearRect(0, 0, w, h);

    const midY = h / 2;
    ctx.strokeStyle = "rgba(255,255,255,0.08)";
    ctx.beginPath();
    ctx.moveTo(0, midY);
    ctx.lineTo(w, midY);
    ctx.stroke();

    if (!picos.length) {
      ctx.fillStyle = "var(--text-muted)";
      ctx.font = "11px system-ui, sans-serif";
      ctx.textAlign = "center";
      ctx.fillText("Sin audio", w / 2, midY + 4);
      return;
    }

    const n = picos.length;
    const colorResuelto = resolverColor(color);
    ctx.fillStyle = colorResuelto;
    for (let x = 0; x < w; x += 1) {
      const idx = Math.min(n - 1, Math.floor((x / w) * n));
      const amp = (picos[idx] ?? 0) * (h * 0.45);
      if (amp < 0.5) continue;
      ctx.fillRect(x, midY - amp, 1, amp * 2);
    }

    // Franja inferior: loudness momentáneo por bucket (EBU, -70..0 LUFS → altura)
    if (lufsBuckets.length) {
      const bandH = Math.max(8, Math.floor(h * 0.18));
      const y0 = h - bandH;
      const nLufs = lufsBuckets.length;
      ctx.fillStyle = "rgba(0,0,0,0.35)";
      ctx.fillRect(0, y0, w, bandH);
      for (let x = 0; x < w; x += 1) {
        const idx = Math.min(nLufs - 1, Math.floor((x / w) * nLufs));
        const lufs = lufsBuckets[idx] ?? -70;
        const norm = Math.max(0, Math.min(1, (lufs + 70) / 70));
        const barH = norm * (bandH - 2);
        ctx.fillStyle = `rgba(120, 180, 255, ${0.25 + norm * 0.55})`;
        ctx.fillRect(x, y0 + bandH - barH, 1, barH);
      }
      // Referencia -23 LUFS (broadcast)
      const refNorm = (-23 + 70) / 70;
      const refY = y0 + bandH - refNorm * (bandH - 2);
      ctx.strokeStyle = "rgba(255, 200, 80, 0.55)";
      ctx.setLineDash([3, 3]);
      ctx.beginPath();
      ctx.moveTo(0, refY);
      ctx.lineTo(w, refY);
      ctx.stroke();
      ctx.setLineDash([]);
    }

    if (duracion > 0) {
      const px = (ptsActual / duracion) * w;
      ctx.strokeStyle = resolverColor("var(--accent)");
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(px, 0);
      ctx.lineTo(px, h);
      ctx.stroke();
    }
  }

  $effect(() => {
    picos;
    lufsBuckets;
    ptsActual;
    duracion;
    pintar();
  });

  function onClick(ev: MouseEvent) {
    if (soloLectura || duracion <= 0) return;
    const el = canvas;
    if (!el) return;
    const rect = el.getBoundingClientRect();
    const x = ev.clientX - rect.left;
    const frac = Math.max(0, Math.min(1, x / rect.width));
    void playerStore.seekPts(frac * duracion);
  }

  function onResize() {
    pintar();
  }
</script>

<svelte:window onresize={onResize} />

<div class="waveform" data-testid={testId}>
  <span class="waveform__label">{etiqueta}</span>
  <canvas
    bind:this={canvas}
    class="waveform__canvas"
    aria-label={etiqueta}
    onclick={onClick}
  ></canvas>
</div>

<style>
  .waveform {
    flex: 1;
    min-height: 80px;
    display: flex;
    flex-direction: column;
    padding: 8px;
    box-sizing: border-box;
  }
  .waveform__label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
    margin-bottom: 4px;
  }
  .waveform__canvas {
    flex: 1;
    width: 100%;
    min-height: 60px;
    border-radius: var(--radius-sm);
    cursor: crosshair;
  }
</style>
