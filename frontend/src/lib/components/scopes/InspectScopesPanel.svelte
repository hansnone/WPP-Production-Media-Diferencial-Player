<script lang="ts">
  import HistogramaScope from "./HistogramaScope.svelte";
  import VectoscopioScope from "./VectoscopioScope.svelte";
  import MonitorLumaScope from "./MonitorLumaScope.svelte";
  import { scopesStore } from "../../stores/scopes.svelte";
  import { formatearPts } from "../../player";
  import { idiomaStore } from "../../i18n/idioma.svelte";

  const scopes = $derived(scopesStore.actual);
  const vacio = $derived(!scopes || scopes.histograma_r.every((v) => v === 0));
</script>

<div class="inspect-scopes" data-testid="inspect-scopes-panel">
  {#if vacio}
    <p class="inspect-scopes__vacio">{idiomaStore.t("scopes.vacio")}</p>
  {:else if scopes}
    <p class="inspect-scopes__meta mono">
      Canal {scopes.canal.toUpperCase()} · PTS {formatearPts(scopes.pts)}
    </p>

    <section class="bloque" data-testid="scope-histograma">
      <h3 class="bloque__titulo">{idiomaStore.t("scopes.histograma")}</h3>
      <HistogramaScope
        histogramaR={scopes.histograma_r}
        histogramaG={scopes.histograma_g}
        histogramaB={scopes.histograma_b}
      />
    </section>

    <section class="bloque" data-testid="scope-vectoscopio">
      <h3 class="bloque__titulo">{idiomaStore.t("scopes.vectoscopio")}</h3>
      <VectoscopioScope vectoscopio={scopes.vectoscopio} />
    </section>

    <section class="bloque" data-testid="scope-monitor-luma">
      <h3 class="bloque__titulo">{idiomaStore.t("scopes.monitorLuma")}</h3>
      <MonitorLumaScope monitorLuma={scopes.monitor_luma} />
    </section>
  {/if}
</div>

<style>
  .inspect-scopes {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .inspect-scopes__vacio {
    font-size: 11px;
    color: var(--text-muted);
    line-height: 1.4;
    margin: 0;
  }

  .inspect-scopes__meta {
    font-size: 10px;
    color: var(--text-muted);
    margin: 0;
  }

  .bloque__titulo {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
    margin: 0 0 4px;
    font-weight: 500;
  }
</style>
