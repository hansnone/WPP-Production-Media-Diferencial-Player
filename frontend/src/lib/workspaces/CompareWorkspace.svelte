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
    /* El vídeo wgpu se dibuja en la ventana overlay; aquí solo reservamos layout. */
    pointer-events: none;
    opacity: 0;
  }
</style>
