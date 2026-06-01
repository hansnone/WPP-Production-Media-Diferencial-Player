# M8 — Métricas objetivas de vídeo (A↔B)

## Qué calcula

| Métrica | Siempre | Notas |
|---------|---------|--------|
| SSIM | Sí | Global sobre luma Rec.709 |
| MS-SSIM | Sí | Tres escalas (×½) |
| MSE / PSNR | Sí | RGB |
| VMAF | Opcional | Requiere `ffmpeg` con filtro `libvmaf` |

## Flujo

1. Abrir vídeo **A** y **B**.
2. El motor lanza escaneo en hilo `metricas-video` (2 muestras/s).
3. Eventos: `metricas-progreso`, `metricas-lista`.
4. UI: panel derecho (Compare), heatmap sobre el slider, botones «Caída SSIM».

## VMAF

Comprobar en terminal:

```bash
ffmpeg -hide_banner -filters 2>&1 | grep libvmaf
```

Si aparece, tras el escaneo SSIM se ejecuta libvmaf sobre los archivos completos y se fusionan puntuaciones por frame.

## Export

- **CSV:** `pts,ssim,ms_ssim,psnr,mse,vmaf`
- **JSON:** serie completa (`SerieMetricasVideo`)

Comando IPC: `exportar_metricas_csv`.

## Código

- `src/metricas_video.rs` — núcleo
- `src/vmaf_ffmpeg.rs` — integración FFmpeg
- `frontend/src/lib/components/metricas/` — UI
