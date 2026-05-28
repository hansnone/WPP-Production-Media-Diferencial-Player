<div class="audio-ws" data-testid="workspace-audio">
  <div class="wave wave--a">
    <span class="label">Canal A</span>
    <div class="wave__placeholder">Waveform (M4)</div>
  </div>
  <div class="wave wave--b">
    <span class="label">Canal B</span>
    <div class="wave__placeholder">Waveform (M4)</div>
  </div>
  <div class="wave wave--diff">
    <span class="label">Diff</span>
    <div class="wave__placeholder">Diff strip (M4)</div>
  </div>
</div>

<style>
  .audio-ws {
    display: flex;
    flex-direction: column;
    gap: 8px;
    height: 100%;
    padding: 8px;
    box-sizing: border-box;
  }
  .wave {
    flex: 1;
    min-height: 80px;
    background: var(--bg-darkest);
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
    display: flex;
    flex-direction: column;
    padding: 8px;
  }
  .wave--a {
    border-left: 3px solid var(--chan-a);
  }
  .wave--b {
    border-left: 3px solid var(--chan-b);
  }
  .label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
    margin-bottom: 4px;
  }
  .wave__placeholder {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 12px;
  }
</style>
