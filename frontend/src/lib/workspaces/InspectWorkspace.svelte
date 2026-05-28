<div class="inspect-ws" data-testid="workspace-inspect">
  <div class="inspect-ws__canvas-wrap">
    <div id="canvas-slot" class="canvas-slot"></div>
    <div class="scopes-overlay">Scopes overlay (M5)</div>
  </div>
</div>

<style>
  .inspect-ws {
    height: 100%;
    padding: 8px;
    box-sizing: border-box;
  }
  .inspect-ws__canvas-wrap {
    position: relative;
    height: 100%;
    min-height: 200px;
  }
  .canvas-slot {
    width: 100%;
    height: 100%;
    background: #000;
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
  }
  .scopes-overlay {
    position: absolute;
    bottom: 8px;
    right: 8px;
    padding: 4px 8px;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
  }
</style>
