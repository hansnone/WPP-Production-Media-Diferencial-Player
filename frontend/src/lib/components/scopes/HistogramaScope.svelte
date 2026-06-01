<script lang="ts">
  import { normalizarHistograma } from "../../scopes";

  interface Props {
    histogramaR: number[];
    histogramaG: number[];
    histogramaB: number[];
  }

  let { histogramaR, histogramaG, histogramaB }: Props = $props();

  let canvas: HTMLCanvasElement | undefined = $state();

  function pintar() {
    const el = canvas;
    if (!el) return;
    const w = Math.max(1, el.clientWidth);
    const h = Math.max(1, el.clientHeight);
    if (el.width !== w || el.height !== h) {
      el.width = w;
      el.height = h;
    }
    const ctx = el.getContext("2d");
    if (!ctx) return;

    ctx.fillStyle = "#0a0c0e";
    ctx.fillRect(0, 0, w, h);

    const nr = normalizarHistograma(histogramaR);
    const ng = normalizarHistograma(histogramaG);
    const nb = normalizarHistograma(histogramaB);

    const dibujarCurva = (datos: number[], color: string) => {
      ctx.strokeStyle = color;
      ctx.lineWidth = 1;
      ctx.beginPath();
      for (let i = 0; i < 256; i++) {
        const x = (i / 255) * w;
        const y = h - (datos[i] ?? 0) * (h - 4) - 2;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.stroke();
    };

    dibujarCurva(nr, "#4ade80");
    dibujarCurva(ng, "#60a5fa");
    dibujarCurva(nb, "#f87171");
  }

  $effect(() => {
    histogramaR;
    histogramaG;
    histogramaB;
    pintar();
  });
</script>

<canvas bind:this={canvas} class="scope-canvas" aria-label="Histograma RGB"></canvas>

<style>
  .scope-canvas {
    width: 100%;
    height: 72px;
    display: block;
    border-radius: var(--radius-sm);
  }
</style>
