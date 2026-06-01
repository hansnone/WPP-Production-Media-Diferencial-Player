<script lang="ts">
  import { ChevronLeft, ChevronRight } from "@lucide/svelte";
  import type { Snippet } from "svelte";
  import { idiomaStore } from "../../i18n/idioma.svelte";

  interface Props {
    titulo: string;
    lado: "izquierdo" | "derecho";
    visible: boolean;
    anchoPx: number;
    onalternar: () => void;
    children: Snippet;
  }

  let { titulo, lado, visible, anchoPx, onalternar, children }: Props = $props();
</script>

{#if visible}
  <aside
    class="panel"
    class:panel--der={lado === "derecho"}
    style="width: {anchoPx}px"
    data-testid="panel-{lado}"
  >
    <header class="panel__cabecera">
      <span class="panel__titulo">{titulo}</span>
      <button
        type="button"
        class="panel__plegar"
        onclick={onalternar}
        aria-label={idiomaStore.t("panel.plegar")}
      >
        {#if lado === "izquierdo"}
          <ChevronLeft size={16} strokeWidth={1.5} />
        {:else}
          <ChevronRight size={16} strokeWidth={1.5} />
        {/if}
      </button>
    </header>
    <div class="panel__cuerpo">
      {@render children()}
    </div>
  </aside>
{:else}
  <button
    type="button"
    class="panel-tab"
    onclick={onalternar}
    data-testid="panel-tab-{lado}"
    aria-label="{idiomaStore.t('panel.expandir')} {titulo}"
  >
    <span class="panel-tab__label">{titulo}</span>
  </button>
{/if}

<style>
  .panel {
    display: flex;
    flex-direction: column;
    background: var(--bg-panel);
    border-right: 1px solid var(--border);
    min-height: 0;
    overflow: hidden;
  }
  .panel--der {
    border-right: none;
    border-left: 1px solid var(--border);
  }
  .panel__cabecera {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 8px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }
  .panel__titulo {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
  }
  .panel__plegar {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    padding: 2px;
  }
  .panel__plegar:hover {
    color: var(--text-primary);
  }
  .panel__cuerpo {
    flex: 1;
    overflow: auto;
    padding: 8px;
    font-size: 12px;
  }
  .panel-tab {
    width: 22px;
    background: var(--bg-panel);
    border: none;
    border-right: 1px solid var(--border);
    color: var(--text-muted);
    cursor: pointer;
    writing-mode: vertical-rl;
    text-orientation: mixed;
    font-size: 9px;
    letter-spacing: var(--letter-label);
    text-transform: uppercase;
  }
  .panel--der + .panel-tab,
  .panel-tab:last-child {
    border-right: none;
    border-left: 1px solid var(--border);
  }
</style>
