# Decodificación por hardware — estado y evolución

## Implementado (M11)

- Detección por SO y fallback a software: ver [`HW_DECODE_M11.md`](HW_DECODE_M11.md).
- Variable de entorno `DIFFPLAYERQC_HW_DECODE=0` para forzar software.
- Etiqueta en UI (`decode_a` / `decode_b` en snapshot) y logs.

## Pendiente (post-M11)

Integración **directa** HW → wgpu sin pasar por RGBA en CPU:

1. Texturas NV12/P010 en GPU (import o copy).
2. Shader YUV→RGB en `diffplayerqc-render` (ver `GPU_YUV_PIPELINE.md` si existe).
3. Menos copias en 4K / ProRes (M12).

## Proxy EXR

`proxy.rs` puede usar `ffmpeg -hwaccel` en línea de comandos si el binario del sistema lo soporta; es independiente del decode integrado del player.

## Referencias

- [FFmpeg Hardware Acceleration](https://ffmpeg.org/doxygen/trunk/group__lavc__hwaccel.html)
