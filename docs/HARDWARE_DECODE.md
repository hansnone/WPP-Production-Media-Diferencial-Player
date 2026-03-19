# Decodificación por hardware (FFmpeg) — fase XL

## Objetivo

Reducir carga de CPU usando **hwaccel** del códec de vídeo (VideoToolbox en macOS, NVDEC en NVIDIA Windows/Linux, D3D11VA, VAAPI en Linux, etc.).

## Complejidad

1. **API C de FFmpeg**: abrir el decoder con `hw_device_ctx` / buffers de hardware según plataforma.
2. **Formato de salida**: muchos hw decoders entregan frames en **memoria de GPU** o formatos NV12/P010; el pipeline actual asume **YUV en CPU → `sws_scale` → RGBA → textura wgpu**. Haría falta:
   - **descarga** a CPU (lento), o
   - **integración directa** con texturas/import (muy dependiente de API nativa), o
   - **YUV en GPU** (alineado con `GPU_YUV_PIPELINE.md`).
3. **Pruebas**: cada SO y cada familia de GPU necesita validación; fallos silenciosos o fallback a software son habituales.

## Orden sugerido en el roadmap

1. Estabilizar **tests + CI** y, si aplica, **YUV→RGB en GPU** para no duplicar trabajo.
2. Prototipo **por una plataforma** (p. ej. VideoToolbox en macOS) con fallback automático a decode por software.
3. Documentar flags y requisitos en README / scripts de empaquetado.

## Proxy EXR

`proxy.rs` invoca el binario **`ffmpeg`** del sistema: se puede documentar el uso opcional de `-hwaccel` en esa línea de comandos **solo** si el FFmpeg del usuario está compilado con soporte y el códec de entrada lo permite; no sustituye el decode integrado del player.

## Referencias

- Documentación FFmpeg: *Hardware Acceleration API*.
- Tablas de compatibilidad por SO/GPU en la wiki de FFmpeg.
