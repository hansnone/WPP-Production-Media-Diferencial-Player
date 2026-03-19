# DiffPlayerQC — Arquitectura

Reproductor diferencial de vídeo para QC (WPP Production). Compara dos fuentes en tiempo real con modos cortina, diferencia absoluta, mapa de calor y lado a lado.

## Flujo de datos (alto nivel)

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  FFmpeg     │────▶│ VideoFrame   │────▶│ wgpu        │
│  (hilo)     │     │ RGBA + PTS   │     │ texturas A/B│
└─────────────┘     └──────────────┘     └──────┬──────┘
       │                                        │
       ▼                                        ▼
┌─────────────┐                         ┌──────────────┐
│ AudioFrame  │────▶ rodio::Sink       │ compare.wgsl │
└─────────────┘                         └──────────────┘
```

- **Decodificación**: un hilo por vídeo (`decoder.rs`) recibe `DecoderCommand` por canal y envía `VideoFrame` / `AudioFrame` por `crossbeam_channel`.
- **UI / reloj**: `DiffPlayerApp` en el hilo principal (`app`) calcula PTS maestro al reproducir, drena canales, actualiza `VideoRenderer`.
- **Render**: `renderer.rs` sube RGBA a texturas y pinta con un callback `egui_wgpu` + shader de comparación.
- **Proxy EXR**: `proxy.rs` invoca FFmpeg externo para generar `proxy.mkv` (FFV1); al terminar se abre como vídeo normal.

## Módulos principales

| Ruta | Rol |
|------|-----|
| `main.rs` | Entrada: logs, icono, `eframe::run_native`. |
| `app/` (`mod.rs`, `playback`, `proxy_bridge`) | Estado global, `eframe::App::update`, canvas, atajos diferidos (macOS). |
| `decoder.rs` | API C de FFmpeg: abrir flujo, seek, paso a paso, `sws_scale` YUV→RGBA. |
| `renderer.rs` | Pipeline wgpu, uniforms, texturas A/B. |
| `proxy.rs` | Lista concat EXR → vídeo proxy temporal. |
| `types.rs` | `VideoFrame`, `PlaybackState`, enums de UI. |
| `ui/` | Menús, panel info, timeline, tema. |
| `trace_log.rs` | Log legible por sesión. |

## Hilos y sincronización

- **Nunca** bloquear el hilo de UI con decode pesado; las acciones de teclado sensibles usan flags `pending_*` procesados al inicio de `update()`.
- Repintado acortado con audio activo para alimentar `rodio` y evitar cortes.

## Documentación adicional

- [CONTRIBUTING.md](CONTRIBUTING.md) — comentarios y estilo.
- [BUILD_PROFILES.md](BUILD_PROFILES.md) — perfiles `release` vs `release-fast`.
- [DEPENDENCIES_MIGRATION.md](DEPENDENCIES_MIGRATION.md) — actualización eframe/wgpu.
- [HARDWARE_DECODE.md](HARDWARE_DECODE.md) — decode por hardware (FFmpeg).
- [GPU_YUV_PIPELINE.md](GPU_YUV_PIPELINE.md) — pipeline de color CPU/GPU.
- [BACKLOG.md](BACKLOG.md) — métricas QC, loop In/Out, informes.
