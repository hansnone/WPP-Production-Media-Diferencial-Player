<script lang="ts">
  import { establecerVistaCompare, type VistaCompare } from "../../viewport";
  import type { CompareMode, DiffMode } from "../../compare";
  import { MODOS_COMPARACION, MODOS_DIFF } from "../../compare";

  let vista = $state<VistaCompare>({
    modo: "SplitScreen",
    diff_mode: "AbsLinear",
    split_pos: 0.5,
    amplifier: 5,
    zoom: 1,
    pan_u: 0,
    pan_v: 0,
    split_horizontal: false,
  });

  async function aplicar() {
    if ("__TAURI_INTERNALS__" in window) {
      await establecerVistaCompare(vista);
    }
  }

  function setModo(m: CompareMode) {
    vista.modo = m;
    void aplicar();
  }

  function setDiff(d: DiffMode) {
    vista.diff_mode = d;
    void aplicar();
  }
</script>

<div class="modos" data-testid="compare-mode-panel">
  <p class="titulo">Modo</p>
  <div class="fila">
    {#each MODOS_COMPARACION as m (m)}
      <button
        type="button"
        class="modo-btn"
        class:activo={vista.modo === m}
        onclick={() => setModo(m)}
      >
        {m}
      </button>
    {/each}
  </div>

  {#if vista.modo === "AbsDiff" || vista.modo === "SideBySide"}
    <p class="titulo">Diff</p>
    <div class="fila">
      {#each MODOS_DIFF as d (d)}
        <button
          type="button"
          class="modo-btn"
          class:activo={vista.diff_mode === d}
          onclick={() => setDiff(d)}
        >
          {d}
        </button>
      {/each}
    </div>
  {/if}

  <label class="slider">
    Split
    <input type="range" min="0" max="1" step="0.01" bind:value={vista.split_pos} onchange={aplicar} />
  </label>
  <label class="slider">
    Amp
    <input
      type="range"
      min="1"
      max="50"
      step="0.5"
      bind:value={vista.amplifier}
      onchange={aplicar}
    />
  </label>
</div>

<style>
  .modos {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .titulo {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
    margin: 0;
  }
  .fila {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }
  .modo-btn {
    font-size: 9px;
    padding: 4px 6px;
    background: var(--bg-app);
    border: 1px solid var(--border);
    color: var(--text-muted);
    border-radius: var(--radius);
    cursor: pointer;
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }
  .modo-btn.activo {
    background: var(--accent-primary);
    color: #fff;
    border-color: var(--accent-primary);
  }
  .slider {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 10px;
    color: var(--text-muted);
  }
  .slider input {
    accent-color: var(--accent-primary);
  }
</style>
