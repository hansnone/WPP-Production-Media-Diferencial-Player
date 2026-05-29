<script lang="ts">
  import { onMount } from "svelte";
  import { ocultarViewport, sincronizarViewport } from "../../viewport";

  interface Props {
    activo?: boolean;
    children?: import("svelte").Snippet;
  }

  let { activo = true, children }: Props = $props();
  let contenedor: HTMLDivElement | undefined = $state();

  function publicarRecto() {
    if (!activo) {
      if ("__TAURI_INTERNALS__" in window) void ocultarViewport();
      return;
    }
    if (!contenedor || !("__TAURI_INTERNALS__" in window)) return;
    const objetivo =
      (contenedor.querySelector("#canvas-slot") as HTMLElement | null) ?? contenedor;
    const r = objetivo.getBoundingClientRect();
    void sincronizarViewport({
      x: r.left,
      y: r.top,
      width: r.width,
      height: r.height,
    });
  }

  onMount(() => {
    publicarRecto();
    const obs = new ResizeObserver(() => publicarRecto());
    if (contenedor) obs.observe(contenedor);
    window.addEventListener("resize", publicarRecto);
    const id = setInterval(publicarRecto, 500);
    return () => {
      obs.disconnect();
      window.removeEventListener("resize", publicarRecto);
      clearInterval(id);
      if ("__TAURI_INTERNALS__" in window) void ocultarViewport();
    };
  });

  $effect(() => {
    if (activo) publicarRecto();
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
  }
</style>
