<script lang="ts">
  interface Props {
    monitorLuma: number[];
  }

  let { monitorLuma }: Props = $props();

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

    ctx.strokeStyle = "rgba(255,255,255,0.08)";
    ctx.beginPath();
    ctx.moveTo(0, h / 2);
    ctx.lineTo(w, h / 2);
    ctx.stroke();

    const n = monitorLuma.length || 256;
    ctx.strokeStyle = "#4ade80";
    ctx.lineWidth = 1;
    ctx.beginPath();
    for (let i = 0; i < n; i++) {
      const x = (i / (n - 1)) * w;
      const y = h - (monitorLuma[i] ?? 0) * (h - 4) - 2;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();
  }

  $effect(() => {
    monitorLuma;
    pintar();
  });
</script>

<canvas bind:this={canvas} class="scope-canvas" aria-label="Monitor de luminancia"></canvas>

<style>
  .scope-canvas {
    width: 100%;
    height: 56px;
    display: block;
    border-radius: var(--radius-sm);
  }
</style>
