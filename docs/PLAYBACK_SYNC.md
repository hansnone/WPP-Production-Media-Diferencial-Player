# Sincronización y rendimiento de reproducción (v2)

## Clips de prueba (`videos-muestra/`)

| Propiedad | Valor |
|-----------|-------|
| Codec | H.264 |
| Resolución fuente | 1920×1080 |
| FPS | 25 (40 ms/frame) |
| Duración | 30 s |

## Cuello de botella real (no el reloj)

A 1080p RGBA × 2 canales compare ≈ **16 MB/frame** + swscale CPU. Los ajustes de timing solos no bastan.

## Optimizaciones aplicadas

1. **Escala en decode (Tauri):** 1920→**1280** px en `swscale` (~56 % menos píxeles y upload).
2. **`SWS_FAST_BILINEAR`:** swscale más rápido en reproducción.
3. **Bind group GPU:** ya **no** se recrea en cada frame (bug grave: 25×/s alloc wgpu).
4. **Upload A+B + 1 present** por tick (`SubirYPresentar`).
5. **Decode ahead** + drop-late de frames obsoletos si el CPU va justo.

## Reloj (v1 / VLC)

- Reloj wall maestro (`PlaybackState::pts_at`).
- Selección: frame con `pts ≤ reloj`.
- Tick: `next_frame_repaint_delay(fps, pts)`.

## Si aún hay tirones

Siguiente hito obligatorio: **pipeline YUV/NV12 en shader** (`docs/GPU_YUV_PIPELINE.md`) — evita RGBA en CPU/GPU por completo.
