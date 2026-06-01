# Changelog

Formato basado en [Keep a Changelog](https://keepachangelog.com/es/ES/1.0.0/).

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
