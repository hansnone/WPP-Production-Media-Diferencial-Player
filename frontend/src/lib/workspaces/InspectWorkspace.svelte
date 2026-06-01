<script lang="ts">
  import { onMount } from "svelte";
  import CanvasViewportSync from "../components/viewport/CanvasViewportSync.svelte";
  import { layoutStore } from "../stores/layout.svelte";
  import { playerStore } from "../stores/player.svelte";
  import { compareViewStore } from "../stores/compareView.svelte";
  import { scopesStore } from "../stores/scopes.svelte";
  import { idiomaStore } from "../i18n/idioma.svelte";
  import { escucharVistaFrames, type VistaFrameEvent } from "../player";
  import { pintarComparacion } from "../compareCompositor";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  const activo = $derived(layoutStore.workspaceActivo === "inspect");

  const usarFallbackCanvas = $derived(
    !compareViewStore.overlayGpuActiva || !compareViewStore.gpuListo,
  );

  const hayArchivo = $derived(
    Boolean(playerStore.snap?.ruta_a || playerStore.snap?.ruta_b),
  );

  let canvas: HTMLCanvasElement | undefined = $state();
  let bitmapA: ImageBitmap | null = $state(null);
  let bitmapB: ImageBitmap | null = $state(null);
  let dimsVideo = $state({ w: 16, h: 9 });
  let seqPendienteA = 0;
  let seqPendienteB = 0;

  async function decodificarJpeg(b64: string): Promise<ImageBitmap> {
    const bin = atob(b64);
    const bytes = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i)!;
    const blob = new Blob([bytes], { type: "image/jpeg" });
    return createImageBitmap(blob);
  }

  function sustituirBitmap(anterior: ImageBitmap | null, nuevo: ImageBitmap): ImageBitmap {
    anterior?.close();
    return nuevo;
  }

  function repintar() {
    if (!usarFallbackCanvas || !canvas) return;
    if (!bitmapA && !bitmapB) return;
    const cw = canvas.clientWidth;
    const ch = canvas.clientHeight;
    if (cw < 2 || ch < 2) return;
    const dpr = window.devicePixelRatio || 1;
    if (canvas.width !== Math.round(cw * dpr) || canvas.height !== Math.round(ch * dpr)) {
      canvas.width = Math.round(cw * dpr);
      canvas.height = Math.round(ch * dpr);
    }
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    pintarComparacion(
      ctx,
      cw,
      ch,
      bitmapA,
      bitmapB,
      compareViewStore.vista,
      dimsVideo.w,
      dimsVideo.h,
    );
  }

  async function aplicarVistaFrame(frame: VistaFrameEvent) {
    if (!usarFallbackCanvas) return;
    try {
      const bmp = await decodificarJpeg(frame.b64);
      if (frame.canal === "a") {
        if (frame.seq < seqPendienteA) {
          bmp.close();
          return;
        }
        seqPendienteA = frame.seq;
        bitmapA = sustituirBitmap(bitmapA, bmp);
      } else if (frame.canal === "b") {
        if (frame.seq < seqPendienteB) {
          bmp.close();
          return;
        }
        seqPendienteB = frame.seq;
        bitmapB = sustituirBitmap(bitmapB, bmp);
      }
      dimsVideo = {
        w: Math.max(dimsVideo.w, frame.ancho || bmp.width),
        h: Math.max(dimsVideo.h, frame.alto || bmp.height),
      };
      requestAnimationFrame(repintar);
    } catch (e) {
      console.warn("inspect vista-frame:", e);
    }
  }

  $effect(() => {
    if (activo && "__TAURI_INTERNALS__" in window) {
      void scopesStore.refrescar();
      void compareViewStore.aplicar();
      window.dispatchEvent(new CustomEvent("diffplayerqc-sync-viewport"));
    }
  });

  $effect(() => {
    if (!usarFallbackCanvas || !activo) return;
    void compareViewStore.vista.modo;
    void compareViewStore.vista.split_pos;
    if (!bitmapA && !bitmapB) return;
    requestAnimationFrame(repintar);
  });

  onMount(() => {
    let desuscribirFrames: UnlistenFn | undefined;
    if ("__TAURI_INTERNALS__" in window) {
      void escucharVistaFrames((frame) => {
        void aplicarVistaFrame(frame);
      }).then((fn) => {
        desuscribirFrames = fn;
      });
    }
    return () => {
      desuscribirFrames?.();
      bitmapA?.close();
      bitmapB?.close();
    };
  });
</script>

<CanvasViewportSync activo={activo && compareViewStore.overlayGpuActiva}>
  <div id="canvas-slot" class="canvas-slot" data-testid="workspace-inspect">
    {#if hayArchivo}
      <canvas
        class="vista-canvas"
        class:oculto-por-gpu={!usarFallbackCanvas}
        bind:this={canvas}
        aria-label="Vista Inspect"
      ></canvas>
      {#if compareViewStore.overlayGpuActiva && compareViewStore.gpuListo}
        <span class="badge-render" title="Render wgpu">GPU</span>
      {/if}
    {:else}
      <p class="estado">{idiomaStore.t("fuentes.sinArchivo")} — A / B</p>
    {/if}
  </div>
</CanvasViewportSync>

<style>
  .canvas-slot {
    position: relative;
    flex: 1;
    min-width: 0;
    min-height: 0;
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    border: 1px solid var(--border);
    background: #000;
    border-radius: var(--radius-max);
    overflow: hidden;
  }
  .vista-canvas {
    display: block;
    width: 100%;
    height: 100%;
  }
  .vista-canvas.oculto-por-gpu {
    opacity: 0;
    pointer-events: none;
  }
  .badge-render {
    position: absolute;
    top: 8px;
    right: 8px;
    z-index: 2;
    font-size: 9px;
    padding: 2px 6px;
    background: rgba(0, 0, 0, 0.6);
    border: 1px solid var(--accent-primary);
    color: var(--accent-primary);
    border-radius: var(--radius);
    letter-spacing: 0.05em;
    pointer-events: none;
  }
  .estado {
    margin: auto;
    padding: 24px;
    color: var(--text-muted);
    font-size: 13px;
    text-align: center;
  }
</style>
