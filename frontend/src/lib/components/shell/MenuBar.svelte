<script lang="ts">
  import { playerStore } from "../../stores/player.svelte";

  interface Props {
    onPaleta: () => void;
  }

  let { onPaleta }: Props = $props();
</script>

<menubar class="menubar" data-testid="menubar">
  <div class="menubar__grupo">
    <span class="menubar__marca">DiffPlayerQC</span>
  </div>
  <div class="menubar__menus">
    <details class="menu">
      <summary>Archivo</summary>
      <div class="menu__dropdown">
        <button type="button" onclick={() => playerStore.abrir("a")}>Abrir A…</button>
        <button type="button" onclick={() => playerStore.abrir("b")}>Abrir B…</button>
      </div>
    </details>
    <details class="menu">
      <summary>Ver</summary>
      <div class="menu__dropdown">
        <button type="button" onclick={onPaleta}>Paleta de comandos…</button>
      </div>
    </details>
    <details class="menu">
      <summary>Ayuda</summary>
      <div class="menu__dropdown">
        <span class="menu__hint">v2 — M2 UI shell</span>
      </div>
    </details>
  </div>
</menubar>

<style>
  .menubar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 28px;
    padding: 0 8px;
    background: var(--bg-darkest);
    border-bottom: 1px solid var(--border);
    font-size: 12px;
  }
  .menubar__marca {
    font-weight: 600;
    letter-spacing: var(--letter-label);
    text-transform: uppercase;
    font-size: 10px;
    color: var(--text-muted);
  }
  .menubar__menus {
    display: flex;
    gap: 4px;
  }
  .menu {
    position: relative;
  }
  .menu summary {
    list-style: none;
    cursor: pointer;
    padding: 4px 8px;
    color: var(--text-primary);
  }
  .menu summary::-webkit-details-marker {
    display: none;
  }
  .menu__dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    min-width: 160px;
    background: var(--bg-elev);
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
    z-index: 50;
    display: flex;
    flex-direction: column;
    padding: 4px;
  }
  .menu__dropdown button {
    text-align: left;
    padding: 6px 10px;
    background: transparent;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 12px;
    border-radius: var(--radius);
  }
  .menu__dropdown button:hover {
    background: var(--bg-hover);
  }
  .menu__hint {
    padding: 6px 10px;
    color: var(--text-muted);
    font-size: 11px;
  }
</style>
