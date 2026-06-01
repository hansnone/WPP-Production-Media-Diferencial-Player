<script lang="ts">
  import { onMount } from "svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { layoutStore } from "../../stores/layout.svelte";
  import { ocultarViewport, rectViewportDesdeElemento, sincronizarViewport } from "../../viewport";

  /** Workspaces que comparten la ventana overlay wgpu sobre `#canvas-slot`. */
  function workspaceConVideo(): boolean {
    const ws = layoutStore.workspaceActivo;
    return ws === "compare" || ws === "inspect";
  }

  interface Props {
    activo?: boolean;
    children?: import("svelte").Snippet;
  }

  let { activo = true, children }: Props = $props();
  let contenedor: HTMLDivElement | undefined = $state();
  let rafId = 0;

  async function publicarRecto() {
    if (!activo) {
      if ("__TAURI_INTERNALS__" in window) void ocultarViewport();
      return;
    }
    if (!contenedor || !("__TAURI_INTERNALS__" in window)) return;

    const slot =
      (contenedor.querySelector("#canvas-slot") as HTMLElement | null) ?? contenedor;
    await sincronizarViewport(rectViewportDesdeElemento(slot));
  }

  /** Evita decenas de IPC por segundo; alinea en el siguiente frame. */
  function programarSync() {
    cancelAnimationFrame(rafId);
    rafId = requestAnimationFrame(() => {
      void publicarRecto();
    });
  }

  onMount(() => {
    programarSync();
    const obs = new ResizeObserver(() => programarSync());
    if (contenedor) obs.observe(contenedor);
    const grid = document.querySelector('[data-testid="workspace-grid"]');
    const toolbar = document.querySelector('[data-testid="toolbar"]');
    if (grid) obs.observe(grid);
    if (toolbar) obs.observe(toolbar);
    window.addEventListener("resize", programarSync);
    const alCargarVideo = () => programarSync();
    window.addEventListener("diffplayerqc-sync-viewport", alCargarVideo);

    let desuscribirMovida: (() => void) | undefined;
    let desuscribirEscala: (() => void) | undefined;
    if ("__TAURI_INTERNALS__" in window) {
      const ventana = getCurrentWebviewWindow();
      void ventana.onMoved(() => programarSync()).then((fn) => {
        desuscribirMovida = fn;
      });
      void ventana.onScaleChanged(() => programarSync()).then((fn) => {
        desuscribirEscala = fn;
      });
    }

    return () => {
      cancelAnimationFrame(rafId);
      obs.disconnect();
      window.removeEventListener("resize", programarSync);
      window.removeEventListener("diffplayerqc-sync-viewport", alCargarVideo);
      desuscribirMovida?.();
      desuscribirEscala?.();
      // No ocultar al cambiar Compare ↔ Inspect (el otro workspace re-sincroniza el rect).
      if ("__TAURI_INTERNALS__" in window && !workspaceConVideo()) {
        void ocultarViewport();
      }
    };
  });

  $effect(() => {
    if (activo) {
      programarSync();
      // Tras cambio de workspace o layout, el slot puede medir 0 en el primer frame.
      requestAnimationFrame(() => programarSync());
    } else if ("__TAURI_INTERNALS__" in window && !workspaceConVideo()) {
      void ocultarViewport();
    }
  });
</script>

<div class="viewport-host" bind:this={contenedor}>
  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  .viewport-host {
    position: relative;
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    min-height: 0;
    width: 100%;
    /* No capturar clics: la overlay nativa va encima solo del slot. */
    pointer-events: none;
  }
</style>
