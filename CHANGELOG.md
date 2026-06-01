# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es/ES/1.0.0/).

## [Unreleased]

### Añadido

- **M8:** métricas objetivas A↔B — `metricas_video.rs`, `vmaf_ffmpeg.rs` (SSIM, MS-SSIM, MSE, PSNR; VMAF opcional); heatmap en timeline; saltos entre caídas SSIM; panel Compare.
- **M9:** loudness EBU R128 — `analisis_loudness.rs`, integración en `forma_onda.rs`; true peak, LRA, alertas; overlay LUFS en waveforms; panel EBU en workspace Audio.
- **M10:** eventos QC y notas — `eventos_qc` en core, persistencia JSON, workspace Report, panel Compare, timeline de marcadores, IPC seek/listar/crear.
- **M11:** decode hardware FFmpeg — `decode_hw.rs`, fallback software, etiquetas `decode_a`/`decode_b` en UI.
- **M12:** ProRes/DNxHR — `formatos_pro.rs`, alto bit depth vía swscale, rango legal/full y Rec.709/2020.

### Planificado

- Roadmap **M13–M16** en [`docs/ROADMAP_M8_M16.md`](docs/ROADMAP_M8_M16.md).

## [2.0.0] — 2026-05-29

### Añadido

- Shell **Tauri 2 + Svelte 5**: workspaces Compare, Inspect, Audio, Report, Export.
- Reproducción A/B con overlay **wgpu**, modos compare y sincronización de viewport.
- Workspace **Audio**: waveforms, diff, LUFS estimado, escaneo offline FFmpeg.
- Workspace **Inspect**: histograma RGB, vectoscopio, monitor de luma.
- **Persistencia**: layout por workspace, idioma Es/En, archivos recientes.
- Audio por altavoz (rodio) con mute A/B en toolbar.
- CI: tests, cobertura `core`, Playwright E2E.
- Workflow **Release** en GitHub al etiquetar `v*`.

### Cambiado

- Migración desde egui v1; binario legacy sigue en rama `egui-legacy` con feature `egui-app`.

### Técnico

- Crate `diffplayerqc` sin feature `egui-app` para builds Tauri (bundle más pequeño).
- Perfil `release-small` para empaquetado.

## [1.3.0] — egui (legacy)

Versión anterior con interfaz egui/wgpu monolítica. Ver rama `egui-legacy`.
