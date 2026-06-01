# M11 — Decodificación acelerada por hardware

## Qué hace

Al abrir un vídeo, el decoder intenta **FFmpeg hwaccel** según el SO:

| SO | Orden de prueba |
|----|-----------------|
| macOS | VideoToolbox |
| Windows | D3D11VA → DXVA2 → CUDA |
| Linux | VAAPI → CUDA |

Si falla `avcodec_open2` o no hay dispositivo, **fallback automático a software** (multihilo FFmpeg como antes).

## Pipeline

1. Frame decodificado en superficie HW.
2. `av_hwframe_transfer_data` → NV12 en CPU (frame reutilizable).
3. `sws_scale` → RGBA → textura wgpu (sin cambio respecto a M0–M10).

La optimización futura (YUV directo en GPU) queda en `docs/HARDWARE_DECODE.md`.

## Cómo comprobarlo

1. `cargo tauri dev` y abre un H.264/HEVC típico.
2. Panel **Fuentes**: etiqueta bajo A/B, p. ej. `hw:videotoolbox` o `software`.
3. Logs: `Decode HW activo: …` o `fallback` en terminal.

Desactivar HW:

```bash
DIFFPLAYERQC_HW_DECODE=0 cargo tauri dev
```

## Código

- `src/decode_hw.rs` — inicialización y transferencia
- `src/decoder.rs` — integración en `open_decoder` / `convert_frame`

## Requisitos

FFmpeg del sistema compilado con el hwaccel correspondiente (`ffmpeg -hwaccels`).
