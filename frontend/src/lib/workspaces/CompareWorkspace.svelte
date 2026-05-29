<script lang="ts">
  import CanvasViewportSync from "../components/viewport/CanvasViewportSync.svelte";
</script>

<CanvasViewportSync activo={true}>
  <div id="canvas-slot" class="canvas-slot" data-testid="workspace-compare" aria-hidden="true"></div>
</CanvasViewportSync>

<style>
  .canvas-slot {
    width: 100%;
    height: 100%;
    min-height: 200px;
    box-sizing: border-box;
    /* Marco de referencia alineado con la overlay nativa. */
    border: 1px dashed var(--border);
    background: transparent;
  }
</style>
