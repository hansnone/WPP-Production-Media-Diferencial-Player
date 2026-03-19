# Backlog de producto (post-base: estabilidad + rendimiento)

Funcionalidades del informe heredado que **no** forman parte del núcleo mínimo de la revisión técnica; conviene abordarlas cuando el pipeline y la suite básica estén consolidados.

## Métricas QC (PSNR / SSIM)

- **Qué**: medir diferencia objetiva entre frames A y B (o regiones).
- **Requisitos**: definición de espacio de color, alineación temporal exacta, ROI opcional, coste CPU/GPU.
- **UI**: panel o export numérico; umbrales configurables.

## Loop In / Out

- **Qué**: marcar puntos de entrada/salida y reproducir solo ese tramo en bucle.
- **Requisitos**: estado en `PlaybackState` / timeline, integración con seek y duración.
- **UI**: controles en barra temporal (`src/ui/timeline.rs` u homólogo).

## Export de informes (PDF / CSV)

- **Qué**: volcar resultados de sesión (metadatos, métricas si existen, capturas).
- **Requisitos**: requisitos legales/editoriales WPP, plantillas, idiomas.
- **Depende de**: métricas y/o logs estructurados.

## Notas

Cada ítem puede dividirse en issues separados (M–XL según alcance). Priorizar según negocio después de **CI**, **perfiles release** y mejoras de pipeline de color.
