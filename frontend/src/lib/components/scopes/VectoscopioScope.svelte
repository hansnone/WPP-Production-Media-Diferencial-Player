<script lang="ts">
  const TAM = 128;

  interface Props {
    vectoscopio: number[];
  }

  let { vectoscopio }: Props = $props();

  let canvas: HTMLCanvasElement | undefined = $state();

  function pintar() {
    const el = canvas;
    if (!el) return;
    const size = Math.min(el.clientWidth, el.clientHeight, 128);
    const s = Math.max(64, Math.floor(size));
    if (el.width !== s || el.height !== s) {
      el.width = s;
      el.height = s;
    }
    const ctx = el.getContext("2d");
    if (!ctx) return;

    ctx.fillStyle = "#0a0c0e";
    ctx.fillRect(0, 0, s, s);

    const max = vectoscopio.reduce((m, v) => Math.max(m, v), 0);
    if (max <= 0) return;

    const img = ctx.createImageData(s, s);
    for (let y = 0; y < s; y++) {
      for (let x = 0; x < s; x++) {
        const sx = Math.floor((x / s) * TAM);
        const sy = Math.floor((y / s) * TAM);
        const v = (vectoscopio[sy * TAM + sx] ?? 0) / max;
        const i = (y * s + x) * 4;
        img.data[i] = Math.min(255, v * 180 + 40);
        img.data[i + 1] = Math.min(255, v * 220);
        img.data[i + 2] = Math.min(255, v * 80);
        img.data[i + 3] = 255;
      }
    }
    ctx.putImageData(img, 0, 0);

    ctx.strokeStyle = "rgba(255,255,255,0.15)";
    ctx.beginPath();
    ctx.moveTo(s / 2, 0);
    ctx.lineTo(s / 2, s);
    ctx.moveTo(0, s / 2);
    ctx.lineTo(s, s / 2);
    ctx.stroke();
  }

  $effect(() => {
    vectoscopio;
    pintar();
  });
</script>

<canvas bind:this={canvas} class="scope-canvas scope-canvas--cuad" aria-label="Vectoscopio"></canvas>

<style>
  .scope-canvas--cuad {
    width: 100%;
    max-width: 128px;
    aspect-ratio: 1;
    display: block;
    margin: 0 auto;
    border-radius: var(--radius-sm);
  }
</style>
