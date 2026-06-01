# M12 — ProRes / DNxHR y color profesional

## Qué cubre

| Aspecto | Implementación |
|---------|----------------|
| ProRes / DNxHD / DNxHR | Detección por `AVCodecID`; decode **software** (10/12 bit vía FFmpeg) |
| Alto bit depth | `sws_scale` desde `yuv422p10le` etc. → RGBA 8-bit para wgpu |
| Rec.709 / Rec.2020 | `sws_setColorspaceDetails` según `color_primaries` |
| Legal vs full | Según `color_range` del stream (`mpeg` / `jpeg`) |
| HW decode (M11) | **Desactivado** en códecs profesionales (evita forzar NV12 y perder 10-bit) |

## Metadatos

`ColorMetadata` incluye `color_range` además de `color_primaries` y `colorspace`.

## Limitaciones

- Sin shader YUV 10-bit nativo en GPU (futuro: pipeline YUV en wgpu).
- Perfiles ProRes exóticos dependen del FFmpeg del sistema.

## Tests

```bash
cargo test -p diffplayerqc --no-default-features formatos_pro
```
