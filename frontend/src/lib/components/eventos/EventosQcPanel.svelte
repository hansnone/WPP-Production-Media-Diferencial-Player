<script lang="ts">
  import { eventosQcStore } from "../../stores/eventosQc.svelte";
  import { playerStore } from "../../stores/player.svelte";
  import { idiomaStore } from "../../i18n/idioma.svelte";
  import { formatearPts } from "../../player";
  import type { FiltroTipoEvento } from "../../eventosQc";

  interface Props {
    /** Vista compacta en panel lateral Compare. */
    compacto?: boolean;
  }

  let { compacto = false }: Props = $props();

  const lista = $derived(eventosQcStore.eventosFiltrados());
  const ptsActual = $derived(playerStore.snap?.pts_actual ?? 0);

  let textoNota = $state("");
  let tituloNuevo = $state("");

  const filtros: { id: FiltroTipoEvento; clave: Parameters<typeof idiomaStore.t>[0] }[] = [
    { id: "todos", clave: "eventos.filtro.todos" },
    { id: "manual", clave: "eventos.filtro.manual" },
    { id: "video", clave: "eventos.filtro.video" },
    { id: "audio", clave: "eventos.filtro.audio" },
  ];

  async function marcarActual() {
    const t = tituloNuevo.trim() || idiomaStore.t("eventos.tituloDefecto");
    await eventosQcStore.marcarEnPlayhead(ptsActual, t, "manual");
    tituloNuevo = "";
  }

  async function anadirNotaSeleccionada() {
    const id = eventosQcStore.eventoSeleccionadoId;
    const txt = textoNota.trim();
    if (id === null || !txt) return;
    await eventosQcStore.anadirNota(id, txt, ptsActual);
    textoNota = "";
  }
</script>

<section class="eventos-panel" class:eventos-panel--compacto={compacto} data-testid="eventos-qc-panel">
  <header class="eventos-panel__head">
    <h3>{idiomaStore.t("eventos.titulo")}</h3>
    {#if !compacto}
      <span class="eventos-panel__count mono">{lista.length}</span>
    {/if}
  </header>

  <div class="eventos-filtros" data-testid="eventos-filtros">
    {#each filtros as f}
      <button
        type="button"
        class="chip-filtro"
        class:chip-filtro--activo={eventosQcStore.filtro === f.id}
        onclick={() => eventosQcStore.establecerFiltro(f.id)}
      >
        {idiomaStore.t(f.clave)}
      </button>
    {/each}
  </div>

  <div class="eventos-nuevo">
    <input
      type="text"
      class="eventos-input"
      placeholder={idiomaStore.t("eventos.placeholderTitulo")}
      bind:value={tituloNuevo}
      data-testid="eventos-input-titulo"
    />
    <button
      type="button"
      class="btn-primario"
      data-testid="eventos-btn-marcar"
      onclick={() => void marcarActual()}
    >
      {idiomaStore.t("eventos.marcarPlayhead")}
    </button>
  </div>

  {#if lista.length === 0}
    <p class="eventos-vacio">{idiomaStore.t("eventos.vacio")}</p>
  {:else}
    <ul class="eventos-lista" data-testid="eventos-lista">
      {#each lista as ev (ev.id)}
        <li
          class="eventos-item"
          class:eventos-item--activo={eventosQcStore.eventoSeleccionadoId === ev.id}
          data-testid="evento-item-{ev.id}"
        >
          <button
            type="button"
            class="eventos-item__main"
            onclick={() => void playerStore.seekAEventoQc(ev.id)}
          >
            <span class="eventos-item__pts mono">{formatearPts(ev.pts_secs)}</span>
            <span class="eventos-item__tipo">{ev.tipo}</span>
            <span class="eventos-item__titulo">{ev.titulo}</span>
            {#if ev.notas.length}
              <span class="eventos-item__notas">{ev.notas.length} n</span>
            {/if}
          </button>
          <button
            type="button"
            class="eventos-item__borrar"
            title={idiomaStore.t("eventos.eliminar")}
            onclick={() => void eventosQcStore.borrar(ev.id)}
          >
            ×
          </button>
        </li>
      {/each}
    </ul>
  {/if}

  {#if eventosQcStore.eventoSeleccionadoId !== null}
    <div class="eventos-nota" data-testid="eventos-nota-form">
      <input
        type="text"
        class="eventos-input"
        placeholder={idiomaStore.t("eventos.placeholderNota")}
        bind:value={textoNota}
      />
      <button type="button" class="btn-secundario" onclick={() => void anadirNotaSeleccionada()}>
        {idiomaStore.t("eventos.anadirNota")}
      </button>
    </div>
  {/if}
</section>

<style>
  .eventos-panel {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px 0;
    border-top: 1px solid var(--border);
    margin-top: 8px;
  }

  .eventos-panel--compacto .eventos-lista {
    max-height: 160px;
  }

  .eventos-panel__head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 8px;
  }

  .eventos-panel__head h3 {
    margin: 0;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
  }

  .eventos-panel__count {
    font-size: 11px;
    color: var(--text-muted);
  }

  .eventos-filtros {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }

  .chip-filtro {
    font-size: 10px;
    padding: 2px 8px;
    border-radius: 999px;
    border: 1px solid var(--border);
    background: var(--bg-darkest);
    color: var(--text-muted);
    cursor: pointer;
  }

  .chip-filtro--activo {
    border-color: var(--accent);
    color: var(--accent);
  }

  .eventos-nuevo,
  .eventos-nota {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .eventos-input {
    width: 100%;
    box-sizing: border-box;
    padding: 6px 8px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    background: var(--bg-darkest);
    color: var(--text-primary);
    font-size: 12px;
  }

  .btn-primario,
  .btn-secundario {
    font-size: 11px;
    padding: 6px 10px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    border: 1px solid var(--border);
  }

  .btn-primario {
    background: var(--accent);
    color: var(--bg-darkest);
    border-color: transparent;
  }

  .btn-secundario {
    background: var(--bg-panel);
    color: var(--text-primary);
  }

  .eventos-vacio {
    margin: 0;
    font-size: 11px;
    color: var(--text-muted);
  }

  .eventos-lista {
    list-style: none;
    margin: 0;
    padding: 0;
    overflow-y: auto;
    max-height: 240px;
  }

  .eventos-item {
    display: flex;
    align-items: stretch;
    border-bottom: 1px solid var(--border);
  }

  .eventos-item--activo {
    background: rgba(255, 255, 255, 0.04);
  }

  .eventos-item__main {
    flex: 1;
    display: grid;
    grid-template-columns: auto auto 1fr auto;
    gap: 6px;
    align-items: center;
    padding: 6px 4px;
    text-align: left;
    background: none;
    border: none;
    color: var(--text-primary);
    cursor: pointer;
    font-size: 11px;
  }

  .eventos-item__pts {
    color: var(--accent);
  }

  .eventos-item__tipo {
    text-transform: uppercase;
    font-size: 9px;
    color: var(--text-muted);
  }

  .eventos-item__titulo {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .eventos-item__notas {
    font-size: 9px;
    color: var(--text-muted);
  }

  .eventos-item__borrar {
    width: 28px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 16px;
  }

  .eventos-item__borrar:hover {
    color: #f87171;
  }
</style>
