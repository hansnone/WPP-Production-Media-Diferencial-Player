<script lang="ts">
  import { onMount } from "svelte";
  import { compareViewStore } from "../../stores/compareView.svelte";
  import { MODOS_COMPARACION, MODOS_DIFF } from "../../compare";

  const vista = $derived(compareViewStore.vista);

  const esSoloA = $derived(
    vista.modo === "SplitScreen" && vista.split_pos > 0.95,
  );
  const esSoloB = $derived(
    vista.modo === "SplitScreen" && vista.split_pos < 0.05,
  );
  const esCortina = $derived(
    vista.modo === "SplitScreen" && !esSoloA && !esSoloB,
  );

  onMount(() => {
    void compareViewStore.aplicar();
  });
</script>

<div class="modos" data-testid="compare-mode-panel">
  <p class="titulo">Vista rápida</p>
  <div class="fila">
    <button
      type="button"
      class="modo-btn"
      class:activo={esSoloA}
      onclick={() => compareViewStore.soloA()}
    >
      Solo A
    </button>
    <button
      type="button"
      class="modo-btn"
      class:activo={esCortina}
      onclick={() => compareViewStore.cortina()}
    >
      Cortina
    </button>
    <button
      type="button"
      class="modo-btn"
      class:activo={esSoloB}
      onclick={() => compareViewStore.soloB()}
    >
      Solo B
    </button>
  </div>

  <p class="titulo">Modo</p>
  <div class="fila">
    {#each MODOS_COMPARACION as m (m)}
      <button
        type="button"
        class="modo-btn"
        class:activo={vista.modo === m}
        onclick={() => compareViewStore.setModo(m)}
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
          onclick={() => compareViewStore.setDiff(d)}
        >
          {d}
        </button>
      {/each}
    </div>
  {/if}

  {#if vista.modo === "SplitScreen"}
    <button
      type="button"
      class="modo-btn ancho"
      onclick={() => {
        compareViewStore.vista.split_horizontal = !compareViewStore.vista.split_horizontal;
        void compareViewStore.aplicar();
      }}
    >
      {vista.split_horizontal ? "Cortina horizontal" : "Cortina vertical"}
    </button>
  {/if}

  <label class="slider">
    Split
    <input
      type="range"
      min="0"
      max="1"
      step="0.01"
      value={vista.split_pos}
      oninput={(e) => {
        compareViewStore.setSplitPos(Number(e.currentTarget.value));
        void compareViewStore.aplicar();
      }}
    />
  </label>
  {#if vista.modo === "AbsDiff" || vista.modo === "Heatmap"}
    <label class="slider">
      Amp
      <input
        type="range"
        min="1"
        max="50"
        step="0.5"
        value={vista.amplifier}
        oninput={(e) => {
          compareViewStore.vista.amplifier = Number(e.currentTarget.value);
          void compareViewStore.aplicar();
        }}
      />
    </label>
  {/if}
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
  .modo-btn.ancho {
    width: 100%;
    text-transform: none;
    font-size: 10px;
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
