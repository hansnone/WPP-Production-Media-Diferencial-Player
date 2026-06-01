# M9 — Loudness EBU R128 (audio QC)

## Qué calcula

| Métrica | Siempre | Notas |
|---------|---------|--------|
| LUFS integrado | Sí | K-weighting BS.1770 @ 48 kHz + gate relativo/absoluto |
| True peak (dBTP) | Sí | Oversampling lineal simple entre muestras |
| LRA | Sí | Rango loudness (short-term 3 s vs integrado) |
| LUFS por bucket | Sí | Overlay en waveform (franja inferior) |
| Silencio / clipping | Sí | Alertas en `ebu.alertas` |

Objetivo broadcast: **-23 LUFS** integrado (±1 LU), true peak **≤ -1 dBTP**.

## Flujo

1. Abrir vídeo **A** y/o **B** (pista de audio).
2. El motor escanea en hilo `forma-onda-{a|b}` (mismo escaneo que M4).
3. Evento: `forma-onda-lista` con `lufs_integrado`, `lufs_buckets`, `ebu`.
4. UI: workspace **Audio** — cabecera LUFS, panel EBU, overlay en canvas.

## Implementación

- `src/analisis_loudness.rs` — K-weighting, bloques 400 ms, gate, alertas
- `src/forma_onda.rs` — alimenta `AnalizadorLoudness` durante el decode FFmpeg
- Sin dependencia de `libebur128` del sistema (Rust puro)

## Tests

```bash
cargo test -p diffplayerqc loudness
```

## Limitaciones conocidas

- True peak sin interpolación cúbica completa (conservador para QC rápido).
- Una sola pista mono derivada del decode (estéreo → mezcla en resampler).
