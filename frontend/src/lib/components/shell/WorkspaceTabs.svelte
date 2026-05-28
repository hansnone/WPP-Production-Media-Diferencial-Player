<script lang="ts">
  import { layoutStore, type WorkspaceId } from "../../stores/layout.svelte";
  import { AudioLines, Clapperboard, FileOutput, FileText, ScanSearch } from "@lucide/svelte";

  const pestanas: {
    id: WorkspaceId;
    etiqueta: string;
    atajo: string;
    icon: typeof Clapperboard;
  }[] = [
    { id: "compare", etiqueta: "Compare", atajo: "1", icon: Clapperboard },
    { id: "inspect", etiqueta: "Inspect", atajo: "2", icon: ScanSearch },
    { id: "audio", etiqueta: "Audio", atajo: "3", icon: AudioLines },
    { id: "report", etiqueta: "Report", atajo: "4", icon: FileText },
    { id: "export", etiqueta: "Export", atajo: "5", icon: FileOutput },
  ];
</script>

<nav class="tabs" aria-label="Workspaces" data-testid="workspace-tabs">
  {#each pestanas as p (p.id)}
    <button
      type="button"
      class="tabs__item"
      class:tabs__item--activo={layoutStore.workspaceActivo === p.id}
      data-testid="workspace-tab-{p.id}"
      onclick={() => layoutStore.cambiarWorkspace(p.id)}
      title="Shift+{p.atajo}"
    >
      <p.icon size={18} strokeWidth={1.5} />
      <span>{p.etiqueta}</span>
    </button>
  {/each}
</nav>

<style>
  .tabs {
    display: flex;
    gap: 2px;
    padding: 4px 8px;
    background: var(--bg-darkest);
    border-bottom: 1px solid var(--border);
  }
  .tabs__item {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    border-radius: var(--radius);
    cursor: pointer;
  }
  .tabs__item:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
  .tabs__item--activo {
    background: var(--bg-panel);
    color: var(--accent-primary);
  }
</style>
