<script lang="ts">
  import { onMount } from "svelte";
  import MenuBar from "./MenuBar.svelte";
  import ToolBar from "./ToolBar.svelte";
  import WorkspaceTabs from "./WorkspaceTabs.svelte";
  import Panel from "./Panel.svelte";
  import CommandPalette from "./CommandPalette.svelte";
  import { layoutStore } from "../../stores/layout.svelte";
  import { playerStore } from "../../stores/player.svelte";
  import { manejarAtajoGlobal } from "../../shortcuts";
  import CompareWorkspace from "../../workspaces/CompareWorkspace.svelte";
  import InspectWorkspace from "../../workspaces/InspectWorkspace.svelte";
  import AudioWorkspace from "../../workspaces/AudioWorkspace.svelte";
  import PlaceholderWorkspace from "../../workspaces/PlaceholderWorkspace.svelte";
  import CompareModePanel from "../compare/CompareModePanel.svelte";
  import { formatearPts } from "../../player";

  let paletaAbierta = $state(false);
  let seekInput = $state(0);

  const disp = $derived(layoutStore.disposicionActual());
  const ws = $derived(layoutStore.workspaceActivo);

  const gridCols = $derived.by(() => {
    const izq = disp.izquierdoVisible ? `${disp.anchoIzquierdoPx}px` : "22px";
    const der = disp.derechoVisible ? `${disp.anchoDerechoPx}px` : "22px";
    return `${izq} 1fr ${der}`;
  });

  onMount(() => {
    void playerStore.iniciar().then(() => {
      if (playerStore.snap) seekInput = playerStore.snap.pts_actual;
    });

    const onKey = (e: KeyboardEvent) => {
      manejarAtajoGlobal(e, { abrirPaleta: () => (paletaAbierta = true) });
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  });

  $effect(() => {
    if (playerStore.snap) seekInput = playerStore.snap.pts_actual;
  });

  async function aplicarSeek() {
    await playerStore.seekPts(seekInput);
  }

  const fabricaPaleta = {
    abrirA: () => playerStore.abrir("a"),
    abrirB: () => playerStore.abrir("b"),
    playPausa: () => playerStore.playPausa(),
    irWorkspace: (id: typeof ws) => layoutStore.cambiarWorkspace(id),
    togglePanelIzq: () => layoutStore.alternarPanel("izquierdo"),
    togglePanelDer: () => layoutStore.alternarPanel("derecho"),
  };
</script>

<div class="app-shell" data-testid="app-shell">
  <MenuBar onPaleta={() => (paletaAbierta = true)} />
  <WorkspaceTabs />
  <ToolBar
    bind:seekInput
    onSeekInput={(v) => (seekInput = v)}
    onSeek={aplicarSeek}
    onPaleta={() => (paletaAbierta = true)}
  />

  {#if !playerStore.enTauri && playerStore.inicializado}
    <p class="aviso">
      Modo navegador: ejecuta <code>cargo tauri dev</code> para IPC completo.
    </p>
  {/if}

  <div
    class="workspace-grid"
    style="grid-template-columns: {gridCols}"
    data-testid="workspace-grid"
  >
    <Panel
      titulo="Fuentes"
      lado="izquierdo"
      visible={disp.izquierdoVisible}
      anchoPx={disp.anchoIzquierdoPx}
      onalternar={() => layoutStore.alternarPanel("izquierdo")}
    >
      <div class="fuentes">
        <div class="fuente fuente--a">
          <span class="chip chip--a">A</span>
          <span class="mono">{playerStore.snap?.ruta_a ?? "Sin archivo"}</span>
        </div>
        <div class="fuente fuente--b">
          <span class="chip chip--b">B</span>
          <span class="mono">{playerStore.snap?.ruta_b ?? "Sin archivo"}</span>
        </div>
      </div>
    </Panel>

    <main class="centro" data-testid="workspace-main">
      {#if ws === "compare"}
        <CompareWorkspace />
      {:else if ws === "inspect"}
        <InspectWorkspace />
      {:else if ws === "audio"}
        <AudioWorkspace />
      {:else if ws === "report"}
        <PlaceholderWorkspace
          id="report"
          titulo="Report"
          mensaje="Vista PDF — disponible en hitos posteriores."
        />
      {:else}
        <PlaceholderWorkspace
          id="export"
          titulo="Export"
          mensaje="Formulario de exportación — disponible en hitos posteriores."
        />
      {/if}
    </main>

    <Panel
      titulo={ws === "audio" ? "Loudness" : ws === "inspect" ? "Histograma" : "Diff / Audio"}
      lado="derecho"
      visible={disp.derechoVisible && ws !== "audio" && ws !== "report" && ws !== "export"}
      anchoPx={disp.anchoDerechoPx}
      onalternar={() => layoutStore.alternarPanel("derecho")}
    >
      {#if ws === "inspect"}
        <p class="mono">RGB / Vectorscope (M5)</p>
      {:else if ws === "compare"}
        <CompareModePanel />
        <div class="niveles">
          <p class="mono">
            PTS {formatearPts(playerStore.snap?.pts_actual ?? 0)}
          </p>
          <p>
            <span class="chip chip--a">A</span>
            {( (playerStore.snap?.nivel_audio_a ?? 0) * 100).toFixed(0)} %
          </p>
          <p>
            <span class="chip chip--b">B</span>
            {( (playerStore.snap?.nivel_audio_b ?? 0) * 100).toFixed(0)} %
          </p>
        </div>
      {:else}
        <div class="niveles">
          <p class="mono">PTS {formatearPts(playerStore.snap?.pts_actual ?? 0)}</p>
        </div>
      {/if}
    </Panel>
  </div>
</div>

<CommandPalette
  abierta={paletaAbierta}
  oncerrar={() => (paletaAbierta = false)}
  fabrica={fabricaPaleta}
/>

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    background: var(--bg-app);
    color: var(--text-primary);
    font-family: var(--font-ui);
  }
  .aviso {
    padding: 6px 12px;
    background: var(--bg-panel);
    color: var(--accent-warn);
    font-size: 12px;
    margin: 0;
    border-bottom: 1px solid var(--border);
  }
  .workspace-grid {
    flex: 1;
    display: grid;
    min-height: 0;
    transition: grid-template-columns var(--panel-transition);
  }
  .centro {
    min-width: 0;
    min-height: 0;
    overflow: hidden;
    padding: 8px;
    background: var(--bg-app);
  }
  .fuentes {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .fuente {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .mono {
    font-family: var(--font-mono);
    font-size: 11px;
    word-break: break-all;
    color: var(--text-muted);
  }
  .chip {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: var(--letter-label);
  }
  .chip--a {
    color: var(--chan-a);
  }
  .chip--b {
    color: var(--chan-b);
  }
  .niveles p {
    margin: 6px 0;
    font-size: 12px;
  }
  .hint {
    color: var(--text-muted);
    font-size: 11px;
  }
</style>
