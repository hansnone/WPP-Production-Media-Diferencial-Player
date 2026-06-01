<script lang="ts">
  import { metricasStore } from "../../stores/metricas.svelte";
  import { playerStore } from "../../stores/player.svelte";
  interface Props {
    duracion: number;
    ptsActual: number;
    onSeekInput: (pts: number) => void;
    onSeek: () => void;
  }

  let { duracion, ptsActual, onSeekInput, onSeek }: Props = $props();

  const serie = $derived(metricasStore.serie);
  const visible = $derived(Boolean(serie && serie.puntos.length > 0 && duracion > 0));

  let canvas: HTMLCanvasElement | undefined = $state();

  function colorSsim(ssim: number, umbral: number): string {
    if (ssim < umbral) return "#f87171";
    const t = (ssim - umbral) / (1 - umbral).max(0.01);
    const g = Math.round(80 + t * 120);
    return `rgb(40,${g},60)`;
  }

  function repintarHeatmap() {
    if (!canvas || !visible || !serie) return;
    const cw = canvas.clientWidth;
    const ch = canvas.clientHeight;
    if (cw < 4) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(cw * dpr);
    canvas.height = Math.round(ch * dpr);
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    const puntos = serie.puntos;
    const maxT = duracion || serie.duracion_secs || 1;
    const bin = Math.max(1, Math.ceil(cw / 2));
    const anchos = new Array<number>(bin).fill(0);
    const sumas = new Array<number>(bin).fill(0);

    for (const p of puntos) {
      const i = Math.min(bin - 1, Math.floor((p.pts / maxT) * bin));
      sumas[i] = (sumas[i] ?? 0) + p.ssim;
      anchos[i] = (anchos[i] ?? 0) + 1;
    }

    const bw = cw / bin;
    for (let i = 0; i < bin; i++) {
      const n = anchos[i] ?? 0;
      if (n === 0) continue;
      const ssim = (sumas[i] ?? 0) / n;
      ctx.fillStyle = colorSsim(ssim, serie.umbral_ssim_bajo);
      ctx.fillRect(i * bw, 0, bw + 0.5, ch);
    }

    // Playhead
    const px = (ptsActual / maxT) * cw;
    ctx.strokeStyle = "#ffff00";
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(px, 0);
    ctx.lineTo(px, ch);
    ctx.stroke();
  }

  function ptsDesdeClick(ev: MouseEvent) {
    if (!canvas || !visible || !serie) return;
    const r = canvas.getBoundingClientRect();
    const frac = Math.min(1, Math.max(0, (ev.clientX - r.left) / r.width));
    return frac * (duracion || serie.duracion_secs);
  }

  function alClick(ev: MouseEvent) {
    const pts = ptsDesdeClick(ev);
    if (pts === undefined) return;
    onSeekInput(pts);
    onSeek();
  }

  $effect(() => {
    void serie;
    void ptsActual;
    void duracion;
    requestAnimationFrame(repintarHeatmap);
  });
</script>

{#if visible && serie}
  <div class="timeline-metricas" data-testid="timeline-metricas">
    <canvas
      class="timeline-metricas__heatmap"
      data-testid="timeline-metricas-heatmap"
      bind:this={canvas}
      aria-label="Mapa SSIM"
      onclick={alClick}
    ></canvas>
  </div>
{/if}

<style>
  .timeline-metricas {
    width: 100%;
    padding: 0 0 2px;
  }
  .timeline-metricas__heatmap {
    display: block;
    width: 100%;
    height: 10px;
    cursor: pointer;
    border-radius: 1px;
  }
</style>
