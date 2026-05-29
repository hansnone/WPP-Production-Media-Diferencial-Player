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
    /* Marco de referencia; el vídeo real está en la ventana nativa overlay. */
    box-sizing: border-box;
    border: 1px dashed var(--border);
    background: #0a0a0a;
    pointer-events: none;
  }
</style>
