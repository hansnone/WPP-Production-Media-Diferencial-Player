<script lang="ts">
  import { onMount } from "svelte";
  import {
    alternarPlay,
    abrirDialogo,
    escucharTicks,
    formatearPts,
    obtenerEstado,
    seek,
    stepAdelante,
    stepAtras,
    type SnapshotReproduccion,
  } from "./lib/player";

  let snap = $state<SnapshotReproduccion | null>(null);
  let seekInput = $state(0);
  let enTauri = $state(false);

  onMount(() => {
    const esTauri = "__TAURI_INTERNALS__" in window;
    enTauri = esTauri;
    if (!esTauri) return;

    obtenerEstado().then((s) => {
      snap = s;
      seekInput = s.pts_actual;
    });

    const unlisten = escucharTicks((s) => {
      snap = s;
      if (!Number.isNaN(seekInput)) {
        seekInput = s.pts_actual;
      }
    });

    const onKey = (e: KeyboardEvent) => {
      if (e.code === "Space") {
        e.preventDefault();
        alternarPlay().then((s) => (snap = s));
      }
      if (e.code === "ArrowRight") {
        stepAdelante().then((s) => (snap = s));
      }
      if (e.code === "ArrowLeft") {
        stepAtras().then((s) => (snap = s));
      }
    };
    window.addEventListener("keydown", onKey);

    return () => {
      unlisten.then((fn) => fn());
      window.removeEventListener("keydown", onKey);
    };
  });

  async function abrir(canal: "a" | "b") {
    const s = await abrirDialogo(canal);
    if (s) snap = s;
  }

  async function aplicarSeek() {
    snap = await seek(seekInput);
  }

  const duracionMax = $derived(
    Math.max(snap?.duracion_a ?? 0, snap?.duracion_b ?? 0),
  );
</script>

<main class="shell">
  <header class="barra">
    <h1>DiffPlayerQC v2</h1>
    <span class="badge">M1 — Playback</span>
  </header>

  {#if !enTauri}
    <p class="aviso">
      Ejecuta desde la raíz del repo con <code>cargo tauri dev</code> o <code>pnpm tauri:dev</code> para
      abrir vídeos y escuchar ticks.
    </p>
  {/if}

  <section class="controles">
    <button type="button" onclick={() => abrir("a")}>Abrir A</button>
    <button type="button" onclick={() => abrir("b")}>Abrir B</button>
    <button type="button" onclick={() => alternarPlay().then((s) => (snap = s))}>
      {snap?.reproduciendo ? "Pausa" : "Play"}
    </button>
    <button type="button" onclick={() => stepAtras().then((s) => (snap = s))}>◀ Frame</button>
    <button type="button" onclick={() => stepAdelante().then((s) => (snap = s))}>Frame ▶</button>
    <label class="seek">
      PTS
      <input type="range" min="0" max={duracionMax || 1} step="0.001" bind:value={seekInput} />
      <input type="number" step="0.001" bind:value={seekInput} class="num" />
      <button type="button" onclick={aplicarSeek}>Seek</button>
    </label>
  </section>

  <section class="visor">
  <!-- Placeholder negro M1/M3: el canvas wgpu se montará aquí -->
    <div id="canvas-slot" class="canvas-slot" aria-label="Área de vídeo (placeholder)"></div>
    <aside class="meta">
      {#if snap}
        <p><strong>PTS:</strong> {formatearPts(snap.pts_actual)} / {formatearPts(duracionMax)}</p>
        <p><strong>FPS:</strong> {snap.fps.toFixed(2)}</p>
        <p><strong>A:</strong> {snap.ruta_a ?? "—"}</p>
        <p><strong>B:</strong> {snap.ruta_b ?? "—"}</p>
        <p>
          <strong>Audio:</strong>
          A {(snap.nivel_audio_a * 100).toFixed(0)}% · B {(snap.nivel_audio_b * 100).toFixed(0)}%
        </p>
      {:else}
        <p>Carga vídeo A y/o B para empezar.</p>
      {/if}
    </aside>
  </section>
</main>

<style>
  .shell {
    font-family: system-ui, sans-serif;
    background: #0f1115;
    color: #e8eaed;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }
  .barra {
    display: flex;
    align-items: baseline;
    gap: 1rem;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid #2a2f38;
  }
  h1 {
    font-size: 1.1rem;
    margin: 0;
  }
  .badge {
    font-size: 0.75rem;
    color: #8b929a;
  }
  .aviso {
    padding: 1rem;
    color: #e8c547;
  }
  .controles {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    align-items: center;
  }
  button {
    background: #2a8fe8;
    color: #fff;
    border: none;
    padding: 0.4rem 0.75rem;
    border-radius: 4px;
    cursor: pointer;
  }
  button:hover {
    background: #4fa8f0;
  }
  .seek {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    min-width: 200px;
  }
  .seek input[type="range"] {
    flex: 1;
  }
  .num {
    width: 5rem;
    background: #1a1d24;
    color: inherit;
    border: 1px solid #2a2f38;
  }
  .visor {
    flex: 1;
    display: grid;
    grid-template-columns: 1fr 280px;
    gap: 0;
    min-height: 0;
    padding: 0 1rem 1rem;
  }
  .canvas-slot {
    background: #000;
    min-height: 360px;
    border: 1px solid #2a2f38;
    border-radius: 4px;
  }
  .meta {
    padding: 0.75rem;
    font-size: 0.8rem;
    color: #8b929a;
    overflow-wrap: anywhere;
  }
  .meta p {
    margin: 0.35rem 0;
  }
</style>
