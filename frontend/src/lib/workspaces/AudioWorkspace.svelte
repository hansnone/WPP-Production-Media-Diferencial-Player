<script lang="ts">
  import WaveformCanvas from "../components/WaveformCanvas.svelte";
  import {
    calcularPicosDiff,
    formatearLufs,
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

  <div class="wave wave--a" data-testid="waveform-strip-a">
    {#if formaOndaStore.escaneandoA}
      <p class="wave__estado">{idiomaStore.t("audio.escaneandoA")}</p>
    {:else}
      <WaveformCanvas
        picos={formaOndaStore.formaA?.picos ?? []}
        duracionSecs={formaOndaStore.formaA?.duracion_secs ?? 0}
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
