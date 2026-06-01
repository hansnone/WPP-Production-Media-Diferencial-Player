<script lang="ts">
  import CanvasViewportSync from "../components/viewport/CanvasViewportSync.svelte";
  import { layoutStore } from "../stores/layout.svelte";
  import { scopesStore } from "../stores/scopes.svelte";

  const activo = $derived(layoutStore.workspaceActivo === "inspect");

  $effect(() => {
    if (activo && "__TAURI_INTERNALS__" in window) {
      void scopesStore.refrescar();
    }
  });
</script>

<CanvasViewportSync {activo}>
  <div class="inspect-ws" data-testid="workspace-inspect">
    <div class="inspect-ws__canvas-wrap">
      <div id="canvas-slot" class="canvas-slot"></div>
    </div>
  </div>
</CanvasViewportSync>

<style>
  .inspect-ws {
    height: 100%;
    padding: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .inspect-ws__canvas-wrap {
    position: relative;
    flex: 1;
    min-height: 200px;
  }
  .canvas-slot {
    width: 100%;
    height: 100%;
    background: #000;
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
  }
</style>
