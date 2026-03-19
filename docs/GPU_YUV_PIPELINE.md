# Pipeline de color: YUV → RGB (CPU hoy, GPU posible)

## Estado actual

1. **Decoder** (`src/decoder.rs`): frames YUV del códec → **`sws_scale`** → **RGBA** empaquetado en `VideoFrame.rgba_data`.
2. **Renderer** (`src/renderer.rs`): sube RGBA a **textura 2D** y el shader `compare.wgsl` solo compara texturas ya en RGB.

## Optimizaciones ya aplicadas en CPU

- **`SWS_FAST_BILINEAR`** en `sws_getContext` (más barato que bilineal estándar para tiempo real).
- **Búfer RGBA reutilizable** (`rgba_scratch`) para evitar `av_frame_alloc` / `av_frame_free` por fotograma.

## Dirección futura (alto impacto)

Subir **planos Y + UV** (o NV12) como **dos o tres texturas** (o una textura con layout acordado) y realizar **YUV→RGB en el fragment shader** (BT.601 / BT.709 según metadatos). Ventajas:

- Menos trabajo en CPU y menos ancho de banda de subida si se evita RGBA completo.
- Encaja con futuro **hw decode** si los frames llegan como NV12.

### Pasos de implementación (resumen)

1. Extender `VideoFrame` (o tipo paralelo) con datos Y/UV y formato de croma.
2. Añadir rutas de upload en `VideoTexture` / `wgpu::Texture` (formatos `R8Uint` / `Rg8Uint` o similares según backend).
3. Nuevo WGSL o ramas en `compare.wgsl` para muestrear YUV y convertir a linear/RGB antes del diff.
4. Mantener **fallback RGBA** para rutas sin metadatos o códecs raros.

Hasta que exista esa rama, el documento **`HARDWARE_DECODE.md`** aplica principalmente a investigación; el cuello de botella típico sigue siendo **swscale + upload RGBA**.
