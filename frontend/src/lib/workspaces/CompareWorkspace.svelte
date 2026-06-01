<script lang="ts">
  import { onMount } from "svelte";
  import CanvasViewportSync from "../components/viewport/CanvasViewportSync.svelte";
  import { playerStore } from "../stores/player.svelte";
  import { compareViewStore } from "../stores/compareView.svelte";
  import { escucharVistaFrames, type VistaFrameEvent } from "../player";
  import {
    calcularRectVideo,
    pintarComparacion,
    splitDesdePuntero,
    type RectVideo,
  } from "../compareCompositor";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  /** Canvas DOM cuando la overlay GPU no está operativa. */
  const usarFallbackCanvas = $derived(
    !compareViewStore.overlayGpuActiva || !compareViewStore.gpuListo,
  );

  let canvas: HTMLCanvasElement | undefined = $state();
  let rectVideo: RectVideo = $state({ x: 0, y: 0, w: 0, h: 0 });
  let arrastrandoSplit = $state(false);
  let tamanoCanvas = $state({ cw: 0, ch: 0, dpr: 1 });
  let rafRepintar = 0;
  let rafBuclePlay = 0;
  /** Bitmaps decodificados (createImageBitmap, más rápido que HTMLImageElement). */
  let bitmapA: ImageBitmap | null = $state(null);
  let bitmapB: ImageBitmap | null = $state(null);
  let ultimoSeqVista = 0;
  let dimsVideo = $state({ w: 16, h: 9 });
  /** Descarta frames JPEG obsoletos si llegan fuera de orden. */
  let seqPendienteA = 0;
  let seqPendienteB = 0;

  const hayArchivo = $derived(
    Boolean(playerStore.snap?.ruta_a || playerStore.snap?.ruta_b),
  );

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
        dimsVideo = {
          w: frame.ancho || bmp.width,
          h: frame.alto || bmp.height,
        };
      } else if (frame.canal === "b") {
        if (frame.seq < seqPendienteB) {
          bmp.close();
          return;
        }
        seqPendienteB = frame.seq;
        bitmapB = sustituirBitmap(bitmapB, bmp);
        dimsVideo = {
          w: Math.max(dimsVideo.w, frame.ancho || bmp.width),
          h: Math.max(dimsVideo.h, frame.alto || bmp.height),
        };
      }
      ultimoSeqVista = Math.max(ultimoSeqVista, frame.seq);
      programarRepintar();
    } catch (e) {
      console.warn("vista-frame decode:", e);
    }
  }

  function repintar() {
    if (!usarFallbackCanvas || !canvas) return;
    if (!bitmapA && !bitmapB) return;

    const cw = canvas.clientWidth;
    const ch = canvas.clientHeight;
    if (cw < 2 || ch < 2) return;

    const dpr = window.devicePixelRatio || 1;
    if (
      tamanoCanvas.cw !== cw ||
      tamanoCanvas.ch !== ch ||
      tamanoCanvas.dpr !== dpr
    ) {
      canvas.width = Math.round(cw * dpr);
      canvas.height = Math.round(ch * dpr);
      tamanoCanvas = { cw, ch, dpr };
    }

    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    rectVideo = pintarComparacion(
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

  function programarRepintar() {
    cancelAnimationFrame(rafRepintar);
    rafRepintar = requestAnimationFrame(() => {
      repintar();
    });
  }

  /** Durante play: repintar a ritmo de pantalla aunque el frame no cambie aún. */
  $effect(() => {
    if (!usarFallbackCanvas) return;
    const reproduciendo = playerStore.snap?.reproduciendo ?? false;
    cancelAnimationFrame(rafBuclePlay);
    if (!reproduciendo) return;
    const bucle = () => {
      if (!playerStore.snap?.reproduciendo) return;
      repintar();
      rafBuclePlay = requestAnimationFrame(bucle);
    };
    rafBuclePlay = requestAnimationFrame(bucle);
    return () => cancelAnimationFrame(rafBuclePlay);
  });

  /** Sincroniza dims cuando el tick indica frame nuevo (GPU off). */
  $effect(() => {
    if (!usarFallbackCanvas) return;
    const snap = playerStore.snap;
    if (!snap) return;
    const seq = snap.vista_seq ?? 0;
    if (seq === ultimoSeqVista) return;
    if (snap.vista_ancho && snap.vista_alto) {
      dimsVideo = { w: snap.vista_ancho, h: snap.vista_alto };
    }
  });

  /** Cambios de modo/cortina/diff: repintar con imágenes ya en caché. */
  $effect(() => {
    if (!usarFallbackCanvas) return;
    void compareViewStore.vista.modo;
    void compareViewStore.vista.split_pos;
    void compareViewStore.vista.diff_mode;
    void compareViewStore.vista.amplifier;
    void compareViewStore.vista.split_horizontal;
    if (!bitmapA && !bitmapB) return;
    programarRepintar();
  });

  const rectParaUi = $derived.by((): RectVideo => {
    if (!canvas) return rectVideo;
    const snap = playerStore.snap;
    const cw = canvas.clientWidth;
    const ch = canvas.clientHeight;
    const vidW = snap?.vista_ancho ?? 16;
    const vidH = snap?.vista_alto ?? 9;
    if (compareViewStore.gpuListo && cw > 1 && ch > 1) {
      return calcularRectVideo(cw, ch, vidW, vidH);
    }
    return rectVideo;
  });

  onMount(() => {
    let desuscribirGpu: UnlistenFn | undefined;
    let desuscribirFrames: UnlistenFn | undefined;
    if ("__TAURI_INTERNALS__" in window) {
      void compareViewStore.iniciarEscuchaGpu().then((fn) => {
        desuscribirGpu = fn;
      });
      void escucharVistaFrames((frame) => {
        void aplicarVistaFrame(frame);
      }).then((fn) => {
        desuscribirFrames = fn;
      });
      void compareViewStore.aplicar();
    }

    const obs = new ResizeObserver(() => programarRepintar());
    if (canvas) obs.observe(canvas);

    return () => {
      cancelAnimationFrame(rafRepintar);
      cancelAnimationFrame(rafBuclePlay);
      desuscribirGpu?.();
      desuscribirFrames?.();
      bitmapA?.close();
      bitmapB?.close();
      obs.disconnect();
    };
  });

  function alIniciarArrastre(e: PointerEvent) {
    if (compareViewStore.vista.modo !== "SplitScreen") return;
    const sp = compareViewStore.vista.split_pos;
    if (sp <= 0.02 || sp >= 0.98) return;
    if (!canvas) return;
    arrastrandoSplit = true;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    compareViewStore.setSplitPos(
      splitDesdePuntero(
        e.clientX,
        e.clientY,
        canvas,
        rectParaUi,
        compareViewStore.vista.split_horizontal,
      ),
    );
    void compareViewStore.aplicar();
    if (usarFallbackCanvas) repintar();
  }

  function alMoverArrastre(e: PointerEvent) {
    if (!arrastrandoSplit || !canvas) return;
    compareViewStore.setSplitPos(
      splitDesdePuntero(
        e.clientX,
        e.clientY,
        canvas,
        rectParaUi,
        compareViewStore.vista.split_horizontal,
      ),
    );
    void compareViewStore.aplicar();
    if (usarFallbackCanvas) repintar();
  }

  function alFinArrastre(e: PointerEvent) {
    if (!arrastrandoSplit) return;
    arrastrandoSplit = false;
    (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
    void compareViewStore.aplicar();
  }

  const mostrarBarraCortina = $derived(
    compareViewStore.vista.modo === "SplitScreen" &&
      compareViewStore.vista.split_pos > 0.02 &&
      compareViewStore.vista.split_pos < 0.98 &&
      Boolean(bitmapA && bitmapB),
  );

  const estiloBarra = $derived.by(() => {
    if (!canvas || rectParaUi.w < 1) return "display:none";
    const br = canvas.getBoundingClientRect();
    const escalaX = br.width / Math.max(1, canvas.clientWidth);
    const escalaY = br.height / Math.max(1, canvas.clientHeight);
    const rx = rectParaUi.x * escalaX;
    const ry = rectParaUi.y * escalaY;
    const rw = rectParaUi.w * escalaX;
    const rh = rectParaUi.h * escalaY;
    const sp = compareViewStore.vista.split_pos;
    if (compareViewStore.vista.split_horizontal) {
      const top = ry + rh * sp;
      return `left:${rx}px;top:${top - 6}px;width:${rw}px;height:12px;cursor:ns-resize`;
    }
    const left = rx + rw * sp;
    return `left:${left - 6}px;top:${ry}px;width:12px;height:${rh}px;cursor:ew-resize`;
  });
</script>

<CanvasViewportSync activo={compareViewStore.overlayGpuActiva}>
  <div id="canvas-slot" class="canvas-slot" data-testid="workspace-compare">
    {#if hayArchivo}
      <canvas
        class="vista-canvas"
        class:oculto-por-gpu={!usarFallbackCanvas}
        bind:this={canvas}
        aria-label="Comparación A/B"
      ></canvas>
      {#if compareViewStore.overlayGpuActiva && compareViewStore.gpuListo}
        <span class="badge-render" title="Render wgpu">GPU</span>
      {/if}
      {#if mostrarBarraCortina}
        <div
          class="barra-cortina"
          style={estiloBarra}
          role="slider"
          aria-label="Posición cortina A/B"
          aria-valuenow={compareViewStore.vista.split_pos}
          tabindex="0"
          onpointerdown={alIniciarArrastre}
          onpointermove={alMoverArrastre}
          onpointerup={alFinArrastre}
          onpointercancel={alFinArrastre}
        ></div>
      {/if}
      <div class="atajos-cortina" data-testid="compare-cortina-bar">
        <button
          type="button"
          class:activo={compareViewStore.vista.modo === "SplitScreen" &&
            compareViewStore.vista.split_pos > 0.95}
          onclick={() => compareViewStore.soloA()}
        >
          Solo A
        </button>
        <button
          type="button"
          class:activo={compareViewStore.vista.modo === "SplitScreen" &&
            compareViewStore.vista.split_pos > 0.02 &&
            compareViewStore.vista.split_pos < 0.98}
          onclick={() => compareViewStore.cortina()}
        >
          Cortina
        </button>
        <button
          type="button"
          class:activo={compareViewStore.vista.modo === "SplitScreen" &&
            compareViewStore.vista.split_pos < 0.05}
          onclick={() => compareViewStore.soloB()}
        >
          Solo B
        </button>
      </div>
    {:else}
      <p class="estado">Abre vídeo A y/o B desde la barra superior</p>
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
    border: 1px dashed var(--border);
    background: #000;
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

  .barra-cortina {
    position: absolute;
    z-index: 2;
    background: #ffff00;
    box-shadow: 0 0 6px rgba(255, 255, 0, 0.8);
    touch-action: none;
  }

  .atajos-cortina {
    position: absolute;
    bottom: 12px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 3;
    display: flex;
    gap: 6px;
    padding: 4px 8px;
    background: rgba(0, 0, 0, 0.75);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    pointer-events: auto;
  }

  .atajos-cortina button {
    font-size: 11px;
    padding: 6px 12px;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    color: var(--text-muted);
    border-radius: var(--radius);
    cursor: pointer;
  }

  .atajos-cortina button.activo {
    background: var(--accent-primary);
    color: #fff;
    border-color: var(--accent-primary);
  }

  .estado {
    margin: 0;
    padding: 1rem;
    color: var(--text-muted, #888);
    font-size: 0.9rem;
    text-align: center;
  }
</style>
