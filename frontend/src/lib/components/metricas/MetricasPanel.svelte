<script lang="ts">
  import { metricasStore } from "../../stores/metricas.svelte";
  import { playerStore } from "../../stores/player.svelte";
  import { idiomaStore } from "../../i18n/idioma.svelte";
  import { formatearPts } from "../../player";

  const serie = $derived(metricasStore.serie);
  const vacio = $derived(!serie || serie.puntos.length === 0);

  let canvas: HTMLCanvasElement | undefined = $state();

  const ssimActual = $derived(playerStore.snap?.ssim_actual);
  const ptsActual = $derived(playerStore.snap?.pts_actual ?? 0);

  const tieneVmaf = $derived(
    Boolean(serie?.puntos.some((p) => p.vmaf !== undefined && p.vmaf !== null)),
  );

  function repintarGrafico() {
    if (!canvas || vacio || !serie) return;
    const puntos = serie.puntos;
    const cw = canvas.clientWidth;
    const ch = canvas.clientHeight;
    if (cw < 8 || ch < 8) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(cw * dpr);
    canvas.height = Math.round(ch * dpr);
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.fillStyle = "#0a0a0a";
    ctx.fillRect(0, 0, cw, ch);

    const pad = { l: 36, r: 8, t: 8, b: 20 };
    const gw = cw - pad.l - pad.r;
    const gh = ch - pad.t - pad.b;
    const maxPts = serie.duracion_secs || puntos[puntos.length - 1]?.pts || 1;

    const yUmbral = pad.t + gh * (1 - serie.umbral_ssim_bajo);
    ctx.strokeStyle = "rgba(255, 180, 0, 0.6)";
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.moveTo(pad.l, yUmbral);
    ctx.lineTo(pad.l + gw, yUmbral);
    ctx.stroke();
    ctx.setLineDash([]);

    // SSIM (verde)
    ctx.strokeStyle = "#4ade80";
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    for (let i = 0; i < puntos.length; i++) {
      const p = puntos[i]!;
      const x = pad.l + (p.pts / maxPts) * gw;
      const y = pad.t + (1 - p.ssim) * gh;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();

    // MS-SSIM (cian, atenuado)
    ctx.strokeStyle = "rgba(34, 211, 238, 0.7)";
    ctx.lineWidth = 1;
    ctx.beginPath();
    for (let i = 0; i < puntos.length; i++) {
      const p = puntos[i]!;
      const x = pad.l + (p.pts / maxPts) * gw;
      const y = pad.t + (1 - p.ms_ssim) * gh;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();

    // VMAF 0–100 mapeado a 0–1 en el mismo eje
    if (tieneVmaf) {
      ctx.strokeStyle = "rgba(167, 139, 250, 0.85)";
      ctx.lineWidth = 1;
      ctx.beginPath();
      let iniciado = false;
      for (const p of puntos) {
        if (p.vmaf === undefined || p.vmaf === null) continue;
        const x = pad.l + (p.pts / maxPts) * gw;
        const y = pad.t + (1 - p.vmaf / 100) * gh;
        if (!iniciado) {
          ctx.moveTo(x, y);
          iniciado = true;
        } else ctx.lineTo(x, y);
      }
      ctx.stroke();
    }

    ctx.fillStyle = "#f87171";
    for (const p of puntos) {
      if (p.ssim >= serie.umbral_ssim_bajo) continue;
      const x = pad.l + (p.pts / maxPts) * gw;
      const y = pad.t + (1 - p.ssim) * gh;
      ctx.beginPath();
      ctx.arc(x, y, 3, 0, Math.PI * 2);
      ctx.fill();
    }

    ctx.strokeStyle = "#444";
    ctx.lineWidth = 1;
    ctx.strokeRect(pad.l, pad.t, gw, gh);
    ctx.fillStyle = "#888";
    ctx.font = "10px var(--font-mono)";
    ctx.fillText("1.0", 4, pad.t + 4);
    ctx.fillText("0", 12, pad.t + gh);
    ctx.fillText("0", pad.l, ch - 4);
    ctx.fillText(formatearPts(maxPts), pad.l + gw - 40, ch - 4);
  }

  $effect(() => {
    void serie;
    void metricasStore.progreso;
    void tieneVmaf;
    requestAnimationFrame(repintarGrafico);
  });
</script>

<div class="metricas-panel" data-testid="metricas-panel">
  <h3 class="metricas-panel__titulo">{idiomaStore.t("metricas.titulo")}</h3>

  {#if metricasStore.escaneando || playerStore.snap?.escaneando_metricas}
    <p class="metricas-panel__estado">
      {idiomaStore.t("metricas.escaneando")}
      {Math.round(metricasStore.progreso * 100)}%
    </p>
  {:else if vacio}
    <p class="metricas-panel__estado">{idiomaStore.t("metricas.vacio")}</p>
  {:else if serie}
    <p class="metricas-panel__meta mono">
      SSIM · {serie.puntos.length} {idiomaStore.t("metricas.muestras")} ·
      {serie.puntos.filter((p) => p.ssim < serie.umbral_ssim_bajo).length}
      {idiomaStore.t("metricas.caidas")}
      {#if serie.vmaf_integrado !== undefined && serie.vmaf_integrado !== null}
        · VMAF Ø {serie.vmaf_integrado.toFixed(1)}
      {:else if !serie.vmaf_disponible_en_sistema}
        · {idiomaStore.t("metricas.sinVmaf")}
      {/if}
    </p>
  {/if}

  {#if ssimActual !== undefined && ssimActual !== null}
    <p class="metricas-panel__live mono">
      {idiomaStore.t("metricas.actual")}: SSIM {(ssimActual * 100).toFixed(1)}%
    </p>
  {/if}

  <canvas
    class="metricas-panel__grafico"
    data-testid="metricas-grafico"
    bind:this={canvas}
    aria-label={idiomaStore.t("metricas.grafico")}
  ></canvas>

  {#if !vacio}
    <div class="metricas-panel__nav">
      <button
        type="button"
        class="btn-nav"
        onclick={() => void metricasStore.anteriorCaida(ptsActual)}
      >
        {idiomaStore.t("metricas.caidaAnterior")}
      </button>
      <button
        type="button"
        class="btn-nav"
        onclick={() => void metricasStore.siguienteCaida(ptsActual)}
      >
        {idiomaStore.t("metricas.caidaSiguiente")}
      </button>
    </div>
  {/if}

  <div class="metricas-panel__acciones">
    <button
      type="button"
      class="btn-export"
      disabled={vacio}
      onclick={() => void metricasStore.descargarCsv()}
    >
      CSV
    </button>
    <button
      type="button"
      class="btn-export"
      disabled={vacio}
      onclick={() => void metricasStore.descargarJson()}
    >
      JSON
    </button>
  </div>
</div>

<style>
  .metricas-panel {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--border);
  }
  .metricas-panel__titulo {
    margin: 0;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: var(--letter-label);
    color: var(--text-muted);
  }
  .metricas-panel__estado {
    margin: 0;
    font-size: 11px;
    color: var(--text-muted);
  }
  .metricas-panel__meta,
  .metricas-panel__live {
    margin: 0;
    font-size: 11px;
    color: var(--text-secondary);
  }
  .metricas-panel__grafico {
    width: 100%;
    height: 140px;
    background: #000;
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }
  .metricas-panel__nav {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }
  .btn-nav {
    font-size: 10px;
    padding: 4px 8px;
    background: var(--bg-app);
    border: 1px solid var(--border);
    color: var(--text-primary);
    border-radius: var(--radius);
    cursor: pointer;
  }
  .btn-nav:hover {
    border-color: var(--accent-primary);
  }
  .metricas-panel__acciones {
    display: flex;
    gap: 8px;
  }
  .btn-export {
    font-size: 11px;
    padding: 4px 10px;
    background: var(--bg-app);
    border: 1px solid var(--border);
    color: var(--text-primary);
    border-radius: var(--radius);
    cursor: pointer;
  }
  .btn-export:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  .btn-export:not(:disabled):hover {
    border-color: var(--accent-primary);
  }
</style>
