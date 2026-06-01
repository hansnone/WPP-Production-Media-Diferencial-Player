<script lang="ts">
  import WaveformCanvas from "../components/WaveformCanvas.svelte";
  import {
    calcularPicosDiff,
    formatearDb,
    formatearLufs,
    type DatosEbuR128,
  } from "../formaOnda";
  import { formaOndaStore } from "../stores/formaOnda.svelte";
  import { idiomaStore } from "../i18n/idioma.svelte";
  import { playerStore } from "../stores/player.svelte";

  // Al montar el workspace, re-sincronizar por si A/B se abrieron antes de entrar aquí.
  $effect(() => {
    if (typeof window !== "undefined" && "__TAURI_INTERNALS__" in window) {
      void formaOndaStore.refrescarDesdeBackend();
    }
  });

  const picosDiff = $derived(
    calcularPicosDiff(formaOndaStore.formaA, formaOndaStore.formaB),
  );

  const duracionDiff = $derived(
    Math.max(
      formaOndaStore.formaA?.duracion_secs ?? 0,
      formaOndaStore.formaB?.duracion_secs ?? 0,
    ),
  );

  const lufsA = $derived(formatearLufs(formaOndaStore.formaA?.lufs_integrado ?? NaN));
  const lufsB = $derived(formatearLufs(formaOndaStore.formaB?.lufs_integrado ?? NaN));

  const deltaLufs = $derived.by(() => {
    const a = formaOndaStore.formaA?.lufs_integrado;
    const b = formaOndaStore.formaB?.lufs_integrado;
    if (a === undefined || b === undefined || !Number.isFinite(a) || !Number.isFinite(b)) {
      return "—";
    }
    const d = b - a;
    const signo = d >= 0 ? "+" : "";
    return `${signo}${d.toFixed(1)} dB`;
  });

  const ebuA = $derived(formaOndaStore.formaA?.ebu ?? null);
  const ebuB = $derived(formaOndaStore.formaB?.ebu ?? null);

  function filaEbu(ebu: DatosEbuR128 | null, canal: "a" | "b") {
    if (!ebu) {
      return null;
    }
    return {
      canal,
      tp: formatearDb(ebu.true_peak_dbtp, "dBTP"),
      lra: formatearDb(ebu.lra, "LU"),
      ok: ebu.dentro_spec_ebu,
      alertas: ebu.alertas,
      silencio: ebu.silencio_detectado,
      clip: ebu.clipping_detectado > 0.001,
    };
  }

  const filasEbu = $derived(
    [filaEbu(ebuA, "a"), filaEbu(ebuB, "b")].filter(
      (f): f is NonNullable<ReturnType<typeof filaEbu>> => f !== null,
    ),
  );
</script>

<div class="audio-ws" data-testid="workspace-audio">
  <header class="audio-ws__loudness" data-testid="audio-loudness">
    <div class="loudness-item loudness-item--a">
      <span class="loudness-label">A</span>
      <span class="mono loudness-val" data-testid="lufs-a">{lufsA}</span>
    </div>
    <div class="loudness-item loudness-item--b">
      <span class="loudness-label">B</span>
      <span class="mono loudness-val" data-testid="lufs-b">{lufsB}</span>
    </div>
    <div class="loudness-item loudness-item--delta">
      <span class="loudness-label">Δ B−A</span>
      <span class="mono loudness-val" data-testid="lufs-delta">{deltaLufs}</span>
    </div>
    <div class="loudness-item loudness-item--live">
      <span class="loudness-label">Nivel</span>
      <span class="mono loudness-val">
        A {( (playerStore.snap?.nivel_audio_a ?? 0) * 100).toFixed(0)}% ·
        B {( (playerStore.snap?.nivel_audio_b ?? 0) * 100).toFixed(0)}%
      </span>
    </div>
  </header>

  {#if filasEbu.length}
    <section class="audio-ws__ebu" data-testid="audio-ebu-panel">
      <h3 class="ebu-titulo">{idiomaStore.t("audio.ebu.titulo")}</h3>
      <div class="ebu-grid">
        {#each filasEbu as fila (fila.canal)}
          <article
            class="ebu-card"
            class:ebu-card--ok={fila.ok}
            class:ebu-card--fail={!fila.ok}
            data-testid="ebu-card-{fila.canal}"
          >
            <header class="ebu-card__head">
              <span class="ebu-canal">{fila.canal.toUpperCase()}</span>
              <span class="ebu-spec">
                {fila.ok ? idiomaStore.t("audio.ebu.specOk") : idiomaStore.t("audio.ebu.specFail")}
              </span>
            </header>
            <dl class="ebu-metrics">
              <div>
                <dt>{idiomaStore.t("audio.ebu.truePeak")}</dt>
                <dd class="mono">{fila.tp}</dd>
              </div>
              <div>
                <dt>{idiomaStore.t("audio.ebu.lra")}</dt>
                <dd class="mono">{fila.lra}</dd>
              </div>
            </dl>
            {#if fila.silencio}
              <p class="ebu-aviso">{idiomaStore.t("audio.ebu.silencio")}</p>
            {/if}
            {#if fila.clip}
              <p class="ebu-aviso ebu-aviso--clip">{idiomaStore.t("audio.ebu.clipping")}</p>
            {/if}
            {#if fila.alertas.length}
              <ul class="ebu-alertas" data-testid="ebu-alertas-{fila.canal}">
                {#each fila.alertas as msg}
                  <li>{msg}</li>
                {/each}
              </ul>
            {/if}
          </article>
        {/each}
      </div>
    </section>
  {/if}

  <div class="wave wave--a" data-testid="waveform-strip-a">
    {#if formaOndaStore.escaneandoA}
      <p class="wave__estado">{idiomaStore.t("audio.escaneandoA")}</p>
    {:else}
      <WaveformCanvas
        picos={formaOndaStore.formaA?.picos ?? []}
        duracionSecs={formaOndaStore.formaA?.duracion_secs ?? 0}
        lufsBuckets={formaOndaStore.formaA?.lufs_buckets ?? []}
        color="var(--chan-a)"
        etiqueta="Canal A"
        testId="waveform-canvas-a"
      />
    {/if}
  </div>

  <div class="wave wave--b" data-testid="waveform-strip-b">
    {#if formaOndaStore.escaneandoB}
      <p class="wave__estado">{idiomaStore.t("audio.escaneandoB")}</p>
    {:else}
      <WaveformCanvas
        picos={formaOndaStore.formaB?.picos ?? []}
        duracionSecs={formaOndaStore.formaB?.duracion_secs ?? 0}
        lufsBuckets={formaOndaStore.formaB?.lufs_buckets ?? []}
        color="var(--chan-b)"
        etiqueta="Canal B"
        testId="waveform-canvas-b"
      />
    {/if}
  </div>

  <div class="wave wave--diff" data-testid="waveform-strip-diff">
    <WaveformCanvas
      picos={picosDiff}
      duracionSecs={duracionDiff}
      color="var(--accent)"
      etiqueta="Diff |A − B|"
      testId="waveform-canvas-diff"
      soloLectura={true}
    />
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

  .audio-ws__loudness {
    display: flex;
    flex-wrap: wrap;
    gap: 12px 24px;
    padding: 8px 12px;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
  }

  .loudness-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .loudness-label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
  }

  .loudness-val {
    font-size: 13px;
    color: var(--text-primary);
  }

  .loudness-item--a .loudness-val {
    color: var(--chan-a);
  }

  .loudness-item--b .loudness-val {
    color: var(--chan-b);
  }

  .audio-ws__ebu {
    padding: 8px 12px;
    background: var(--bg-panel);
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
  }

  .ebu-titulo {
    margin: 0 0 8px;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
    font-weight: 600;
  }

  .ebu-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 8px;
  }

  .ebu-card {
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    background: var(--bg-darkest);
  }

  .ebu-card--ok {
    border-left: 3px solid var(--ok, #4ade80);
  }

  .ebu-card--fail {
    border-left: 3px solid var(--warn, #f87171);
  }

  .ebu-card__head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 6px;
  }

  .ebu-canal {
    font-size: 11px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .ebu-spec {
    font-size: 10px;
    color: var(--text-muted);
  }

  .ebu-metrics {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4px 12px;
    margin: 0;
  }

  .ebu-metrics dt {
    font-size: 9px;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .ebu-metrics dd {
    margin: 0;
    font-size: 12px;
    color: var(--text-primary);
  }

  .ebu-aviso {
    margin: 6px 0 0;
    font-size: 11px;
    color: var(--warn, #fbbf24);
  }

  .ebu-aviso--clip {
    color: var(--warn, #f87171);
  }

  .ebu-alertas {
    margin: 6px 0 0;
    padding-left: 1.1rem;
    font-size: 11px;
    color: var(--text-secondary, var(--text-muted));
  }

  .wave {
    flex: 1;
    min-height: 80px;
    background: var(--bg-darkest);
    border: 1px solid var(--border);
    border-radius: var(--radius-max);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .wave--a {
    border-left: 3px solid var(--chan-a);
  }

  .wave--b {
    border-left: 3px solid var(--chan-b);
  }

  .wave--diff {
    border-left: 3px solid var(--accent);
  }

  .wave__estado {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 12px;
    margin: 0;
  }
</style>
