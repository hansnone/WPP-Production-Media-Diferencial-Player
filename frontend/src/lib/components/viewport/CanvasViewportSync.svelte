<script lang="ts">
  import { onMount } from "svelte";
  import { ocultarViewport, rectDesdeElemento, sincronizarViewport } from "../../viewport";

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
    await sincronizarViewport(rectDesdeElemento(slot));
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
    window.addEventListener("resize", programarSync);
    const alCargarVideo = () => programarSync();
    window.addEventListener("diffplayerqc-sync-viewport", alCargarVideo);
    return () => {
      cancelAnimationFrame(rafId);
      obs.disconnect();
      window.removeEventListener("resize", programarSync);
      window.removeEventListener("diffplayerqc-sync-viewport", alCargarVideo);
      if ("__TAURI_INTERNALS__" in window) void ocultarViewport();
    };
  });

  $effect(() => {
    if (activo) programarSync();
    else if ("__TAURI_INTERNALS__" in window) void ocultarViewport();
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
    width: 100%;
    height: 100%;
    min-height: 200px;
    /* No capturar clics: la overlay nativa va encima solo del slot. */
    pointer-events: none;
  }
</style>
