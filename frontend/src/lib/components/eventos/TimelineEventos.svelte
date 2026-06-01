<script lang="ts">
  import { eventosQcStore } from "../../stores/eventosQc.svelte";
  import { playerStore } from "../../stores/player.svelte";

  interface Props {
    duracion: number;
    ptsActual: number;
    onSeekInput: (pts: number) => void;
    onSeek: () => void;
  }

  let { duracion, ptsActual, onSeekInput, onSeek }: Props = $props();

  const eventos = $derived(eventosQcStore.eventos);
  const visible = $derived(duracion > 0 && eventos.length > 0);

  let canvas: HTMLCanvasElement | undefined = $state();

  function repintar() {
    if (!canvas || !visible) return;
    const cw = canvas.clientWidth;
    const ch = canvas.clientHeight;
    if (cw < 4) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(cw * dpr);
    canvas.height = Math.round(ch * dpr);
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, cw, ch);

    const colorTipo = (tipo: string) => {
      if (tipo === "video") return "#4ade80";
      if (tipo === "audio") return "#60a5fa";
      return "#fbbf24";
    };

    for (const ev of eventos) {
      const x = (ev.pts_secs / duracion) * cw;
      const activo = eventosQcStore.eventoSeleccionadoId === ev.id;
      ctx.fillStyle = colorTipo(ev.tipo);
      ctx.beginPath();
      ctx.moveTo(x, ch);
      ctx.lineTo(x - 4, 0);
      ctx.lineTo(x + 4, 0);
      ctx.closePath();
      ctx.globalAlpha = activo ? 1 : 0.75;
      ctx.fill();
      ctx.globalAlpha = 1;
    }

    const px = (ptsActual / duracion) * cw;
    ctx.strokeStyle = "#ffff00";
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(px, 0);
    ctx.lineTo(px, ch);
    ctx.stroke();
  }

  function alClick(ev: MouseEvent) {
    if (!canvas || !visible) return;
    const r = canvas.getBoundingClientRect();
    const frac = Math.min(1, Math.max(0, (ev.clientX - r.left) / r.width));
    const pts = frac * duracion;

    const cercano = eventos.reduce<{ ev: (typeof eventos)[0]; d: number } | null>(
      (best, e) => {
        const d = Math.abs(e.pts_secs - pts);
        if (d * duracion < 12 && (!best || d < best.d)) {
          return { ev: e, d };
        }
        return best;
      },
      null,
    );

    if (cercano) {
      void playerStore.seekAEventoQc(cercano.ev.id);
    } else {
      onSeekInput(pts);
      onSeek();
    }
  }

  $effect(() => {
    void eventos;
    void ptsActual;
    void duracion;
    void eventosQcStore.eventoSeleccionadoId;
    requestAnimationFrame(repintar);
  });
</script>

{#if visible}
  <div class="timeline-eventos" data-testid="timeline-eventos">
    <canvas
      class="timeline-eventos__canvas"
      bind:this={canvas}
      aria-label="Marcadores de eventos QC"
      onclick={alClick}
    ></canvas>
  </div>
{/if}

<style>
  .timeline-eventos {
    width: 100%;
    padding: 0 0 2px;
  }
  .timeline-eventos__canvas {
    display: block;
    width: 100%;
    height: 8px;
    cursor: pointer;
  }
</style>
