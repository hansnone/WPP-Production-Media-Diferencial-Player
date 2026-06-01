<script lang="ts">
  import EventosQcPanel from "../components/eventos/EventosQcPanel.svelte";
  import { eventosQcStore } from "../stores/eventosQc.svelte";
  import { playerStore } from "../stores/player.svelte";
  import { idiomaStore } from "../i18n/idioma.svelte";

  $effect(() => {
    const a = playerStore.snap?.ruta_a;
    const b = playerStore.snap?.ruta_b;
    void eventosQcStore.sincronizarProyecto(a, b);
  });
</script>

<div class="report-ws" data-testid="workspace-report">
  <header class="report-ws__intro">
    <h2>{idiomaStore.t("workspace.report")}</h2>
    <p>{idiomaStore.t("eventos.reportIntro")}</p>
  </header>
  <EventosQcPanel compacto={false} />
</div>

<style>
  .report-ws {
    height: 100%;
    padding: 12px 16px;
    box-sizing: border-box;
    overflow-y: auto;
    background: var(--bg-darkest);
  }

  .report-ws__intro h2 {
    margin: 0 0 4px;
    font-size: 16px;
    font-weight: 600;
  }

  .report-ws__intro p {
    margin: 0 0 12px;
    font-size: 12px;
    color: var(--text-muted);
    max-width: 52ch;
  }

  .report-ws :global(.eventos-panel) {
    border-top: none;
    margin-top: 0;
    max-width: 720px;
  }

  .report-ws :global(.eventos-lista) {
    max-height: none;
  }
</style>
