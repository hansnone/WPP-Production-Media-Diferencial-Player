This file is a merged representation of the entire codebase, combined into a single document by Repomix.

<file_summary>
This section contains a summary of this file.

<purpose>
This file contains a packed representation of the entire repository's contents.
It is designed to be easily consumable by AI systems for analysis, code review,
or other automated processes.
</purpose>

<file_format>
The content is organized as follows:
1. This summary section
2. Repository information
3. Directory structure
4. Repository files (if enabled)
5. Multiple file entries, each consisting of:
  - File path as an attribute
  - Full contents of the file
</file_format>

<usage_guidelines>
- This file should be treated as read-only. Any changes should be made to the
  original repository files, not this packed version.
- When processing this file, use the file path to distinguish
  between different files in the repository.
- Be aware that this file may contain sensitive information. Handle it with
  the same level of security as you would the original repository.
</usage_guidelines>

<notes>
- Some files may have been excluded based on .gitignore rules and Repomix's configuration
- Binary files are not included in this packed representation. Please refer to the Repository Structure section for a complete list of file paths, including binary files
- Files matching patterns in .gitignore are excluded
- Files matching default ignore patterns are excluded
- Files are sorted by Git change count (files with more changes are at the bottom)
</notes>

</file_summary>

<directory_structure>
.cargo/
  config.toml
.github/
  workflows/
    ci.yml
assets/
  youlean_settings/
    App Settings.ini
    License
    License Info
    Settings
    Settings-Global
  AppIcon.icns
  Icon-iOS-Default-1024x1024@1x.png
  icon.ico
  Youlean-Loudness-Meter-2-V2.5.14-macOS-1.dmg
docs/
  ARQUITECTURA.md
  BACKLOG.md
  BUILD_PROFILES.md
  CONTRIBUTING.md
  DEPENDENCIES_MIGRATION.md
  GPU_YUV_PIPELINE.md
  HARDWARE_DECODE.md
patches/
  ffmpeg-next/
    .github/
      ISSUE_TEMPLATE/
        bug.md
        config.yml
        feature.md
      workflows/
        build.yml
        release.yml
    examples/
      chapters.rs
      codec-info.rs
      dump-frames.rs
      metadata.rs
      remux.rs
      transcode-audio.rs
      transcode-x264.rs
    src/
      codec/
        decoder/
          audio.rs
          check.rs
          conceal.rs
          decoder.rs
          mod.rs
          opened.rs
          slice.rs
          subtitle.rs
          video.rs
        encoder/
          audio.rs
          comparison.rs
          decision.rs
          encoder.rs
          mod.rs
          motion_estimation.rs
          prediction.rs
          subtitle.rs
          video.rs
        packet/
          borrow.rs
          flag.rs
          mod.rs
          packet.rs
          side_data.rs
          traits.rs
        subtitle/
          flag.rs
          mod.rs
          rect_mut.rs
          rect.rs
        audio_service.rs
        audio.rs
        capabilities.rs
        codec.rs
        compliance.rs
        context.rs
        debug.rs
        discard.rs
        field_order.rs
        flag.rs
        id.rs
        mod.rs
        parameters.rs
        picture.rs
        profile.rs
        threading.rs
        traits.rs
        video.rs
      device/
        extensions.rs
        input.rs
        mod.rs
        output.rs
      filter/
        context/
          context.rs
          mod.rs
          sink.rs
          source.rs
        filter.rs
        flag.rs
        graph.rs
        mod.rs
        pad.rs
      format/
        chapter/
          chapter_mut.rs
          chapter.rs
          mod.rs
        context/
          common.rs
          destructor.rs
          input.rs
          mod.rs
          output.rs
        format/
          flag.rs
          input.rs
          iter.rs
          mod.rs
          output.rs
        stream/
          disposition.rs
          mod.rs
          stream_mut.rs
          stream.rs
        mod.rs
        network.rs
      software/
        resampling/
          context.rs
          delay.rs
          dither.rs
          engine.rs
          extensions.rs
          filter.rs
          flag.rs
          mod.rs
        scaling/
          color_space.rs
          context.rs
          extensions.rs
          filter.rs
          flag.rs
          mod.rs
          support.rs
          vector.rs
        mod.rs
      util/
        chroma/
          location.rs
          mod.rs
        color/
          mod.rs
          primaries.rs
          range.rs
          space.rs
          transfer_characteristic.rs
        dictionary/
          immutable.rs
          iter.rs
          mod.rs
          mutable.rs
          owned.rs
        format/
          mod.rs
          pixel.rs
          sample.rs
        frame/
          audio.rs
          flag.rs
          mod.rs
          side_data.rs
          video.rs
        log/
          flag.rs
          level.rs
          mod.rs
        mathematics/
          mod.rs
          rescale.rs
          rounding.rs
        option/
          mod.rs
          traits.rs
        channel_layout.rs
        error.rs
        interrupt.rs
        media.rs
        mod.rs
        picture.rs
        range.rs
        rational.rs
        rounding.rs
        time.rs
      lib.rs
    .cargo_vcs_info.json
    .cargo-ok
    .gitignore
    build.rs
    Cargo.toml
    CHANGELOG.md
    LICENSE
    README.md
shaders/
  compare.wgsl
src/
  app/
    mod.rs
    playback.rs
    proxy_bridge.rs
  ui/
    controls.rs
    design.rs
    i18n.rs
    info_panel.rs
    markers.rs
    mod.rs
    theme.rs
    timeline.rs
    vu_meter.rs
  decoder.rs
  error.rs
  main.rs
  metrics.rs
  proxy.rs
  renderer.rs
  trace_log.rs
  types.rs
.gitignore
build.ps1
build.sh
Cargo.toml
CHANGELOG.md
features.md
generar_repomix.sh
LICENSE
PLANTILLAS.txt
README.md
</directory_structure>

<files>
This section contains the contents of the repository's files.

<file path="src/ui/markers.rs">
use crate::app::DiffPlayerApp;
use crate::ui::design::{tr, FONT_MONO_SMALL};
use egui::{ScrollArea, Window};

pub fn show(ctx: &egui::Context, app: &mut DiffPlayerApp) {
    let mut show_panel = true; // We might want a toggle in view state for this
                               // Let's assume there's a view state toggle `show_markers_panel`.
    if !app.view().show_hud {
        // Defaulting to hide with HUD for now, but usually it's its own window
        return;
    }

    Window::new(tr(app.view().lang, "Marcadores", "Markers", "Markers"))
        .open(&mut show_panel)
        .resizable(true)
        .default_width(300.0)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let mut markers_to_remove = Vec::new();
                let mut seek_to = None;
                let fps = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
                for (idx, marker) in app.session.markers.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("▶").on_hover_text("Ir a marcador").clicked() {
                                seek_to = Some(marker.pts);
                            }

                            let tc = format_timecode(marker.pts, fps);
                            ui.label(
                                egui::RichText::new(tc)
                                    .font(egui::FontId::monospace(FONT_MONO_SMALL)),
                            );

                            if ui.button("🗑").on_hover_text("Eliminar").clicked() {
                                markers_to_remove.push(idx);
                            }
                        });
                        ui.text_edit_multiline(&mut marker.note);
                    });
                }

                for idx in markers_to_remove.into_iter().rev() {
                    app.session.markers.remove(idx);
                }

                if let Some(pts) = seek_to {
                    app.do_seek(pts, ctx);
                }
            });
        });
}

pub fn format_timecode(secs: f64, fps: f64) -> String {
    let total = secs.max(0.0) as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    let fps_val = fps.max(1.0);
    let f = ((secs.fract()) * fps_val).round() as u64 % (fps_val.round() as u64).max(1);
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
</file>

<file path="src/error.rs">
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("No se encontró el binario de FFmpeg. La generación de proxy EXR requiere la herramienta de línea de comandos ffmpeg. Instala FFmpeg o configúralo en el PATH.")]
    FfmpegNotFound,

    #[error("El comando FFmpeg falló (status: {status:?}): {stderr}")]
    FfmpegCommandFailed { status: Option<i32>, stderr: String },

    #[error("Plataforma no soportada para la característica: {feature}")]
    UnsupportedPlatform { feature: String },

    #[error("Error de decode: {0}")]
    Decode(String),

    #[error("Error de audio: {0}")]
    Audio(String),

    #[error("Error de renderer: {0}")]
    Renderer(String),

    #[error("Error de I/O: {0}")]
    Io(#[from] std::io::Error),
}
</file>

<file path="src/metrics.rs">
pub fn compute_psnr(frame_a: &[u8], frame_b: &[u8]) -> Option<f64> {
    if frame_a.len() != frame_b.len() || frame_a.is_empty() {
        return None;
    }

    let mut mse_sum = 0.0;
    // Assuming RGBA (4 channels), computing MSE across all channels or RGB only?
    // Let's compute across all channels for simplicity, or just R, G, B.
    // If it's RGBA, we can skip alpha if we want, but doing all 4 is standard for raw buffers
    // unless specified otherwise. Let's do all 4 channels to be fast and simple.

    // Process in chunks of 4 (RGBA)
    let len = frame_a.len();
    for i in 0..len {
        let diff = frame_a[i] as f64 - frame_b[i] as f64;
        mse_sum += diff * diff;
    }

    let mse = mse_sum / len as f64;

    if mse == 0.0 {
        return Some(f64::INFINITY); // Perfect match
    }

    let max_i2 = 255.0 * 255.0;
    let psnr = 10.0 * (max_i2 / mse).log10();

    Some(psnr)
}
</file>

<file path="generar_repomix.sh">
#!/bin/bash
# Este script genera un archivo markdown con todo el código fuente del proyecto
# ignorando automáticamente los archivos en .gitignore (como la carpeta target/)

echo "Generando snapshot del código con repomix..."

# --output: nombre del archivo de salida
# Puedes cambiar la extensión a .txt si prefieres texto plano en lugar de markdown
npx repomix --output repomix_codigo.md

echo "¡Completado! El código se ha guardado en 'repomix_codigo.md'"
</file>

<file path=".github/workflows/ci.yml">
name: CI

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

env:
  CARGO_TERM_COLOR: always

jobs:
  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  clippy-test:
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - name: Install FFmpeg (Linux)
        if: runner.os == 'Linux'
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            pkg-config \
            libavcodec-dev \
            libavformat-dev \
            libavutil-dev \
            libswscale-dev \
            libswresample-dev
      - name: Install FFmpeg (macOS)
        if: runner.os == 'macOS'
        run: brew install ffmpeg
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy --all-targets --all-features
      - run: cargo test --all-features
</file>

<file path="assets/youlean_settings/App Settings.ini">
[app]
crashed=0

[audio]
driver=2
in1=1
in10=0
in11=0
in12=0
in13=0
in14=0
in15=0
in16=0
in17=0
in18=0
in19=0
in2=2
in20=0
in21=0
in22=0
in23=0
in24=0
in3=0
in4=0
in5=0
in6=0
in7=0
in8=0
in9=0
indev=Default Input Device
iovs=512
out1=1
out10=10
out11=11
out12=12
out13=13
out14=14
out15=15
out16=16
out17=17
out18=18
out19=19
out2=2
out20=20
out21=21
out22=22
out23=23
out24=24
out3=3
out4=4
out5=5
out6=6
out7=7
out8=8
out9=9
outdev=Default Output Device
passaudio=0
sr=System Default

[gui]
audioSetupMessageShown=1
centerPos=0
forcetop=0
fullscreen=0
posL=775
posT=212
version=20514

[midi]
inchan=0
indev=off
outchan=0
outdev=off
</file>

<file path="assets/youlean_settings/License">
0!P)600))0bwMk#B;_
</file>

<file path="assets/youlean_settings/License Info">
EMPTY
</file>

<file path="assets/youlean_settings/Settings">
?]?PSH<|9^
	,x?mmQ{BTO>^$gIJ9m}nTBRDG",=5.ZRqbqzo}KOj[Xo3NJFX[/3:>
fuYh_N{L2[t1/N?Sx<&%,WZQ/\M80A*,M{8.m_t*,njSU6"m~Z{j5o/iJ?{~Z
FcT21FIJo|h*;qrG;"ZhNKNViMf&p1p
2b!F\9Zd]R*|;c21Kh~N
</file>

<file path="docs/ARQUITECTURA.md">
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
</file>

<file path="docs/BACKLOG.md">
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
</file>

<file path="docs/BUILD_PROFILES.md">
# Perfiles de compilación

## `release` (por defecto)

- **`opt-level = 3`**: prioriza rendimiento en CPU (decodificación, copias, UI). Recomendado para sesiones de QC en máquina de trabajo.
- **`lto = true`**, **`codegen-units = 1`**: mejora de optimización a costa de tiempos de link más largos.
- **`strip = true`**, **`panic = "abort"`**: binario más compacto y sin unwinding en pánico.

```bash
cargo build --release
```

## `release-small`

Hereda `release` pero usa **`opt-level = "s"`** (optimizar tamaño). Útil si el objetivo principal es empaquetado/portabilidad y se acepta algo menos de velocidad en rutas calientes.

```bash
cargo build --profile release-small
```

## Trade-off resumido

| Perfil           | Tamaño binario | Velocidad CPU típica |
|------------------|----------------|----------------------|
| `release`        | Mayor          | Mejor                |
| `release-small`  | Menor          | Menor                |

Para comparación A/B con dos decoders y `sws_scale`, **`release`** suele notarse más que el ahorro de tamaño de `release-small`.
</file>

<file path="docs/CONTRIBUTING.md">
# Contribuir — Comentarios y documentación (español)

## Política de comentarios

**No** se exige comentario en cada línea: genera ruido y dificulta el mantenimiento.

### Qué sí hacer

1. **`//!` (documentación de módulo)** al inicio de cada archivo `.rs`: qué problema resuelve, qué hilos toca, dependencias importantes.
2. **`///` (rustdoc)** en todo lo **público** (`pub fn`, `pub struct`, `pub enum`): parámetros, errores posibles, invariantes.
3. **Bloques `//`** antes de lógica delicada: diferimiento de entrada en macOS, cálculo de PTS, drenado de canales, condiciones de carrera.

### Qué evitar

- Comentar imports, getters triviales o código que ya se lee claro en Rust.
- Duplicar en comentario lo que el nombre del símbolo ya dice.

### Idioma

Comentarios y rustdoc del proyecto: **español**, salvo términos técnicos habituales en inglés (PTS, shader, frame).

## Formato

- `cargo fmt` antes de commit.
- `cargo clippy` sin warnings nuevos cuando sea posible.
</file>

<file path="docs/DEPENDENCIES_MIGRATION.md">
# Migración mayor: eframe / egui / wgpu

## Estado actual (referencia)

El `Cargo.toml` del proyecto fija versiones en la familia **eframe 0.27 / wgpu 0.19** (y crates alineados: `egui`, `egui-wgpu`). El informe interno mencionaba saltos hacia **eframe 0.31 / wgpu 0.25**: implica cambios de API en varios frentes.

## Por qué aplazar hasta tener base sólida

1. **wgpu + Metal/Vulkan/GL**: regresiones de backend y límites de textura afectan directamente al visor.
2. **eframe + persistencia**: cambios en `eframe::App`, almacenamiento y hooks de ventana.
3. **egui-wgpu**: el callback de pintado personalizado (`RenderCallback`) debe revalidarse contra la nueva integración.

## Orden recomendado

1. Mantener **CI** (`fmt`, `clippy`, `test`) verde en la versión actual.
2. Revisar **changelog** de `eframe`, `egui`, `wgpu` y `egui-wgpu` en un branch dedicado.
3. Subir **una major a la vez** si es posible (p. ej. primero `wgpu` + `egui-wgpu`, luego `eframe`), ejecutando pruebas manuales en **macOS y Windows** como mínimo.
4. Tras el bump, repetir pruebas de: carga A/B, seek, modos de comparación, proxy EXR, audio.

## Comando útil

```bash
cargo upgrade  # si usas cargo-edit; revisar diffs a mano
```

No automatizar el bump sin revisar los breaking changes publicados en cada crate.
</file>

<file path="docs/GPU_YUV_PIPELINE.md">
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
</file>

<file path="docs/HARDWARE_DECODE.md">
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
</file>

<file path="patches/ffmpeg-next/.github/ISSUE_TEMPLATE/bug.md">
---
name: New issue for reproducible bug
about: If you found a reproducible bug, submit it along with as much info as possible.
---

*Please include as much info as possible to save me (solo maintainer helping for free) some time. A [minimal, complete, and reproducible example](https://stackoverflow.com/help/minimal-reproducible-example) is a must. Link to a gist if you don't feel like posting all the code inline. At the same time, please leave out unnecessary code so I don't need to wade through a hundred lines to get to the problematic part. Tell me your OS, FFmpeg version, etc. if there's even a slim chance of relevancy.*
</file>

<file path="patches/ffmpeg-next/.github/ISSUE_TEMPLATE/config.yml">
blank_issues_enabled: false

contact_links:
- name: Questions about usage
  url: https://github.com/zmwangx/rust-ffmpeg/discussions
  about: If you have a question about usage, please use discussions instead of opening a (non-)issue. Note that I (solo maintainer short on time) unfortunately might not be able to respond, though I try to be helpful when time permits.
</file>

<file path="patches/ffmpeg-next/.github/ISSUE_TEMPLATE/feature.md">
---
name: New feature request
about: If you have a feature request, your best bet is probably a PR. However, for anything nontrivial, please open an issue to discuss it first.
---

*Please discuss your new feature before implementing it if it's nontrivial. Adding a small method is probably trivial, anything larger than that, maybe not. Note that API stability is paramount.*
</file>

<file path="patches/ffmpeg-next/.github/workflows/build.yml">
name: build
on:
  push:
  pull_request:
  schedule:
    - cron: "0 0 * * *"
jobs:
  build-test-lint-linux:
    name: Linux - FFmpeg ${{ matrix.ffmpeg_version }} - build, test and lint
    runs-on: ubuntu-latest
    container: jrottenberg/ffmpeg:${{ matrix.ffmpeg_version }}-ubuntu
    strategy:
      matrix:
        ffmpeg_version: ["3.4", "4.0", "4.1", "4.2", "4.3", "4.4", "5.0", "5.1", "6.0"]
      fail-fast: false
    steps:
      - uses: actions/checkout@v2
      - name: Install dependencies
        run: |
          apt update
          apt install -y --no-install-recommends clang curl pkg-config
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          components: rustfmt, clippy
      - name: Build
        run: |
          cargo build --examples
      - name: Test
        run: |
          cargo test --examples
      - name: Lint
        run: |
          cargo clippy --examples -- -D warnings
      - name: Check format
        run: |
          cargo fmt -- --check

  build-test-lint-macos:
    name: macOS - FFmpeg latest - build, test and lint
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install dependencies
        run: |
          brew install ffmpeg pkg-config
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          components: rustfmt, clippy
      - name: Build
        run: |
          cargo build --examples
      - name: Test
        run: |
          cargo test --examples
      - name: Lint
        run: |
          cargo clippy --examples -- -D warnings
      - name: Check format
        run: |
          cargo fmt -- --check

  build-test-lint-windows:
    name: Windows - FFmpeg ${{ matrix.ffmpeg_version }} - build, test and lint
    runs-on: windows-latest
    strategy:
      matrix:
        include:
          - ffmpeg_version: latest
            ffmpeg_download_url: https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-full-shared.7z
      fail-fast: false
    env:
      FFMPEG_DOWNLOAD_URL: ${{ matrix.ffmpeg_download_url }}
    steps:
      - uses: actions/checkout@v2
      - name: Install dependencies
        run: |
          $VCINSTALLDIR = $(& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -property installationPath)
          Add-Content $env:GITHUB_ENV "LIBCLANG_PATH=${VCINSTALLDIR}\VC\Tools\LLVM\x64\bin`n"
          Invoke-WebRequest "${env:FFMPEG_DOWNLOAD_URL}" -OutFile ffmpeg-release-full-shared.7z
          7z x ffmpeg-release-full-shared.7z
          mkdir ffmpeg
          mv ffmpeg-*/* ffmpeg/
          Add-Content $env:GITHUB_ENV "FFMPEG_DIR=${pwd}\ffmpeg`n"
          Add-Content $env:GITHUB_PATH "${pwd}\ffmpeg\bin`n"
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          components: rustfmt, clippy
      - name: Build
        run: |
          cargo build --examples
      - name: Test
        run: |
          cargo test --examples
      - name: Lint
        run: |
          cargo clippy --examples -- -D warnings
      - name: Check format
        run: |
          cargo fmt -- --check
  # Added only because there is no ffmpeg6.1 docker image here yet
  # https://github.com/jrottenberg/ffmpeg
  build-test-lint-latest:
    name: FFmpeg Latest - build, test and lint
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
    steps:
      - uses: actions/checkout@v2
      - name: Install dependencies
        run: |
          sudo apt update
          sudo apt install -y software-properties-common
          sudo add-apt-repository ppa:ubuntuhandbook1/ffmpeg6
          sudo apt update
          sudo apt install -y --no-install-recommends clang curl pkg-config ffmpeg libavutil-dev libavcodec-dev libavformat-dev libavfilter-dev libavfilter-dev libavdevice-dev libswresample-dev
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          components: rustfmt, clippy
      - name: Build
        run: |
          cargo build --examples
      - name: Test
        run: |
          cargo test --examples
      - name: Lint
        run: |
          cargo clippy --examples -- -D warnings
      - name: Check format
        run: |
          cargo fmt -- --check
</file>

<file path="patches/ffmpeg-next/.github/workflows/release.yml">
name: create release
on:
  push:
    tags:
      - "v*"
jobs:
  release:
    name: Create release
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Generate release notes
        run: |
          cat >release_notes.md <<EOF
          See [CHANGELOG.md](https://github.com/zmwangx/rust-ffmpeg/blob/${GITHUB_REF##*/}/CHANGELOG.md) for changes.
          EOF
      - name: Create release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: ${{ github.ref }}
          release_name: ${{ github.ref }}
          body_path: release_notes.md
          draft: false
          prerelease: false
</file>

<file path="patches/ffmpeg-next/examples/chapters.rs">
extern crate ffmpeg_next as ffmpeg;

use std::env;

fn main() {
    ffmpeg::init().unwrap();

    match ffmpeg::format::input(&env::args().nth(1).expect("missing input file name")) {
        Ok(ictx) => {
            println!("Nb chapters: {}", ictx.nb_chapters());

            for chapter in ictx.chapters() {
                println!("chapter id {}:", chapter.id());
                println!("\ttime_base: {}", chapter.time_base());
                println!("\tstart: {}", chapter.start());
                println!("\tend: {}", chapter.end());

                for (k, v) in chapter.metadata().iter() {
                    println!("\t{}: {}", k, v);
                }
            }

            let mut octx = ffmpeg::format::output(&"test.mkv").expect("Couldn't open test file");

            for chapter in ictx.chapters() {
                let title = match chapter.metadata().get("title") {
                    Some(title) => String::from(title),
                    None => String::new(),
                };

                match octx.add_chapter(
                    chapter.id(),
                    chapter.time_base(),
                    chapter.start(),
                    chapter.end(),
                    &title,
                ) {
                    Ok(chapter) => println!("Added chapter with id {} to output", chapter.id()),
                    Err(error) => {
                        println!("Error adding chapter with id: {} - {}", chapter.id(), error)
                    }
                }
            }

            println!("\nOuput: nb chapters: {}", octx.nb_chapters());
            for chapter in octx.chapters() {
                println!("chapter id {}:", chapter.id());
                println!("\ttime_base: {}", chapter.time_base());
                println!("\tstart: {}", chapter.start());
                println!("\tend: {}", chapter.end());
                for (k, v) in chapter.metadata().iter() {
                    println!("\t{}: {}", k, v);
                }
            }
        }

        Err(error) => println!("error: {}", error),
    }
}
</file>

<file path="patches/ffmpeg-next/examples/codec-info.rs">
extern crate ffmpeg_next as ffmpeg;

use std::env;

fn main() {
    ffmpeg::init().unwrap();

    for arg in env::args().skip(1) {
        if let Some(codec) = ffmpeg::decoder::find_by_name(&arg) {
            println!("type: decoder");
            println!("\t id: {:?}", codec.id());
            println!("\t name: {}", codec.name());
            println!("\t description: {}", codec.description());
            println!("\t medium: {:?}", codec.medium());
            println!("\t capabilities: {:?}", codec.capabilities());

            if let Some(profiles) = codec.profiles() {
                println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
            } else {
                println!("\t profiles: none");
            }

            if let Ok(video) = codec.video() {
                if let Some(rates) = video.rates() {
                    println!("\t rates: {:?}", rates.collect::<Vec<_>>());
                } else {
                    println!("\t rates: any");
                }

                if let Some(formats) = video.formats() {
                    println!("\t formats: {:?}", formats.collect::<Vec<_>>());
                } else {
                    println!("\t formats: any");
                }
            }

            if let Ok(audio) = codec.audio() {
                if let Some(rates) = audio.rates() {
                    println!("\t rates: {:?}", rates.collect::<Vec<_>>());
                } else {
                    println!("\t rates: any");
                }

                if let Some(formats) = audio.formats() {
                    println!("\t formats: {:?}", formats.collect::<Vec<_>>());
                } else {
                    println!("\t formats: any");
                }

                if let Some(layouts) = audio.channel_layouts() {
                    println!("\t channel_layouts: {:?}", layouts.collect::<Vec<_>>());
                } else {
                    println!("\t channel_layouts: any");
                }
            }

            println!("\t max_lowres: {:?}", codec.max_lowres());
        }

        if let Some(codec) = ffmpeg::encoder::find_by_name(&arg) {
            println!();
            println!("type: encoder");
            println!("\t id: {:?}", codec.id());
            println!("\t name: {}", codec.name());
            println!("\t description: {}", codec.description());
            println!("\t medium: {:?}", codec.medium());
            println!("\t capabilities: {:?}", codec.capabilities());

            if let Some(profiles) = codec.profiles() {
                println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
            }

            if let Ok(video) = codec.video() {
                if let Some(rates) = video.rates() {
                    println!("\t rates: {:?}", rates.collect::<Vec<_>>());
                } else {
                    println!("\t rates: any");
                }

                if let Some(formats) = video.formats() {
                    println!("\t formats: {:?}", formats.collect::<Vec<_>>());
                } else {
                    println!("\t formats: any");
                }
            }

            if let Ok(audio) = codec.audio() {
                if let Some(rates) = audio.rates() {
                    println!("\t rates: {:?}", rates.collect::<Vec<_>>());
                } else {
                    println!("\t rates: any");
                }

                if let Some(formats) = audio.formats() {
                    println!("\t formats: {:?}", formats.collect::<Vec<_>>());
                } else {
                    println!("\t formats: any");
                }

                if let Some(layouts) = audio.channel_layouts() {
                    println!("\t channel_layouts: {:?}", layouts.collect::<Vec<_>>());
                } else {
                    println!("\t channel_layouts: any");
                }
            }

            println!("\t max_lowres: {:?}", codec.max_lowres());
        }
    }
}
</file>

<file path="patches/ffmpeg-next/examples/dump-frames.rs">
extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();

    if let Ok(mut ictx) = input(&env::args().nth(1).expect("Cannot open file.")) {
        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        let mut frame_index = 0;

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
                let mut decoded = Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    save_file(&rgb_frame, frame_index).unwrap();
                    frame_index += 1;
                }
                Ok(())
            };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
    }

    Ok(())
}

fn save_file(frame: &Video, index: usize) -> std::result::Result<(), std::io::Error> {
    let mut file = File::create(format!("frame{}.ppm", index))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))?;
    Ok(())
}
</file>

<file path="patches/ffmpeg-next/examples/metadata.rs">
extern crate ffmpeg_next as ffmpeg;

use std::env;

fn main() -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();

    match ffmpeg::format::input(&env::args().nth(1).expect("missing file")) {
        Ok(context) => {
            for (k, v) in context.metadata().iter() {
                println!("{}: {}", k, v);
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Video) {
                println!("Best video stream index: {}", stream.index());
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Audio) {
                println!("Best audio stream index: {}", stream.index());
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Subtitle) {
                println!("Best subtitle stream index: {}", stream.index());
            }

            println!(
                "duration (seconds): {:.2}",
                context.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE)
            );

            for stream in context.streams() {
                println!("stream index {}:", stream.index());
                println!("\ttime_base: {}", stream.time_base());
                println!("\tstart_time: {}", stream.start_time());
                println!("\tduration (stream timebase): {}", stream.duration());
                println!(
                    "\tduration (seconds): {:.2}",
                    stream.duration() as f64 * f64::from(stream.time_base())
                );
                println!("\tframes: {}", stream.frames());
                println!("\tdisposition: {:?}", stream.disposition());
                println!("\tdiscard: {:?}", stream.discard());
                println!("\trate: {}", stream.rate());

                let codec = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
                println!("\tmedium: {:?}", codec.medium());
                println!("\tid: {:?}", codec.id());

                if codec.medium() == ffmpeg::media::Type::Video {
                    if let Ok(video) = codec.decoder().video() {
                        println!("\tbit_rate: {}", video.bit_rate());
                        println!("\tmax_rate: {}", video.max_bit_rate());
                        println!("\tdelay: {}", video.delay());
                        println!("\tvideo.width: {}", video.width());
                        println!("\tvideo.height: {}", video.height());
                        println!("\tvideo.format: {:?}", video.format());
                        println!("\tvideo.has_b_frames: {}", video.has_b_frames());
                        println!("\tvideo.aspect_ratio: {}", video.aspect_ratio());
                        println!("\tvideo.color_space: {:?}", video.color_space());
                        println!("\tvideo.color_range: {:?}", video.color_range());
                        println!("\tvideo.color_primaries: {:?}", video.color_primaries());
                        println!(
                            "\tvideo.color_transfer_characteristic: {:?}",
                            video.color_transfer_characteristic()
                        );
                        println!("\tvideo.chroma_location: {:?}", video.chroma_location());
                        println!("\tvideo.references: {}", video.references());
                        println!("\tvideo.intra_dc_precision: {}", video.intra_dc_precision());
                    }
                } else if codec.medium() == ffmpeg::media::Type::Audio {
                    if let Ok(audio) = codec.decoder().audio() {
                        println!("\tbit_rate: {}", audio.bit_rate());
                        println!("\tmax_rate: {}", audio.max_bit_rate());
                        println!("\tdelay: {}", audio.delay());
                        println!("\taudio.rate: {}", audio.rate());
                        println!("\taudio.channels: {}", audio.channels());
                        println!("\taudio.format: {:?}", audio.format());
                        println!("\taudio.frames: {}", audio.frames());
                        println!("\taudio.align: {}", audio.align());
                        println!("\taudio.channel_layout: {:?}", audio.channel_layout());
                    }
                }
            }
        }

        Err(error) => println!("error: {}", error),
    }
    Ok(())
}
</file>

<file path="patches/ffmpeg-next/examples/remux.rs">
extern crate ffmpeg_next as ffmpeg;

use std::env;

use ffmpeg::{codec, encoder, format, log, media, Rational};

fn main() {
    let input_file = env::args().nth(1).expect("missing input file");
    let output_file = env::args().nth(2).expect("missing output file");

    ffmpeg::init().unwrap();
    log::set_level(log::Level::Warning);

    let mut ictx = format::input(&input_file).unwrap();
    let mut octx = format::output(&output_file).unwrap();

    let mut stream_mapping = vec![0; ictx.nb_streams() as _];
    let mut ist_time_bases = vec![Rational(0, 1); ictx.nb_streams() as _];
    let mut ost_index = 0;
    for (ist_index, ist) in ictx.streams().enumerate() {
        let ist_medium = ist.parameters().medium();
        if ist_medium != media::Type::Audio
            && ist_medium != media::Type::Video
            && ist_medium != media::Type::Subtitle
        {
            stream_mapping[ist_index] = -1;
            continue;
        }
        stream_mapping[ist_index] = ost_index;
        ist_time_bases[ist_index] = ist.time_base();
        ost_index += 1;
        let mut ost = octx.add_stream(encoder::find(codec::Id::None)).unwrap();
        ost.set_parameters(ist.parameters());
        // We need to set codec_tag to 0 lest we run into incompatible codec tag
        // issues when muxing into a different container format. Unfortunately
        // there's no high level API to do this (yet).
        unsafe {
            (*ost.parameters().as_mut_ptr()).codec_tag = 0;
        }
    }

    octx.set_metadata(ictx.metadata().to_owned());
    octx.write_header().unwrap();

    for (stream, mut packet) in ictx.packets() {
        let ist_index = stream.index();
        let ost_index = stream_mapping[ist_index];
        if ost_index < 0 {
            continue;
        }
        let ost = octx.stream(ost_index as _).unwrap();
        packet.rescale_ts(ist_time_bases[ist_index], ost.time_base());
        packet.set_position(-1);
        packet.set_stream(ost_index as _);
        packet.write_interleaved(&mut octx).unwrap();
    }

    octx.write_trailer().unwrap();
}
</file>

<file path="patches/ffmpeg-next/examples/transcode-audio.rs">
extern crate ffmpeg_next as ffmpeg;

use std::env;
use std::path::Path;

use ffmpeg::{codec, filter, format, frame, media};
use ffmpeg::{rescale, Rescale};

fn filter(
    spec: &str,
    decoder: &codec::decoder::Audio,
    encoder: &codec::encoder::Audio,
) -> Result<filter::Graph, ffmpeg::Error> {
    let mut filter = filter::Graph::new();

    let args = format!(
        "time_base={}:sample_rate={}:sample_fmt={}:channel_layout=0x{:x}",
        decoder.time_base(),
        decoder.rate(),
        decoder.format().name(),
        decoder.channel_layout().bits()
    );

    filter.add(&filter::find("abuffer").unwrap(), "in", &args)?;
    filter.add(&filter::find("abuffersink").unwrap(), "out", "")?;

    {
        let mut out = filter.get("out").unwrap();

        out.set_sample_format(encoder.format());
        out.set_channel_layout(encoder.channel_layout());
        out.set_sample_rate(encoder.rate());
    }

    filter.output("in", 0)?.input("out", 0)?.parse(spec)?;
    filter.validate()?;

    println!("{}", filter.dump());

    if let Some(codec) = encoder.codec() {
        if !codec
            .capabilities()
            .contains(ffmpeg::codec::capabilities::Capabilities::VARIABLE_FRAME_SIZE)
        {
            filter
                .get("out")
                .unwrap()
                .sink()
                .set_frame_size(encoder.frame_size());
        }
    }

    Ok(filter)
}

struct Transcoder {
    stream: usize,
    filter: filter::Graph,
    decoder: codec::decoder::Audio,
    encoder: codec::encoder::Audio,
    in_time_base: ffmpeg::Rational,
    out_time_base: ffmpeg::Rational,
}

fn transcoder<P: AsRef<Path>>(
    ictx: &mut format::context::Input,
    octx: &mut format::context::Output,
    path: &P,
    filter_spec: &str,
) -> Result<Transcoder, ffmpeg::Error> {
    let input = ictx
        .streams()
        .best(media::Type::Audio)
        .expect("could not find best audio stream");
    let context = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = context.decoder().audio()?;
    let codec = ffmpeg::encoder::find(octx.format().codec(path, media::Type::Audio))
        .expect("failed to find encoder")
        .audio()?;
    let global = octx
        .format()
        .flags()
        .contains(ffmpeg::format::flag::Flags::GLOBAL_HEADER);

    decoder.set_parameters(input.parameters())?;

    let mut output = octx.add_stream(codec)?;
    let context = ffmpeg::codec::context::Context::from_parameters(output.parameters())?;
    let mut encoder = context.encoder().audio()?;

    let channel_layout = codec
        .channel_layouts()
        .map(|cls| cls.best(decoder.channel_layout().channels()))
        .unwrap_or(ffmpeg::channel_layout::ChannelLayout::STEREO);

    if global {
        encoder.set_flags(ffmpeg::codec::flag::Flags::GLOBAL_HEADER);
    }

    encoder.set_rate(decoder.rate() as i32);
    encoder.set_channel_layout(channel_layout);
    encoder.set_channels(channel_layout.channels());
    encoder.set_format(
        codec
            .formats()
            .expect("unknown supported formats")
            .next()
            .unwrap(),
    );
    encoder.set_bit_rate(decoder.bit_rate());
    encoder.set_max_bit_rate(decoder.max_bit_rate());

    encoder.set_time_base((1, decoder.rate() as i32));
    output.set_time_base((1, decoder.rate() as i32));

    let encoder = encoder.open_as(codec)?;
    output.set_parameters(&encoder);

    let filter = filter(filter_spec, &decoder, &encoder)?;

    let in_time_base = decoder.time_base();
    let out_time_base = output.time_base();

    Ok(Transcoder {
        stream: input.index(),
        filter,
        decoder,
        encoder,
        in_time_base,
        out_time_base,
    })
}

impl Transcoder {
    fn send_frame_to_encoder(&mut self, frame: &ffmpeg::Frame) {
        self.encoder.send_frame(frame).unwrap();
    }

    fn send_eof_to_encoder(&mut self) {
        self.encoder.send_eof().unwrap();
    }

    fn receive_and_process_encoded_packets(&mut self, octx: &mut format::context::Output) {
        let mut encoded = ffmpeg::Packet::empty();
        while self.encoder.receive_packet(&mut encoded).is_ok() {
            encoded.set_stream(0);
            encoded.rescale_ts(self.in_time_base, self.out_time_base);
            encoded.write_interleaved(octx).unwrap();
        }
    }

    fn add_frame_to_filter(&mut self, frame: &ffmpeg::Frame) {
        self.filter.get("in").unwrap().source().add(frame).unwrap();
    }

    fn flush_filter(&mut self) {
        self.filter.get("in").unwrap().source().flush().unwrap();
    }

    fn get_and_process_filtered_frames(&mut self, octx: &mut format::context::Output) {
        let mut filtered = frame::Audio::empty();
        while self
            .filter
            .get("out")
            .unwrap()
            .sink()
            .frame(&mut filtered)
            .is_ok()
        {
            self.send_frame_to_encoder(&filtered);
            self.receive_and_process_encoded_packets(octx);
        }
    }

    fn send_packet_to_decoder(&mut self, packet: &ffmpeg::Packet) {
        self.decoder.send_packet(packet).unwrap();
    }

    fn send_eof_to_decoder(&mut self) {
        self.decoder.send_eof().unwrap();
    }

    fn receive_and_process_decoded_frames(&mut self, octx: &mut format::context::Output) {
        let mut decoded = frame::Audio::empty();
        while self.decoder.receive_frame(&mut decoded).is_ok() {
            let timestamp = decoded.timestamp();
            decoded.set_pts(timestamp);
            self.add_frame_to_filter(&decoded);
            self.get_and_process_filtered_frames(octx);
        }
    }
}

// Transcode the `best` audio stream of the input file into a the output file while applying a
// given filter. If no filter was specified the stream gets copied (`anull` filter).
//
// Example 1: Transcode *.mp3 file to *.wmv while speeding it up
// transcode-audio in.mp3 out.wmv "atempo=1.2"
//
// Example 2: Overlay an audio file
// transcode-audio in.mp3 out.mp3 "amovie=overlay.mp3 [ov]; [in][ov] amerge [out]"
//
// Example 3: Seek to a specified position (in seconds)
// transcode-audio in.mp3 out.mp3 anull 30
fn main() {
    ffmpeg::init().unwrap();

    let input = env::args().nth(1).expect("missing input");
    let output = env::args().nth(2).expect("missing output");
    let filter = env::args().nth(3).unwrap_or_else(|| "anull".to_owned());
    let seek = env::args().nth(4).and_then(|s| s.parse::<i64>().ok());

    let mut ictx = format::input(&input).unwrap();
    let mut octx = format::output(&output).unwrap();
    let mut transcoder = transcoder(&mut ictx, &mut octx, &output, &filter).unwrap();

    if let Some(position) = seek {
        // If the position was given in seconds, rescale it to ffmpegs base timebase.
        let position = position.rescale((1, 1), rescale::TIME_BASE);
        // If this seek was embedded in the transcoding loop, a call of `flush()`
        // for every opened buffer after the successful seek would be advisable.
        ictx.seek(position, ..position).unwrap();
    }

    octx.set_metadata(ictx.metadata().to_owned());
    octx.write_header().unwrap();

    for (stream, mut packet) in ictx.packets() {
        if stream.index() == transcoder.stream {
            packet.rescale_ts(stream.time_base(), transcoder.in_time_base);
            transcoder.send_packet_to_decoder(&packet);
            transcoder.receive_and_process_decoded_frames(&mut octx);
        }
    }

    transcoder.send_eof_to_decoder();
    transcoder.receive_and_process_decoded_frames(&mut octx);

    transcoder.flush_filter();
    transcoder.get_and_process_filtered_frames(&mut octx);

    transcoder.send_eof_to_encoder();
    transcoder.receive_and_process_encoded_packets(&mut octx);

    octx.write_trailer().unwrap();
}
</file>

<file path="patches/ffmpeg-next/examples/transcode-x264.rs">
// Given an input file, transcode all video streams into H.264 (using libx264)
// while copying audio and subtitle streams.
//
// Invocation:
//
//   transcode-x264 <input> <output> [<x264_opts>]
//
// <x264_opts> is a comma-delimited list of key=val. default is "preset=medium".
// See https://ffmpeg.org/ffmpeg-codecs.html#libx264_002c-libx264rgb and
// https://trac.ffmpeg.org/wiki/Encode/H.264 for available and commonly used
// options.
//
// Examples:
//
//   transcode-x264 input.flv output.mp4
//   transcode-x264 input.mkv output.mkv 'preset=veryslow,crf=18'

extern crate ffmpeg_next as ffmpeg;

use std::collections::HashMap;
use std::env;
use std::time::Instant;

use ffmpeg::{
    codec, decoder, encoder, format, frame, log, media, picture, Dictionary, Packet, Rational,
};

const DEFAULT_X264_OPTS: &str = "preset=medium";

struct Transcoder {
    ost_index: usize,
    decoder: decoder::Video,
    encoder: encoder::video::Video,
    logging_enabled: bool,
    frame_count: usize,
    last_log_frame_count: usize,
    starting_time: Instant,
    last_log_time: Instant,
}

impl Transcoder {
    fn new(
        ist: &format::stream::Stream,
        octx: &mut format::context::Output,
        ost_index: usize,
        x264_opts: Dictionary,
        enable_logging: bool,
    ) -> Result<Self, ffmpeg::Error> {
        let global_header = octx.format().flags().contains(format::Flags::GLOBAL_HEADER);
        let decoder = ffmpeg::codec::context::Context::from_parameters(ist.parameters())?
            .decoder()
            .video()?;
        let mut ost = octx.add_stream(encoder::find(codec::Id::H264))?;
        let mut encoder = codec::context::Context::from_parameters(ost.parameters())?
            .encoder()
            .video()?;
        encoder.set_height(decoder.height());
        encoder.set_width(decoder.width());
        encoder.set_aspect_ratio(decoder.aspect_ratio());
        encoder.set_format(decoder.format());
        encoder.set_frame_rate(decoder.frame_rate());
        encoder.set_time_base(decoder.frame_rate().unwrap().invert());
        if global_header {
            encoder.set_flags(codec::Flags::GLOBAL_HEADER);
        }

        encoder
            .open_with(x264_opts)
            .expect("error opening libx264 encoder with supplied settings");
        encoder = codec::context::Context::from_parameters(ost.parameters())?
            .encoder()
            .video()?;
        ost.set_parameters(&encoder);
        Ok(Self {
            ost_index,
            decoder,
            encoder: codec::context::Context::from_parameters(ost.parameters())?
                .encoder()
                .video()?,
            logging_enabled: enable_logging,
            frame_count: 0,
            last_log_frame_count: 0,
            starting_time: Instant::now(),
            last_log_time: Instant::now(),
        })
    }

    fn send_packet_to_decoder(&mut self, packet: &Packet) {
        self.decoder.send_packet(packet).unwrap();
    }

    fn send_eof_to_decoder(&mut self) {
        self.decoder.send_eof().unwrap();
    }

    fn receive_and_process_decoded_frames(
        &mut self,
        octx: &mut format::context::Output,
        ost_time_base: Rational,
    ) {
        let mut frame = frame::Video::empty();
        while self.decoder.receive_frame(&mut frame).is_ok() {
            self.frame_count += 1;
            let timestamp = frame.timestamp();
            self.log_progress(f64::from(
                Rational(timestamp.unwrap_or(0) as i32, 1) * self.decoder.time_base(),
            ));
            frame.set_pts(timestamp);
            frame.set_kind(picture::Type::None);
            self.send_frame_to_encoder(&frame);
            self.receive_and_process_encoded_packets(octx, ost_time_base);
        }
    }

    fn send_frame_to_encoder(&mut self, frame: &frame::Video) {
        self.encoder.send_frame(frame).unwrap();
    }

    fn send_eof_to_encoder(&mut self) {
        self.encoder.send_eof().unwrap();
    }

    fn receive_and_process_encoded_packets(
        &mut self,
        octx: &mut format::context::Output,
        ost_time_base: Rational,
    ) {
        let mut encoded = Packet::empty();
        while self.encoder.receive_packet(&mut encoded).is_ok() {
            encoded.set_stream(self.ost_index);
            encoded.rescale_ts(self.decoder.time_base(), ost_time_base);
            encoded.write_interleaved(octx).unwrap();
        }
    }

    fn log_progress(&mut self, timestamp: f64) {
        if !self.logging_enabled
            || (self.frame_count - self.last_log_frame_count < 100
                && self.last_log_time.elapsed().as_secs_f64() < 1.0)
        {
            return;
        }
        eprintln!(
            "time elpased: \t{:8.2}\tframe count: {:8}\ttimestamp: {:8.2}",
            self.starting_time.elapsed().as_secs_f64(),
            self.frame_count,
            timestamp
        );
        self.last_log_frame_count = self.frame_count;
        self.last_log_time = Instant::now();
    }
}

fn parse_opts<'a>(s: String) -> Option<Dictionary<'a>> {
    let mut dict = Dictionary::new();
    for keyval in s.split_terminator(',') {
        let tokens: Vec<&str> = keyval.split('=').collect();
        match tokens[..] {
            [key, val] => dict.set(key, val),
            _ => return None,
        }
    }
    Some(dict)
}

fn main() {
    let input_file = env::args().nth(1).expect("missing input file");
    let output_file = env::args().nth(2).expect("missing output file");
    let x264_opts = parse_opts(
        env::args()
            .nth(3)
            .unwrap_or_else(|| DEFAULT_X264_OPTS.to_string()),
    )
    .expect("invalid x264 options string");

    eprintln!("x264 options: {:?}", x264_opts);

    ffmpeg::init().unwrap();
    log::set_level(log::Level::Info);

    let mut ictx = format::input(&input_file).unwrap();
    let mut octx = format::output(&output_file).unwrap();

    format::context::input::dump(&ictx, 0, Some(&input_file));

    let best_video_stream_index = ictx
        .streams()
        .best(media::Type::Video)
        .map(|stream| stream.index());
    let mut stream_mapping: Vec<isize> = vec![0; ictx.nb_streams() as _];
    let mut ist_time_bases = vec![Rational(0, 0); ictx.nb_streams() as _];
    let mut ost_time_bases = vec![Rational(0, 0); ictx.nb_streams() as _];
    let mut transcoders = HashMap::new();
    let mut ost_index = 0;
    for (ist_index, ist) in ictx.streams().enumerate() {
        let ist_medium = ist.parameters().medium();
        if ist_medium != media::Type::Audio
            && ist_medium != media::Type::Video
            && ist_medium != media::Type::Subtitle
        {
            stream_mapping[ist_index] = -1;
            continue;
        }
        stream_mapping[ist_index] = ost_index;
        ist_time_bases[ist_index] = ist.time_base();
        if ist_medium == media::Type::Video {
            // Initialize transcoder for video stream.
            transcoders.insert(
                ist_index,
                Transcoder::new(
                    &ist,
                    &mut octx,
                    ost_index as _,
                    x264_opts.to_owned(),
                    Some(ist_index) == best_video_stream_index,
                )
                .unwrap(),
            );
        } else {
            // Set up for stream copy for non-video stream.
            let mut ost = octx.add_stream(encoder::find(codec::Id::None)).unwrap();
            ost.set_parameters(ist.parameters());
            // We need to set codec_tag to 0 lest we run into incompatible codec tag
            // issues when muxing into a different container format. Unfortunately
            // there's no high level API to do this (yet).
            unsafe {
                (*ost.parameters().as_mut_ptr()).codec_tag = 0;
            }
        }
        ost_index += 1;
    }

    octx.set_metadata(ictx.metadata().to_owned());
    format::context::output::dump(&octx, 0, Some(&output_file));
    octx.write_header().unwrap();

    for (ost_index, _) in octx.streams().enumerate() {
        ost_time_bases[ost_index] = octx.stream(ost_index as _).unwrap().time_base();
    }

    for (stream, mut packet) in ictx.packets() {
        let ist_index = stream.index();
        let ost_index = stream_mapping[ist_index];
        if ost_index < 0 {
            continue;
        }
        let ost_time_base = ost_time_bases[ost_index as usize];
        match transcoders.get_mut(&ist_index) {
            Some(transcoder) => {
                packet.rescale_ts(stream.time_base(), transcoder.decoder.time_base());
                transcoder.send_packet_to_decoder(&packet);
                transcoder.receive_and_process_decoded_frames(&mut octx, ost_time_base);
            }
            None => {
                // Do stream copy on non-video streams.
                packet.rescale_ts(ist_time_bases[ist_index], ost_time_base);
                packet.set_position(-1);
                packet.set_stream(ost_index as _);
                packet.write_interleaved(&mut octx).unwrap();
            }
        }
    }

    // Flush encoders and decoders.
    for (ost_index, transcoder) in transcoders.iter_mut() {
        let ost_time_base = ost_time_bases[*ost_index];
        transcoder.send_eof_to_decoder();
        transcoder.receive_and_process_decoded_frames(&mut octx, ost_time_base);
        transcoder.send_eof_to_encoder();
        transcoder.receive_and_process_encoded_packets(&mut octx, ost_time_base);
    }

    octx.write_trailer().unwrap();
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/audio.rs">
use std::ops::{Deref, DerefMut};

#[cfg(not(feature = "ffmpeg_5_0"))]
use ffi::*;
#[cfg(not(feature = "ffmpeg_5_0"))]
use libc::c_int;

use super::Opened;
use codec::Context;
#[cfg(not(feature = "ffmpeg_5_0"))]
use frame;
use util::format;
#[cfg(not(feature = "ffmpeg_5_0"))]
use {packet, Error};
use {AudioService, ChannelLayout};

pub struct Audio(pub Opened);

impl Audio {
    #[deprecated(
        since = "4.4.0",
        note = "Underlying API avcodec_decode_audio4 has been deprecated since FFmpeg 3.1; \
        consider switching to send_packet() and receive_frame()"
    )]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn decode<P: packet::Ref>(
        &mut self,
        packet: &P,
        out: &mut frame::Audio,
    ) -> Result<bool, Error> {
        unsafe {
            let mut got: c_int = 0;

            match avcodec_decode_audio4(
                self.as_mut_ptr(),
                out.as_mut_ptr(),
                &mut got,
                packet.as_ptr(),
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }

    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }

    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }

    pub fn format(&self) -> format::Sample {
        unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
    }

    pub fn request_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).request_sample_fmt = value.into();
        }
    }

    pub fn frames(&self) -> usize {
        unsafe { (*self.as_ptr()).frame_number as usize }
    }

    pub fn align(&self) -> usize {
        unsafe { (*self.as_ptr()).block_align as usize }
    }

    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe { ChannelLayout::from_bits_truncate((*self.as_ptr()).channel_layout) }
    }

    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            (*self.as_mut_ptr()).channel_layout = value.bits();
        }
    }

    pub fn request_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            (*self.as_mut_ptr()).request_channel_layout = value.bits();
        }
    }

    pub fn audio_service(&mut self) -> AudioService {
        unsafe { AudioService::from((*self.as_mut_ptr()).audio_service_type) }
    }

    pub fn max_bit_rate(&self) -> usize {
        unsafe { (*self.as_ptr()).rc_max_rate as usize }
    }

    pub fn frame_size(&self) -> u32 {
        unsafe { (*self.as_ptr()).frame_size as u32 }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn frame_start(&self) -> Option<usize> {
        unsafe {
            // Removed in ffmpeg >= 5.0 in favor of using encoder
            // private options.
            match (*self.as_ptr()).timecode_frame_start {
                -1 => None,
                n => Some(n as usize),
            }
        }
    }
}

impl Deref for Audio {
    type Target = Opened;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Audio {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Audio {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/check.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Check: c_int {
        const CRC      = AV_EF_CRCCHECK;
        const BISTREAM = AV_EF_BITSTREAM;
        const BUFFER   = AV_EF_BUFFER;
        const EXPLODE  = AV_EF_EXPLODE;

        const IGNORE_ERROR = AV_EF_IGNORE_ERR;
        const CAREFUL      = AV_EF_CAREFUL;
        const COMPLIANT    = AV_EF_COMPLIANT;
        const AGGRESSIVE   = AV_EF_AGGRESSIVE;
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/conceal.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Conceal: c_int {
        const GUESS_MVS   = FF_EC_GUESS_MVS;
        const DEBLOCK     = FF_EC_DEBLOCK;
        const FAVOR_INTER = FF_EC_FAVOR_INTER;
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/decoder.rs">
use std::ops::{Deref, DerefMut};
use std::ptr;

use super::{Audio, Check, Conceal, Opened, Subtitle, Video};
use codec::{traits, Context};
use ffi::*;
use {Dictionary, Discard, Error, Rational};

pub struct Decoder(pub Context);

impl Decoder {
    pub fn open(mut self) -> Result<Opened, Error> {
        unsafe {
            match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
                0 => Ok(Opened(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as<D: traits::Decoder>(mut self, codec: D) -> Result<Opened, Error> {
        unsafe {
            if let Some(codec) = codec.decoder() {
                match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
                    0 => Ok(Opened(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::DecoderNotFound)
            }
        }
    }

    pub fn open_as_with<D: traits::Decoder>(
        mut self,
        codec: D,
        options: Dictionary,
    ) -> Result<Opened, Error> {
        unsafe {
            if let Some(codec) = codec.decoder() {
                let mut opts = options.disown();
                let res = avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut opts);

                Dictionary::own(opts);

                match res {
                    0 => Ok(Opened(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::DecoderNotFound)
            }
        }
    }

    pub fn video(self) -> Result<Video, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.video())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn audio(self) -> Result<Audio, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.audio())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn subtitle(self) -> Result<Subtitle, Error> {
        if let Some(codec) = super::find(self.id()) {
            self.open_as(codec).and_then(|o| o.subtitle())
        } else {
            Err(Error::DecoderNotFound)
        }
    }

    pub fn conceal(&mut self, value: Conceal) {
        unsafe {
            (*self.as_mut_ptr()).error_concealment = value.bits();
        }
    }

    pub fn check(&mut self, value: Check) {
        unsafe {
            (*self.as_mut_ptr()).err_recognition = value.bits();
        }
    }

    pub fn skip_loop_filter(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_loop_filter = value.into();
        }
    }

    pub fn skip_idct(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_idct = value.into();
        }
    }

    pub fn skip_frame(&mut self, value: Discard) {
        unsafe {
            (*self.as_mut_ptr()).skip_frame = value.into();
        }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).time_base) }
    }
}

impl Deref for Decoder {
    type Target = Context;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Decoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Decoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/mod.rs">
pub mod decoder;
pub use self::decoder::Decoder;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod subtitle;
pub use self::subtitle::Subtitle;

pub mod slice;

pub mod conceal;
pub use self::conceal::Conceal;

pub mod check;
pub use self::check::Check;

pub mod opened;
pub use self::opened::Opened;

use std::ffi::CString;

use codec::Context;
use codec::Id;
use ffi::*;
use Codec;

pub fn new() -> Decoder {
    Context::new().decoder()
}

pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        // We get a clippy warning in 4.4 but not in 5.0 and newer, so we allow that cast to not complicate the code
        #[allow(clippy::unnecessary_cast)]
        let ptr = avcodec_find_decoder(id.into()) as *mut AVCodec;

        if ptr.is_null() {
            None
        } else {
            Some(Codec::wrap(ptr))
        }
    }
}

pub fn find_by_name(name: &str) -> Option<Codec> {
    unsafe {
        let name = CString::new(name).unwrap();
        #[allow(clippy::unnecessary_cast)]
        let ptr = avcodec_find_decoder_by_name(name.as_ptr()) as *mut AVCodec;

        if ptr.is_null() {
            None
        } else {
            Some(Codec::wrap(ptr))
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/opened.rs">
use std::ops::{Deref, DerefMut};
use std::ptr;

use super::{Audio, Decoder, Subtitle, Video};
use codec::{Context, Profile};
use ffi::*;
use {media, packet, Error, Frame, Rational};

pub struct Opened(pub Decoder);

impl Opened {
    pub fn video(self) -> Result<Video, Error> {
        if self.medium() == media::Type::Video {
            Ok(Video(self))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub fn audio(self) -> Result<Audio, Error> {
        if self.medium() == media::Type::Audio {
            Ok(Audio(self))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub fn subtitle(self) -> Result<Subtitle, Error> {
        if self.medium() == media::Type::Subtitle {
            Ok(Subtitle(self))
        } else {
            Err(Error::InvalidData)
        }
    }

    pub fn send_packet<P: packet::Ref>(&mut self, packet: &P) -> Result<(), Error> {
        unsafe {
            match avcodec_send_packet(self.as_mut_ptr(), packet.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    /// Sends a NULL packet to the decoder to signal end of stream and enter
    /// draining mode.
    pub fn send_eof(&mut self) -> Result<(), Error> {
        unsafe {
            match avcodec_send_packet(self.as_mut_ptr(), ptr::null()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn receive_frame(&mut self, frame: &mut Frame) -> Result<(), Error> {
        unsafe {
            match avcodec_receive_frame(self.as_mut_ptr(), frame.as_mut_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn bit_rate(&self) -> usize {
        unsafe { (*self.as_ptr()).bit_rate as usize }
    }

    pub fn delay(&self) -> usize {
        unsafe { (*self.as_ptr()).delay as usize }
    }

    pub fn profile(&self) -> Profile {
        unsafe { Profile::from((self.id(), (*self.as_ptr()).profile)) }
    }

    pub fn frame_rate(&self) -> Option<Rational> {
        unsafe {
            let value = (*self.as_ptr()).framerate;

            if value == (AVRational { num: 0, den: 1 }) {
                None
            } else {
                Some(Rational::from(value))
            }
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            avcodec_flush_buffers(self.as_mut_ptr());
        }
    }
}

impl Drop for Opened {
    fn drop(&mut self) {
        unsafe {
            avcodec_close(self.as_mut_ptr());
        }
    }
}

impl Deref for Opened {
    type Target = Decoder;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Opened {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Opened {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Opened {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/slice.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const CODED_ORDER = SLICE_FLAG_CODED_ORDER;
        const ALLOW_FIELD = SLICE_FLAG_ALLOW_FIELD;
        const ALLOW_PLANE = SLICE_FLAG_ALLOW_PLANE;
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/subtitle.rs">
use std::ops::{Deref, DerefMut};

use ffi::*;
use libc::c_int;

use super::Opened;
use codec::Context;
use {packet, Error};

pub struct Subtitle(pub Opened);

impl Subtitle {
    pub fn decode<P: packet::Ref>(
        &mut self,
        packet: &P,
        out: &mut ::Subtitle,
    ) -> Result<bool, Error> {
        unsafe {
            let mut got: c_int = 0;

            match avcodec_decode_subtitle2(
                self.as_mut_ptr(),
                out.as_mut_ptr(),
                &mut got,
                packet.as_ptr() as *mut _,
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }
}

impl Deref for Subtitle {
    type Target = Opened;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Subtitle {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Subtitle {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Subtitle {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/decoder/video.rs">
use std::ops::{Deref, DerefMut};

#[cfg(not(feature = "ffmpeg_5_0"))]
use ffi::*;
use libc::c_int;

use super::{slice, Opened};
use codec::Context;
use color;
#[cfg(not(feature = "ffmpeg_5_0"))]
use frame;
use util::chroma;
use util::format;
#[cfg(not(feature = "ffmpeg_5_0"))]
use {packet, Error};
use {FieldOrder, Rational};

pub struct Video(pub Opened);

impl Video {
    #[deprecated(
        since = "4.4.0",
        note = "Underlying API avcodec_decode_video2 has been deprecated since FFmpeg 3.1; \
        consider switching to send_packet() and receive_frame()"
    )]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn decode<P: packet::Ref>(
        &mut self,
        packet: &P,
        out: &mut frame::Video,
    ) -> Result<bool, Error> {
        unsafe {
            let mut got: c_int = 0;

            match avcodec_decode_video2(
                self.as_mut_ptr(),
                out.as_mut_ptr(),
                &mut got,
                packet.as_ptr(),
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { (*self.as_ptr()).width as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.as_ptr()).height as u32 }
    }

    pub fn format(&self) -> format::Pixel {
        unsafe { format::Pixel::from((*self.as_ptr()).pix_fmt) }
    }

    pub fn has_b_frames(&self) -> bool {
        unsafe { (*self.as_ptr()).has_b_frames != 0 }
    }

    pub fn aspect_ratio(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).sample_aspect_ratio) }
    }

    pub fn color_space(&self) -> color::Space {
        unsafe { color::Space::from((*self.as_ptr()).colorspace) }
    }

    pub fn color_range(&self) -> color::Range {
        unsafe { color::Range::from((*self.as_ptr()).color_range) }
    }

    pub fn color_primaries(&self) -> color::Primaries {
        unsafe { color::Primaries::from((*self.as_ptr()).color_primaries) }
    }

    pub fn color_transfer_characteristic(&self) -> color::TransferCharacteristic {
        unsafe { color::TransferCharacteristic::from((*self.as_ptr()).color_trc) }
    }

    pub fn chroma_location(&self) -> chroma::Location {
        unsafe { chroma::Location::from((*self.as_ptr()).chroma_sample_location) }
    }

    pub fn set_slice_count(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).slice_count = value as c_int;
        }
    }

    pub fn set_slice_flags(&mut self, value: slice::Flags) {
        unsafe {
            (*self.as_mut_ptr()).slice_flags = value.bits();
        }
    }

    pub fn skip_top(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).skip_top = value as c_int;
        }
    }

    pub fn skip_bottom(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).skip_bottom = value as c_int;
        }
    }

    pub fn references(&self) -> usize {
        unsafe { (*self.as_ptr()).refs as usize }
    }

    pub fn set_field_order(&mut self, value: FieldOrder) {
        unsafe {
            (*self.as_mut_ptr()).field_order = value.into();
        }
    }

    // intra_matrix
    // inter_matrix

    pub fn intra_dc_precision(&self) -> u8 {
        unsafe { (*self.as_ptr()).intra_dc_precision as u8 }
    }

    pub fn max_bit_rate(&self) -> usize {
        unsafe { (*self.as_ptr()).rc_max_rate as usize }
    }
}

impl Deref for Video {
    type Target = Opened;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Video {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Video {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Video {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/audio.rs">
use std::ops::{Deref, DerefMut};
use std::ptr;

use ffi::*;
#[cfg(not(feature = "ffmpeg_5_0"))]
use libc::c_int;

use super::Encoder as Super;
use codec::{traits, Context};
use util::format;
#[cfg(not(feature = "ffmpeg_5_0"))]
use {frame, packet};
use {ChannelLayout, Dictionary, Error};

pub struct Audio(pub Super);

impl Audio {
    pub fn open(mut self) -> Result<Encoder, Error> {
        unsafe {
            match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as<E: traits::Encoder>(mut self, codec: E) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    pub fn open_with(mut self, options: Dictionary) -> Result<Encoder, Error> {
        unsafe {
            let mut opts = options.disown();
            let res = avcodec_open2(self.as_mut_ptr(), ptr::null(), &mut opts);

            Dictionary::own(opts);

            match res {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as_with<E: traits::Encoder>(
        mut self,
        codec: E,
        options: Dictionary,
    ) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                let mut opts = options.disown();
                let res = avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut opts);

                Dictionary::own(opts);

                match res {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    pub fn set_rate(&mut self, rate: i32) {
        unsafe {
            (*self.as_mut_ptr()).sample_rate = rate;
        }
    }

    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }

    pub fn set_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).sample_fmt = value.into();
        }
    }

    pub fn format(&self) -> format::Sample {
        unsafe { format::Sample::from((*self.as_ptr()).sample_fmt) }
    }

    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe {
            (*self.as_mut_ptr()).channel_layout = value.bits();
        }
    }

    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe { ChannelLayout::from_bits_truncate((*self.as_ptr()).channel_layout) }
    }

    pub fn set_channels(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).channels = value;
        }
    }

    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }
}

impl Deref for Audio {
    type Target = Super;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Audio {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Audio {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}

pub struct Encoder(pub Audio);

impl Encoder {
    #[deprecated(
        since = "4.4.0",
        note = "Underlying API avcodec_encode_audio2 has been deprecated since FFmpeg 3.1; \
        consider switching to send_frame() and receive_packet()"
    )]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn encode<P: packet::Mut>(
        &mut self,
        frame: &frame::Audio,
        out: &mut P,
    ) -> Result<bool, Error> {
        unsafe {
            if self.format() != frame.format() {
                return Err(Error::InvalidData);
            }

            let mut got: c_int = 0;

            match avcodec_encode_audio2(
                self.0.as_mut_ptr(),
                out.as_mut_ptr(),
                frame.as_ptr(),
                &mut got,
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }

    #[deprecated(
        since = "4.4.0",
        note = "Underlying API avcodec_encode_audio2 has been deprecated since FFmpeg 3.1; \
        consider switching to send_eof() and receive_packet()"
    )]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn flush<P: packet::Mut>(&mut self, out: &mut P) -> Result<bool, Error> {
        unsafe {
            let mut got: c_int = 0;

            match avcodec_encode_audio2(
                self.0.as_mut_ptr(),
                out.as_mut_ptr(),
                ptr::null(),
                &mut got,
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }

    pub fn frame_size(&self) -> u32 {
        unsafe { (*self.as_ptr()).frame_size as u32 }
    }
}

impl Deref for Encoder {
    type Target = Audio;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Encoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Encoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/comparison.rs">
use ffi::*;
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Comparison {
    SAD,
    SSE,
    SATD,
    DCT,
    PSNR,
    BIT,
    RD,
    ZERO,
    VSAD,
    VSSE,
    NSSE,
    W53,
    W97,
    DCTMAX,
    DCT264,
    CHROMA,
}

impl From<c_int> for Comparison {
    fn from(value: c_int) -> Comparison {
        match value {
            FF_CMP_SAD => Comparison::SAD,
            FF_CMP_SSE => Comparison::SSE,
            FF_CMP_SATD => Comparison::SATD,
            FF_CMP_DCT => Comparison::DCT,
            FF_CMP_PSNR => Comparison::PSNR,
            FF_CMP_BIT => Comparison::BIT,
            FF_CMP_RD => Comparison::RD,
            FF_CMP_ZERO => Comparison::ZERO,
            FF_CMP_VSAD => Comparison::VSAD,
            FF_CMP_VSSE => Comparison::VSSE,
            FF_CMP_NSSE => Comparison::NSSE,
            FF_CMP_W53 => Comparison::W53,
            FF_CMP_W97 => Comparison::W97,
            FF_CMP_DCTMAX => Comparison::DCTMAX,
            FF_CMP_DCT264 => Comparison::DCT264,
            FF_CMP_CHROMA => Comparison::CHROMA,

            _ => Comparison::ZERO,
        }
    }
}

impl From<Comparison> for c_int {
    fn from(value: Comparison) -> c_int {
        match value {
            Comparison::SAD => FF_CMP_SAD,
            Comparison::SSE => FF_CMP_SSE,
            Comparison::SATD => FF_CMP_SATD,
            Comparison::DCT => FF_CMP_DCT,
            Comparison::PSNR => FF_CMP_PSNR,
            Comparison::BIT => FF_CMP_BIT,
            Comparison::RD => FF_CMP_RD,
            Comparison::ZERO => FF_CMP_ZERO,
            Comparison::VSAD => FF_CMP_VSAD,
            Comparison::VSSE => FF_CMP_VSSE,
            Comparison::NSSE => FF_CMP_NSSE,
            Comparison::W53 => FF_CMP_W53,
            Comparison::W97 => FF_CMP_W97,
            Comparison::DCTMAX => FF_CMP_DCTMAX,
            Comparison::DCT264 => FF_CMP_DCT264,
            Comparison::CHROMA => FF_CMP_CHROMA,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/decision.rs">
use ffi::*;
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Decision {
    Simple,
    Bits,
    RateDistortion,
}

impl From<c_int> for Decision {
    fn from(value: c_int) -> Decision {
        match value {
            FF_MB_DECISION_SIMPLE => Decision::Simple,
            FF_MB_DECISION_BITS => Decision::Bits,
            FF_MB_DECISION_RD => Decision::RateDistortion,

            _ => Decision::Simple,
        }
    }
}

impl From<Decision> for c_int {
    fn from(value: Decision) -> c_int {
        match value {
            Decision::Simple => FF_MB_DECISION_SIMPLE,
            Decision::Bits => FF_MB_DECISION_BITS,
            Decision::RateDistortion => FF_MB_DECISION_RD,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/encoder.rs">
use std::ops::{Deref, DerefMut};
use std::ptr;

use ffi::*;
use libc::c_int;

use super::{audio, subtitle, video};
use codec::Context;
use {media, packet, Error, Frame, Rational};

pub struct Encoder(pub Context);

impl Encoder {
    pub fn video(mut self) -> Result<video::Video, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    (*self.as_mut_ptr()).codec_type = media::Type::Video.into();
                }

                Ok(video::Video(self))
            }

            media::Type::Video => Ok(video::Video(self)),

            _ => Err(Error::InvalidData),
        }
    }

    pub fn audio(mut self) -> Result<audio::Audio, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    (*self.as_mut_ptr()).codec_type = media::Type::Audio.into();
                }

                Ok(audio::Audio(self))
            }

            media::Type::Audio => Ok(audio::Audio(self)),

            _ => Err(Error::InvalidData),
        }
    }

    pub fn subtitle(mut self) -> Result<subtitle::Subtitle, Error> {
        match self.medium() {
            media::Type::Unknown => {
                unsafe {
                    (*self.as_mut_ptr()).codec_type = media::Type::Subtitle.into();
                }

                Ok(subtitle::Subtitle(self))
            }

            media::Type::Subtitle => Ok(subtitle::Subtitle(self)),

            _ => Err(Error::InvalidData),
        }
    }

    pub fn send_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match avcodec_send_frame(self.as_mut_ptr(), frame.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    /// Sends a NULL packet to the encoder to signal end of stream and enter
    /// draining mode.
    pub fn send_eof(&mut self) -> Result<(), Error> {
        unsafe { self.send_frame(&Frame::wrap(ptr::null_mut())) }
    }

    pub fn receive_packet<P: packet::Mut>(&mut self, packet: &mut P) -> Result<(), Error> {
        unsafe {
            match avcodec_receive_packet(self.as_mut_ptr(), packet.as_mut_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn set_bit_rate(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).bit_rate = value as i64;
        }
    }

    pub fn set_max_bit_rate(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).rc_max_rate = value as i64;
        }
    }

    pub fn set_tolerance(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).bit_rate_tolerance = value as c_int;
        }
    }

    pub fn set_quality(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).global_quality = value as c_int;
        }
    }

    pub fn set_compression(&mut self, value: Option<usize>) {
        unsafe {
            if let Some(value) = value {
                (*self.as_mut_ptr()).compression_level = value as c_int;
            } else {
                (*self.as_mut_ptr()).compression_level = -1;
            }
        }
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }

    pub fn set_frame_rate<R: Into<Rational>>(&mut self, value: Option<R>) {
        unsafe {
            if let Some(value) = value {
                (*self.as_mut_ptr()).framerate = value.into().into();
            } else {
                (*self.as_mut_ptr()).framerate.num = 0;
                (*self.as_mut_ptr()).framerate.den = 1;
            }
        }
    }
}

impl Deref for Encoder {
    type Target = Context;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Encoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Encoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut *self
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/mod.rs">
pub mod encoder;
pub use self::encoder::Encoder;

pub mod video;
pub use self::video::Encoder as Video;

pub mod audio;
pub use self::audio::Encoder as Audio;

pub mod subtitle;
pub use self::subtitle::Encoder as Subtitle;

pub mod motion_estimation;
pub use self::motion_estimation::MotionEstimation;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub mod prediction;
#[cfg(not(feature = "ffmpeg_5_0"))]
pub use self::prediction::Prediction;

pub mod comparison;
pub use self::comparison::Comparison;

pub mod decision;
pub use self::decision::Decision;

use std::ffi::CString;

use codec::Context;
use codec::Id;
use ffi::*;
use Codec;

pub fn new() -> Encoder {
    Context::new().encoder()
}

pub fn find(id: Id) -> Option<Codec> {
    unsafe {
        // We get a clippy warning in 4.4 but not in 5.0 and newer, so we allow that cast to not complicate the code
        #[allow(clippy::unnecessary_cast)]
        let ptr = avcodec_find_encoder(id.into()) as *mut AVCodec;

        if ptr.is_null() {
            None
        } else {
            Some(Codec::wrap(ptr))
        }
    }
}

pub fn find_by_name(name: &str) -> Option<Codec> {
    unsafe {
        let name = CString::new(name).unwrap();
        #[allow(clippy::unnecessary_cast)]
        let ptr = avcodec_find_encoder_by_name(name.as_ptr()) as *mut AVCodec;

        if ptr.is_null() {
            None
        } else {
            Some(Codec::wrap(ptr))
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/motion_estimation.rs">
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MotionEstimation {
    Zero,
    Full,
    Log,
    Phods,
    Epzs,
    X1,
    Hex,
    Umh,
    Iter,
    Tesa,
}

impl From<c_int> for MotionEstimation {
    fn from(value: c_int) -> MotionEstimation {
        match value {
            1 => MotionEstimation::Zero,
            2 => MotionEstimation::Full,
            3 => MotionEstimation::Log,
            4 => MotionEstimation::Phods,
            5 => MotionEstimation::Epzs,
            6 => MotionEstimation::X1,
            7 => MotionEstimation::Hex,
            8 => MotionEstimation::Umh,
            9 => MotionEstimation::Iter,
            10 => MotionEstimation::Tesa,

            _ => MotionEstimation::Zero,
        }
    }
}

impl From<MotionEstimation> for c_int {
    fn from(value: MotionEstimation) -> c_int {
        match value {
            MotionEstimation::Zero => 1,
            MotionEstimation::Full => 2,
            MotionEstimation::Log => 3,
            MotionEstimation::Phods => 4,
            MotionEstimation::Epzs => 5,
            MotionEstimation::X1 => 6,
            MotionEstimation::Hex => 7,
            MotionEstimation::Umh => 8,
            MotionEstimation::Iter => 9,
            MotionEstimation::Tesa => 10,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/prediction.rs">
use ffi::*;
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Prediction {
    Left,
    Plane,
    Median,
}

impl From<c_int> for Prediction {
    fn from(value: c_int) -> Prediction {
        match value {
            FF_PRED_LEFT => Prediction::Left,
            FF_PRED_PLANE => Prediction::Plane,
            FF_PRED_MEDIAN => Prediction::Median,

            _ => Prediction::Left,
        }
    }
}

impl From<Prediction> for c_int {
    fn from(value: Prediction) -> c_int {
        match value {
            Prediction::Left => FF_PRED_LEFT,
            Prediction::Plane => FF_PRED_PLANE,
            Prediction::Median => FF_PRED_MEDIAN,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/subtitle.rs">
use std::ops::{Deref, DerefMut};
use std::ptr;

use ffi::*;
use libc::c_int;

use super::Encoder as Super;
use codec::{traits, Context};
use {Dictionary, Error};

pub struct Subtitle(pub Super);

impl Subtitle {
    pub fn open(mut self) -> Result<Encoder, Error> {
        unsafe {
            match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn open_as<E: traits::Encoder>(mut self, codec: E) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    pub fn open_as_with<E: traits::Encoder>(
        mut self,
        codec: E,
        options: Dictionary,
    ) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                let mut opts = options.disown();
                let res = avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut opts);

                Dictionary::own(opts);

                match res {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }
}

impl Deref for Subtitle {
    type Target = Super;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Subtitle {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Subtitle {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Subtitle {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}

pub struct Encoder(pub Subtitle);

impl Encoder {
    pub fn encode(&mut self, subtitle: &::Subtitle, out: &mut [u8]) -> Result<bool, Error> {
        unsafe {
            match avcodec_encode_subtitle(
                self.0.as_mut_ptr(),
                out.as_mut_ptr(),
                out.len() as c_int,
                subtitle.as_ptr(),
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(true),
            }
        }
    }
}

impl Deref for Encoder {
    type Target = Subtitle;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Encoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Encoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/encoder/video.rs">
use std::ops::{Deref, DerefMut};
use std::ptr;

use ffi::*;
use libc::{c_float, c_int};

use super::Encoder as Super;
use super::{Comparison, Decision};
#[cfg(not(feature = "ffmpeg_5_0"))]
use super::{MotionEstimation, Prediction};
use codec::{traits, Context};
use {color, format, Dictionary, Error, Rational};
#[cfg(not(feature = "ffmpeg_5_0"))]
use {frame, packet};

pub struct Video(pub Super);

impl Video {
    #[inline]
    pub fn open(mut self) -> Result<Encoder, Error> {
        unsafe {
            match avcodec_open2(self.as_mut_ptr(), ptr::null(), ptr::null_mut()) {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    #[inline]
    pub fn open_as<E: traits::Encoder>(mut self, codec: E) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                match avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), ptr::null_mut()) {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    #[inline]
    pub fn open_with(mut self, options: Dictionary) -> Result<Encoder, Error> {
        unsafe {
            let mut opts = options.disown();
            let res = avcodec_open2(self.as_mut_ptr(), ptr::null(), &mut opts);

            Dictionary::own(opts);

            match res {
                0 => Ok(Encoder(self)),
                e => Err(Error::from(e)),
            }
        }
    }

    #[inline]
    pub fn open_as_with<E: traits::Encoder>(
        mut self,
        codec: E,
        options: Dictionary,
    ) -> Result<Encoder, Error> {
        unsafe {
            if let Some(codec) = codec.encoder() {
                let mut opts = options.disown();
                let res = avcodec_open2(self.as_mut_ptr(), codec.as_ptr(), &mut opts);

                Dictionary::own(opts);

                match res {
                    0 => Ok(Encoder(self)),
                    e => Err(Error::from(e)),
                }
            } else {
                Err(Error::EncoderNotFound)
            }
        }
    }

    #[inline]
    pub fn set_width(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).width = value as c_int;
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        unsafe { (*self.as_ptr()).width as u32 }
    }

    #[inline]
    pub fn set_height(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).height = value as c_int;
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        unsafe { (*self.as_ptr()).height as u32 }
    }

    #[inline]
    pub fn set_gop(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).gop_size = value as c_int;
        }
    }

    #[inline]
    pub fn set_format(&mut self, value: format::Pixel) {
        unsafe {
            (*self.as_mut_ptr()).pix_fmt = value.into();
        }
    }

    #[inline]
    pub fn format(&self) -> format::Pixel {
        unsafe { format::Pixel::from((*self.as_ptr()).pix_fmt) }
    }

    #[inline]
    #[cfg(feature = "ff_api_motion_est")]
    pub fn set_motion_estimation(&mut self, value: MotionEstimation) {
        unsafe {
            (*self.as_mut_ptr()).me_method = value.into();
        }
    }

    #[inline]
    pub fn set_max_b_frames(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).max_b_frames = value as c_int;
        }
    }

    #[inline]
    pub fn set_b_quant_factor(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).b_quant_factor = value as c_float;
        }
    }

    #[inline]
    pub fn set_b_quant_offset(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).b_quant_offset = value as c_float;
        }
    }

    #[inline]
    pub fn set_i_quant_factor(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).i_quant_factor = value as c_float;
        }
    }

    #[inline]
    pub fn set_i_quant_offset(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).i_quant_offset = value as c_float;
        }
    }

    #[inline]
    pub fn set_lumi_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).lumi_masking = value as c_float;
        }
    }

    #[inline]
    pub fn set_temporal_cplx_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).temporal_cplx_masking = value as c_float;
        }
    }

    #[inline]
    pub fn set_spatial_cplx_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).spatial_cplx_masking = value as c_float;
        }
    }

    #[inline]
    pub fn set_p_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).p_masking = value as c_float;
        }
    }

    #[inline]
    pub fn set_dark_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).dark_masking = value as c_float;
        }
    }

    #[inline]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn set_prediction(&mut self, value: Prediction) {
        unsafe {
            (*self.as_mut_ptr()).prediction_method = value.into();
        }
    }

    #[inline]
    pub fn set_aspect_ratio<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).sample_aspect_ratio = value.into().into();
        }
    }

    #[inline]
    pub fn set_me_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).me_cmp = value.into();
        }
    }

    #[inline]
    pub fn set_me_sub_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).me_sub_cmp = value.into();
        }
    }

    #[inline]
    pub fn set_mb_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).mb_cmp = value.into();
        }
    }

    #[inline]
    pub fn set_ildct_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).ildct_cmp = value.into();
        }
    }

    #[inline]
    pub fn set_dia_size(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).dia_size = value as c_int;
        }
    }

    #[inline]
    pub fn set_last_predictors(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).last_predictor_count = value as c_int;
        }
    }

    #[inline]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn set_pre_me(&mut self, value: MotionEstimation) {
        unsafe {
            (*self.as_mut_ptr()).pre_me = value.into();
        }
    }

    #[inline]
    pub fn set_me_pre_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).me_pre_cmp = value.into();
        }
    }

    #[inline]
    pub fn set_pre_dia_size(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).pre_dia_size = value as c_int;
        }
    }

    #[inline]
    pub fn set_me_subpel_quality(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).me_subpel_quality = value as c_int;
        }
    }

    #[inline]
    pub fn set_me_range(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).me_range = value as c_int;
        }
    }

    #[inline]
    #[cfg(feature = "ff_api_quant_bias")]
    pub fn set_intra_quant_bias(&mut self, value: Option<usize>) {
        unsafe {
            if let Some(value) = value {
                (*self.as_mut_ptr()).intra_quant_bias = value as c_int;
            } else {
                (*self.as_mut_ptr()).intra_quant_bias = FF_DEFAULT_QUANT_BIAS;
            }
        }
    }

    #[inline]
    #[cfg(feature = "ff_api_quant_bias")]
    pub fn set_inter_quant_bias(&mut self, value: Option<usize>) {
        unsafe {
            if let Some(value) = value {
                (*self.as_mut_ptr()).inter_quant_bias = value as c_int;
            } else {
                (*self.as_mut_ptr()).inter_quant_bias = FF_DEFAULT_QUANT_BIAS;
            }
        }
    }

    #[inline]
    pub fn set_mb_decision(&mut self, value: Decision) {
        unsafe {
            (*self.as_mut_ptr()).mb_decision = value.into();
        }
    }

    #[inline]
    pub fn set_mb_lmin(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).mb_lmin = value as c_int;
        }
    }

    #[inline]
    pub fn set_mb_lmax(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).mb_lmax = value as c_int;
        }
    }

    #[inline]
    pub fn set_intra_dc_precision(&mut self, value: u8) {
        unsafe {
            (*self.as_mut_ptr()).intra_dc_precision = i32::from(value);
        }
    }

    #[inline]
    pub fn set_qmin(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).qmin = value as c_int;
        }
    }

    #[inline]
    pub fn set_qmax(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).qmax = value as c_int;
        }
    }

    #[inline]
    pub fn set_global_quality(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).global_quality = value as c_int;
        }
    }

    #[inline]
    pub fn set_colorspace(&mut self, value: color::Space) {
        unsafe {
            (*self.as_mut_ptr()).colorspace = value.into();
        }
    }

    #[inline]
    pub fn colorspace(&self) -> color::Space {
        unsafe { (*self.as_ptr()).colorspace.into() }
    }

    #[inline]
    pub fn set_color_range(&mut self, value: color::Range) {
        unsafe {
            (*self.as_mut_ptr()).color_range = value.into();
        }
    }

    #[inline]
    pub fn color_range(&self) -> color::Range {
        unsafe { (*self.as_ptr()).color_range.into() }
    }
}

impl Deref for Video {
    type Target = Super;

    #[inline(always)]
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Video {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Video {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Video {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}

pub struct Encoder(pub Video);

impl Encoder {
    #[deprecated(
        since = "4.4.0",
        note = "Underlying API avcodec_encode_video2 has been deprecated since FFmpeg 3.1; \
        consider switching to send_frame() and receive_packet()"
    )]
    #[inline]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn encode<P: packet::Mut>(
        &mut self,
        frame: &frame::Video,
        out: &mut P,
    ) -> Result<bool, Error> {
        unsafe {
            if self.format() != frame.format()
                || self.width() != frame.width()
                || self.height() != frame.height()
            {
                return Err(Error::InvalidData);
            }

            let mut got: c_int = 0;

            match avcodec_encode_video2(
                self.0.as_mut_ptr(),
                out.as_mut_ptr(),
                frame.as_ptr(),
                &mut got,
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }

    #[deprecated(
        since = "4.4.0",
        note = "Underlying API avcodec_encode_video2 has been deprecated since FFmpeg 3.1; \
        consider switching to send_frame() and receive_packet()"
    )]
    #[inline]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn flush<P: packet::Mut>(&mut self, out: &mut P) -> Result<bool, Error> {
        unsafe {
            let mut got: c_int = 0;

            match avcodec_encode_video2(
                self.0.as_mut_ptr(),
                out.as_mut_ptr(),
                ptr::null(),
                &mut got,
            ) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(got != 0),
            }
        }
    }

    #[inline]
    pub fn frame_size(&self) -> u32 {
        unsafe { (*self.as_ptr()).frame_size as u32 }
    }
}

impl Deref for Encoder {
    type Target = Video;

    #[inline]
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    #[inline]
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl AsRef<Context> for Encoder {
    fn as_ref(&self) -> &Context {
        self
    }
}

impl AsMut<Context> for Encoder {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/packet/borrow.rs">
use std::mem;
use std::ptr;

use super::Ref;
use ffi::*;
use libc::c_int;

pub struct Borrow<'a> {
    packet: AVPacket,
    data: &'a [u8],
}

impl<'a> Borrow<'a> {
    pub fn new(data: &[u8]) -> Borrow {
        unsafe {
            let mut packet: AVPacket = mem::zeroed();

            packet.data = data.as_ptr() as *mut _;
            packet.size = data.len() as c_int;

            Borrow { packet, data }
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.packet.size as usize
    }

    #[inline]
    pub fn data(&self) -> Option<&[u8]> {
        Some(self.data)
    }
}

impl<'a> Ref for Borrow<'a> {
    fn as_ptr(&self) -> *const AVPacket {
        &self.packet
    }
}

impl<'a> Drop for Borrow<'a> {
    fn drop(&mut self) {
        unsafe {
            self.packet.data = ptr::null_mut();
            self.packet.size = 0;

            av_packet_unref(&mut self.packet);
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/packet/flag.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const KEY     = AV_PKT_FLAG_KEY;
        const CORRUPT = AV_PKT_FLAG_CORRUPT;
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/packet/mod.rs">
pub mod traits;
pub use self::traits::{Mut, Ref};

pub mod packet;
pub use self::packet::Packet;

pub mod borrow;
pub use self::borrow::Borrow;

pub mod side_data;
pub use self::side_data::SideData;

pub mod flag;
pub use self::flag::Flags;
</file>

<file path="patches/ffmpeg-next/src/codec/packet/packet.rs">
use std::marker::PhantomData;
use std::mem;
use std::slice;

use super::{Borrow, Flags, Mut, Ref, SideData};
use ffi::*;
use libc::c_int;
use {format, Error, Rational};

pub struct Packet(AVPacket);

unsafe impl Send for Packet {}
unsafe impl Sync for Packet {}

impl Packet {
    #[inline(always)]
    pub unsafe fn is_empty(&self) -> bool {
        self.0.size == 0
    }
}

impl Packet {
    #[inline]
    pub fn empty() -> Self {
        unsafe {
            let mut pkt: AVPacket = mem::zeroed();

            av_init_packet(&mut pkt);

            Packet(pkt)
        }
    }

    #[inline]
    pub fn new(size: usize) -> Self {
        unsafe {
            let mut pkt: AVPacket = mem::zeroed();

            av_init_packet(&mut pkt);
            av_new_packet(&mut pkt, size as c_int);

            Packet(pkt)
        }
    }

    #[inline]
    pub fn copy(data: &[u8]) -> Self {
        use std::io::Write;

        let mut packet = Packet::new(data.len());
        packet.data_mut().unwrap().write_all(data).unwrap();

        packet
    }

    #[inline]
    pub fn borrow(data: &[u8]) -> Borrow {
        Borrow::new(data)
    }

    #[inline]
    pub fn shrink(&mut self, size: usize) {
        unsafe {
            av_shrink_packet(&mut self.0, size as c_int);
        }
    }

    #[inline]
    pub fn grow(&mut self, size: usize) {
        unsafe {
            av_grow_packet(&mut self.0, size as c_int);
        }
    }

    #[inline]
    pub fn rescale_ts<S, D>(&mut self, source: S, destination: D)
    where
        S: Into<Rational>,
        D: Into<Rational>,
    {
        unsafe {
            av_packet_rescale_ts(
                self.as_mut_ptr(),
                source.into().into(),
                destination.into().into(),
            );
        }
    }

    #[inline]
    pub fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.0.flags)
    }

    #[inline]
    pub fn set_flags(&mut self, value: Flags) {
        self.0.flags = value.bits();
    }

    #[inline]
    pub fn is_key(&self) -> bool {
        self.flags().contains(Flags::KEY)
    }

    #[inline]
    pub fn is_corrupt(&self) -> bool {
        self.flags().contains(Flags::CORRUPT)
    }

    #[inline]
    pub fn stream(&self) -> usize {
        self.0.stream_index as usize
    }

    #[inline]
    pub fn set_stream(&mut self, index: usize) {
        self.0.stream_index = index as c_int;
    }

    #[inline]
    pub fn pts(&self) -> Option<i64> {
        match self.0.pts {
            AV_NOPTS_VALUE => None,
            pts => Some(pts),
        }
    }

    #[inline]
    pub fn set_pts(&mut self, value: Option<i64>) {
        self.0.pts = value.unwrap_or(AV_NOPTS_VALUE);
    }

    #[inline]
    pub fn dts(&self) -> Option<i64> {
        match self.0.dts {
            AV_NOPTS_VALUE => None,
            dts => Some(dts),
        }
    }

    #[inline]
    pub fn set_dts(&mut self, value: Option<i64>) {
        self.0.dts = value.unwrap_or(AV_NOPTS_VALUE);
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.0.size as usize
    }

    #[inline]
    pub fn duration(&self) -> i64 {
        self.0.duration
    }

    #[inline]
    pub fn set_duration(&mut self, value: i64) {
        self.0.duration = value;
    }

    #[inline]
    pub fn position(&self) -> isize {
        self.0.pos as isize
    }

    #[inline]
    pub fn set_position(&mut self, value: isize) {
        self.0.pos = value as i64
    }

    #[inline]
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn convergence(&self) -> isize {
        self.0.convergence_duration as isize
    }

    #[inline]
    pub fn side_data(&self) -> SideDataIter {
        SideDataIter::new(&self.0)
    }

    #[inline]
    pub fn data(&self) -> Option<&[u8]> {
        unsafe {
            if self.0.data.is_null() {
                None
            } else {
                Some(slice::from_raw_parts(self.0.data, self.0.size as usize))
            }
        }
    }

    #[inline]
    pub fn data_mut(&mut self) -> Option<&mut [u8]> {
        unsafe {
            if self.0.data.is_null() {
                None
            } else {
                Some(slice::from_raw_parts_mut(self.0.data, self.0.size as usize))
            }
        }
    }

    #[inline]
    pub fn read(&mut self, format: &mut format::context::Input) -> Result<(), Error> {
        unsafe {
            match av_read_frame(format.as_mut_ptr(), self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    #[inline]
    pub fn write(&self, format: &mut format::context::Output) -> Result<bool, Error> {
        unsafe {
            if self.is_empty() {
                return Err(Error::InvalidData);
            }

            match av_write_frame(format.as_mut_ptr(), self.as_ptr() as *mut _) {
                1 => Ok(true),
                0 => Ok(false),
                e => Err(Error::from(e)),
            }
        }
    }

    #[inline]
    pub fn write_interleaved(&self, format: &mut format::context::Output) -> Result<(), Error> {
        unsafe {
            if self.is_empty() {
                return Err(Error::InvalidData);
            }

            match av_interleaved_write_frame(format.as_mut_ptr(), self.as_ptr() as *mut _) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Ref for Packet {
    fn as_ptr(&self) -> *const AVPacket {
        &self.0
    }
}

impl Mut for Packet {
    fn as_mut_ptr(&mut self) -> *mut AVPacket {
        &mut self.0
    }
}

impl Clone for Packet {
    #[inline]
    fn clone(&self) -> Self {
        let mut pkt = Packet::empty();
        pkt.clone_from(self);

        pkt
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        #[cfg(feature = "ffmpeg_4_0")]
        unsafe {
            av_packet_ref(&mut self.0, &source.0);
            av_packet_make_writable(&mut self.0);
        }
        #[cfg(not(feature = "ffmpeg_4_0"))]
        unsafe {
            av_copy_packet(&mut self.0, &source.0);
        }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            av_packet_unref(&mut self.0);
        }
    }
}

pub struct SideDataIter<'a> {
    ptr: *const AVPacket,
    cur: c_int,

    _marker: PhantomData<&'a Packet>,
}

impl<'a> SideDataIter<'a> {
    pub fn new(ptr: *const AVPacket) -> Self {
        SideDataIter {
            ptr,
            cur: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for SideDataIter<'a> {
    type Item = SideData<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.cur >= (*self.ptr).side_data_elems {
                None
            } else {
                self.cur += 1;
                Some(SideData::wrap(
                    (*self.ptr).side_data.offset((self.cur - 1) as isize),
                ))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.ptr).side_data_elems as usize;

            (length - self.cur as usize, Some(length - self.cur as usize))
        }
    }
}

impl<'a> ExactSizeIterator for SideDataIter<'a> {}
</file>

<file path="patches/ffmpeg-next/src/codec/packet/side_data.rs">
use std::marker::PhantomData;
use std::slice;

use super::Packet;
use ffi::AVPacketSideDataType::*;
use ffi::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
    Palette,
    NewExtraData,
    ParamChange,
    H263MbInfo,
    ReplayGain,
    DisplayMatrix,
    Stereo3d,
    AudioServiceType,
    QualityStats,
    FallbackTrack,
    CBPProperties,
    SkipSamples,
    JpDualMono,
    StringsMetadata,
    SubtitlePosition,
    MatroskaBlockAdditional,
    WebVTTIdentifier,
    WebVTTSettings,
    MetadataUpdate,
    MPEGTSStreamID,
    MasteringDisplayMetadata,
    DataSpherical,
    DataNb,

    ContentLightLevel,
    A53CC,

    #[cfg(feature = "ffmpeg_4_0")]
    EncryptionInitInfo,
    #[cfg(feature = "ffmpeg_4_0")]
    EncryptionInfo,

    #[cfg(feature = "ffmpeg_4_1")]
    AFD,

    #[cfg(feature = "ffmpeg_4_3")]
    PRFT,
    #[cfg(feature = "ffmpeg_4_3")]
    ICC_PROFILE,
    #[cfg(feature = "ffmpeg_4_3")]
    DOVI_CONF,

    #[cfg(feature = "ffmpeg_4_4")]
    S12M_TIMECODE,

    #[cfg(feature = "ffmpeg_5_0")]
    DYNAMIC_HDR10_PLUS,
}

impl From<AVPacketSideDataType> for Type {
    fn from(value: AVPacketSideDataType) -> Self {
        match value {
            AV_PKT_DATA_PALETTE => Type::Palette,
            AV_PKT_DATA_NEW_EXTRADATA => Type::NewExtraData,
            AV_PKT_DATA_PARAM_CHANGE => Type::ParamChange,
            AV_PKT_DATA_H263_MB_INFO => Type::H263MbInfo,
            AV_PKT_DATA_REPLAYGAIN => Type::ReplayGain,
            AV_PKT_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
            AV_PKT_DATA_STEREO3D => Type::Stereo3d,
            AV_PKT_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV_PKT_DATA_QUALITY_STATS => Type::QualityStats,
            AV_PKT_DATA_FALLBACK_TRACK => Type::FallbackTrack,
            AV_PKT_DATA_CPB_PROPERTIES => Type::CBPProperties,
            AV_PKT_DATA_SKIP_SAMPLES => Type::SkipSamples,
            AV_PKT_DATA_JP_DUALMONO => Type::JpDualMono,
            AV_PKT_DATA_STRINGS_METADATA => Type::StringsMetadata,
            AV_PKT_DATA_SUBTITLE_POSITION => Type::SubtitlePosition,
            AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL => Type::MatroskaBlockAdditional,
            AV_PKT_DATA_WEBVTT_IDENTIFIER => Type::WebVTTIdentifier,
            AV_PKT_DATA_WEBVTT_SETTINGS => Type::WebVTTSettings,
            AV_PKT_DATA_METADATA_UPDATE => Type::MetadataUpdate,
            AV_PKT_DATA_MPEGTS_STREAM_ID => Type::MPEGTSStreamID,
            AV_PKT_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV_PKT_DATA_SPHERICAL => Type::DataSpherical,
            AV_PKT_DATA_NB => Type::DataNb,

            AV_PKT_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV_PKT_DATA_A53_CC => Type::A53CC,

            #[cfg(feature = "ffmpeg_4_0")]
            AV_PKT_DATA_ENCRYPTION_INIT_INFO => Type::EncryptionInitInfo,
            #[cfg(feature = "ffmpeg_4_0")]
            AV_PKT_DATA_ENCRYPTION_INFO => Type::EncryptionInfo,

            #[cfg(feature = "ffmpeg_4_1")]
            AV_PKT_DATA_AFD => Type::AFD,

            #[cfg(feature = "ffmpeg_4_3")]
            AV_PKT_DATA_PRFT => Type::PRFT,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_PKT_DATA_ICC_PROFILE => Type::ICC_PROFILE,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_PKT_DATA_DOVI_CONF => Type::DOVI_CONF,

            #[cfg(feature = "ffmpeg_4_4")]
            AV_PKT_DATA_S12M_TIMECODE => Type::S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_5_0")]
            AV_PKT_DATA_DYNAMIC_HDR10_PLUS => Type::DYNAMIC_HDR10_PLUS,

            // FFmpeg 7.x added new variants not covered by ffmpeg-next 6.1 — ignore them
            #[allow(unreachable_patterns)]
            _ => Type::DataNb,
        }
    }
}

impl From<Type> for AVPacketSideDataType {
    fn from(value: Type) -> AVPacketSideDataType {
        match value {
            Type::Palette => AV_PKT_DATA_PALETTE,
            Type::NewExtraData => AV_PKT_DATA_NEW_EXTRADATA,
            Type::ParamChange => AV_PKT_DATA_PARAM_CHANGE,
            Type::H263MbInfo => AV_PKT_DATA_H263_MB_INFO,
            Type::ReplayGain => AV_PKT_DATA_REPLAYGAIN,
            Type::DisplayMatrix => AV_PKT_DATA_DISPLAYMATRIX,
            Type::Stereo3d => AV_PKT_DATA_STEREO3D,
            Type::AudioServiceType => AV_PKT_DATA_AUDIO_SERVICE_TYPE,
            Type::QualityStats => AV_PKT_DATA_QUALITY_STATS,
            Type::FallbackTrack => AV_PKT_DATA_FALLBACK_TRACK,
            Type::CBPProperties => AV_PKT_DATA_CPB_PROPERTIES,
            Type::SkipSamples => AV_PKT_DATA_SKIP_SAMPLES,
            Type::JpDualMono => AV_PKT_DATA_JP_DUALMONO,
            Type::StringsMetadata => AV_PKT_DATA_STRINGS_METADATA,
            Type::SubtitlePosition => AV_PKT_DATA_SUBTITLE_POSITION,
            Type::MatroskaBlockAdditional => AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL,
            Type::WebVTTIdentifier => AV_PKT_DATA_WEBVTT_IDENTIFIER,
            Type::WebVTTSettings => AV_PKT_DATA_WEBVTT_SETTINGS,
            Type::MetadataUpdate => AV_PKT_DATA_METADATA_UPDATE,
            Type::MPEGTSStreamID => AV_PKT_DATA_MPEGTS_STREAM_ID,
            Type::MasteringDisplayMetadata => AV_PKT_DATA_MASTERING_DISPLAY_METADATA,
            Type::DataSpherical => AV_PKT_DATA_SPHERICAL,
            Type::DataNb => AV_PKT_DATA_NB,

            Type::ContentLightLevel => AV_PKT_DATA_CONTENT_LIGHT_LEVEL,
            Type::A53CC => AV_PKT_DATA_A53_CC,

            #[cfg(feature = "ffmpeg_4_0")]
            Type::EncryptionInitInfo => AV_PKT_DATA_ENCRYPTION_INIT_INFO,
            #[cfg(feature = "ffmpeg_4_0")]
            Type::EncryptionInfo => AV_PKT_DATA_ENCRYPTION_INFO,

            #[cfg(feature = "ffmpeg_4_1")]
            Type::AFD => AV_PKT_DATA_AFD,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::PRFT => AV_PKT_DATA_PRFT,
            #[cfg(feature = "ffmpeg_4_3")]
            Type::ICC_PROFILE => AV_PKT_DATA_ICC_PROFILE,
            #[cfg(feature = "ffmpeg_4_3")]
            Type::DOVI_CONF => AV_PKT_DATA_DOVI_CONF,

            #[cfg(feature = "ffmpeg_4_4")]
            Type::S12M_TIMECODE => AV_PKT_DATA_S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_5_0")]
            Type::DYNAMIC_HDR10_PLUS => AV_PKT_DATA_DYNAMIC_HDR10_PLUS,
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVPacketSideData,

    _marker: PhantomData<&'a Packet>,
}

impl<'a> SideData<'a> {
    pub unsafe fn wrap(ptr: *mut AVPacketSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVPacketSideData {
        self.ptr as *const _
    }
}

impl<'a> SideData<'a> {
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    pub fn data(&self) -> &[u8] {
        #[allow(clippy::unnecessary_cast)]
        unsafe {
            slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize)
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/packet/traits.rs">
use ffi::*;

pub trait Ref {
    fn as_ptr(&self) -> *const AVPacket;
}

pub trait Mut {
    fn as_mut_ptr(&mut self) -> *mut AVPacket;
}
</file>

<file path="patches/ffmpeg-next/src/codec/subtitle/flag.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const FORCED = AV_SUBTITLE_FLAG_FORCED;
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/subtitle/mod.rs">
pub mod flag;
pub use self::flag::Flags;

mod rect;
pub use self::rect::{Ass, Bitmap, Rect, Text};

mod rect_mut;
pub use self::rect_mut::{AssMut, BitmapMut, RectMut, TextMut};

use std::marker::PhantomData;
use std::mem;

use ffi::AVSubtitleType::*;
use ffi::*;
use libc::{c_uint, size_t};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Bitmap,
    Text,
    Ass,
}

impl From<AVSubtitleType> for Type {
    fn from(value: AVSubtitleType) -> Type {
        match value {
            SUBTITLE_NONE => Type::None,
            SUBTITLE_BITMAP => Type::Bitmap,
            SUBTITLE_TEXT => Type::Text,
            SUBTITLE_ASS => Type::Ass,
        }
    }
}

impl From<Type> for AVSubtitleType {
    fn from(value: Type) -> AVSubtitleType {
        match value {
            Type::None => SUBTITLE_NONE,
            Type::Bitmap => SUBTITLE_BITMAP,
            Type::Text => SUBTITLE_TEXT,
            Type::Ass => SUBTITLE_ASS,
        }
    }
}

pub struct Subtitle(AVSubtitle);

impl Subtitle {
    pub unsafe fn as_ptr(&self) -> *const AVSubtitle {
        &self.0
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVSubtitle {
        &mut self.0
    }
}

impl Subtitle {
    pub fn new() -> Self {
        unsafe { Subtitle(mem::zeroed()) }
    }

    pub fn pts(&self) -> Option<i64> {
        match self.0.pts {
            AV_NOPTS_VALUE => None,
            pts => Some(pts),
        }
    }

    pub fn set_pts(&mut self, value: Option<i64>) {
        self.0.pts = value.unwrap_or(AV_NOPTS_VALUE);
    }

    pub fn start(&self) -> u32 {
        self.0.start_display_time
    }

    pub fn set_start(&mut self, value: u32) {
        self.0.start_display_time = value;
    }

    pub fn end(&self) -> u32 {
        self.0.end_display_time
    }

    pub fn set_end(&mut self, value: u32) {
        self.0.end_display_time = value;
    }

    pub fn rects(&self) -> RectIter {
        RectIter::new(&self.0)
    }

    pub fn rects_mut(&mut self) -> RectMutIter {
        RectMutIter::new(&mut self.0)
    }

    pub fn add_rect(&mut self, kind: Type) -> RectMut {
        unsafe {
            self.0.num_rects += 1;
            self.0.rects = av_realloc(
                self.0.rects as *mut _,
                (mem::size_of::<*const AVSubtitleRect>() * self.0.num_rects as usize) as size_t,
            ) as *mut _;

            let rect =
                av_mallocz(mem::size_of::<AVSubtitleRect>() as size_t) as *mut AVSubtitleRect;
            (*rect).type_ = kind.into();

            *self.0.rects.offset((self.0.num_rects - 1) as isize) = rect;

            RectMut::wrap(rect)
        }
    }
}

impl Default for Subtitle {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RectIter<'a> {
    ptr: *const AVSubtitle,
    cur: c_uint,

    _marker: PhantomData<&'a Subtitle>,
}

impl<'a> RectIter<'a> {
    pub fn new(ptr: *const AVSubtitle) -> Self {
        RectIter {
            ptr,
            cur: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for RectIter<'a> {
    type Item = Rect<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.cur >= (*self.ptr).num_rects {
                None
            } else {
                self.cur += 1;
                Some(Rect::wrap(
                    *(*self.ptr).rects.offset((self.cur - 1) as isize),
                ))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.ptr).num_rects as usize;

            (length - self.cur as usize, Some(length - self.cur as usize))
        }
    }
}

impl<'a> ExactSizeIterator for RectIter<'a> {}

pub struct RectMutIter<'a> {
    ptr: *mut AVSubtitle,
    cur: c_uint,

    _marker: PhantomData<&'a Subtitle>,
}

impl<'a> RectMutIter<'a> {
    pub fn new(ptr: *mut AVSubtitle) -> Self {
        RectMutIter {
            ptr,
            cur: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for RectMutIter<'a> {
    type Item = RectMut<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.cur >= (*self.ptr).num_rects {
                None
            } else {
                self.cur += 1;
                Some(RectMut::wrap(
                    *(*self.ptr).rects.offset((self.cur - 1) as isize),
                ))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.ptr).num_rects as usize;

            (length - self.cur as usize, Some(length - self.cur as usize))
        }
    }
}

impl<'a> ExactSizeIterator for RectMutIter<'a> {}
</file>

<file path="patches/ffmpeg-next/src/codec/subtitle/rect_mut.rs">
use std::ffi::CString;
use std::ops::Deref;

use super::{Ass, Bitmap, Flags, Text, Type};
use ffi::*;
use libc::c_int;

pub enum RectMut<'a> {
    None(*mut AVSubtitleRect),
    Bitmap(BitmapMut<'a>),
    Text(TextMut<'a>),
    Ass(AssMut<'a>),
}

impl<'a> RectMut<'a> {
    pub unsafe fn wrap(ptr: *mut AVSubtitleRect) -> Self {
        match Type::from((*ptr).type_) {
            Type::None => RectMut::None(ptr),
            Type::Bitmap => RectMut::Bitmap(BitmapMut::wrap(ptr)),
            Type::Text => RectMut::Text(TextMut::wrap(ptr)),
            Type::Ass => RectMut::Ass(AssMut::wrap(ptr)),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        match *self {
            RectMut::None(ptr) => ptr as *const _,
            RectMut::Bitmap(ref b) => b.as_ptr(),
            RectMut::Text(ref t) => t.as_ptr(),
            RectMut::Ass(ref a) => a.as_ptr(),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVSubtitleRect {
        match *self {
            RectMut::None(ptr) => ptr,
            RectMut::Bitmap(ref mut b) => b.as_mut_ptr(),
            RectMut::Text(ref mut t) => t.as_mut_ptr(),
            RectMut::Ass(ref mut a) => a.as_mut_ptr(),
        }
    }
}

impl<'a> RectMut<'a> {
    pub fn flags(&self) -> Flags {
        unsafe {
            Flags::from_bits_truncate(match *self {
                RectMut::None(ptr) => (*ptr).flags,
                RectMut::Bitmap(ref b) => (*b.as_ptr()).flags,
                RectMut::Text(ref t) => (*t.as_ptr()).flags,
                RectMut::Ass(ref a) => (*a.as_ptr()).flags,
            })
        }
    }
}

pub struct BitmapMut<'a> {
    immutable: Bitmap<'a>,
}

impl<'a> BitmapMut<'a> {
    pub unsafe fn wrap(ptr: *mut AVSubtitleRect) -> Self {
        BitmapMut {
            immutable: Bitmap::wrap(ptr as *const _),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVSubtitleRect {
        self.as_ptr() as *mut _
    }
}

impl<'a> BitmapMut<'a> {
    pub fn set_x(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).x = value as c_int;
        }
    }

    pub fn set_y(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).y = value as c_int;
        }
    }

    pub fn set_width(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).w = value as c_int;
        }
    }

    pub fn set_height(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).h = value as c_int;
        }
    }

    pub fn set_colors(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).nb_colors = value as c_int;
        }
    }
}

impl<'a> Deref for BitmapMut<'a> {
    type Target = Bitmap<'a>;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}

pub struct TextMut<'a> {
    immutable: Text<'a>,
}

impl<'a> TextMut<'a> {
    pub unsafe fn wrap(ptr: *mut AVSubtitleRect) -> Self {
        TextMut {
            immutable: Text::wrap(ptr as *const _),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVSubtitleRect {
        self.as_ptr() as *mut _
    }
}

impl<'a> TextMut<'a> {
    pub fn set(&mut self, value: &str) {
        let value = CString::new(value).unwrap();

        unsafe {
            (*self.as_mut_ptr()).text = av_strdup(value.as_ptr());
        }
    }
}

impl<'a> Deref for TextMut<'a> {
    type Target = Text<'a>;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}

pub struct AssMut<'a> {
    immutable: Ass<'a>,
}

impl<'a> AssMut<'a> {
    pub unsafe fn wrap(ptr: *mut AVSubtitleRect) -> Self {
        AssMut {
            immutable: Ass::wrap(ptr),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVSubtitleRect {
        self.as_ptr() as *mut _
    }
}

impl<'a> AssMut<'a> {
    pub fn set(&mut self, value: &str) {
        let value = CString::new(value).unwrap();

        unsafe {
            (*self.as_mut_ptr()).ass = av_strdup(value.as_ptr());
        }
    }
}

impl<'a> Deref for AssMut<'a> {
    type Target = Ass<'a>;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/subtitle/rect.rs">
use std::ffi::CStr;
use std::marker::PhantomData;
use std::str::from_utf8_unchecked;

use super::{Flags, Type};
use ffi::*;
#[cfg(not(feature = "ffmpeg_5_0"))]
use {format, Picture};

pub enum Rect<'a> {
    None(*const AVSubtitleRect),
    Bitmap(Bitmap<'a>),
    Text(Text<'a>),
    Ass(Ass<'a>),
}

impl<'a> Rect<'a> {
    pub unsafe fn wrap(ptr: *const AVSubtitleRect) -> Self {
        match Type::from((*ptr).type_) {
            Type::None => Rect::None(ptr),
            Type::Bitmap => Rect::Bitmap(Bitmap::wrap(ptr)),
            Type::Text => Rect::Text(Text::wrap(ptr)),
            Type::Ass => Rect::Ass(Ass::wrap(ptr)),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        match *self {
            Rect::None(ptr) => ptr,
            Rect::Bitmap(ref b) => b.as_ptr(),
            Rect::Text(ref t) => t.as_ptr(),
            Rect::Ass(ref a) => a.as_ptr(),
        }
    }
}

impl<'a> Rect<'a> {
    pub fn flags(&self) -> Flags {
        unsafe {
            Flags::from_bits_truncate(match *self {
                Rect::None(ptr) => (*ptr).flags,
                Rect::Bitmap(ref b) => (*b.as_ptr()).flags,
                Rect::Text(ref t) => (*t.as_ptr()).flags,
                Rect::Ass(ref a) => (*a.as_ptr()).flags,
            })
        }
    }
}

pub struct Bitmap<'a> {
    ptr: *const AVSubtitleRect,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Bitmap<'a> {
    pub unsafe fn wrap(ptr: *const AVSubtitleRect) -> Self {
        Bitmap {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        self.ptr
    }
}

impl<'a> Bitmap<'a> {
    pub fn x(&self) -> usize {
        unsafe { (*self.as_ptr()).x as usize }
    }

    pub fn y(&self) -> usize {
        unsafe { (*self.as_ptr()).y as usize }
    }

    pub fn width(&self) -> u32 {
        unsafe { (*self.as_ptr()).w as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.as_ptr()).h as u32 }
    }

    pub fn colors(&self) -> usize {
        unsafe { (*self.as_ptr()).nb_colors as usize }
    }

    // XXX: must split Picture and PictureMut
    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn picture(&self, format: format::Pixel) -> Picture<'a> {
        unsafe {
            Picture::wrap(
                &(*self.as_ptr()).pict as *const _ as *mut _,
                format,
                (*self.as_ptr()).w as u32,
                (*self.as_ptr()).h as u32,
            )
        }
    }
}

pub struct Text<'a> {
    ptr: *const AVSubtitleRect,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Text<'a> {
    pub unsafe fn wrap(ptr: *const AVSubtitleRect) -> Self {
        Text {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        self.ptr
    }
}

impl<'a> Text<'a> {
    pub fn get(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).text).to_bytes()) }
    }
}

pub struct Ass<'a> {
    ptr: *const AVSubtitleRect,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Ass<'a> {
    pub unsafe fn wrap(ptr: *const AVSubtitleRect) -> Self {
        Ass {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVSubtitleRect {
        self.ptr
    }
}

impl<'a> Ass<'a> {
    pub fn get(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).ass).to_bytes()) }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/audio_service.rs">
use ffi::AVAudioServiceType::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum AudioService {
    Main,
    Effects,
    VisuallyImpaired,
    HearingImpaired,
    Dialogue,
    Commentary,
    Emergency,
    VoiceOver,
    Karaoke,
}

impl From<AVAudioServiceType> for AudioService {
    fn from(value: AVAudioServiceType) -> Self {
        match value {
            AV_AUDIO_SERVICE_TYPE_MAIN => AudioService::Main,
            AV_AUDIO_SERVICE_TYPE_EFFECTS => AudioService::Effects,
            AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED => AudioService::VisuallyImpaired,
            AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED => AudioService::HearingImpaired,
            AV_AUDIO_SERVICE_TYPE_DIALOGUE => AudioService::Dialogue,
            AV_AUDIO_SERVICE_TYPE_COMMENTARY => AudioService::Commentary,
            AV_AUDIO_SERVICE_TYPE_EMERGENCY => AudioService::Emergency,
            AV_AUDIO_SERVICE_TYPE_VOICE_OVER => AudioService::VoiceOver,
            AV_AUDIO_SERVICE_TYPE_KARAOKE => AudioService::Karaoke,
            AV_AUDIO_SERVICE_TYPE_NB => AudioService::Main,
        }
    }
}

impl From<AudioService> for AVAudioServiceType {
    fn from(value: AudioService) -> AVAudioServiceType {
        match value {
            AudioService::Main => AV_AUDIO_SERVICE_TYPE_MAIN,
            AudioService::Effects => AV_AUDIO_SERVICE_TYPE_EFFECTS,
            AudioService::VisuallyImpaired => AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED,
            AudioService::HearingImpaired => AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED,
            AudioService::Dialogue => AV_AUDIO_SERVICE_TYPE_DIALOGUE,
            AudioService::Commentary => AV_AUDIO_SERVICE_TYPE_COMMENTARY,
            AudioService::Emergency => AV_AUDIO_SERVICE_TYPE_EMERGENCY,
            AudioService::VoiceOver => AV_AUDIO_SERVICE_TYPE_VOICE_OVER,
            AudioService::Karaoke => AV_AUDIO_SERVICE_TYPE_KARAOKE,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/audio.rs">
use std::ops::Deref;

use super::codec::Codec;
use ffi::*;
use {format, ChannelLayout};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Audio {
    codec: Codec,
}

impl Audio {
    pub unsafe fn new(codec: Codec) -> Audio {
        Audio { codec }
    }
}

impl Audio {
    pub fn rates(&self) -> Option<RateIter> {
        unsafe {
            if (*self.as_ptr()).supported_samplerates.is_null() {
                None
            } else {
                Some(RateIter::new((*self.codec.as_ptr()).supported_samplerates))
            }
        }
    }

    pub fn formats(&self) -> Option<FormatIter> {
        unsafe {
            if (*self.codec.as_ptr()).sample_fmts.is_null() {
                None
            } else {
                Some(FormatIter::new((*self.codec.as_ptr()).sample_fmts))
            }
        }
    }

    pub fn channel_layouts(&self) -> Option<ChannelLayoutIter> {
        unsafe {
            if (*self.codec.as_ptr()).channel_layouts.is_null() {
                None
            } else {
                Some(ChannelLayoutIter::new(
                    (*self.codec.as_ptr()).channel_layouts,
                ))
            }
        }
    }
}

impl Deref for Audio {
    type Target = Codec;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}

pub struct RateIter {
    ptr: *const i32,
}

impl RateIter {
    pub fn new(ptr: *const i32) -> Self {
        RateIter { ptr }
    }
}

impl Iterator for RateIter {
    type Item = i32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == 0 {
                return None;
            }

            let rate = *self.ptr;
            self.ptr = self.ptr.offset(1);

            Some(rate)
        }
    }
}

pub struct FormatIter {
    ptr: *const AVSampleFormat,
}

impl FormatIter {
    pub fn new(ptr: *const AVSampleFormat) -> Self {
        FormatIter { ptr }
    }
}

impl Iterator for FormatIter {
    type Item = format::Sample;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == AVSampleFormat::AV_SAMPLE_FMT_NONE {
                return None;
            }

            let format = (*self.ptr).into();
            self.ptr = self.ptr.offset(1);

            Some(format)
        }
    }
}

pub struct ChannelLayoutIter {
    ptr: *const u64,
}

impl ChannelLayoutIter {
    pub fn new(ptr: *const u64) -> Self {
        ChannelLayoutIter { ptr }
    }

    pub fn best(self, max: i32) -> ChannelLayout {
        self.fold(ChannelLayout::MONO, |acc, cur| {
            if cur.channels() > acc.channels() && cur.channels() <= max {
                cur
            } else {
                acc
            }
        })
    }
}

impl Iterator for ChannelLayoutIter {
    type Item = ChannelLayout;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == 0 {
                return None;
            }

            let layout = ChannelLayout::from_bits_truncate(*self.ptr);
            self.ptr = self.ptr.offset(1);

            Some(layout)
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/capabilities.rs">
use ffi::*;
use libc::c_uint;

bitflags! {
    pub struct Capabilities: c_uint {
        const DRAW_HORIZ_BAND     = AV_CODEC_CAP_DRAW_HORIZ_BAND;
        const DR1                 = AV_CODEC_CAP_DR1;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const TRUNCATED           = AV_CODEC_CAP_TRUNCATED;
        const DELAY               = AV_CODEC_CAP_DELAY;
        const SMALL_LAST_FRAME    = AV_CODEC_CAP_SMALL_LAST_FRAME;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const HWACCEL_VDPAU       = AV_CODEC_CAP_HWACCEL_VDPAU;
        const SUBFRAMES           = AV_CODEC_CAP_SUBFRAMES;
        const EXPERIMENTAL        = AV_CODEC_CAP_EXPERIMENTAL;
        const CHANNEL_CONF        = AV_CODEC_CAP_CHANNEL_CONF;
        const FRAME_THREADS       = AV_CODEC_CAP_FRAME_THREADS;
        const SLICE_THREADS       = AV_CODEC_CAP_SLICE_THREADS;
        const PARAM_CHANGE        = AV_CODEC_CAP_PARAM_CHANGE;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const AUTO_THREADS        = AV_CODEC_CAP_AUTO_THREADS;
        #[cfg(feature = "ffmpeg_6_0")]
        const OTHER_THREADS       = AV_CODEC_CAP_OTHER_THREADS;
        const VARIABLE_FRAME_SIZE = AV_CODEC_CAP_VARIABLE_FRAME_SIZE;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const INTRA_ONLY          = AV_CODEC_CAP_INTRA_ONLY;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const LOSSLESS            = AV_CODEC_CAP_LOSSLESS;
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/codec.rs">
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use super::{Audio, Capabilities, Id, Profile, Video};
use ffi::*;
use {media, Error};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Codec {
    ptr: *mut AVCodec,
}

unsafe impl Send for Codec {}
unsafe impl Sync for Codec {}

impl Codec {
    pub unsafe fn wrap(ptr: *mut AVCodec) -> Self {
        Codec { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodec {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodec {
        self.ptr
    }
}

impl Codec {
    pub fn is_encoder(&self) -> bool {
        unsafe { av_codec_is_encoder(self.as_ptr()) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { av_codec_is_decoder(self.as_ptr()) != 0 }
    }

    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        unsafe {
            let long_name = (*self.as_ptr()).long_name;
            if long_name.is_null() {
                ""
            } else {
                from_utf8_unchecked(CStr::from_ptr(long_name).to_bytes())
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).type_) }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).id) }
    }

    pub fn is_video(&self) -> bool {
        self.medium() == media::Type::Video
    }

    pub fn video(self) -> Result<Video, Error> {
        unsafe {
            if self.medium() == media::Type::Video {
                Ok(Video::new(self))
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn is_audio(&self) -> bool {
        self.medium() == media::Type::Audio
    }

    pub fn audio(self) -> Result<Audio, Error> {
        unsafe {
            if self.medium() == media::Type::Audio {
                Ok(Audio::new(self))
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn max_lowres(&self) -> i32 {
        unsafe { (*self.as_ptr()).max_lowres.into() }
    }

    pub fn capabilities(&self) -> Capabilities {
        unsafe { Capabilities::from_bits_truncate((*self.as_ptr()).capabilities as u32) }
    }

    pub fn profiles(&self) -> Option<ProfileIter> {
        unsafe {
            if (*self.as_ptr()).profiles.is_null() {
                None
            } else {
                Some(ProfileIter::new(self.id(), (*self.as_ptr()).profiles))
            }
        }
    }
}

pub struct ProfileIter {
    id: Id,
    ptr: *const AVProfile,
}

impl ProfileIter {
    pub fn new(id: Id, ptr: *const AVProfile) -> Self {
        ProfileIter { id, ptr }
    }
}

impl Iterator for ProfileIter {
    type Item = Profile;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr).profile == FF_PROFILE_UNKNOWN {
                return None;
            }

            let profile = Profile::from((self.id, (*self.ptr).profile));
            self.ptr = self.ptr.offset(1);

            Some(profile)
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/compliance.rs">
use ffi::*;
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Compliance {
    VeryStrict,
    Strict,
    Normal,
    Unofficial,
    Experimental,
}

impl From<c_int> for Compliance {
    fn from(value: c_int) -> Self {
        match value {
            FF_COMPLIANCE_VERY_STRICT => Compliance::VeryStrict,
            FF_COMPLIANCE_STRICT => Compliance::Strict,
            FF_COMPLIANCE_NORMAL => Compliance::Normal,
            FF_COMPLIANCE_UNOFFICIAL => Compliance::Unofficial,
            FF_COMPLIANCE_EXPERIMENTAL => Compliance::Experimental,

            _ => Compliance::Normal,
        }
    }
}

impl From<Compliance> for c_int {
    fn from(value: Compliance) -> c_int {
        match value {
            Compliance::VeryStrict => FF_COMPLIANCE_VERY_STRICT,
            Compliance::Strict => FF_COMPLIANCE_STRICT,
            Compliance::Normal => FF_COMPLIANCE_NORMAL,
            Compliance::Unofficial => FF_COMPLIANCE_UNOFFICIAL,
            Compliance::Experimental => FF_COMPLIANCE_EXPERIMENTAL,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/context.rs">
use std::any::Any;
use std::ptr;
use std::rc::Rc;

use super::decoder::Decoder;
use super::encoder::Encoder;
use super::{threading, Compliance, Debug, Flags, Id, Parameters};
use ffi::*;
use libc::c_int;
use media;
use {Codec, Error};

pub struct Context {
    ptr: *mut AVCodecContext,
    owner: Option<Rc<dyn Any>>,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: *mut AVCodecContext, owner: Option<Rc<dyn Any>>) -> Self {
        Context { ptr, owner }
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodecContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecContext {
        self.ptr
    }
}

impl Context {
    pub fn new() -> Self {
        unsafe {
            Context {
                ptr: avcodec_alloc_context3(ptr::null()),
                owner: None,
            }
        }
    }

    pub fn from_parameters<P: Into<Parameters>>(parameters: P) -> Result<Self, Error> {
        let parameters = parameters.into();
        let mut context = Self::new();

        unsafe {
            match avcodec_parameters_to_context(context.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(context),
            }
        }
    }

    pub fn decoder(self) -> Decoder {
        Decoder(self)
    }

    pub fn encoder(self) -> Encoder {
        Encoder(self)
    }

    pub fn codec(&self) -> Option<Codec> {
        unsafe {
            if (*self.as_ptr()).codec.is_null() {
                None
            } else {
                Some(Codec::wrap((*self.as_ptr()).codec as *mut _))
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }

    pub fn set_flags(&mut self, value: Flags) {
        unsafe {
            (*self.as_mut_ptr()).flags = value.bits() as c_int;
        }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).codec_id) }
    }

    pub fn compliance(&mut self, value: Compliance) {
        unsafe {
            (*self.as_mut_ptr()).strict_std_compliance = value.into();
        }
    }

    pub fn debug(&mut self, value: Debug) {
        unsafe {
            (*self.as_mut_ptr()).debug = value.bits();
        }
    }

    pub fn set_threading(&mut self, config: threading::Config) {
        unsafe {
            (*self.as_mut_ptr()).thread_type = config.kind.into();
            (*self.as_mut_ptr()).thread_count = config.count as c_int;
            #[cfg(not(feature = "ffmpeg_6_0"))]
            {
                (*self.as_mut_ptr()).thread_safe_callbacks = if config.safe { 1 } else { 0 };
            }
        }
    }

    pub fn threading(&self) -> threading::Config {
        unsafe {
            threading::Config {
                kind: threading::Type::from((*self.as_ptr()).active_thread_type),
                count: (*self.as_ptr()).thread_count as usize,
                #[cfg(not(feature = "ffmpeg_6_0"))]
                safe: (*self.as_ptr()).thread_safe_callbacks != 0,
            }
        }
    }

    pub fn set_parameters<P: Into<Parameters>>(&mut self, parameters: P) -> Result<(), Error> {
        let parameters = parameters.into();

        unsafe {
            match avcodec_parameters_to_context(self.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from(e)),
                _ => Ok(()),
            }
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                avcodec_free_context(&mut self.as_mut_ptr());
            }
        }
    }
}

#[cfg(not(feature = "ffmpeg_5_0"))]
impl Clone for Context {
    fn clone(&self) -> Self {
        let mut ctx = Context::new();
        ctx.clone_from(self);

        ctx
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            // Removed in ffmpeg >= 5.0.
            avcodec_copy_context(self.as_mut_ptr(), source.as_ptr());
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/debug.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Debug: c_int {
        const PICT_INFO   = FF_DEBUG_PICT_INFO;
        const RC          = FF_DEBUG_RC;
        const BITSTREAM   = FF_DEBUG_BITSTREAM;
        const MB_TYPE     = FF_DEBUG_MB_TYPE;
        const QP          = FF_DEBUG_QP;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const MV          = FF_DEBUG_MV;
        const DCT_COEFF   = FF_DEBUG_DCT_COEFF;
        const SKIP        = FF_DEBUG_SKIP;
        const STARTCODE   = FF_DEBUG_STARTCODE;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const PTS         = FF_DEBUG_PTS;
        const ER          = FF_DEBUG_ER;
        const MMCO        = FF_DEBUG_MMCO;
        const BUGS        = FF_DEBUG_BUGS;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const VIS_QP      = FF_DEBUG_VIS_QP;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const VIS_MB_TYPE = FF_DEBUG_VIS_MB_TYPE;
        const BUFFERS     = FF_DEBUG_BUFFERS;
        const THREADS     = FF_DEBUG_THREADS;
        const NOMC        = FF_DEBUG_NOMC;
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/discard.rs">
use ffi::AVDiscard::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Discard {
    None,
    Default,
    NonReference,
    Bidirectional,
    NonIntra,
    NonKey,
    All,
}

impl From<AVDiscard> for Discard {
    fn from(value: AVDiscard) -> Self {
        match value {
            AVDISCARD_NONE => Discard::None,
            AVDISCARD_DEFAULT => Discard::Default,
            AVDISCARD_NONREF => Discard::NonReference,
            AVDISCARD_BIDIR => Discard::Bidirectional,
            AVDISCARD_NONINTRA => Discard::NonIntra,
            AVDISCARD_NONKEY => Discard::NonKey,
            AVDISCARD_ALL => Discard::All,
        }
    }
}

impl From<Discard> for AVDiscard {
    fn from(value: Discard) -> AVDiscard {
        match value {
            Discard::None => AVDISCARD_NONE,
            Discard::Default => AVDISCARD_DEFAULT,
            Discard::NonReference => AVDISCARD_NONREF,
            Discard::Bidirectional => AVDISCARD_BIDIR,
            Discard::NonIntra => AVDISCARD_NONINTRA,
            Discard::NonKey => AVDISCARD_NONKEY,
            Discard::All => AVDISCARD_ALL,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/field_order.rs">
use ffi::AVFieldOrder::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FieldOrder {
    Unknown,
    Progressive,
    TT,
    BB,
    TB,
    BT,
}

impl From<AVFieldOrder> for FieldOrder {
    fn from(value: AVFieldOrder) -> Self {
        match value {
            AV_FIELD_UNKNOWN => FieldOrder::Unknown,
            AV_FIELD_PROGRESSIVE => FieldOrder::Progressive,
            AV_FIELD_TT => FieldOrder::TT,
            AV_FIELD_BB => FieldOrder::BB,
            AV_FIELD_TB => FieldOrder::TB,
            AV_FIELD_BT => FieldOrder::BT,
        }
    }
}

impl From<FieldOrder> for AVFieldOrder {
    fn from(value: FieldOrder) -> AVFieldOrder {
        match value {
            FieldOrder::Unknown => AV_FIELD_UNKNOWN,
            FieldOrder::Progressive => AV_FIELD_PROGRESSIVE,
            FieldOrder::TT => AV_FIELD_TT,
            FieldOrder::BB => AV_FIELD_BB,
            FieldOrder::TB => AV_FIELD_TB,
            FieldOrder::BT => AV_FIELD_BT,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/flag.rs">
use ffi::*;
use libc::c_uint;

bitflags! {
    pub struct Flags: c_uint {
        const UNALIGNED       = AV_CODEC_FLAG_UNALIGNED;
        const QSCALE          = AV_CODEC_FLAG_QSCALE;
        const _4MV            = AV_CODEC_FLAG_4MV;
        const OUTPUT_CORRUPT  = AV_CODEC_FLAG_OUTPUT_CORRUPT;
        const QPEL            = AV_CODEC_FLAG_QPEL;
        const PASS1           = AV_CODEC_FLAG_PASS1;
        const PASS2           = AV_CODEC_FLAG_PASS2;
        const GRAY            = AV_CODEC_FLAG_GRAY;
        const PSNR            = AV_CODEC_FLAG_PSNR;
        #[cfg(not(feature = "ffmpeg_6_0"))]
        const TRUNCATED       = AV_CODEC_FLAG_TRUNCATED;
        const INTERLACED_DCT  = AV_CODEC_FLAG_INTERLACED_DCT;
        const LOW_DELAY       = AV_CODEC_FLAG_LOW_DELAY;
        const GLOBAL_HEADER   = AV_CODEC_FLAG_GLOBAL_HEADER;
        const BITEXACT        = AV_CODEC_FLAG_BITEXACT;
        const AC_PRED         = AV_CODEC_FLAG_AC_PRED;
        const LOOP_FILTER     = AV_CODEC_FLAG_LOOP_FILTER;
        const INTERLACED_ME   = AV_CODEC_FLAG_INTERLACED_ME;
        const CLOSED_GOP      = AV_CODEC_FLAG_CLOSED_GOP;
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/id.rs">
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVCodecID::*;
use ffi::*;
use util::media;

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Id {
    None,

    // video codecs
    MPEG1VIDEO,
    MPEG2VIDEO,
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    MPEG2VIDEO_XVMC,
    H261,
    H263,
    RV10,
    RV20,
    MJPEG,
    MJPEGB,
    LJPEG,
    SP5X,
    JPEGLS,
    MPEG4,
    RAWVIDEO,
    MSMPEG4V1,
    MSMPEG4V2,
    MSMPEG4V3,
    WMV1,
    WMV2,
    H263P,
    H263I,
    FLV1,
    SVQ1,
    SVQ3,
    DVVIDEO,
    HUFFYUV,
    CYUV,
    H264,
    INDEO3,
    VP3,
    THEORA,
    ASV1,
    ASV2,
    FFV1,
    XM4,
    VCR1,
    CLJR,
    MDEC,
    ROQ,
    INTERPLAY_VIDEO,
    XAN_WC3,
    XAN_WC4,
    RPZA,
    CINEPAK,
    WS_VQA,
    MSRLE,
    MSVIDEO1,
    IDCIN,
    BPS8,
    SMC,
    FLIC,
    TRUEMOTION1,
    VMDVIDEO,
    MSZH,
    ZLIB,
    QTRLE,
    TSCC,
    ULTI,
    QDRAW,
    VIXL,
    QPEG,
    PNG,
    PPM,
    PBM,
    PGM,
    PGMYUV,
    PAM,
    FFVHUFF,
    RV30,
    RV40,
    VC1,
    WMV3,
    LOCO,
    WNV1,
    AASC,
    INDEO2,
    FRAPS,
    TRUEMOTION2,
    BMP,
    CSCD,
    MMVIDEO,
    ZMBV,
    AVS,
    SMACKVIDEO,
    NUV,
    KMVC,
    FLASHSV,
    CAVS,
    JPEG2000,
    VMNC,
    VP5,
    VP6,
    VP6F,
    TARGA,
    DSICINVIDEO,
    TIERTEXSEQVIDEO,
    TIFF,
    GIF,
    DXA,
    DNXHD,
    THP,
    SGI,
    C93,
    BETHSOFTVID,
    PTX,
    TXD,
    VP6A,
    AMV,
    VB,
    PCX,
    SUNRAST,
    INDEO4,
    INDEO5,
    MIMIC,
    RL2,
    ESCAPE124,
    DIRAC,
    BFI,
    CMV,
    MOTIONPIXELS,
    TGV,
    TGQ,
    TQI,
    AURA,
    AURA2,
    V210X,
    TMV,
    V210,
    DPX,
    MAD,
    FRWU,
    FLASHSV2,
    CDGRAPHICS,
    R210,
    ANM,
    BINKVIDEO,
    IFF_ILBM,
    IFF_BYTERUN1,
    KGV1,
    YOP,
    VP8,
    PICTOR,
    ANSI,
    A64_MULTI,
    A64_MULTI5,
    R10K,
    MXPEG,
    LAGARITH,
    PRORES,
    JV,
    DFA,
    WMV3IMAGE,
    VC1IMAGE,
    UTVIDEO,
    BMV_VIDEO,
    VBLE,
    DXTORY,
    V410,
    XWD,
    CDXL,
    XBM,
    ZEROCODEC,
    MSS1,
    MSA1,
    TSCC2,
    MTS2,
    CLLC,
    MSS2,
    VP9,
    AIC,
    ESCAPE130,
    G2M,
    WEBP,
    HNM4_VIDEO,
    HEVC,
    H265,
    FIC,
    ALIAS_PIX,
    BRENDER_PIX,
    PAF_VIDEO,
    EXR,
    VP7,
    SANM,
    SGIRLE,
    MVC1,
    MVC2,
    HQX,
    TDSC,
    HQ_HQA,
    HAP,
    DDS,
    DXV,
    SCREENPRESSO,
    RSCC,

    Y41P,
    AVRP,
    V012,
    AVUI,
    AYUV,
    TARGA_Y216,
    V308,
    V408,
    YUV4,
    AVRN,
    CPIA,
    XFACE,
    SNOW,
    SMVJPEG,
    APNG,
    DAALA,
    CFHD,
    TRUEMOTION2RT,
    M101,
    MAGICYUV,
    SHEERVIDEO,
    YLC,

    // various PCM "codecs"
    PCM_S16LE,
    PCM_S16BE,
    PCM_U16LE,
    PCM_U16BE,
    PCM_S8,
    PCM_U8,
    PCM_MULAW,
    PCM_ALAW,
    PCM_S32LE,
    PCM_S32BE,
    PCM_U32LE,
    PCM_U32BE,
    PCM_S24LE,
    PCM_S24BE,
    PCM_U24LE,
    PCM_U24BE,
    PCM_S24DAUD,
    PCM_ZORK,
    PCM_S16LE_PLANAR,
    PCM_DVD,
    PCM_F32BE,
    PCM_F32LE,
    PCM_F64BE,
    PCM_F64LE,
    PCM_BLURAY,
    PCM_LXF,
    S302M,
    PCM_S8_PLANAR,
    PCM_S24LE_PLANAR,
    PCM_S32LE_PLANAR,
    PCM_S16BE_PLANAR,

    PCM_S64LE,
    PCM_S64BE,

    // various ADPCM codecs
    ADPCM_IMA_QT,
    ADPCM_IMA_WAV,
    ADPCM_IMA_DK3,
    ADPCM_IMA_DK4,
    ADPCM_IMA_WS,
    ADPCM_IMA_SMJPEG,
    ADPCM_MS,
    ADPCM_4XM,
    ADPCM_XA,
    ADPCM_ADX,
    ADPCM_EA,
    ADPCM_G726,
    ADPCM_CT,
    ADPCM_SWF,
    ADPCM_YAMAHA,
    ADPCM_SBPRO_4,
    ADPCM_SBPRO_3,
    ADPCM_SBPRO_2,
    ADPCM_THP,
    ADPCM_IMA_AMV,
    ADPCM_EA_R1,
    ADPCM_EA_R3,
    ADPCM_EA_R2,
    ADPCM_IMA_EA_SEAD,
    ADPCM_IMA_EA_EACS,
    ADPCM_EA_XAS,
    ADPCM_EA_MAXIS_XA,
    ADPCM_IMA_ISS,
    ADPCM_G722,
    ADPCM_IMA_APC,
    ADPCM_VIMA,

    ADPCM_AFC,
    ADPCM_IMA_OKI,
    ADPCM_DTK,
    ADPCM_IMA_RAD,
    ADPCM_G726LE,
    ADPCM_THP_LE,
    ADPCM_PSX,
    ADPCM_AICA,
    ADPCM_IMA_DAT4,
    ADPCM_MTAF,

    // AMR
    AMR_NB,
    AMR_WB,

    // RealAudio codecs
    RA_144,
    RA_288,

    // various DPCM codecs
    ROQ_DPCM,
    INTERPLAY_DPCM,
    XAN_DPCM,
    SOL_DPCM,

    SDX2_DPCM,

    // audio codecs
    MP2,
    MP3,
    AAC,
    AC3,
    DTS,
    VORBIS,
    DVAUDIO,
    WMAV1,
    WMAV2,
    MACE3,
    MACE6,
    VMDAUDIO,
    FLAC,
    MP3ADU,
    MP3ON4,
    SHORTEN,
    ALAC,
    WESTWOOD_SND1,
    GSM,
    QDM2,
    COOK,
    TRUESPEECH,
    TTA,
    SMACKAUDIO,
    QCELP,
    WAVPACK,
    DSICINAUDIO,
    IMC,
    MUSEPACK7,
    MLP,
    GSM_MS,
    ATRAC3,
    #[cfg(feature = "ff_api_voxware")]
    VOXWARE,
    APE,
    NELLYMOSER,
    MUSEPACK8,
    SPEEX,
    WMAVOICE,
    WMAPRO,
    WMALOSSLESS,
    ATRAC3P,
    EAC3,
    SIPR,
    MP1,
    TWINVQ,
    TRUEHD,
    MP4ALS,
    ATRAC1,
    BINKAUDIO_RDFT,
    BINKAUDIO_DCT,
    AAC_LATM,
    QDMC,
    CELT,
    G723_1,
    G729,
    SVX_EXP8,
    SVX_FIB8,
    BMV_AUDIO,
    RALF,
    IAC,
    ILBC,
    OPUS,
    COMFORT_NOISE,
    TAK,
    METASOUND,
    PAF_AUDIO,
    ON2AVC,
    DSS_SP,

    #[cfg(feature = "ffmpeg_4_0")]
    CODEC2,
    FFWAVESYNTH,
    SONIC,
    SONIC_LS,
    EVRC,
    SMV,
    DSD_LSBF,
    DSD_MSBF,
    DSD_LSBF_PLANAR,
    DSD_MSBF_PLANAR,
    _4GV,
    INTERPLAY_ACM,
    XMA1,
    XMA2,
    DST,

    // subtitle codecs
    DVD_SUBTITLE,
    DVB_SUBTITLE,
    TEXT,
    XSUB,
    SSA,
    MOV_TEXT,
    HDMV_PGS_SUBTITLE,
    DVB_TELETEXT,
    SRT,

    MICRODVD,
    EIA_608,
    JACOSUB,
    SAMI,
    REALTEXT,
    STL,
    SUBVIEWER1,
    SUBVIEWER,
    SUBRIP,
    WEBVTT,
    MPL2,
    VPLAYER,
    PJS,
    ASS,
    HDMV_TEXT_SUBTITLE,

    // other specific kind of codecs (generally used for attachments)
    TTF,

    SCTE_35,
    BINTEXT,
    XBIN,
    IDF,
    OTF,
    SMPTE_KLV,
    DVD_NAV,
    TIMED_ID3,
    BIN_DATA,

    PROBE,

    MPEG2TS,
    MPEG4SYSTEMS,
    FFMETADATA,
    WRAPPED_AVFRAME,

    PSD,
    PIXLET,
    SPEEDHQ,
    CLEARVIDEO,
    FMVC,
    SCPR,
    XPM,
    AV1,
    PCM_F16LE,
    PCM_F24LE,
    ATRAC3AL,
    ATRAC3PAL,

    BITPACKED,
    MSCC,
    SRGC,
    SVG,
    GDV,
    FITS,
    GREMLIN_DPCM,
    DOLBY_E,

    #[cfg(feature = "ffmpeg_4_0")]
    APTX,
    #[cfg(feature = "ffmpeg_4_0")]
    APTX_HD,
    #[cfg(feature = "ffmpeg_4_0")]
    SBC,

    #[cfg(feature = "ffmpeg_4_1")]
    AVS2,
    #[cfg(feature = "ffmpeg_4_1")]
    IMM4,
    #[cfg(feature = "ffmpeg_4_1")]
    PROSUMER,
    #[cfg(feature = "ffmpeg_4_1")]
    MWSC,
    #[cfg(feature = "ffmpeg_4_1")]
    WCMV,
    #[cfg(feature = "ffmpeg_4_1")]
    RASC,
    #[cfg(feature = "ffmpeg_4_1")]
    PCM_VIDC,
    #[cfg(feature = "ffmpeg_4_1")]
    ATRAC9,
    #[cfg(feature = "ffmpeg_4_1")]
    TTML,

    #[cfg(feature = "ffmpeg_4_2")]
    HYMT,
    #[cfg(feature = "ffmpeg_4_2")]
    ARBC,
    #[cfg(feature = "ffmpeg_4_2")]
    AGM,
    #[cfg(feature = "ffmpeg_4_2")]
    LSCR,
    #[cfg(feature = "ffmpeg_4_2")]
    VP4,
    #[cfg(feature = "ffmpeg_4_2")]
    ADPCM_AGM,
    #[cfg(feature = "ffmpeg_4_2")]
    HCOM,
    #[cfg(feature = "ffmpeg_4_2")]
    ARIB_CAPTION,

    #[cfg(feature = "ffmpeg_4_3")]
    IMM5,
    #[cfg(feature = "ffmpeg_4_3")]
    MVDV,
    #[cfg(feature = "ffmpeg_4_3")]
    MVHA,
    #[cfg(feature = "ffmpeg_4_3")]
    CDTOONS,
    #[cfg(feature = "ffmpeg_4_3")]
    MV30,
    #[cfg(feature = "ffmpeg_4_3")]
    NOTCHLC,
    #[cfg(feature = "ffmpeg_4_3")]
    PFM,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_ARGO,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_SSI,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_ZORK,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_APM,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_ALP,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_MTF,
    #[cfg(feature = "ffmpeg_4_3")]
    ADPCM_IMA_CUNNING,
    #[cfg(feature = "ffmpeg_4_3")]
    DERF_DPCM,
    #[cfg(feature = "ffmpeg_4_3")]
    ACELP_KELVIN,
    #[cfg(feature = "ffmpeg_4_3")]
    MPEGH_3D_AUDIO,
    #[cfg(feature = "ffmpeg_4_3")]
    SIREN,
    #[cfg(feature = "ffmpeg_4_3")]
    HCA,
    #[cfg(feature = "ffmpeg_4_3")]
    EPG,

    #[cfg(feature = "ffmpeg_4_4")]
    AVS3,
    #[cfg(feature = "ffmpeg_4_4")]
    PGX,
    #[cfg(feature = "ffmpeg_4_4")]
    MSP2,
    #[cfg(feature = "ffmpeg_4_4")]
    VVC,
    #[cfg(feature = "ffmpeg_4_4")]
    MOBICLIP,
    #[cfg(feature = "ffmpeg_4_4")]
    PHOTOCD,
    #[cfg(feature = "ffmpeg_4_4")]
    ARGO,
    #[cfg(feature = "ffmpeg_4_4")]
    CRI,
    #[cfg(feature = "ffmpeg_4_4")]
    IPU,
    #[cfg(feature = "ffmpeg_4_4")]
    SIMBIOSIS_IMX,
    #[cfg(feature = "ffmpeg_4_4")]
    SGA_VIDEO,
    #[cfg(feature = "ffmpeg_4_4")]
    PCM_SGA,
    #[cfg(feature = "ffmpeg_4_4")]
    ADPCM_IMA_MOFLEX,
    #[cfg(feature = "ffmpeg_4_4")]
    FASTAUDIO,

    #[cfg(feature = "ffmpeg_5_0")]
    GEM,
    #[cfg(feature = "ffmpeg_5_0")]
    ADPCM_IMA_ACORN,
    #[cfg(feature = "ffmpeg_5_0")]
    MSNSIREN,

    #[cfg(feature = "ffmpeg_5_1")]
    VBN,
    #[cfg(feature = "ffmpeg_5_1")]
    JPEGXL,
    #[cfg(feature = "ffmpeg_5_1")]
    QOI,
    #[cfg(feature = "ffmpeg_5_1")]
    PHM,
    #[cfg(feature = "ffmpeg_5_1")]
    DFPWM,

    #[cfg(feature = "ffmpeg_6_0")]
    RADIANCE_HDR,
    #[cfg(feature = "ffmpeg_6_0")]
    WBMP,
    #[cfg(feature = "ffmpeg_6_0")]
    MEDIA100,
    #[cfg(feature = "ffmpeg_6_0")]
    VQC,
    #[cfg(feature = "ffmpeg_6_0")]
    ADPCM_XMD,
    #[cfg(feature = "ffmpeg_6_0")]
    WADY_DPCM,
    #[cfg(feature = "ffmpeg_6_0")]
    CBD2_DPCM,
    #[cfg(feature = "ffmpeg_6_0")]
    BONK,
    #[cfg(feature = "ffmpeg_6_0")]
    MISC4,
    #[cfg(feature = "ffmpeg_6_0")]
    APAC,
    #[cfg(feature = "ffmpeg_6_0")]
    FTR,
    #[cfg(feature = "ffmpeg_6_0")]
    WAVARC,
    #[cfg(feature = "ffmpeg_6_0")]
    RKA,
    #[cfg(feature = "ffmpeg_6_0")]
    VNULL,
    #[cfg(feature = "ffmpeg_6_0")]
    ANULL,

    #[cfg(feature = "ffmpeg_6_1")]
    PDV,
    #[cfg(feature = "ffmpeg_6_1")]
    EVC,
    #[cfg(feature = "ffmpeg_6_1")]
    RTV1,
    #[cfg(feature = "ffmpeg_6_1")]
    VMIX,
    #[cfg(feature = "ffmpeg_6_1")]
    AC4,
    #[cfg(feature = "ffmpeg_6_1")]
    SMPTE_2038,
    #[cfg(feature = "ffmpeg_6_1")]
    OSQ,
}

impl Id {
    #[cfg(feature = "ff_api_vima_decoder")]
    pub const VIMA: Id = Id::ADPCM_VIMA;

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from(avcodec_get_type((*self).into())) }
    }

    pub fn name(&self) -> &'static str {
        unsafe { from_utf8_unchecked(CStr::from_ptr(avcodec_get_name((*self).into())).to_bytes()) }
    }
}

impl From<AVCodecID> for Id {
    fn from(value: AVCodecID) -> Self {
        match value {
            AV_CODEC_ID_NONE => Id::None,

            /* video codecs */
            AV_CODEC_ID_MPEG1VIDEO => Id::MPEG1VIDEO,
            AV_CODEC_ID_MPEG2VIDEO => Id::MPEG2VIDEO,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            AV_CODEC_ID_MPEG2VIDEO_XVMC => Id::MPEG2VIDEO_XVMC,
            AV_CODEC_ID_H261 => Id::H261,
            AV_CODEC_ID_H263 => Id::H263,
            AV_CODEC_ID_RV10 => Id::RV10,
            AV_CODEC_ID_RV20 => Id::RV20,
            AV_CODEC_ID_MJPEG => Id::MJPEG,
            AV_CODEC_ID_MJPEGB => Id::MJPEGB,
            AV_CODEC_ID_LJPEG => Id::LJPEG,
            AV_CODEC_ID_SP5X => Id::SP5X,
            AV_CODEC_ID_JPEGLS => Id::JPEGLS,
            AV_CODEC_ID_MPEG4 => Id::MPEG4,
            AV_CODEC_ID_RAWVIDEO => Id::RAWVIDEO,
            AV_CODEC_ID_MSMPEG4V1 => Id::MSMPEG4V1,
            AV_CODEC_ID_MSMPEG4V2 => Id::MSMPEG4V2,
            AV_CODEC_ID_MSMPEG4V3 => Id::MSMPEG4V3,
            AV_CODEC_ID_WMV1 => Id::WMV1,
            AV_CODEC_ID_WMV2 => Id::WMV2,
            AV_CODEC_ID_H263P => Id::H263P,
            AV_CODEC_ID_H263I => Id::H263I,
            AV_CODEC_ID_FLV1 => Id::FLV1,
            AV_CODEC_ID_SVQ1 => Id::SVQ1,
            AV_CODEC_ID_SVQ3 => Id::SVQ3,
            AV_CODEC_ID_DVVIDEO => Id::DVVIDEO,
            AV_CODEC_ID_HUFFYUV => Id::HUFFYUV,
            AV_CODEC_ID_CYUV => Id::CYUV,
            AV_CODEC_ID_H264 => Id::H264,
            AV_CODEC_ID_INDEO3 => Id::INDEO3,
            AV_CODEC_ID_VP3 => Id::VP3,
            AV_CODEC_ID_THEORA => Id::THEORA,
            AV_CODEC_ID_ASV1 => Id::ASV1,
            AV_CODEC_ID_ASV2 => Id::ASV2,
            AV_CODEC_ID_FFV1 => Id::FFV1,
            AV_CODEC_ID_4XM => Id::XM4,
            AV_CODEC_ID_VCR1 => Id::VCR1,
            AV_CODEC_ID_CLJR => Id::CLJR,
            AV_CODEC_ID_MDEC => Id::MDEC,
            AV_CODEC_ID_ROQ => Id::ROQ,
            AV_CODEC_ID_INTERPLAY_VIDEO => Id::INTERPLAY_VIDEO,
            AV_CODEC_ID_XAN_WC3 => Id::XAN_WC3,
            AV_CODEC_ID_XAN_WC4 => Id::XAN_WC4,
            AV_CODEC_ID_RPZA => Id::RPZA,
            AV_CODEC_ID_CINEPAK => Id::CINEPAK,
            AV_CODEC_ID_WS_VQA => Id::WS_VQA,
            AV_CODEC_ID_MSRLE => Id::MSRLE,
            AV_CODEC_ID_MSVIDEO1 => Id::MSVIDEO1,
            AV_CODEC_ID_IDCIN => Id::IDCIN,
            AV_CODEC_ID_8BPS => Id::BPS8,
            AV_CODEC_ID_SMC => Id::SMC,
            AV_CODEC_ID_FLIC => Id::FLIC,
            AV_CODEC_ID_TRUEMOTION1 => Id::TRUEMOTION1,
            AV_CODEC_ID_VMDVIDEO => Id::VMDVIDEO,
            AV_CODEC_ID_MSZH => Id::MSZH,
            AV_CODEC_ID_ZLIB => Id::ZLIB,
            AV_CODEC_ID_QTRLE => Id::QTRLE,
            AV_CODEC_ID_TSCC => Id::TSCC,
            AV_CODEC_ID_ULTI => Id::ULTI,
            AV_CODEC_ID_QDRAW => Id::QDRAW,
            AV_CODEC_ID_VIXL => Id::VIXL,
            AV_CODEC_ID_QPEG => Id::QPEG,
            AV_CODEC_ID_PNG => Id::PNG,
            AV_CODEC_ID_PPM => Id::PPM,
            AV_CODEC_ID_PBM => Id::PBM,
            AV_CODEC_ID_PGM => Id::PGM,
            AV_CODEC_ID_PGMYUV => Id::PGMYUV,
            AV_CODEC_ID_PAM => Id::PAM,
            AV_CODEC_ID_FFVHUFF => Id::FFVHUFF,
            AV_CODEC_ID_RV30 => Id::RV30,
            AV_CODEC_ID_RV40 => Id::RV40,
            AV_CODEC_ID_VC1 => Id::VC1,
            AV_CODEC_ID_WMV3 => Id::WMV3,
            AV_CODEC_ID_LOCO => Id::LOCO,
            AV_CODEC_ID_WNV1 => Id::WNV1,
            AV_CODEC_ID_AASC => Id::AASC,
            AV_CODEC_ID_INDEO2 => Id::INDEO2,
            AV_CODEC_ID_FRAPS => Id::FRAPS,
            AV_CODEC_ID_TRUEMOTION2 => Id::TRUEMOTION2,
            AV_CODEC_ID_BMP => Id::BMP,
            AV_CODEC_ID_CSCD => Id::CSCD,
            AV_CODEC_ID_MMVIDEO => Id::MMVIDEO,
            AV_CODEC_ID_ZMBV => Id::ZMBV,
            AV_CODEC_ID_AVS => Id::AVS,
            AV_CODEC_ID_SMACKVIDEO => Id::SMACKVIDEO,
            AV_CODEC_ID_NUV => Id::NUV,
            AV_CODEC_ID_KMVC => Id::KMVC,
            AV_CODEC_ID_FLASHSV => Id::FLASHSV,
            AV_CODEC_ID_CAVS => Id::CAVS,
            AV_CODEC_ID_JPEG2000 => Id::JPEG2000,
            AV_CODEC_ID_VMNC => Id::VMNC,
            AV_CODEC_ID_VP5 => Id::VP5,
            AV_CODEC_ID_VP6 => Id::VP6,
            AV_CODEC_ID_VP6F => Id::VP6F,
            AV_CODEC_ID_TARGA => Id::TARGA,
            AV_CODEC_ID_DSICINVIDEO => Id::DSICINVIDEO,
            AV_CODEC_ID_TIERTEXSEQVIDEO => Id::TIERTEXSEQVIDEO,
            AV_CODEC_ID_TIFF => Id::TIFF,
            AV_CODEC_ID_GIF => Id::GIF,
            AV_CODEC_ID_DXA => Id::DXA,
            AV_CODEC_ID_DNXHD => Id::DNXHD,
            AV_CODEC_ID_THP => Id::THP,
            AV_CODEC_ID_SGI => Id::SGI,
            AV_CODEC_ID_C93 => Id::C93,
            AV_CODEC_ID_BETHSOFTVID => Id::BETHSOFTVID,
            AV_CODEC_ID_PTX => Id::PTX,
            AV_CODEC_ID_TXD => Id::TXD,
            AV_CODEC_ID_VP6A => Id::VP6A,
            AV_CODEC_ID_AMV => Id::AMV,
            AV_CODEC_ID_VB => Id::VB,
            AV_CODEC_ID_PCX => Id::PCX,
            AV_CODEC_ID_SUNRAST => Id::SUNRAST,
            AV_CODEC_ID_INDEO4 => Id::INDEO4,
            AV_CODEC_ID_INDEO5 => Id::INDEO5,
            AV_CODEC_ID_MIMIC => Id::MIMIC,
            AV_CODEC_ID_RL2 => Id::RL2,
            AV_CODEC_ID_ESCAPE124 => Id::ESCAPE124,
            AV_CODEC_ID_DIRAC => Id::DIRAC,
            AV_CODEC_ID_BFI => Id::BFI,
            AV_CODEC_ID_CMV => Id::CMV,
            AV_CODEC_ID_MOTIONPIXELS => Id::MOTIONPIXELS,
            AV_CODEC_ID_TGV => Id::TGV,
            AV_CODEC_ID_TGQ => Id::TGQ,
            AV_CODEC_ID_TQI => Id::TQI,
            AV_CODEC_ID_AURA => Id::AURA,
            AV_CODEC_ID_AURA2 => Id::AURA2,
            AV_CODEC_ID_V210X => Id::V210X,
            AV_CODEC_ID_TMV => Id::TMV,
            AV_CODEC_ID_V210 => Id::V210,
            AV_CODEC_ID_DPX => Id::DPX,
            AV_CODEC_ID_MAD => Id::MAD,
            AV_CODEC_ID_FRWU => Id::FRWU,
            AV_CODEC_ID_FLASHSV2 => Id::FLASHSV2,
            AV_CODEC_ID_CDGRAPHICS => Id::CDGRAPHICS,
            AV_CODEC_ID_R210 => Id::R210,
            AV_CODEC_ID_ANM => Id::ANM,
            AV_CODEC_ID_BINKVIDEO => Id::BINKVIDEO,
            AV_CODEC_ID_IFF_ILBM => Id::IFF_ILBM,
            AV_CODEC_ID_KGV1 => Id::KGV1,
            AV_CODEC_ID_YOP => Id::YOP,
            AV_CODEC_ID_VP8 => Id::VP8,
            AV_CODEC_ID_PICTOR => Id::PICTOR,
            AV_CODEC_ID_ANSI => Id::ANSI,
            AV_CODEC_ID_A64_MULTI => Id::A64_MULTI,
            AV_CODEC_ID_A64_MULTI5 => Id::A64_MULTI5,
            AV_CODEC_ID_R10K => Id::R10K,
            AV_CODEC_ID_MXPEG => Id::MXPEG,
            AV_CODEC_ID_LAGARITH => Id::LAGARITH,
            AV_CODEC_ID_PRORES => Id::PRORES,
            AV_CODEC_ID_JV => Id::JV,
            AV_CODEC_ID_DFA => Id::DFA,
            AV_CODEC_ID_WMV3IMAGE => Id::WMV3IMAGE,
            AV_CODEC_ID_VC1IMAGE => Id::VC1IMAGE,
            AV_CODEC_ID_UTVIDEO => Id::UTVIDEO,
            AV_CODEC_ID_BMV_VIDEO => Id::BMV_VIDEO,
            AV_CODEC_ID_VBLE => Id::VBLE,
            AV_CODEC_ID_DXTORY => Id::DXTORY,
            AV_CODEC_ID_V410 => Id::V410,
            AV_CODEC_ID_XWD => Id::XWD,
            AV_CODEC_ID_CDXL => Id::CDXL,
            AV_CODEC_ID_XBM => Id::XBM,
            AV_CODEC_ID_ZEROCODEC => Id::ZEROCODEC,
            AV_CODEC_ID_MSS1 => Id::MSS1,
            AV_CODEC_ID_MSA1 => Id::MSA1,
            AV_CODEC_ID_TSCC2 => Id::TSCC2,
            AV_CODEC_ID_MTS2 => Id::MTS2,
            AV_CODEC_ID_CLLC => Id::CLLC,
            AV_CODEC_ID_MSS2 => Id::MSS2,
            AV_CODEC_ID_VP9 => Id::VP9,
            AV_CODEC_ID_AIC => Id::AIC,
            AV_CODEC_ID_ESCAPE130 => Id::ESCAPE130,
            AV_CODEC_ID_G2M => Id::G2M,
            AV_CODEC_ID_WEBP => Id::WEBP,
            AV_CODEC_ID_HNM4_VIDEO => Id::HNM4_VIDEO,
            AV_CODEC_ID_HEVC => Id::HEVC,
            AV_CODEC_ID_FIC => Id::FIC,
            AV_CODEC_ID_ALIAS_PIX => Id::ALIAS_PIX,
            AV_CODEC_ID_BRENDER_PIX => Id::BRENDER_PIX,
            AV_CODEC_ID_PAF_VIDEO => Id::PAF_VIDEO,
            AV_CODEC_ID_EXR => Id::EXR,
            AV_CODEC_ID_VP7 => Id::VP7,
            AV_CODEC_ID_SANM => Id::SANM,
            AV_CODEC_ID_SGIRLE => Id::SGIRLE,
            AV_CODEC_ID_MVC1 => Id::MVC1,
            AV_CODEC_ID_MVC2 => Id::MVC2,
            AV_CODEC_ID_HQX => Id::HQX,
            AV_CODEC_ID_TDSC => Id::TDSC,
            AV_CODEC_ID_HQ_HQA => Id::HQ_HQA,
            AV_CODEC_ID_HAP => Id::HAP,
            AV_CODEC_ID_DDS => Id::DDS,
            AV_CODEC_ID_DXV => Id::DXV,
            AV_CODEC_ID_SCREENPRESSO => Id::SCREENPRESSO,
            AV_CODEC_ID_RSCC => Id::RSCC,

            AV_CODEC_ID_Y41P => Id::Y41P,
            AV_CODEC_ID_AVRP => Id::AVRP,
            AV_CODEC_ID_012V => Id::V012,
            AV_CODEC_ID_AVUI => Id::AVUI,
            AV_CODEC_ID_AYUV => Id::AYUV,
            AV_CODEC_ID_TARGA_Y216 => Id::TARGA_Y216,
            AV_CODEC_ID_V308 => Id::V308,
            AV_CODEC_ID_V408 => Id::V408,
            AV_CODEC_ID_YUV4 => Id::YUV4,
            AV_CODEC_ID_AVRN => Id::AVRN,
            AV_CODEC_ID_CPIA => Id::CPIA,
            AV_CODEC_ID_XFACE => Id::XFACE,
            AV_CODEC_ID_SNOW => Id::SNOW,
            AV_CODEC_ID_SMVJPEG => Id::SMVJPEG,
            AV_CODEC_ID_APNG => Id::APNG,
            AV_CODEC_ID_DAALA => Id::DAALA,
            AV_CODEC_ID_CFHD => Id::CFHD,
            AV_CODEC_ID_TRUEMOTION2RT => Id::TRUEMOTION2RT,
            AV_CODEC_ID_M101 => Id::M101,
            AV_CODEC_ID_MAGICYUV => Id::MAGICYUV,
            AV_CODEC_ID_SHEERVIDEO => Id::SHEERVIDEO,
            AV_CODEC_ID_YLC => Id::YLC,

            /* various PCM "codecs" */
            AV_CODEC_ID_PCM_S16LE => Id::PCM_S16LE,
            AV_CODEC_ID_PCM_S16BE => Id::PCM_S16BE,
            AV_CODEC_ID_PCM_U16LE => Id::PCM_U16LE,
            AV_CODEC_ID_PCM_U16BE => Id::PCM_U16BE,
            AV_CODEC_ID_PCM_S8 => Id::PCM_S8,
            AV_CODEC_ID_PCM_U8 => Id::PCM_U8,
            AV_CODEC_ID_PCM_MULAW => Id::PCM_MULAW,
            AV_CODEC_ID_PCM_ALAW => Id::PCM_ALAW,
            AV_CODEC_ID_PCM_S32LE => Id::PCM_S32LE,
            AV_CODEC_ID_PCM_S32BE => Id::PCM_S32BE,
            AV_CODEC_ID_PCM_U32LE => Id::PCM_U32LE,
            AV_CODEC_ID_PCM_U32BE => Id::PCM_U32BE,
            AV_CODEC_ID_PCM_S24LE => Id::PCM_S24LE,
            AV_CODEC_ID_PCM_S24BE => Id::PCM_S24BE,
            AV_CODEC_ID_PCM_U24LE => Id::PCM_U24LE,
            AV_CODEC_ID_PCM_U24BE => Id::PCM_U24BE,
            AV_CODEC_ID_PCM_S24DAUD => Id::PCM_S24DAUD,
            AV_CODEC_ID_PCM_ZORK => Id::PCM_ZORK,
            AV_CODEC_ID_PCM_S16LE_PLANAR => Id::PCM_S16LE_PLANAR,
            AV_CODEC_ID_PCM_DVD => Id::PCM_DVD,
            AV_CODEC_ID_PCM_F32BE => Id::PCM_F32BE,
            AV_CODEC_ID_PCM_F32LE => Id::PCM_F32LE,
            AV_CODEC_ID_PCM_F64BE => Id::PCM_F64BE,
            AV_CODEC_ID_PCM_F64LE => Id::PCM_F64LE,
            AV_CODEC_ID_PCM_BLURAY => Id::PCM_BLURAY,
            AV_CODEC_ID_PCM_LXF => Id::PCM_LXF,
            AV_CODEC_ID_S302M => Id::S302M,
            AV_CODEC_ID_PCM_S8_PLANAR => Id::PCM_S8_PLANAR,
            AV_CODEC_ID_PCM_S24LE_PLANAR => Id::PCM_S24LE_PLANAR,
            AV_CODEC_ID_PCM_S32LE_PLANAR => Id::PCM_S32LE_PLANAR,
            AV_CODEC_ID_PCM_S16BE_PLANAR => Id::PCM_S16BE_PLANAR,

            AV_CODEC_ID_PCM_S64LE => Id::PCM_S64LE,
            AV_CODEC_ID_PCM_S64BE => Id::PCM_S64BE,

            /* various ADPCM codecs */
            AV_CODEC_ID_ADPCM_IMA_QT => Id::ADPCM_IMA_QT,
            AV_CODEC_ID_ADPCM_IMA_WAV => Id::ADPCM_IMA_WAV,
            AV_CODEC_ID_ADPCM_IMA_DK3 => Id::ADPCM_IMA_DK3,
            AV_CODEC_ID_ADPCM_IMA_DK4 => Id::ADPCM_IMA_DK4,
            AV_CODEC_ID_ADPCM_IMA_WS => Id::ADPCM_IMA_WS,
            AV_CODEC_ID_ADPCM_IMA_SMJPEG => Id::ADPCM_IMA_SMJPEG,
            AV_CODEC_ID_ADPCM_MS => Id::ADPCM_MS,
            AV_CODEC_ID_ADPCM_4XM => Id::ADPCM_4XM,
            AV_CODEC_ID_ADPCM_XA => Id::ADPCM_XA,
            AV_CODEC_ID_ADPCM_ADX => Id::ADPCM_ADX,
            AV_CODEC_ID_ADPCM_EA => Id::ADPCM_EA,
            AV_CODEC_ID_ADPCM_G726 => Id::ADPCM_G726,
            AV_CODEC_ID_ADPCM_CT => Id::ADPCM_CT,
            AV_CODEC_ID_ADPCM_SWF => Id::ADPCM_SWF,
            AV_CODEC_ID_ADPCM_YAMAHA => Id::ADPCM_YAMAHA,
            AV_CODEC_ID_ADPCM_SBPRO_4 => Id::ADPCM_SBPRO_4,
            AV_CODEC_ID_ADPCM_SBPRO_3 => Id::ADPCM_SBPRO_3,
            AV_CODEC_ID_ADPCM_SBPRO_2 => Id::ADPCM_SBPRO_2,
            AV_CODEC_ID_ADPCM_THP => Id::ADPCM_THP,
            AV_CODEC_ID_ADPCM_IMA_AMV => Id::ADPCM_IMA_AMV,
            AV_CODEC_ID_ADPCM_EA_R1 => Id::ADPCM_EA_R1,
            AV_CODEC_ID_ADPCM_EA_R3 => Id::ADPCM_EA_R3,
            AV_CODEC_ID_ADPCM_EA_R2 => Id::ADPCM_EA_R2,
            AV_CODEC_ID_ADPCM_IMA_EA_SEAD => Id::ADPCM_IMA_EA_SEAD,
            AV_CODEC_ID_ADPCM_IMA_EA_EACS => Id::ADPCM_IMA_EA_EACS,
            AV_CODEC_ID_ADPCM_EA_XAS => Id::ADPCM_EA_XAS,
            AV_CODEC_ID_ADPCM_EA_MAXIS_XA => Id::ADPCM_EA_MAXIS_XA,
            AV_CODEC_ID_ADPCM_IMA_ISS => Id::ADPCM_IMA_ISS,
            AV_CODEC_ID_ADPCM_G722 => Id::ADPCM_G722,
            AV_CODEC_ID_ADPCM_IMA_APC => Id::ADPCM_IMA_APC,
            AV_CODEC_ID_ADPCM_VIMA => Id::ADPCM_VIMA,

            AV_CODEC_ID_ADPCM_AFC => Id::ADPCM_AFC,
            AV_CODEC_ID_ADPCM_IMA_OKI => Id::ADPCM_IMA_OKI,
            AV_CODEC_ID_ADPCM_DTK => Id::ADPCM_DTK,
            AV_CODEC_ID_ADPCM_IMA_RAD => Id::ADPCM_IMA_RAD,
            AV_CODEC_ID_ADPCM_G726LE => Id::ADPCM_G726LE,
            AV_CODEC_ID_ADPCM_THP_LE => Id::ADPCM_THP_LE,
            AV_CODEC_ID_ADPCM_PSX => Id::ADPCM_PSX,
            AV_CODEC_ID_ADPCM_AICA => Id::ADPCM_AICA,
            AV_CODEC_ID_ADPCM_IMA_DAT4 => Id::ADPCM_IMA_DAT4,
            AV_CODEC_ID_ADPCM_MTAF => Id::ADPCM_MTAF,

            /* AMR */
            AV_CODEC_ID_AMR_NB => Id::AMR_NB,
            AV_CODEC_ID_AMR_WB => Id::AMR_WB,

            /* RealAudio codecs*/
            AV_CODEC_ID_RA_144 => Id::RA_144,
            AV_CODEC_ID_RA_288 => Id::RA_288,

            /* various DPCM codecs */
            AV_CODEC_ID_ROQ_DPCM => Id::ROQ_DPCM,
            AV_CODEC_ID_INTERPLAY_DPCM => Id::INTERPLAY_DPCM,
            AV_CODEC_ID_XAN_DPCM => Id::XAN_DPCM,
            AV_CODEC_ID_SOL_DPCM => Id::SOL_DPCM,

            AV_CODEC_ID_SDX2_DPCM => Id::SDX2_DPCM,

            /* audio codecs */
            AV_CODEC_ID_MP2 => Id::MP2,
            AV_CODEC_ID_MP3 => Id::MP3,
            AV_CODEC_ID_AAC => Id::AAC,
            AV_CODEC_ID_AC3 => Id::AC3,
            AV_CODEC_ID_DTS => Id::DTS,
            AV_CODEC_ID_VORBIS => Id::VORBIS,
            AV_CODEC_ID_DVAUDIO => Id::DVAUDIO,
            AV_CODEC_ID_WMAV1 => Id::WMAV1,
            AV_CODEC_ID_WMAV2 => Id::WMAV2,
            AV_CODEC_ID_MACE3 => Id::MACE3,
            AV_CODEC_ID_MACE6 => Id::MACE6,
            AV_CODEC_ID_VMDAUDIO => Id::VMDAUDIO,
            AV_CODEC_ID_FLAC => Id::FLAC,
            AV_CODEC_ID_MP3ADU => Id::MP3ADU,
            AV_CODEC_ID_MP3ON4 => Id::MP3ON4,
            AV_CODEC_ID_SHORTEN => Id::SHORTEN,
            AV_CODEC_ID_ALAC => Id::ALAC,
            AV_CODEC_ID_WESTWOOD_SND1 => Id::WESTWOOD_SND1,
            AV_CODEC_ID_GSM => Id::GSM,
            AV_CODEC_ID_QDM2 => Id::QDM2,
            AV_CODEC_ID_COOK => Id::COOK,
            AV_CODEC_ID_TRUESPEECH => Id::TRUESPEECH,
            AV_CODEC_ID_TTA => Id::TTA,
            AV_CODEC_ID_SMACKAUDIO => Id::SMACKAUDIO,
            AV_CODEC_ID_QCELP => Id::QCELP,
            AV_CODEC_ID_WAVPACK => Id::WAVPACK,
            AV_CODEC_ID_DSICINAUDIO => Id::DSICINAUDIO,
            AV_CODEC_ID_IMC => Id::IMC,
            AV_CODEC_ID_MUSEPACK7 => Id::MUSEPACK7,
            AV_CODEC_ID_MLP => Id::MLP,
            AV_CODEC_ID_GSM_MS => Id::GSM_MS,
            AV_CODEC_ID_ATRAC3 => Id::ATRAC3,
            #[cfg(feature = "ff_api_voxware")]
            AV_CODEC_ID_VOXWARE => Id::VOXWARE,
            AV_CODEC_ID_APE => Id::APE,
            AV_CODEC_ID_NELLYMOSER => Id::NELLYMOSER,
            AV_CODEC_ID_MUSEPACK8 => Id::MUSEPACK8,
            AV_CODEC_ID_SPEEX => Id::SPEEX,
            AV_CODEC_ID_WMAVOICE => Id::WMAVOICE,
            AV_CODEC_ID_WMAPRO => Id::WMAPRO,
            AV_CODEC_ID_WMALOSSLESS => Id::WMALOSSLESS,
            AV_CODEC_ID_ATRAC3P => Id::ATRAC3P,
            AV_CODEC_ID_EAC3 => Id::EAC3,
            AV_CODEC_ID_SIPR => Id::SIPR,
            AV_CODEC_ID_MP1 => Id::MP1,
            AV_CODEC_ID_TWINVQ => Id::TWINVQ,
            AV_CODEC_ID_TRUEHD => Id::TRUEHD,
            AV_CODEC_ID_MP4ALS => Id::MP4ALS,
            AV_CODEC_ID_ATRAC1 => Id::ATRAC1,
            AV_CODEC_ID_BINKAUDIO_RDFT => Id::BINKAUDIO_RDFT,
            AV_CODEC_ID_BINKAUDIO_DCT => Id::BINKAUDIO_DCT,
            AV_CODEC_ID_AAC_LATM => Id::AAC_LATM,
            AV_CODEC_ID_QDMC => Id::QDMC,
            AV_CODEC_ID_CELT => Id::CELT,
            AV_CODEC_ID_G723_1 => Id::G723_1,
            AV_CODEC_ID_G729 => Id::G729,
            AV_CODEC_ID_8SVX_EXP => Id::SVX_EXP8,
            AV_CODEC_ID_8SVX_FIB => Id::SVX_FIB8,
            AV_CODEC_ID_BMV_AUDIO => Id::BMV_AUDIO,
            AV_CODEC_ID_RALF => Id::RALF,
            AV_CODEC_ID_IAC => Id::IAC,
            AV_CODEC_ID_ILBC => Id::ILBC,
            AV_CODEC_ID_OPUS => Id::OPUS,
            AV_CODEC_ID_COMFORT_NOISE => Id::COMFORT_NOISE,
            AV_CODEC_ID_TAK => Id::TAK,
            AV_CODEC_ID_METASOUND => Id::METASOUND,
            AV_CODEC_ID_PAF_AUDIO => Id::PAF_AUDIO,
            AV_CODEC_ID_ON2AVC => Id::ON2AVC,
            AV_CODEC_ID_DSS_SP => Id::DSS_SP,

            #[cfg(feature = "ffmpeg_4_0")]
            AV_CODEC_ID_CODEC2 => Id::CODEC2,
            AV_CODEC_ID_FFWAVESYNTH => Id::FFWAVESYNTH,
            AV_CODEC_ID_SONIC => Id::SONIC,
            AV_CODEC_ID_SONIC_LS => Id::SONIC_LS,
            AV_CODEC_ID_EVRC => Id::EVRC,
            AV_CODEC_ID_SMV => Id::SMV,
            AV_CODEC_ID_DSD_LSBF => Id::DSD_LSBF,
            AV_CODEC_ID_DSD_MSBF => Id::DSD_MSBF,
            AV_CODEC_ID_DSD_LSBF_PLANAR => Id::DSD_LSBF_PLANAR,
            AV_CODEC_ID_DSD_MSBF_PLANAR => Id::DSD_MSBF_PLANAR,
            AV_CODEC_ID_4GV => Id::_4GV,
            AV_CODEC_ID_INTERPLAY_ACM => Id::INTERPLAY_ACM,
            AV_CODEC_ID_XMA1 => Id::XMA1,
            AV_CODEC_ID_XMA2 => Id::XMA2,
            AV_CODEC_ID_DST => Id::DST,

            /* subtitle codecs */
            AV_CODEC_ID_DVD_SUBTITLE => Id::DVD_SUBTITLE,
            AV_CODEC_ID_DVB_SUBTITLE => Id::DVB_SUBTITLE,
            AV_CODEC_ID_TEXT => Id::TEXT,
            AV_CODEC_ID_XSUB => Id::XSUB,
            AV_CODEC_ID_SSA => Id::SSA,
            AV_CODEC_ID_MOV_TEXT => Id::MOV_TEXT,
            AV_CODEC_ID_HDMV_PGS_SUBTITLE => Id::HDMV_PGS_SUBTITLE,
            AV_CODEC_ID_DVB_TELETEXT => Id::DVB_TELETEXT,
            AV_CODEC_ID_SRT => Id::SRT,

            AV_CODEC_ID_MICRODVD => Id::MICRODVD,
            AV_CODEC_ID_EIA_608 => Id::EIA_608,
            AV_CODEC_ID_JACOSUB => Id::JACOSUB,
            AV_CODEC_ID_SAMI => Id::SAMI,
            AV_CODEC_ID_REALTEXT => Id::REALTEXT,
            AV_CODEC_ID_STL => Id::STL,
            AV_CODEC_ID_SUBVIEWER1 => Id::SUBVIEWER1,
            AV_CODEC_ID_SUBVIEWER => Id::SUBVIEWER,
            AV_CODEC_ID_SUBRIP => Id::SUBRIP,
            AV_CODEC_ID_WEBVTT => Id::WEBVTT,
            AV_CODEC_ID_MPL2 => Id::MPL2,
            AV_CODEC_ID_VPLAYER => Id::VPLAYER,
            AV_CODEC_ID_PJS => Id::PJS,
            AV_CODEC_ID_ASS => Id::ASS,
            AV_CODEC_ID_HDMV_TEXT_SUBTITLE => Id::HDMV_TEXT_SUBTITLE,

            /* other specific kind of codecs (generally used for attachments) */
            AV_CODEC_ID_TTF => Id::TTF,

            AV_CODEC_ID_SCTE_35 => Id::SCTE_35,
            AV_CODEC_ID_BINTEXT => Id::BINTEXT,
            AV_CODEC_ID_XBIN => Id::XBIN,
            AV_CODEC_ID_IDF => Id::IDF,
            AV_CODEC_ID_OTF => Id::OTF,
            AV_CODEC_ID_SMPTE_KLV => Id::SMPTE_KLV,
            AV_CODEC_ID_DVD_NAV => Id::DVD_NAV,
            AV_CODEC_ID_TIMED_ID3 => Id::TIMED_ID3,
            AV_CODEC_ID_BIN_DATA => Id::BIN_DATA,

            AV_CODEC_ID_PROBE => Id::PROBE,

            AV_CODEC_ID_MPEG2TS => Id::MPEG2TS,
            AV_CODEC_ID_MPEG4SYSTEMS => Id::MPEG4SYSTEMS,
            AV_CODEC_ID_FFMETADATA => Id::FFMETADATA,
            AV_CODEC_ID_WRAPPED_AVFRAME => Id::WRAPPED_AVFRAME,
            AV_CODEC_ID_PSD => Id::PSD,
            AV_CODEC_ID_PIXLET => Id::PIXLET,
            AV_CODEC_ID_SPEEDHQ => Id::SPEEDHQ,
            AV_CODEC_ID_CLEARVIDEO => Id::CLEARVIDEO,
            AV_CODEC_ID_FMVC => Id::FMVC,
            AV_CODEC_ID_SCPR => Id::SCPR,
            AV_CODEC_ID_XPM => Id::XPM,
            AV_CODEC_ID_AV1 => Id::AV1,
            AV_CODEC_ID_PCM_F16LE => Id::PCM_F16LE,
            AV_CODEC_ID_PCM_F24LE => Id::PCM_F24LE,
            AV_CODEC_ID_ATRAC3AL => Id::ATRAC3AL,
            AV_CODEC_ID_ATRAC3PAL => Id::ATRAC3PAL,

            AV_CODEC_ID_BITPACKED => Id::BITPACKED,
            AV_CODEC_ID_MSCC => Id::MSCC,
            AV_CODEC_ID_SRGC => Id::SRGC,
            AV_CODEC_ID_SVG => Id::SVG,
            AV_CODEC_ID_GDV => Id::GDV,
            AV_CODEC_ID_FITS => Id::FITS,
            AV_CODEC_ID_GREMLIN_DPCM => Id::GREMLIN_DPCM,
            AV_CODEC_ID_DOLBY_E => Id::DOLBY_E,

            #[cfg(feature = "ffmpeg_4_0")]
            AV_CODEC_ID_APTX => Id::APTX,
            #[cfg(feature = "ffmpeg_4_0")]
            AV_CODEC_ID_APTX_HD => Id::APTX_HD,
            #[cfg(feature = "ffmpeg_4_0")]
            AV_CODEC_ID_SBC => Id::SBC,

            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_AVS2 => Id::AVS2,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_IMM4 => Id::IMM4,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_PROSUMER => Id::PROSUMER,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_MWSC => Id::MWSC,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_WCMV => Id::WCMV,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_RASC => Id::RASC,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_PCM_VIDC => Id::PCM_VIDC,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_ATRAC9 => Id::ATRAC9,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_CODEC_ID_TTML => Id::TTML,

            #[cfg(feature = "ffmpeg_4_2")]
            AV_CODEC_ID_HYMT => Id::HYMT,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_CODEC_ID_ARBC => Id::ARBC,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_CODEC_ID_AGM => Id::AGM,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_CODEC_ID_LSCR => Id::LSCR,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_CODEC_ID_VP4 => Id::VP4,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_CODEC_ID_ADPCM_AGM => Id::ADPCM_AGM,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_CODEC_ID_HCOM => Id::HCOM,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_CODEC_ID_ARIB_CAPTION => Id::ARIB_CAPTION,

            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_IMM5 => Id::IMM5,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_MVDV => Id::MVDV,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_MVHA => Id::MVHA,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_CDTOONS => Id::CDTOONS,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_MV30 => Id::MV30,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_NOTCHLC => Id::NOTCHLC,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_PFM => Id::PFM,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_ADPCM_ARGO => Id::ADPCM_ARGO,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_ADPCM_IMA_SSI => Id::ADPCM_IMA_SSI,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_ADPCM_ZORK => Id::ADPCM_ZORK,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_ADPCM_IMA_APM => Id::ADPCM_IMA_APM,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_ADPCM_IMA_ALP => Id::ADPCM_IMA_ALP,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_ADPCM_IMA_MTF => Id::ADPCM_IMA_MTF,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_ADPCM_IMA_CUNNING => Id::ADPCM_IMA_CUNNING,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_DERF_DPCM => Id::DERF_DPCM,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_ACELP_KELVIN => Id::ACELP_KELVIN,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_MPEGH_3D_AUDIO => Id::MPEGH_3D_AUDIO,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_SIREN => Id::SIREN,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_HCA => Id::HCA,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_CODEC_ID_EPG => Id::EPG,

            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_PGX => Id::PGX,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_AVS3 => Id::AVS3,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_MSP2 => Id::MSP2,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_VVC => Id::VVC,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_MOBICLIP => Id::MOBICLIP,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_PHOTOCD => Id::PHOTOCD,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_IPU => Id::IPU,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_ARGO => Id::ARGO,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_CRI => Id::CRI,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_SIMBIOSIS_IMX => Id::SIMBIOSIS_IMX,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_SGA_VIDEO => Id::SGA_VIDEO,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_PCM_SGA => Id::PCM_SGA,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_ADPCM_IMA_MOFLEX => Id::ADPCM_IMA_MOFLEX,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_CODEC_ID_FASTAUDIO => Id::FASTAUDIO,

            #[cfg(feature = "ffmpeg_5_0")]
            AV_CODEC_ID_GEM => Id::GEM,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_CODEC_ID_ADPCM_IMA_ACORN => Id::ADPCM_IMA_ACORN,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_CODEC_ID_MSNSIREN => Id::MSNSIREN,

            #[cfg(feature = "ffmpeg_5_1")]
            AV_CODEC_ID_VBN => Id::VBN,
            #[cfg(feature = "ffmpeg_5_1")]
            AV_CODEC_ID_JPEGXL => Id::JPEGXL,
            #[cfg(feature = "ffmpeg_5_1")]
            AV_CODEC_ID_QOI => Id::QOI,
            #[cfg(feature = "ffmpeg_5_1")]
            AV_CODEC_ID_PHM => Id::PHM,
            #[cfg(feature = "ffmpeg_5_1")]
            AV_CODEC_ID_DFPWM => Id::DFPWM,

            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_RADIANCE_HDR => Id::RADIANCE_HDR,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_WBMP => Id::WBMP,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_MEDIA100 => Id::MEDIA100,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_VQC => Id::VQC,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_ADPCM_XMD => Id::ADPCM_XMD,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_WADY_DPCM => Id::WADY_DPCM,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_CBD2_DPCM => Id::CBD2_DPCM,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_BONK => Id::BONK,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_MISC4 => Id::MISC4,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_APAC => Id::APAC,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_FTR => Id::FTR,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_WAVARC => Id::WAVARC,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_RKA => Id::RKA,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_VNULL => Id::VNULL,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_CODEC_ID_ANULL => Id::ANULL,

            #[cfg(feature = "ffmpeg_6_1")]
            AV_CODEC_ID_PDV => Id::PDV,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_CODEC_ID_EVC => Id::EVC,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_CODEC_ID_RTV1 => Id::RTV1,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_CODEC_ID_VMIX => Id::VMIX,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_CODEC_ID_AC4 => Id::AC4,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_CODEC_ID_SMPTE_2038 => Id::SMPTE_2038,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_CODEC_ID_OSQ => Id::OSQ,

            // FFmpeg 7.x added new codec IDs not covered by ffmpeg-next 6.1
            #[allow(unreachable_patterns)]
            _ => Id::None,
        }
    }
}

impl From<Id> for AVCodecID {
    fn from(value: Id) -> AVCodecID {
        match value {
            Id::None => AV_CODEC_ID_NONE,

            /* video codecs */
            Id::MPEG1VIDEO => AV_CODEC_ID_MPEG1VIDEO,
            Id::MPEG2VIDEO => AV_CODEC_ID_MPEG2VIDEO,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            Id::MPEG2VIDEO_XVMC => AV_CODEC_ID_MPEG2VIDEO_XVMC,
            Id::H261 => AV_CODEC_ID_H261,
            Id::H263 => AV_CODEC_ID_H263,
            Id::RV10 => AV_CODEC_ID_RV10,
            Id::RV20 => AV_CODEC_ID_RV20,
            Id::MJPEG => AV_CODEC_ID_MJPEG,
            Id::MJPEGB => AV_CODEC_ID_MJPEGB,
            Id::LJPEG => AV_CODEC_ID_LJPEG,
            Id::SP5X => AV_CODEC_ID_SP5X,
            Id::JPEGLS => AV_CODEC_ID_JPEGLS,
            Id::MPEG4 => AV_CODEC_ID_MPEG4,
            Id::RAWVIDEO => AV_CODEC_ID_RAWVIDEO,
            Id::MSMPEG4V1 => AV_CODEC_ID_MSMPEG4V1,
            Id::MSMPEG4V2 => AV_CODEC_ID_MSMPEG4V2,
            Id::MSMPEG4V3 => AV_CODEC_ID_MSMPEG4V3,
            Id::WMV1 => AV_CODEC_ID_WMV1,
            Id::WMV2 => AV_CODEC_ID_WMV2,
            Id::H263P => AV_CODEC_ID_H263P,
            Id::H263I => AV_CODEC_ID_H263I,
            Id::FLV1 => AV_CODEC_ID_FLV1,
            Id::SVQ1 => AV_CODEC_ID_SVQ1,
            Id::SVQ3 => AV_CODEC_ID_SVQ3,
            Id::DVVIDEO => AV_CODEC_ID_DVVIDEO,
            Id::HUFFYUV => AV_CODEC_ID_HUFFYUV,
            Id::CYUV => AV_CODEC_ID_CYUV,
            Id::H264 => AV_CODEC_ID_H264,
            Id::INDEO3 => AV_CODEC_ID_INDEO3,
            Id::VP3 => AV_CODEC_ID_VP3,
            Id::THEORA => AV_CODEC_ID_THEORA,
            Id::ASV1 => AV_CODEC_ID_ASV1,
            Id::ASV2 => AV_CODEC_ID_ASV2,
            Id::FFV1 => AV_CODEC_ID_FFV1,
            Id::XM4 => AV_CODEC_ID_4XM,
            Id::VCR1 => AV_CODEC_ID_VCR1,
            Id::CLJR => AV_CODEC_ID_CLJR,
            Id::MDEC => AV_CODEC_ID_MDEC,
            Id::ROQ => AV_CODEC_ID_ROQ,
            Id::INTERPLAY_VIDEO => AV_CODEC_ID_INTERPLAY_VIDEO,
            Id::XAN_WC3 => AV_CODEC_ID_XAN_WC3,
            Id::XAN_WC4 => AV_CODEC_ID_XAN_WC4,
            Id::RPZA => AV_CODEC_ID_RPZA,
            Id::CINEPAK => AV_CODEC_ID_CINEPAK,
            Id::WS_VQA => AV_CODEC_ID_WS_VQA,
            Id::MSRLE => AV_CODEC_ID_MSRLE,
            Id::MSVIDEO1 => AV_CODEC_ID_MSVIDEO1,
            Id::IDCIN => AV_CODEC_ID_IDCIN,
            Id::BPS8 => AV_CODEC_ID_8BPS,
            Id::SMC => AV_CODEC_ID_SMC,
            Id::FLIC => AV_CODEC_ID_FLIC,
            Id::TRUEMOTION1 => AV_CODEC_ID_TRUEMOTION1,
            Id::VMDVIDEO => AV_CODEC_ID_VMDVIDEO,
            Id::MSZH => AV_CODEC_ID_MSZH,
            Id::ZLIB => AV_CODEC_ID_ZLIB,
            Id::QTRLE => AV_CODEC_ID_QTRLE,
            Id::TSCC => AV_CODEC_ID_TSCC,
            Id::ULTI => AV_CODEC_ID_ULTI,
            Id::QDRAW => AV_CODEC_ID_QDRAW,
            Id::VIXL => AV_CODEC_ID_VIXL,
            Id::QPEG => AV_CODEC_ID_QPEG,
            Id::PNG => AV_CODEC_ID_PNG,
            Id::PPM => AV_CODEC_ID_PPM,
            Id::PBM => AV_CODEC_ID_PBM,
            Id::PGM => AV_CODEC_ID_PGM,
            Id::PGMYUV => AV_CODEC_ID_PGMYUV,
            Id::PAM => AV_CODEC_ID_PAM,
            Id::FFVHUFF => AV_CODEC_ID_FFVHUFF,
            Id::RV30 => AV_CODEC_ID_RV30,
            Id::RV40 => AV_CODEC_ID_RV40,
            Id::VC1 => AV_CODEC_ID_VC1,
            Id::WMV3 => AV_CODEC_ID_WMV3,
            Id::LOCO => AV_CODEC_ID_LOCO,
            Id::WNV1 => AV_CODEC_ID_WNV1,
            Id::AASC => AV_CODEC_ID_AASC,
            Id::INDEO2 => AV_CODEC_ID_INDEO2,
            Id::FRAPS => AV_CODEC_ID_FRAPS,
            Id::TRUEMOTION2 => AV_CODEC_ID_TRUEMOTION2,
            Id::BMP => AV_CODEC_ID_BMP,
            Id::CSCD => AV_CODEC_ID_CSCD,
            Id::MMVIDEO => AV_CODEC_ID_MMVIDEO,
            Id::ZMBV => AV_CODEC_ID_ZMBV,
            Id::AVS => AV_CODEC_ID_AVS,
            Id::SMACKVIDEO => AV_CODEC_ID_SMACKVIDEO,
            Id::NUV => AV_CODEC_ID_NUV,
            Id::KMVC => AV_CODEC_ID_KMVC,
            Id::FLASHSV => AV_CODEC_ID_FLASHSV,
            Id::CAVS => AV_CODEC_ID_CAVS,
            Id::JPEG2000 => AV_CODEC_ID_JPEG2000,
            Id::VMNC => AV_CODEC_ID_VMNC,
            Id::VP5 => AV_CODEC_ID_VP5,
            Id::VP6 => AV_CODEC_ID_VP6,
            Id::VP6F => AV_CODEC_ID_VP6F,
            Id::TARGA => AV_CODEC_ID_TARGA,
            Id::DSICINVIDEO => AV_CODEC_ID_DSICINVIDEO,
            Id::TIERTEXSEQVIDEO => AV_CODEC_ID_TIERTEXSEQVIDEO,
            Id::TIFF => AV_CODEC_ID_TIFF,
            Id::GIF => AV_CODEC_ID_GIF,
            Id::DXA => AV_CODEC_ID_DXA,
            Id::DNXHD => AV_CODEC_ID_DNXHD,
            Id::THP => AV_CODEC_ID_THP,
            Id::SGI => AV_CODEC_ID_SGI,
            Id::C93 => AV_CODEC_ID_C93,
            Id::BETHSOFTVID => AV_CODEC_ID_BETHSOFTVID,
            Id::PTX => AV_CODEC_ID_PTX,
            Id::TXD => AV_CODEC_ID_TXD,
            Id::VP6A => AV_CODEC_ID_VP6A,
            Id::AMV => AV_CODEC_ID_AMV,
            Id::VB => AV_CODEC_ID_VB,
            Id::PCX => AV_CODEC_ID_PCX,
            Id::SUNRAST => AV_CODEC_ID_SUNRAST,
            Id::INDEO4 => AV_CODEC_ID_INDEO4,
            Id::INDEO5 => AV_CODEC_ID_INDEO5,
            Id::MIMIC => AV_CODEC_ID_MIMIC,
            Id::RL2 => AV_CODEC_ID_RL2,
            Id::ESCAPE124 => AV_CODEC_ID_ESCAPE124,
            Id::DIRAC => AV_CODEC_ID_DIRAC,
            Id::BFI => AV_CODEC_ID_BFI,
            Id::CMV => AV_CODEC_ID_CMV,
            Id::MOTIONPIXELS => AV_CODEC_ID_MOTIONPIXELS,
            Id::TGV => AV_CODEC_ID_TGV,
            Id::TGQ => AV_CODEC_ID_TGQ,
            Id::TQI => AV_CODEC_ID_TQI,
            Id::AURA => AV_CODEC_ID_AURA,
            Id::AURA2 => AV_CODEC_ID_AURA2,
            Id::V210X => AV_CODEC_ID_V210X,
            Id::TMV => AV_CODEC_ID_TMV,
            Id::V210 => AV_CODEC_ID_V210,
            Id::DPX => AV_CODEC_ID_DPX,
            Id::MAD => AV_CODEC_ID_MAD,
            Id::FRWU => AV_CODEC_ID_FRWU,
            Id::FLASHSV2 => AV_CODEC_ID_FLASHSV2,
            Id::CDGRAPHICS => AV_CODEC_ID_CDGRAPHICS,
            Id::R210 => AV_CODEC_ID_R210,
            Id::ANM => AV_CODEC_ID_ANM,
            Id::BINKVIDEO => AV_CODEC_ID_BINKVIDEO,
            Id::IFF_ILBM => AV_CODEC_ID_IFF_ILBM,
            Id::IFF_BYTERUN1 => AV_CODEC_ID_IFF_ILBM,
            Id::KGV1 => AV_CODEC_ID_KGV1,
            Id::YOP => AV_CODEC_ID_YOP,
            Id::VP8 => AV_CODEC_ID_VP8,
            Id::PICTOR => AV_CODEC_ID_PICTOR,
            Id::ANSI => AV_CODEC_ID_ANSI,
            Id::A64_MULTI => AV_CODEC_ID_A64_MULTI,
            Id::A64_MULTI5 => AV_CODEC_ID_A64_MULTI5,
            Id::R10K => AV_CODEC_ID_R10K,
            Id::MXPEG => AV_CODEC_ID_MXPEG,
            Id::LAGARITH => AV_CODEC_ID_LAGARITH,
            Id::PRORES => AV_CODEC_ID_PRORES,
            Id::JV => AV_CODEC_ID_JV,
            Id::DFA => AV_CODEC_ID_DFA,
            Id::WMV3IMAGE => AV_CODEC_ID_WMV3IMAGE,
            Id::VC1IMAGE => AV_CODEC_ID_VC1IMAGE,
            Id::UTVIDEO => AV_CODEC_ID_UTVIDEO,
            Id::BMV_VIDEO => AV_CODEC_ID_BMV_VIDEO,
            Id::VBLE => AV_CODEC_ID_VBLE,
            Id::DXTORY => AV_CODEC_ID_DXTORY,
            Id::V410 => AV_CODEC_ID_V410,
            Id::XWD => AV_CODEC_ID_XWD,
            Id::CDXL => AV_CODEC_ID_CDXL,
            Id::XBM => AV_CODEC_ID_XBM,
            Id::ZEROCODEC => AV_CODEC_ID_ZEROCODEC,
            Id::MSS1 => AV_CODEC_ID_MSS1,
            Id::MSA1 => AV_CODEC_ID_MSA1,
            Id::TSCC2 => AV_CODEC_ID_TSCC2,
            Id::MTS2 => AV_CODEC_ID_MTS2,
            Id::CLLC => AV_CODEC_ID_CLLC,
            Id::MSS2 => AV_CODEC_ID_MSS2,
            Id::VP9 => AV_CODEC_ID_VP9,
            Id::AIC => AV_CODEC_ID_AIC,
            Id::ESCAPE130 => AV_CODEC_ID_ESCAPE130,
            Id::G2M => AV_CODEC_ID_G2M,
            Id::WEBP => AV_CODEC_ID_WEBP,
            Id::HNM4_VIDEO => AV_CODEC_ID_HNM4_VIDEO,
            Id::HEVC => AV_CODEC_ID_HEVC,
            Id::H265 => AV_CODEC_ID_HEVC,
            Id::FIC => AV_CODEC_ID_FIC,
            Id::ALIAS_PIX => AV_CODEC_ID_ALIAS_PIX,
            Id::BRENDER_PIX => AV_CODEC_ID_BRENDER_PIX,
            Id::PAF_VIDEO => AV_CODEC_ID_PAF_VIDEO,
            Id::EXR => AV_CODEC_ID_EXR,
            Id::VP7 => AV_CODEC_ID_VP7,
            Id::SANM => AV_CODEC_ID_SANM,
            Id::SGIRLE => AV_CODEC_ID_SGIRLE,
            Id::MVC1 => AV_CODEC_ID_MVC1,
            Id::MVC2 => AV_CODEC_ID_MVC2,
            Id::HQX => AV_CODEC_ID_HQX,
            Id::TDSC => AV_CODEC_ID_TDSC,
            Id::HQ_HQA => AV_CODEC_ID_HQ_HQA,
            Id::HAP => AV_CODEC_ID_HAP,
            Id::DDS => AV_CODEC_ID_DDS,
            Id::DXV => AV_CODEC_ID_DXV,
            Id::SCREENPRESSO => AV_CODEC_ID_SCREENPRESSO,
            Id::RSCC => AV_CODEC_ID_RSCC,

            Id::Y41P => AV_CODEC_ID_Y41P,
            Id::AVRP => AV_CODEC_ID_AVRP,
            Id::V012 => AV_CODEC_ID_012V,
            Id::AVUI => AV_CODEC_ID_AVUI,
            Id::AYUV => AV_CODEC_ID_AYUV,
            Id::TARGA_Y216 => AV_CODEC_ID_TARGA_Y216,
            Id::V308 => AV_CODEC_ID_V308,
            Id::V408 => AV_CODEC_ID_V408,
            Id::YUV4 => AV_CODEC_ID_YUV4,
            Id::AVRN => AV_CODEC_ID_AVRN,
            Id::CPIA => AV_CODEC_ID_CPIA,
            Id::XFACE => AV_CODEC_ID_XFACE,
            Id::SNOW => AV_CODEC_ID_SNOW,
            Id::SMVJPEG => AV_CODEC_ID_SMVJPEG,
            Id::APNG => AV_CODEC_ID_APNG,
            Id::DAALA => AV_CODEC_ID_DAALA,
            Id::CFHD => AV_CODEC_ID_CFHD,
            Id::TRUEMOTION2RT => AV_CODEC_ID_TRUEMOTION2RT,
            Id::M101 => AV_CODEC_ID_M101,
            Id::MAGICYUV => AV_CODEC_ID_MAGICYUV,
            Id::SHEERVIDEO => AV_CODEC_ID_SHEERVIDEO,
            Id::YLC => AV_CODEC_ID_YLC,

            /* various PCM "codecs" */
            Id::PCM_S16LE => AV_CODEC_ID_PCM_S16LE,
            Id::PCM_S16BE => AV_CODEC_ID_PCM_S16BE,
            Id::PCM_U16LE => AV_CODEC_ID_PCM_U16LE,
            Id::PCM_U16BE => AV_CODEC_ID_PCM_U16BE,
            Id::PCM_S8 => AV_CODEC_ID_PCM_S8,
            Id::PCM_U8 => AV_CODEC_ID_PCM_U8,
            Id::PCM_MULAW => AV_CODEC_ID_PCM_MULAW,
            Id::PCM_ALAW => AV_CODEC_ID_PCM_ALAW,
            Id::PCM_S32LE => AV_CODEC_ID_PCM_S32LE,
            Id::PCM_S32BE => AV_CODEC_ID_PCM_S32BE,
            Id::PCM_U32LE => AV_CODEC_ID_PCM_U32LE,
            Id::PCM_U32BE => AV_CODEC_ID_PCM_U32BE,
            Id::PCM_S24LE => AV_CODEC_ID_PCM_S24LE,
            Id::PCM_S24BE => AV_CODEC_ID_PCM_S24BE,
            Id::PCM_U24LE => AV_CODEC_ID_PCM_U24LE,
            Id::PCM_U24BE => AV_CODEC_ID_PCM_U24BE,
            Id::PCM_S24DAUD => AV_CODEC_ID_PCM_S24DAUD,
            Id::PCM_ZORK => AV_CODEC_ID_PCM_ZORK,
            Id::PCM_S16LE_PLANAR => AV_CODEC_ID_PCM_S16LE_PLANAR,
            Id::PCM_DVD => AV_CODEC_ID_PCM_DVD,
            Id::PCM_F32BE => AV_CODEC_ID_PCM_F32BE,
            Id::PCM_F32LE => AV_CODEC_ID_PCM_F32LE,
            Id::PCM_F64BE => AV_CODEC_ID_PCM_F64BE,
            Id::PCM_F64LE => AV_CODEC_ID_PCM_F64LE,
            Id::PCM_BLURAY => AV_CODEC_ID_PCM_BLURAY,
            Id::PCM_LXF => AV_CODEC_ID_PCM_LXF,
            Id::S302M => AV_CODEC_ID_S302M,
            Id::PCM_S8_PLANAR => AV_CODEC_ID_PCM_S8_PLANAR,
            Id::PCM_S24LE_PLANAR => AV_CODEC_ID_PCM_S24LE_PLANAR,
            Id::PCM_S32LE_PLANAR => AV_CODEC_ID_PCM_S32LE_PLANAR,
            Id::PCM_S16BE_PLANAR => AV_CODEC_ID_PCM_S16BE_PLANAR,

            Id::PCM_S64LE => AV_CODEC_ID_PCM_S64LE,
            Id::PCM_S64BE => AV_CODEC_ID_PCM_S64BE,

            /* various ADPCM codecs */
            Id::ADPCM_IMA_QT => AV_CODEC_ID_ADPCM_IMA_QT,
            Id::ADPCM_IMA_WAV => AV_CODEC_ID_ADPCM_IMA_WAV,
            Id::ADPCM_IMA_DK3 => AV_CODEC_ID_ADPCM_IMA_DK3,
            Id::ADPCM_IMA_DK4 => AV_CODEC_ID_ADPCM_IMA_DK4,
            Id::ADPCM_IMA_WS => AV_CODEC_ID_ADPCM_IMA_WS,
            Id::ADPCM_IMA_SMJPEG => AV_CODEC_ID_ADPCM_IMA_SMJPEG,
            Id::ADPCM_MS => AV_CODEC_ID_ADPCM_MS,
            Id::ADPCM_4XM => AV_CODEC_ID_ADPCM_4XM,
            Id::ADPCM_XA => AV_CODEC_ID_ADPCM_XA,
            Id::ADPCM_ADX => AV_CODEC_ID_ADPCM_ADX,
            Id::ADPCM_EA => AV_CODEC_ID_ADPCM_EA,
            Id::ADPCM_G726 => AV_CODEC_ID_ADPCM_G726,
            Id::ADPCM_CT => AV_CODEC_ID_ADPCM_CT,
            Id::ADPCM_SWF => AV_CODEC_ID_ADPCM_SWF,
            Id::ADPCM_YAMAHA => AV_CODEC_ID_ADPCM_YAMAHA,
            Id::ADPCM_SBPRO_4 => AV_CODEC_ID_ADPCM_SBPRO_4,
            Id::ADPCM_SBPRO_3 => AV_CODEC_ID_ADPCM_SBPRO_3,
            Id::ADPCM_SBPRO_2 => AV_CODEC_ID_ADPCM_SBPRO_2,
            Id::ADPCM_THP => AV_CODEC_ID_ADPCM_THP,
            Id::ADPCM_IMA_AMV => AV_CODEC_ID_ADPCM_IMA_AMV,
            Id::ADPCM_EA_R1 => AV_CODEC_ID_ADPCM_EA_R1,
            Id::ADPCM_EA_R3 => AV_CODEC_ID_ADPCM_EA_R3,
            Id::ADPCM_EA_R2 => AV_CODEC_ID_ADPCM_EA_R2,
            Id::ADPCM_IMA_EA_SEAD => AV_CODEC_ID_ADPCM_IMA_EA_SEAD,
            Id::ADPCM_IMA_EA_EACS => AV_CODEC_ID_ADPCM_IMA_EA_EACS,
            Id::ADPCM_EA_XAS => AV_CODEC_ID_ADPCM_EA_XAS,
            Id::ADPCM_EA_MAXIS_XA => AV_CODEC_ID_ADPCM_EA_MAXIS_XA,
            Id::ADPCM_IMA_ISS => AV_CODEC_ID_ADPCM_IMA_ISS,
            Id::ADPCM_G722 => AV_CODEC_ID_ADPCM_G722,
            Id::ADPCM_IMA_APC => AV_CODEC_ID_ADPCM_IMA_APC,
            Id::ADPCM_VIMA => AV_CODEC_ID_ADPCM_VIMA,

            Id::ADPCM_AFC => AV_CODEC_ID_ADPCM_AFC,
            Id::ADPCM_IMA_OKI => AV_CODEC_ID_ADPCM_IMA_OKI,
            Id::ADPCM_DTK => AV_CODEC_ID_ADPCM_DTK,
            Id::ADPCM_IMA_RAD => AV_CODEC_ID_ADPCM_IMA_RAD,
            Id::ADPCM_G726LE => AV_CODEC_ID_ADPCM_G726LE,
            Id::ADPCM_THP_LE => AV_CODEC_ID_ADPCM_THP_LE,
            Id::ADPCM_PSX => AV_CODEC_ID_ADPCM_PSX,
            Id::ADPCM_AICA => AV_CODEC_ID_ADPCM_AICA,
            Id::ADPCM_IMA_DAT4 => AV_CODEC_ID_ADPCM_IMA_DAT4,
            Id::ADPCM_MTAF => AV_CODEC_ID_ADPCM_MTAF,

            /* AMR */
            Id::AMR_NB => AV_CODEC_ID_AMR_NB,
            Id::AMR_WB => AV_CODEC_ID_AMR_WB,

            /* RealAudio codecs*/
            Id::RA_144 => AV_CODEC_ID_RA_144,
            Id::RA_288 => AV_CODEC_ID_RA_288,

            /* various DPCM codecs */
            Id::ROQ_DPCM => AV_CODEC_ID_ROQ_DPCM,
            Id::INTERPLAY_DPCM => AV_CODEC_ID_INTERPLAY_DPCM,
            Id::XAN_DPCM => AV_CODEC_ID_XAN_DPCM,
            Id::SOL_DPCM => AV_CODEC_ID_SOL_DPCM,

            Id::SDX2_DPCM => AV_CODEC_ID_SDX2_DPCM,

            /* audio codecs */
            Id::MP2 => AV_CODEC_ID_MP2,
            Id::MP3 => AV_CODEC_ID_MP3,
            Id::AAC => AV_CODEC_ID_AAC,
            Id::AC3 => AV_CODEC_ID_AC3,
            Id::DTS => AV_CODEC_ID_DTS,
            Id::VORBIS => AV_CODEC_ID_VORBIS,
            Id::DVAUDIO => AV_CODEC_ID_DVAUDIO,
            Id::WMAV1 => AV_CODEC_ID_WMAV1,
            Id::WMAV2 => AV_CODEC_ID_WMAV2,
            Id::MACE3 => AV_CODEC_ID_MACE3,
            Id::MACE6 => AV_CODEC_ID_MACE6,
            Id::VMDAUDIO => AV_CODEC_ID_VMDAUDIO,
            Id::FLAC => AV_CODEC_ID_FLAC,
            Id::MP3ADU => AV_CODEC_ID_MP3ADU,
            Id::MP3ON4 => AV_CODEC_ID_MP3ON4,
            Id::SHORTEN => AV_CODEC_ID_SHORTEN,
            Id::ALAC => AV_CODEC_ID_ALAC,
            Id::WESTWOOD_SND1 => AV_CODEC_ID_WESTWOOD_SND1,
            Id::GSM => AV_CODEC_ID_GSM,
            Id::QDM2 => AV_CODEC_ID_QDM2,
            Id::COOK => AV_CODEC_ID_COOK,
            Id::TRUESPEECH => AV_CODEC_ID_TRUESPEECH,
            Id::TTA => AV_CODEC_ID_TTA,
            Id::SMACKAUDIO => AV_CODEC_ID_SMACKAUDIO,
            Id::QCELP => AV_CODEC_ID_QCELP,
            Id::WAVPACK => AV_CODEC_ID_WAVPACK,
            Id::DSICINAUDIO => AV_CODEC_ID_DSICINAUDIO,
            Id::IMC => AV_CODEC_ID_IMC,
            Id::MUSEPACK7 => AV_CODEC_ID_MUSEPACK7,
            Id::MLP => AV_CODEC_ID_MLP,
            Id::GSM_MS => AV_CODEC_ID_GSM_MS,
            Id::ATRAC3 => AV_CODEC_ID_ATRAC3,
            #[cfg(feature = "ff_api_voxware")]
            Id::VOXWARE => AV_CODEC_ID_VOXWARE,
            Id::APE => AV_CODEC_ID_APE,
            Id::NELLYMOSER => AV_CODEC_ID_NELLYMOSER,
            Id::MUSEPACK8 => AV_CODEC_ID_MUSEPACK8,
            Id::SPEEX => AV_CODEC_ID_SPEEX,
            Id::WMAVOICE => AV_CODEC_ID_WMAVOICE,
            Id::WMAPRO => AV_CODEC_ID_WMAPRO,
            Id::WMALOSSLESS => AV_CODEC_ID_WMALOSSLESS,
            Id::ATRAC3P => AV_CODEC_ID_ATRAC3P,
            Id::EAC3 => AV_CODEC_ID_EAC3,
            Id::SIPR => AV_CODEC_ID_SIPR,
            Id::MP1 => AV_CODEC_ID_MP1,
            Id::TWINVQ => AV_CODEC_ID_TWINVQ,
            Id::TRUEHD => AV_CODEC_ID_TRUEHD,
            Id::MP4ALS => AV_CODEC_ID_MP4ALS,
            Id::ATRAC1 => AV_CODEC_ID_ATRAC1,
            Id::BINKAUDIO_RDFT => AV_CODEC_ID_BINKAUDIO_RDFT,
            Id::BINKAUDIO_DCT => AV_CODEC_ID_BINKAUDIO_DCT,
            Id::AAC_LATM => AV_CODEC_ID_AAC_LATM,
            Id::QDMC => AV_CODEC_ID_QDMC,
            Id::CELT => AV_CODEC_ID_CELT,
            Id::G723_1 => AV_CODEC_ID_G723_1,
            Id::G729 => AV_CODEC_ID_G729,
            Id::SVX_EXP8 => AV_CODEC_ID_8SVX_EXP,
            Id::SVX_FIB8 => AV_CODEC_ID_8SVX_FIB,
            Id::BMV_AUDIO => AV_CODEC_ID_BMV_AUDIO,
            Id::RALF => AV_CODEC_ID_RALF,
            Id::IAC => AV_CODEC_ID_IAC,
            Id::ILBC => AV_CODEC_ID_ILBC,
            Id::OPUS => AV_CODEC_ID_OPUS,
            Id::COMFORT_NOISE => AV_CODEC_ID_COMFORT_NOISE,
            Id::TAK => AV_CODEC_ID_TAK,
            Id::METASOUND => AV_CODEC_ID_METASOUND,
            Id::PAF_AUDIO => AV_CODEC_ID_PAF_AUDIO,
            Id::ON2AVC => AV_CODEC_ID_ON2AVC,
            Id::DSS_SP => AV_CODEC_ID_DSS_SP,

            #[cfg(feature = "ffmpeg_4_0")]
            Id::CODEC2 => AV_CODEC_ID_CODEC2,
            Id::FFWAVESYNTH => AV_CODEC_ID_FFWAVESYNTH,
            Id::SONIC => AV_CODEC_ID_SONIC,
            Id::SONIC_LS => AV_CODEC_ID_SONIC_LS,
            Id::EVRC => AV_CODEC_ID_EVRC,
            Id::SMV => AV_CODEC_ID_SMV,
            Id::DSD_LSBF => AV_CODEC_ID_DSD_LSBF,
            Id::DSD_MSBF => AV_CODEC_ID_DSD_MSBF,
            Id::DSD_LSBF_PLANAR => AV_CODEC_ID_DSD_LSBF_PLANAR,
            Id::DSD_MSBF_PLANAR => AV_CODEC_ID_DSD_MSBF_PLANAR,
            Id::_4GV => AV_CODEC_ID_4GV,
            Id::INTERPLAY_ACM => AV_CODEC_ID_INTERPLAY_ACM,
            Id::XMA1 => AV_CODEC_ID_XMA1,
            Id::XMA2 => AV_CODEC_ID_XMA2,
            Id::DST => AV_CODEC_ID_DST,

            /* subtitle codecs */
            Id::DVD_SUBTITLE => AV_CODEC_ID_DVD_SUBTITLE,
            Id::DVB_SUBTITLE => AV_CODEC_ID_DVB_SUBTITLE,
            Id::TEXT => AV_CODEC_ID_TEXT,
            Id::XSUB => AV_CODEC_ID_XSUB,
            Id::SSA => AV_CODEC_ID_SSA,
            Id::MOV_TEXT => AV_CODEC_ID_MOV_TEXT,
            Id::HDMV_PGS_SUBTITLE => AV_CODEC_ID_HDMV_PGS_SUBTITLE,
            Id::DVB_TELETEXT => AV_CODEC_ID_DVB_TELETEXT,
            Id::SRT => AV_CODEC_ID_SRT,

            Id::MICRODVD => AV_CODEC_ID_MICRODVD,
            Id::EIA_608 => AV_CODEC_ID_EIA_608,
            Id::JACOSUB => AV_CODEC_ID_JACOSUB,
            Id::SAMI => AV_CODEC_ID_SAMI,
            Id::REALTEXT => AV_CODEC_ID_REALTEXT,
            Id::STL => AV_CODEC_ID_STL,
            Id::SUBVIEWER1 => AV_CODEC_ID_SUBVIEWER1,
            Id::SUBVIEWER => AV_CODEC_ID_SUBVIEWER,
            Id::SUBRIP => AV_CODEC_ID_SUBRIP,
            Id::WEBVTT => AV_CODEC_ID_WEBVTT,
            Id::MPL2 => AV_CODEC_ID_MPL2,
            Id::VPLAYER => AV_CODEC_ID_VPLAYER,
            Id::PJS => AV_CODEC_ID_PJS,
            Id::ASS => AV_CODEC_ID_ASS,
            Id::HDMV_TEXT_SUBTITLE => AV_CODEC_ID_HDMV_TEXT_SUBTITLE,

            /* other specific kind of codecs (generally used for attachments) */
            Id::TTF => AV_CODEC_ID_TTF,

            Id::SCTE_35 => AV_CODEC_ID_SCTE_35,
            Id::BINTEXT => AV_CODEC_ID_BINTEXT,
            Id::XBIN => AV_CODEC_ID_XBIN,
            Id::IDF => AV_CODEC_ID_IDF,
            Id::OTF => AV_CODEC_ID_OTF,
            Id::SMPTE_KLV => AV_CODEC_ID_SMPTE_KLV,
            Id::DVD_NAV => AV_CODEC_ID_DVD_NAV,
            Id::TIMED_ID3 => AV_CODEC_ID_TIMED_ID3,
            Id::BIN_DATA => AV_CODEC_ID_BIN_DATA,

            Id::PROBE => AV_CODEC_ID_PROBE,

            Id::MPEG2TS => AV_CODEC_ID_MPEG2TS,
            Id::MPEG4SYSTEMS => AV_CODEC_ID_MPEG4SYSTEMS,
            Id::FFMETADATA => AV_CODEC_ID_FFMETADATA,
            Id::WRAPPED_AVFRAME => AV_CODEC_ID_WRAPPED_AVFRAME,

            Id::PSD => AV_CODEC_ID_PSD,
            Id::PIXLET => AV_CODEC_ID_PIXLET,
            Id::SPEEDHQ => AV_CODEC_ID_SPEEDHQ,
            Id::FMVC => AV_CODEC_ID_FMVC,
            Id::CLEARVIDEO => AV_CODEC_ID_CLEARVIDEO,
            Id::SCPR => AV_CODEC_ID_SCPR,
            Id::XPM => AV_CODEC_ID_XPM,
            Id::AV1 => AV_CODEC_ID_AV1,
            Id::PCM_F16LE => AV_CODEC_ID_PCM_F16LE,
            Id::PCM_F24LE => AV_CODEC_ID_PCM_F24LE,
            Id::ATRAC3AL => AV_CODEC_ID_ATRAC3AL,
            Id::ATRAC3PAL => AV_CODEC_ID_ATRAC3PAL,

            Id::BITPACKED => AV_CODEC_ID_BITPACKED,
            Id::MSCC => AV_CODEC_ID_MSCC,
            Id::SRGC => AV_CODEC_ID_SRGC,
            Id::SVG => AV_CODEC_ID_SVG,
            Id::GDV => AV_CODEC_ID_GDV,
            Id::FITS => AV_CODEC_ID_FITS,
            Id::GREMLIN_DPCM => AV_CODEC_ID_GREMLIN_DPCM,
            Id::DOLBY_E => AV_CODEC_ID_DOLBY_E,

            #[cfg(feature = "ffmpeg_4_0")]
            Id::APTX => AV_CODEC_ID_APTX,
            #[cfg(feature = "ffmpeg_4_0")]
            Id::APTX_HD => AV_CODEC_ID_APTX_HD,
            #[cfg(feature = "ffmpeg_4_0")]
            Id::SBC => AV_CODEC_ID_SBC,

            #[cfg(feature = "ffmpeg_4_1")]
            Id::AVS2 => AV_CODEC_ID_AVS2,
            #[cfg(feature = "ffmpeg_4_1")]
            Id::IMM4 => AV_CODEC_ID_IMM4,
            #[cfg(feature = "ffmpeg_4_1")]
            Id::PROSUMER => AV_CODEC_ID_PROSUMER,
            #[cfg(feature = "ffmpeg_4_1")]
            Id::MWSC => AV_CODEC_ID_MWSC,
            #[cfg(feature = "ffmpeg_4_1")]
            Id::WCMV => AV_CODEC_ID_WCMV,
            #[cfg(feature = "ffmpeg_4_1")]
            Id::RASC => AV_CODEC_ID_RASC,
            #[cfg(feature = "ffmpeg_4_1")]
            Id::PCM_VIDC => AV_CODEC_ID_PCM_VIDC,
            #[cfg(feature = "ffmpeg_4_1")]
            Id::ATRAC9 => AV_CODEC_ID_ATRAC9,
            #[cfg(feature = "ffmpeg_4_1")]
            Id::TTML => AV_CODEC_ID_TTML,

            #[cfg(feature = "ffmpeg_4_2")]
            Id::HYMT => AV_CODEC_ID_HYMT,
            #[cfg(feature = "ffmpeg_4_2")]
            Id::ARBC => AV_CODEC_ID_ARBC,
            #[cfg(feature = "ffmpeg_4_2")]
            Id::AGM => AV_CODEC_ID_AGM,
            #[cfg(feature = "ffmpeg_4_2")]
            Id::LSCR => AV_CODEC_ID_LSCR,
            #[cfg(feature = "ffmpeg_4_2")]
            Id::VP4 => AV_CODEC_ID_VP4,
            #[cfg(feature = "ffmpeg_4_2")]
            Id::ADPCM_AGM => AV_CODEC_ID_ADPCM_AGM,
            #[cfg(feature = "ffmpeg_4_2")]
            Id::HCOM => AV_CODEC_ID_HCOM,
            #[cfg(feature = "ffmpeg_4_2")]
            Id::ARIB_CAPTION => AV_CODEC_ID_ARIB_CAPTION,

            #[cfg(feature = "ffmpeg_4_3")]
            Id::IMM5 => AV_CODEC_ID_IMM5,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::MVDV => AV_CODEC_ID_MVDV,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::MVHA => AV_CODEC_ID_MVHA,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::CDTOONS => AV_CODEC_ID_CDTOONS,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::MV30 => AV_CODEC_ID_MV30,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::NOTCHLC => AV_CODEC_ID_NOTCHLC,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::PFM => AV_CODEC_ID_PFM,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::ADPCM_ARGO => AV_CODEC_ID_ADPCM_ARGO,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::ADPCM_IMA_SSI => AV_CODEC_ID_ADPCM_IMA_SSI,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::ADPCM_ZORK => AV_CODEC_ID_ADPCM_ZORK,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::ADPCM_IMA_APM => AV_CODEC_ID_ADPCM_IMA_APM,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::ADPCM_IMA_ALP => AV_CODEC_ID_ADPCM_IMA_ALP,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::ADPCM_IMA_MTF => AV_CODEC_ID_ADPCM_IMA_MTF,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::ADPCM_IMA_CUNNING => AV_CODEC_ID_ADPCM_IMA_CUNNING,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::DERF_DPCM => AV_CODEC_ID_DERF_DPCM,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::ACELP_KELVIN => AV_CODEC_ID_ACELP_KELVIN,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::MPEGH_3D_AUDIO => AV_CODEC_ID_MPEGH_3D_AUDIO,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::SIREN => AV_CODEC_ID_SIREN,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::HCA => AV_CODEC_ID_HCA,
            #[cfg(feature = "ffmpeg_4_3")]
            Id::EPG => AV_CODEC_ID_EPG,

            #[cfg(feature = "ffmpeg_4_4")]
            Id::PGX => AV_CODEC_ID_PGX,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::AVS3 => AV_CODEC_ID_AVS3,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::MSP2 => AV_CODEC_ID_MSP2,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::VVC => AV_CODEC_ID_VVC,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::MOBICLIP => AV_CODEC_ID_MOBICLIP,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::PHOTOCD => AV_CODEC_ID_PHOTOCD,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::IPU => AV_CODEC_ID_IPU,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::ARGO => AV_CODEC_ID_ARGO,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::CRI => AV_CODEC_ID_CRI,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::SIMBIOSIS_IMX => AV_CODEC_ID_SIMBIOSIS_IMX,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::SGA_VIDEO => AV_CODEC_ID_SGA_VIDEO,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::PCM_SGA => AV_CODEC_ID_PCM_SGA,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::ADPCM_IMA_MOFLEX => AV_CODEC_ID_ADPCM_IMA_MOFLEX,
            #[cfg(feature = "ffmpeg_4_4")]
            Id::FASTAUDIO => AV_CODEC_ID_FASTAUDIO,

            #[cfg(feature = "ffmpeg_5_0")]
            Id::GEM => AV_CODEC_ID_GEM,
            #[cfg(feature = "ffmpeg_5_0")]
            Id::ADPCM_IMA_ACORN => AV_CODEC_ID_ADPCM_IMA_ACORN,
            #[cfg(feature = "ffmpeg_5_0")]
            Id::MSNSIREN => AV_CODEC_ID_MSNSIREN,

            #[cfg(feature = "ffmpeg_5_1")]
            Id::VBN => AV_CODEC_ID_VBN,
            #[cfg(feature = "ffmpeg_5_1")]
            Id::JPEGXL => AV_CODEC_ID_JPEGXL,
            #[cfg(feature = "ffmpeg_5_1")]
            Id::QOI => AV_CODEC_ID_QOI,
            #[cfg(feature = "ffmpeg_5_1")]
            Id::PHM => AV_CODEC_ID_PHM,
            #[cfg(feature = "ffmpeg_5_1")]
            Id::DFPWM => AV_CODEC_ID_DFPWM,

            #[cfg(feature = "ffmpeg_6_0")]
            Id::RADIANCE_HDR => AV_CODEC_ID_RADIANCE_HDR,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::WBMP => AV_CODEC_ID_WBMP,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::MEDIA100 => AV_CODEC_ID_MEDIA100,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::VQC => AV_CODEC_ID_VQC,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::ADPCM_XMD => AV_CODEC_ID_ADPCM_XMD,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::WADY_DPCM => AV_CODEC_ID_WADY_DPCM,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::CBD2_DPCM => AV_CODEC_ID_CBD2_DPCM,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::BONK => AV_CODEC_ID_BONK,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::MISC4 => AV_CODEC_ID_MISC4,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::APAC => AV_CODEC_ID_APAC,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::FTR => AV_CODEC_ID_FTR,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::WAVARC => AV_CODEC_ID_WAVARC,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::RKA => AV_CODEC_ID_RKA,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::VNULL => AV_CODEC_ID_VNULL,
            #[cfg(feature = "ffmpeg_6_0")]
            Id::ANULL => AV_CODEC_ID_ANULL,

            #[cfg(feature = "ffmpeg_6_1")]
            Id::PDV => AV_CODEC_ID_PDV,
            #[cfg(feature = "ffmpeg_6_1")]
            Id::EVC => AV_CODEC_ID_EVC,
            #[cfg(feature = "ffmpeg_6_1")]
            Id::RTV1 => AV_CODEC_ID_RTV1,
            #[cfg(feature = "ffmpeg_6_1")]
            Id::VMIX => AV_CODEC_ID_VMIX,
            #[cfg(feature = "ffmpeg_6_1")]
            Id::AC4 => AV_CODEC_ID_AC4,
            #[cfg(feature = "ffmpeg_6_1")]
            Id::SMPTE_2038 => AV_CODEC_ID_SMPTE_2038,
            #[cfg(feature = "ffmpeg_6_1")]
            Id::OSQ => AV_CODEC_ID_OSQ,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/mod.rs">
pub mod flag;
pub use self::flag::Flags;

pub mod id;
pub use self::id::Id;

pub mod packet;

pub mod subtitle;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub mod picture;

pub mod discard;

pub mod context;
pub use self::context::Context;

pub mod capabilities;
pub use self::capabilities::Capabilities;

pub mod codec;

pub mod parameters;
pub use self::parameters::Parameters;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod audio_service;
pub mod field_order;

pub mod compliance;
pub use self::compliance::Compliance;

pub mod debug;
pub use self::debug::Debug;

pub mod profile;
pub use self::profile::Profile;

pub mod threading;

pub mod decoder;
pub mod encoder;
pub mod traits;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub fn version() -> u32 {
    unsafe { avcodec_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avcodec_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avcodec_license()).to_bytes()) }
}
</file>

<file path="patches/ffmpeg-next/src/codec/parameters.rs">
use std::any::Any;
use std::rc::Rc;

use super::{Context, Id};
use ffi::*;
use media;

pub struct Parameters {
    ptr: *mut AVCodecParameters,
    owner: Option<Rc<dyn Any>>,
}

unsafe impl Send for Parameters {}

impl Parameters {
    pub unsafe fn wrap(ptr: *mut AVCodecParameters, owner: Option<Rc<dyn Any>>) -> Self {
        Parameters { ptr, owner }
    }

    pub unsafe fn as_ptr(&self) -> *const AVCodecParameters {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVCodecParameters {
        self.ptr
    }
}

impl Parameters {
    pub fn new() -> Self {
        unsafe {
            Parameters {
                ptr: avcodec_parameters_alloc(),
                owner: None,
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from((*self.as_ptr()).codec_type) }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::from((*self.as_ptr()).codec_id) }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Parameters {
    fn drop(&mut self) {
        unsafe {
            if self.owner.is_none() {
                avcodec_parameters_free(&mut self.as_mut_ptr());
            }
        }
    }
}

impl Clone for Parameters {
    fn clone(&self) -> Self {
        let mut ctx = Parameters::new();
        ctx.clone_from(self);

        ctx
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            avcodec_parameters_copy(self.as_mut_ptr(), source.as_ptr());
        }
    }
}

impl<C: AsRef<Context>> From<C> for Parameters {
    fn from(context: C) -> Parameters {
        let mut parameters = Parameters::new();
        let context = context.as_ref();
        unsafe {
            avcodec_parameters_from_context(parameters.as_mut_ptr(), context.as_ptr());
        }
        parameters
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/picture.rs">
use std::marker::PhantomData;
use std::mem;
use std::slice;

use ffi::*;
use format;
use libc::{c_int, size_t};
use Error;

pub struct Picture<'a> {
    ptr: *mut AVPicture,

    format: format::Pixel,
    width: u32,
    height: u32,

    _own: bool,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Picture<'a> {
    pub unsafe fn wrap(
        ptr: *mut AVPicture,
        format: format::Pixel,
        width: u32,
        height: u32,
    ) -> Self {
        Picture {
            ptr,

            format,
            width,
            height,

            _own: false,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVPicture {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVPicture {
        self.ptr
    }
}

impl<'a> Picture<'a> {
    pub fn size(format: format::Pixel, width: u32, height: u32) -> Result<usize, Error> {
        unsafe {
            match avpicture_get_size(format.into(), width as c_int, height as c_int) {
                v if v >= 0 => Ok(v as usize),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn new(format: format::Pixel, width: u32, height: u32) -> Result<Self, Error> {
        unsafe {
            let ptr = av_malloc(mem::size_of::<AVPicture>() as size_t) as *mut AVPicture;

            match avpicture_alloc(ptr, format.into(), width as c_int, height as c_int) {
                0 => Ok(Picture {
                    ptr,

                    format,
                    width,
                    height,

                    _own: true,
                    _marker: PhantomData,
                }),

                e => Err(Error::from(e)),
            }
        }
    }

    pub fn format(&self) -> format::Pixel {
        self.format
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn layout(&self, out: &mut [u8]) -> Result<usize, Error> {
        unsafe {
            match avpicture_layout(
                self.ptr,
                self.format.into(),
                self.width as c_int,
                self.height as c_int,
                out.as_mut_ptr(),
                out.len() as c_int,
            ) {
                s if s >= 0 => Ok(s as usize),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn layout_as(
        &self,
        format: format::Pixel,
        width: u32,
        height: u32,
        out: &mut [u8],
    ) -> Result<usize, Error> {
        unsafe {
            match avpicture_layout(
                self.as_ptr(),
                format.into(),
                width as c_int,
                height as c_int,
                out.as_mut_ptr(),
                out.len() as c_int,
            ) {
                s if s >= 0 => Ok(s as usize),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn crop(&self, source: &mut Picture, top: u32, left: u32) -> Result<(), Error> {
        if self.format != source.format {
            return Err(Error::Bug);
        }

        unsafe {
            match av_picture_crop(
                source.as_mut_ptr(),
                self.as_ptr(),
                self.format.into(),
                top as c_int,
                left as c_int,
            ) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn data(&self) -> Vec<&[u8]> {
        let mut result = Vec::new();

        unsafe {
            for (i, length) in (*self.as_ptr())
                .linesize
                .iter()
                .take_while(|l| **l > 0)
                .enumerate()
            {
                result.push(slice::from_raw_parts(
                    (*self.as_ptr()).data[i],
                    (*length as usize) * (self.height as usize),
                ))
            }
        }

        result
    }

    pub fn data_mut(&mut self) -> Vec<&mut [u8]> {
        let mut result = Vec::new();

        unsafe {
            for (i, length) in (*self.as_ptr())
                .linesize
                .iter()
                .take_while(|l| **l > 0)
                .enumerate()
            {
                result.push(slice::from_raw_parts_mut(
                    (*self.as_ptr()).data[i],
                    (*length as usize) * (self.height as usize),
                ))
            }
        }

        result
    }
}

impl<'a> Clone for Picture<'a> {
    fn clone(&self) -> Self {
        let mut pic = Picture::new(self.format, self.width, self.height).unwrap();
        pic.clone_from(self);

        pic
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            av_picture_copy(
                self.as_mut_ptr(),
                source.as_ptr(),
                source.format.into(),
                source.width as c_int,
                source.height as c_int,
            );
        }
    }
}

impl<'a> Drop for Picture<'a> {
    fn drop(&mut self) {
        if self._own {
            unsafe {
                av_free(self.as_mut_ptr() as *mut _);
            }
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/profile.rs">
use super::Id;
use ffi::*;
use libc::c_int;

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Profile {
    Unknown,
    Reserved,

    AAC(AAC),
    MPEG2(MPEG2),
    DTS(DTS),
    H264(H264),
    VC1(VC1),
    MPEG4(MPEG4),
    JPEG2000(JPEG2000),
    HEVC(HEVC),
    VP9(VP9),
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum AAC {
    Main,
    Low,
    SSR,
    LTP,
    HE,
    HEv2,
    LD,
    ELD,

    MPEG2Low,
    MPEG2HE,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum DTS {
    Default,
    ES,
    _96_24,
    HD_HRA,
    HD_MA,
    Express,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MPEG2 {
    _422,
    High,
    SS,
    SNRScalable,
    Main,
    Simple,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum H264 {
    Constrained,
    Intra,
    Baseline,
    ConstrainedBaseline,
    Main,
    Extended,
    High,
    High10,
    High10Intra,
    High422,
    High422Intra,
    High444,
    High444Predictive,
    High444Intra,
    CAVLC444,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VC1 {
    Simple,
    Main,
    Complex,
    Advanced,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MPEG4 {
    Simple,
    SimpleScalable,
    Core,
    Main,
    NBit,
    ScalableTexture,
    SimpleFaceAnimation,
    BasicAnimatedTexture,
    Hybrid,
    AdvancedRealTime,
    CoreScalable,
    AdvancedCoding,
    AdvancedCore,
    AdvancedScalableTexture,
    SimpleStudio,
    AdvancedSimple,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum JPEG2000 {
    CStreamRestriction0,
    CStreamRestriction1,
    CStreamNoRestriction,
    DCinema2K,
    DCinema4K,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum HEVC {
    Main,
    Main10,
    MainStillPicture,
    Rext,
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VP9 {
    _0,
    _1,
    _2,
    _3,
}

impl From<(Id, c_int)> for Profile {
    fn from((id, value): (Id, c_int)) -> Profile {
        if value == FF_PROFILE_UNKNOWN {
            return Profile::Unknown;
        }

        if value == FF_PROFILE_RESERVED {
            return Profile::Reserved;
        }

        match id {
            Id::AAC => match value {
                FF_PROFILE_AAC_MAIN => Profile::AAC(AAC::Main),
                FF_PROFILE_AAC_LOW => Profile::AAC(AAC::Low),
                FF_PROFILE_AAC_SSR => Profile::AAC(AAC::SSR),
                FF_PROFILE_AAC_LTP => Profile::AAC(AAC::LTP),
                FF_PROFILE_AAC_HE => Profile::AAC(AAC::HE),
                FF_PROFILE_AAC_HE_V2 => Profile::AAC(AAC::HEv2),
                FF_PROFILE_AAC_LD => Profile::AAC(AAC::LD),
                FF_PROFILE_AAC_ELD => Profile::AAC(AAC::ELD),

                FF_PROFILE_MPEG2_AAC_LOW => Profile::AAC(AAC::MPEG2Low),
                FF_PROFILE_MPEG2_AAC_HE => Profile::AAC(AAC::MPEG2HE),

                _ => Profile::Unknown,
            },

            Id::DTS => match value {
                FF_PROFILE_DTS => Profile::DTS(DTS::Default),
                FF_PROFILE_DTS_ES => Profile::DTS(DTS::ES),
                FF_PROFILE_DTS_96_24 => Profile::DTS(DTS::_96_24),
                FF_PROFILE_DTS_HD_HRA => Profile::DTS(DTS::HD_HRA),
                FF_PROFILE_DTS_HD_MA => Profile::DTS(DTS::HD_MA),
                FF_PROFILE_DTS_EXPRESS => Profile::DTS(DTS::Express),

                _ => Profile::Unknown,
            },

            Id::MPEG2VIDEO => match value {
                FF_PROFILE_MPEG2_422 => Profile::MPEG2(MPEG2::_422),
                FF_PROFILE_MPEG2_HIGH => Profile::MPEG2(MPEG2::High),
                FF_PROFILE_MPEG2_SS => Profile::MPEG2(MPEG2::SS),
                FF_PROFILE_MPEG2_SNR_SCALABLE => Profile::MPEG2(MPEG2::SNRScalable),
                FF_PROFILE_MPEG2_MAIN => Profile::MPEG2(MPEG2::Main),
                FF_PROFILE_MPEG2_SIMPLE => Profile::MPEG2(MPEG2::Simple),

                _ => Profile::Unknown,
            },

            Id::H264 => match value {
                FF_PROFILE_H264_CONSTRAINED => Profile::H264(H264::Constrained),
                FF_PROFILE_H264_INTRA => Profile::H264(H264::Intra),
                FF_PROFILE_H264_BASELINE => Profile::H264(H264::Baseline),
                FF_PROFILE_H264_CONSTRAINED_BASELINE => Profile::H264(H264::ConstrainedBaseline),
                FF_PROFILE_H264_MAIN => Profile::H264(H264::Main),
                FF_PROFILE_H264_EXTENDED => Profile::H264(H264::Extended),
                FF_PROFILE_H264_HIGH => Profile::H264(H264::High),
                FF_PROFILE_H264_HIGH_10 => Profile::H264(H264::High10),
                FF_PROFILE_H264_HIGH_10_INTRA => Profile::H264(H264::High10Intra),
                FF_PROFILE_H264_HIGH_422 => Profile::H264(H264::High422),
                FF_PROFILE_H264_HIGH_422_INTRA => Profile::H264(H264::High422Intra),
                FF_PROFILE_H264_HIGH_444 => Profile::H264(H264::High444),
                FF_PROFILE_H264_HIGH_444_PREDICTIVE => Profile::H264(H264::High444Predictive),
                FF_PROFILE_H264_HIGH_444_INTRA => Profile::H264(H264::High444Intra),
                FF_PROFILE_H264_CAVLC_444 => Profile::H264(H264::CAVLC444),

                _ => Profile::Unknown,
            },

            Id::VC1 => match value {
                FF_PROFILE_VC1_SIMPLE => Profile::VC1(VC1::Simple),
                FF_PROFILE_VC1_MAIN => Profile::VC1(VC1::Main),
                FF_PROFILE_VC1_COMPLEX => Profile::VC1(VC1::Complex),
                FF_PROFILE_VC1_ADVANCED => Profile::VC1(VC1::Advanced),

                _ => Profile::Unknown,
            },

            Id::MPEG4 => match value {
                FF_PROFILE_MPEG4_SIMPLE => Profile::MPEG4(MPEG4::Simple),
                FF_PROFILE_MPEG4_SIMPLE_SCALABLE => Profile::MPEG4(MPEG4::SimpleScalable),
                FF_PROFILE_MPEG4_CORE => Profile::MPEG4(MPEG4::Core),
                FF_PROFILE_MPEG4_MAIN => Profile::MPEG4(MPEG4::Main),
                FF_PROFILE_MPEG4_N_BIT => Profile::MPEG4(MPEG4::NBit),
                FF_PROFILE_MPEG4_SCALABLE_TEXTURE => Profile::MPEG4(MPEG4::ScalableTexture),
                FF_PROFILE_MPEG4_SIMPLE_FACE_ANIMATION => {
                    Profile::MPEG4(MPEG4::SimpleFaceAnimation)
                }
                FF_PROFILE_MPEG4_BASIC_ANIMATED_TEXTURE => {
                    Profile::MPEG4(MPEG4::BasicAnimatedTexture)
                }
                FF_PROFILE_MPEG4_HYBRID => Profile::MPEG4(MPEG4::Hybrid),
                FF_PROFILE_MPEG4_ADVANCED_REAL_TIME => Profile::MPEG4(MPEG4::AdvancedRealTime),
                FF_PROFILE_MPEG4_CORE_SCALABLE => Profile::MPEG4(MPEG4::CoreScalable),
                FF_PROFILE_MPEG4_ADVANCED_CODING => Profile::MPEG4(MPEG4::AdvancedCoding),
                FF_PROFILE_MPEG4_ADVANCED_CORE => Profile::MPEG4(MPEG4::AdvancedCore),
                FF_PROFILE_MPEG4_ADVANCED_SCALABLE_TEXTURE => {
                    Profile::MPEG4(MPEG4::AdvancedScalableTexture)
                }
                FF_PROFILE_MPEG4_SIMPLE_STUDIO => Profile::MPEG4(MPEG4::SimpleStudio),
                FF_PROFILE_MPEG4_ADVANCED_SIMPLE => Profile::MPEG4(MPEG4::AdvancedSimple),

                _ => Profile::Unknown,
            },

            Id::JPEG2000 => match value {
                FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_0 => {
                    Profile::JPEG2000(JPEG2000::CStreamRestriction0)
                }
                FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_1 => {
                    Profile::JPEG2000(JPEG2000::CStreamRestriction1)
                }
                FF_PROFILE_JPEG2000_CSTREAM_NO_RESTRICTION => {
                    Profile::JPEG2000(JPEG2000::CStreamNoRestriction)
                }
                FF_PROFILE_JPEG2000_DCINEMA_2K => Profile::JPEG2000(JPEG2000::DCinema2K),
                FF_PROFILE_JPEG2000_DCINEMA_4K => Profile::JPEG2000(JPEG2000::DCinema4K),

                _ => Profile::Unknown,
            },

            Id::HEVC => match value {
                FF_PROFILE_HEVC_MAIN => Profile::HEVC(HEVC::Main),
                FF_PROFILE_HEVC_MAIN_10 => Profile::HEVC(HEVC::Main10),
                FF_PROFILE_HEVC_MAIN_STILL_PICTURE => Profile::HEVC(HEVC::MainStillPicture),
                FF_PROFILE_HEVC_REXT => Profile::HEVC(HEVC::Rext),

                _ => Profile::Unknown,
            },

            Id::VP9 => match value {
                FF_PROFILE_VP9_0 => Profile::VP9(VP9::_0),
                FF_PROFILE_VP9_1 => Profile::VP9(VP9::_1),
                FF_PROFILE_VP9_2 => Profile::VP9(VP9::_2),
                FF_PROFILE_VP9_3 => Profile::VP9(VP9::_3),

                _ => Profile::Unknown,
            },

            _ => Profile::Unknown,
        }
    }
}

impl From<Profile> for c_int {
    fn from(value: Profile) -> c_int {
        match value {
            Profile::Unknown => FF_PROFILE_UNKNOWN,
            Profile::Reserved => FF_PROFILE_RESERVED,

            Profile::AAC(AAC::Main) => FF_PROFILE_AAC_MAIN,
            Profile::AAC(AAC::Low) => FF_PROFILE_AAC_LOW,
            Profile::AAC(AAC::SSR) => FF_PROFILE_AAC_SSR,
            Profile::AAC(AAC::LTP) => FF_PROFILE_AAC_LTP,
            Profile::AAC(AAC::HE) => FF_PROFILE_AAC_HE,
            Profile::AAC(AAC::HEv2) => FF_PROFILE_AAC_HE_V2,
            Profile::AAC(AAC::LD) => FF_PROFILE_AAC_LD,
            Profile::AAC(AAC::ELD) => FF_PROFILE_AAC_ELD,

            Profile::AAC(AAC::MPEG2Low) => FF_PROFILE_MPEG2_AAC_LOW,
            Profile::AAC(AAC::MPEG2HE) => FF_PROFILE_MPEG2_AAC_HE,

            Profile::DTS(DTS::Default) => FF_PROFILE_DTS,
            Profile::DTS(DTS::ES) => FF_PROFILE_DTS_ES,
            Profile::DTS(DTS::_96_24) => FF_PROFILE_DTS_96_24,
            Profile::DTS(DTS::HD_HRA) => FF_PROFILE_DTS_HD_HRA,
            Profile::DTS(DTS::HD_MA) => FF_PROFILE_DTS_HD_MA,
            Profile::DTS(DTS::Express) => FF_PROFILE_DTS_EXPRESS,

            Profile::MPEG2(MPEG2::_422) => FF_PROFILE_MPEG2_422,
            Profile::MPEG2(MPEG2::High) => FF_PROFILE_MPEG2_HIGH,
            Profile::MPEG2(MPEG2::SS) => FF_PROFILE_MPEG2_SS,
            Profile::MPEG2(MPEG2::SNRScalable) => FF_PROFILE_MPEG2_SNR_SCALABLE,
            Profile::MPEG2(MPEG2::Main) => FF_PROFILE_MPEG2_MAIN,
            Profile::MPEG2(MPEG2::Simple) => FF_PROFILE_MPEG2_SIMPLE,

            Profile::H264(H264::Constrained) => FF_PROFILE_H264_CONSTRAINED,
            Profile::H264(H264::Intra) => FF_PROFILE_H264_INTRA,
            Profile::H264(H264::Baseline) => FF_PROFILE_H264_BASELINE,
            Profile::H264(H264::ConstrainedBaseline) => FF_PROFILE_H264_CONSTRAINED_BASELINE,
            Profile::H264(H264::Main) => FF_PROFILE_H264_MAIN,
            Profile::H264(H264::Extended) => FF_PROFILE_H264_EXTENDED,
            Profile::H264(H264::High) => FF_PROFILE_H264_HIGH,
            Profile::H264(H264::High10) => FF_PROFILE_H264_HIGH_10,
            Profile::H264(H264::High10Intra) => FF_PROFILE_H264_HIGH_10_INTRA,
            Profile::H264(H264::High422) => FF_PROFILE_H264_HIGH_422,
            Profile::H264(H264::High422Intra) => FF_PROFILE_H264_HIGH_422_INTRA,
            Profile::H264(H264::High444) => FF_PROFILE_H264_HIGH_444,
            Profile::H264(H264::High444Predictive) => FF_PROFILE_H264_HIGH_444_PREDICTIVE,
            Profile::H264(H264::High444Intra) => FF_PROFILE_H264_HIGH_444_INTRA,
            Profile::H264(H264::CAVLC444) => FF_PROFILE_H264_CAVLC_444,

            Profile::VC1(VC1::Simple) => FF_PROFILE_VC1_SIMPLE,
            Profile::VC1(VC1::Main) => FF_PROFILE_VC1_MAIN,
            Profile::VC1(VC1::Complex) => FF_PROFILE_VC1_COMPLEX,
            Profile::VC1(VC1::Advanced) => FF_PROFILE_VC1_ADVANCED,

            Profile::MPEG4(MPEG4::Simple) => FF_PROFILE_MPEG4_SIMPLE,
            Profile::MPEG4(MPEG4::SimpleScalable) => FF_PROFILE_MPEG4_SIMPLE_SCALABLE,
            Profile::MPEG4(MPEG4::Core) => FF_PROFILE_MPEG4_CORE,
            Profile::MPEG4(MPEG4::Main) => FF_PROFILE_MPEG4_MAIN,
            Profile::MPEG4(MPEG4::NBit) => FF_PROFILE_MPEG4_N_BIT,
            Profile::MPEG4(MPEG4::ScalableTexture) => FF_PROFILE_MPEG4_SCALABLE_TEXTURE,
            Profile::MPEG4(MPEG4::SimpleFaceAnimation) => FF_PROFILE_MPEG4_SIMPLE_FACE_ANIMATION,
            Profile::MPEG4(MPEG4::BasicAnimatedTexture) => FF_PROFILE_MPEG4_BASIC_ANIMATED_TEXTURE,
            Profile::MPEG4(MPEG4::Hybrid) => FF_PROFILE_MPEG4_HYBRID,
            Profile::MPEG4(MPEG4::AdvancedRealTime) => FF_PROFILE_MPEG4_ADVANCED_REAL_TIME,
            Profile::MPEG4(MPEG4::CoreScalable) => FF_PROFILE_MPEG4_CORE_SCALABLE,
            Profile::MPEG4(MPEG4::AdvancedCoding) => FF_PROFILE_MPEG4_ADVANCED_CODING,
            Profile::MPEG4(MPEG4::AdvancedCore) => FF_PROFILE_MPEG4_ADVANCED_CORE,
            Profile::MPEG4(MPEG4::AdvancedScalableTexture) => {
                FF_PROFILE_MPEG4_ADVANCED_SCALABLE_TEXTURE
            }
            Profile::MPEG4(MPEG4::SimpleStudio) => FF_PROFILE_MPEG4_SIMPLE_STUDIO,
            Profile::MPEG4(MPEG4::AdvancedSimple) => FF_PROFILE_MPEG4_ADVANCED_SIMPLE,

            Profile::JPEG2000(JPEG2000::CStreamRestriction0) => {
                FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_0
            }
            Profile::JPEG2000(JPEG2000::CStreamRestriction1) => {
                FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_1
            }
            Profile::JPEG2000(JPEG2000::CStreamNoRestriction) => {
                FF_PROFILE_JPEG2000_CSTREAM_NO_RESTRICTION
            }
            Profile::JPEG2000(JPEG2000::DCinema2K) => FF_PROFILE_JPEG2000_DCINEMA_2K,
            Profile::JPEG2000(JPEG2000::DCinema4K) => FF_PROFILE_JPEG2000_DCINEMA_4K,

            Profile::HEVC(HEVC::Main) => FF_PROFILE_HEVC_MAIN,
            Profile::HEVC(HEVC::Main10) => FF_PROFILE_HEVC_MAIN_10,
            Profile::HEVC(HEVC::MainStillPicture) => FF_PROFILE_HEVC_MAIN_STILL_PICTURE,
            Profile::HEVC(HEVC::Rext) => FF_PROFILE_HEVC_REXT,

            Profile::VP9(VP9::_0) => FF_PROFILE_VP9_0,
            Profile::VP9(VP9::_1) => FF_PROFILE_VP9_1,
            Profile::VP9(VP9::_2) => FF_PROFILE_VP9_2,
            Profile::VP9(VP9::_3) => FF_PROFILE_VP9_3,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/threading.rs">
use ffi::*;
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Config {
    pub kind: Type,
    pub count: usize,
    #[cfg(not(feature = "ffmpeg_6_0"))]
    pub safe: bool,
}

impl Config {
    pub fn kind(value: Type) -> Self {
        Config {
            kind: value,
            ..Default::default()
        }
    }

    pub fn count(value: usize) -> Self {
        Config {
            count: value,
            ..Default::default()
        }
    }

    #[cfg(not(feature = "ffmpeg_6_0"))]
    pub fn safe(value: bool) -> Self {
        Config {
            safe: value,
            ..Default::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            kind: Type::None,
            count: 0,
            #[cfg(not(feature = "ffmpeg_6_0"))]
            safe: false,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Frame,
    Slice,
}

impl From<c_int> for Type {
    fn from(value: c_int) -> Type {
        match value {
            FF_THREAD_FRAME => Type::Frame,
            FF_THREAD_SLICE => Type::Slice,

            _ => Type::None,
        }
    }
}

impl From<Type> for c_int {
    fn from(value: Type) -> c_int {
        match value {
            Type::None => 0,
            Type::Frame => FF_THREAD_FRAME,
            Type::Slice => FF_THREAD_SLICE,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/traits.rs">
use super::{decoder, encoder};
use codec::{Audio, Id, Video};
use Codec;

pub trait Decoder {
    fn decoder(self) -> Option<Codec>;
}

impl<'a> Decoder for &'a str {
    fn decoder(self) -> Option<Codec> {
        decoder::find_by_name(self)
    }
}

impl Decoder for Id {
    fn decoder(self) -> Option<Codec> {
        decoder::find(self)
    }
}

impl Decoder for Codec {
    fn decoder(self) -> Option<Codec> {
        if self.is_decoder() {
            Some(self)
        } else {
            None
        }
    }
}

impl Decoder for Option<Codec> {
    fn decoder(self) -> Option<Codec> {
        self.and_then(|c| c.decoder())
    }
}

impl Decoder for Audio {
    fn decoder(self) -> Option<Codec> {
        if self.is_decoder() {
            Some(*self)
        } else {
            None
        }
    }
}

impl Decoder for Video {
    fn decoder(self) -> Option<Codec> {
        if self.is_decoder() {
            Some(*self)
        } else {
            None
        }
    }
}

pub trait Encoder {
    fn encoder(self) -> Option<Codec>;
}

impl<'a> Encoder for &'a str {
    fn encoder(self) -> Option<Codec> {
        encoder::find_by_name(self)
    }
}

impl Encoder for Id {
    fn encoder(self) -> Option<Codec> {
        encoder::find(self)
    }
}

impl Encoder for Codec {
    fn encoder(self) -> Option<Codec> {
        if self.is_encoder() {
            Some(self)
        } else {
            None
        }
    }
}

impl Encoder for Option<Codec> {
    fn encoder(self) -> Option<Codec> {
        self.and_then(|c| c.encoder())
    }
}

impl Encoder for Audio {
    fn encoder(self) -> Option<Codec> {
        if self.is_encoder() {
            Some(*self)
        } else {
            None
        }
    }
}

impl Encoder for Video {
    fn encoder(self) -> Option<Codec> {
        if self.is_encoder() {
            Some(*self)
        } else {
            None
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/codec/video.rs">
use std::ops::Deref;

use super::codec::Codec;
use ffi::*;
use {format, Rational};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Video {
    codec: Codec,
}

impl Video {
    pub unsafe fn new(codec: Codec) -> Video {
        Video { codec }
    }
}

impl Video {
    pub fn rates(&self) -> Option<RateIter> {
        unsafe {
            if (*self.codec.as_ptr()).supported_framerates.is_null() {
                None
            } else {
                Some(RateIter::new((*self.codec.as_ptr()).supported_framerates))
            }
        }
    }

    pub fn formats(&self) -> Option<FormatIter> {
        unsafe {
            if (*self.codec.as_ptr()).pix_fmts.is_null() {
                None
            } else {
                Some(FormatIter::new((*self.codec.as_ptr()).pix_fmts))
            }
        }
    }
}

impl Deref for Video {
    type Target = Codec;

    fn deref(&self) -> &Self::Target {
        &self.codec
    }
}

pub struct RateIter {
    ptr: *const AVRational,
}

impl RateIter {
    pub fn new(ptr: *const AVRational) -> Self {
        RateIter { ptr }
    }
}

impl Iterator for RateIter {
    type Item = Rational;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if (*self.ptr).num == 0 && (*self.ptr).den == 0 {
                return None;
            }

            let rate = (*self.ptr).into();
            self.ptr = self.ptr.offset(1);

            Some(rate)
        }
    }
}

pub struct FormatIter {
    ptr: *const AVPixelFormat,
}

impl FormatIter {
    pub fn new(ptr: *const AVPixelFormat) -> Self {
        FormatIter { ptr }
    }
}

impl Iterator for FormatIter {
    type Item = format::Pixel;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if *self.ptr == AVPixelFormat::AV_PIX_FMT_NONE {
                return None;
            }

            let format = (*self.ptr).into();
            self.ptr = self.ptr.offset(1);

            Some(format)
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/device/extensions.rs">
use std::marker::PhantomData;
use std::ptr;

use device;
use ffi::*;
use format::context::common::Context;
use libc::c_int;
use Error;

impl Context {
    pub fn devices(&self) -> Result<DeviceIter, Error> {
        unsafe { DeviceIter::wrap(self.as_ptr()) }
    }
}

pub struct DeviceIter<'a> {
    ptr: *mut AVDeviceInfoList,
    cur: c_int,

    _marker: PhantomData<&'a ()>,
}

impl<'a> DeviceIter<'a> {
    pub unsafe fn wrap(ctx: *const AVFormatContext) -> Result<Self, Error> {
        let mut ptr: *mut AVDeviceInfoList = ptr::null_mut();

        match avdevice_list_devices(ctx as *mut _, &mut ptr) {
            n if n < 0 => Err(Error::from(n)),

            _ => Ok(DeviceIter {
                ptr,
                cur: 0,
                _marker: PhantomData,
            }),
        }
    }
}

impl<'a> DeviceIter<'a> {
    pub fn default(&self) -> usize {
        unsafe { (*self.ptr).default_device as usize }
    }
}

impl<'a> Drop for DeviceIter<'a> {
    fn drop(&mut self) {
        unsafe {
            avdevice_free_list_devices(&mut self.ptr);
        }
    }
}

impl<'a> Iterator for DeviceIter<'a> {
    type Item = device::Info<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.cur >= (*self.ptr).nb_devices {
                None
            } else {
                self.cur += 1;
                Some(device::Info::wrap(
                    *(*self.ptr).devices.offset((self.cur - 1) as isize),
                ))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.ptr).nb_devices as usize;

            (length - self.cur as usize, Some(length - self.cur as usize))
        }
    }
}

impl<'a> ExactSizeIterator for DeviceIter<'a> {}
</file>

<file path="patches/ffmpeg-next/src/device/input.rs">
use std::ptr;

use ffi::*;
use format;
use Format;

pub struct AudioIter(*mut AVInputFormat);

impl Iterator for AudioIter {
    type Item = Format;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            // We get a clippy warning in 4.4 but not in 5.0 and newer, so we allow that cast to not complicate the code
            #[allow(clippy::unnecessary_cast)]
            let ptr = av_input_audio_device_next(self.0) as *mut AVInputFormat;

            if ptr.is_null() && !self.0.is_null() {
                None
            } else {
                self.0 = ptr;

                Some(Format::Input(format::Input::wrap(ptr)))
            }
        }
    }
}

pub fn audio() -> AudioIter {
    AudioIter(ptr::null_mut())
}

pub struct VideoIter(*mut AVInputFormat);

impl Iterator for VideoIter {
    type Item = Format;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            // We get a clippy warning in 4.4 but not in 5.0 and newer, so we allow that cast to not complicate the code
            #[allow(clippy::unnecessary_cast)]
            let ptr = av_input_video_device_next(self.0) as *mut AVInputFormat;

            if ptr.is_null() && !self.0.is_null() {
                None
            } else {
                self.0 = ptr;

                Some(Format::Input(format::Input::wrap(ptr)))
            }
        }
    }
}

pub fn video() -> VideoIter {
    VideoIter(ptr::null_mut())
}
</file>

<file path="patches/ffmpeg-next/src/device/mod.rs">
pub mod extensions;
pub mod input;
pub mod output;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::str::from_utf8_unchecked;

use ffi::*;

pub struct Info<'a> {
    ptr: *mut AVDeviceInfo,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Info<'a> {
    pub unsafe fn wrap(ptr: *mut AVDeviceInfo) -> Self {
        Info {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVDeviceInfo {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVDeviceInfo {
        self.ptr
    }
}

impl<'a> Info<'a> {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).device_name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).device_description).to_bytes())
        }
    }
}

pub fn register_all() {
    unsafe {
        avdevice_register_all();
    }
}

pub fn version() -> u32 {
    unsafe { avdevice_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avdevice_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avdevice_license()).to_bytes()) }
}
</file>

<file path="patches/ffmpeg-next/src/device/output.rs">
use std::ptr;

use ffi::*;
use format;
use Format;

pub struct AudioIter(*mut AVOutputFormat);

impl Iterator for AudioIter {
    type Item = Format;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            // We get a clippy warning in 4.4 but not in 5.0 and newer, so we allow that cast to not complicate the code
            #[allow(clippy::unnecessary_cast)]
            let ptr = av_output_audio_device_next(self.0) as *mut AVOutputFormat;

            if ptr.is_null() && !self.0.is_null() {
                None
            } else {
                self.0 = ptr;

                Some(Format::Output(format::Output::wrap(ptr)))
            }
        }
    }
}

pub fn audio() -> AudioIter {
    AudioIter(ptr::null_mut())
}

pub struct VideoIter(*mut AVOutputFormat);

impl Iterator for VideoIter {
    type Item = Format;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            // We get a clippy warning in 4.4 but not in 5.0 and newer, so we allow that cast to not complicate the code
            #[allow(clippy::unnecessary_cast)]
            let ptr = av_output_video_device_next(self.0) as *mut AVOutputFormat;

            if ptr.is_null() && !self.0.is_null() {
                None
            } else {
                self.0 = ptr;

                Some(Format::Output(format::Output::wrap(ptr)))
            }
        }
    }
}

pub fn video() -> VideoIter {
    VideoIter(ptr::null_mut())
}
</file>

<file path="patches/ffmpeg-next/src/filter/context/context.rs">
use std::marker::PhantomData;

use super::{Sink, Source};
use ffi::*;
use libc::c_void;
use {format, option, ChannelLayout};

pub struct Context<'a> {
    ptr: *mut AVFilterContext,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    pub unsafe fn wrap(ptr: *mut AVFilterContext) -> Self {
        Context {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFilterContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterContext {
        self.ptr
    }
}

impl<'a> Context<'a> {
    pub fn source(&'a mut self) -> Source<'a> {
        unsafe { Source::wrap(self) }
    }

    pub fn sink(&'a mut self) -> Sink<'a> {
        unsafe { Sink::wrap(self) }
    }

    pub fn set_pixel_format(&mut self, value: format::Pixel) {
        let _ = option::Settable::set::<AVPixelFormat>(self, "pix_fmts", &value.into());
    }

    pub fn set_sample_format(&mut self, value: format::Sample) {
        let _ = option::Settable::set::<AVSampleFormat>(self, "sample_fmts", &value.into());
    }

    pub fn set_sample_rate(&mut self, value: u32) {
        let _ = option::Settable::set(self, "sample_rates", &i64::from(value));
    }

    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        let _ = option::Settable::set(self, "channel_layouts", &value.bits());
    }
}

unsafe impl<'a> option::Target for Context<'a> {
    fn as_ptr(&self) -> *const c_void {
        self.ptr as *const _
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        self.ptr as *mut _
    }
}

impl<'a> option::Settable for Context<'a> {}
</file>

<file path="patches/ffmpeg-next/src/filter/context/mod.rs">
mod context;
pub use self::context::Context;

mod source;
pub use self::source::Source;

mod sink;
pub use self::sink::Sink;
</file>

<file path="patches/ffmpeg-next/src/filter/context/sink.rs">
use super::Context;
use ffi::*;
use libc::c_int;
use {Error, Frame};

pub struct Sink<'a> {
    ctx: &'a mut Context<'a>,
}

impl<'a> Sink<'a> {
    pub unsafe fn wrap<'b>(ctx: &'b mut Context<'b>) -> Sink<'b> {
        Sink { ctx }
    }
}

impl<'a> Sink<'a> {
    pub fn frame(&mut self, frame: &mut Frame) -> Result<(), Error> {
        unsafe {
            match av_buffersink_get_frame(self.ctx.as_mut_ptr(), frame.as_mut_ptr()) {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn samples(&mut self, frame: &mut Frame, samples: usize) -> Result<(), Error> {
        unsafe {
            match av_buffersink_get_samples(
                self.ctx.as_mut_ptr(),
                frame.as_mut_ptr(),
                samples as c_int,
            ) {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn set_frame_size(&mut self, value: u32) {
        unsafe {
            av_buffersink_set_frame_size(self.ctx.as_mut_ptr(), value);
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/filter/context/source.rs">
use std::ptr;

use super::Context;
use ffi::*;
use {Error, Frame};

pub struct Source<'a> {
    ctx: &'a mut Context<'a>,
}

impl<'a> Source<'a> {
    pub unsafe fn wrap<'b>(ctx: &'b mut Context<'b>) -> Source<'b> {
        Source { ctx }
    }
}

impl<'a> Source<'a> {
    pub fn failed_requests(&self) -> usize {
        unsafe { av_buffersrc_get_nb_failed_requests(self.ctx.as_ptr() as *mut _) as usize }
    }

    pub fn add(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            match av_buffersrc_add_frame(self.ctx.as_mut_ptr(), frame.as_ptr() as *mut _) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        unsafe { self.add(&Frame::wrap(ptr::null_mut())) }
    }

    pub fn close(&mut self, pts: i64) -> Result<(), Error> {
        unsafe {
            match av_buffersrc_close(self.ctx.as_mut_ptr(), pts, 0) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/filter/filter.rs">
use std::ffi::CStr;
use std::marker::PhantomData;
use std::str::from_utf8_unchecked;

use super::{Flags, Pad};
use ffi::*;

pub struct Filter {
    ptr: *mut AVFilter,
}

impl Filter {
    pub unsafe fn wrap(ptr: *mut AVFilter) -> Self {
        Filter { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFilter {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilter {
        self.ptr
    }
}

impl Filter {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn description(&self) -> Option<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).description;

            if ptr.is_null() {
                None
            } else {
                Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
            }
        }
    }

    pub fn inputs(&self) -> Option<PadIter> {
        unsafe {
            let ptr = (*self.as_ptr()).inputs;

            if ptr.is_null() {
                None
            } else {
                #[cfg(not(feature = "ffmpeg_6_0"))]
                let nb_inputs = avfilter_pad_count((*self.as_ptr()).inputs) as isize;
                #[cfg(feature = "ffmpeg_6_0")]
                let nb_inputs = (*self.as_ptr()).nb_inputs as isize;

                Some(PadIter::new((*self.as_ptr()).inputs, nb_inputs))
            }
        }
    }

    pub fn outputs(&self) -> Option<PadIter> {
        unsafe {
            let ptr = (*self.as_ptr()).outputs;

            if ptr.is_null() {
                None
            } else {
                #[cfg(not(feature = "ffmpeg_6_0"))]
                let nb_outputs = avfilter_pad_count((*self.as_ptr()).outputs) as isize;
                #[cfg(feature = "ffmpeg_6_0")]
                let nb_outputs = (*self.as_ptr()).nb_outputs as isize;

                Some(PadIter::new((*self.as_ptr()).outputs, nb_outputs))
            }
        }
    }

    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags) }
    }
}

pub struct PadIter<'a> {
    ptr: *const AVFilterPad,
    count: isize,
    cur: isize,

    _marker: PhantomData<&'a ()>,
}

impl<'a> PadIter<'a> {
    pub fn new(ptr: *const AVFilterPad, count: isize) -> Self {
        PadIter {
            ptr,
            count,
            cur: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for PadIter<'a> {
    type Item = Pad<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.cur >= self.count {
                return None;
            }

            let pad = Pad::wrap(self.ptr, self.cur);
            self.cur += 1;

            Some(pad)
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/filter/flag.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const DYNAMIC_INPUTS            = AVFILTER_FLAG_DYNAMIC_INPUTS;
        const DYNAMIC_OUTPUTS           = AVFILTER_FLAG_DYNAMIC_OUTPUTS;
        const SLICE_THREADS             = AVFILTER_FLAG_SLICE_THREADS;
        const SUPPORT_TIMELINE_GENERIC  = AVFILTER_FLAG_SUPPORT_TIMELINE_GENERIC;
        const SUPPORT_TIMELINE_INTERNAL = AVFILTER_FLAG_SUPPORT_TIMELINE_INTERNAL;
        const SUPPORT_TIMELINE          = AVFILTER_FLAG_SUPPORT_TIMELINE;
    }
}
</file>

<file path="patches/ffmpeg-next/src/filter/graph.rs">
use std::ffi::{CStr, CString};
use std::ptr;
use std::str::from_utf8_unchecked;

use super::{Context, Filter};
use ffi::*;
use libc::c_int;
use Error;

pub struct Graph {
    ptr: *mut AVFilterGraph,
}

unsafe impl Send for Graph {}
unsafe impl Sync for Graph {}

impl Graph {
    pub unsafe fn wrap(ptr: *mut AVFilterGraph) -> Self {
        Graph { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFilterGraph {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterGraph {
        self.ptr
    }
}

impl Graph {
    pub fn new() -> Self {
        unsafe {
            let ptr = avfilter_graph_alloc();

            if ptr.is_null() {
                panic!("out of memory");
            }

            Graph::wrap(ptr)
        }
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        unsafe {
            match avfilter_graph_config(self.as_mut_ptr(), ptr::null_mut()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn add<'a, 'b>(
        &'a mut self,
        filter: &Filter,
        name: &str,
        args: &str,
    ) -> Result<Context<'b>, Error>
    where
        'a: 'b,
    {
        unsafe {
            let name = CString::new(name).unwrap();
            let args = CString::new(args).unwrap();
            let mut context = ptr::null_mut();

            match avfilter_graph_create_filter(
                &mut context as *mut *mut AVFilterContext,
                filter.as_ptr(),
                name.as_ptr(),
                args.as_ptr(),
                ptr::null_mut(),
                self.as_mut_ptr(),
            ) {
                n if n >= 0 => Ok(Context::wrap(context)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn get<'a, 'b>(&'b mut self, name: &str) -> Option<Context<'b>>
    where
        'a: 'b,
    {
        unsafe {
            let name = CString::new(name).unwrap();
            let ptr = avfilter_graph_get_filter(self.as_mut_ptr(), name.as_ptr());

            if ptr.is_null() {
                None
            } else {
                Some(Context::wrap(ptr))
            }
        }
    }

    pub fn dump(&self) -> String {
        unsafe {
            let ptr = avfilter_graph_dump(self.as_ptr() as *mut _, ptr::null());
            let cstr = from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes());
            let string = cstr.to_owned();

            av_free(ptr as *mut _);

            string
        }
    }

    pub fn input(&mut self, name: &str, pad: usize) -> Result<Parser, Error> {
        Parser::new(self).input(name, pad)
    }

    pub fn output(&mut self, name: &str, pad: usize) -> Result<Parser, Error> {
        Parser::new(self).output(name, pad)
    }

    pub fn parse(&mut self, spec: &str) -> Result<(), Error> {
        Parser::new(self).parse(spec)
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            avfilter_graph_free(&mut self.as_mut_ptr());
        }
    }
}

pub struct Parser<'a> {
    graph: &'a mut Graph,
    inputs: *mut AVFilterInOut,
    outputs: *mut AVFilterInOut,
}

impl<'a> Parser<'a> {
    pub fn new(graph: &mut Graph) -> Parser {
        Parser {
            graph,
            inputs: ptr::null_mut(),
            outputs: ptr::null_mut(),
        }
    }

    pub fn input(mut self, name: &str, pad: usize) -> Result<Self, Error> {
        unsafe {
            let mut context = self.graph.get(name).ok_or(Error::InvalidData)?;
            let input = avfilter_inout_alloc();

            if input.is_null() {
                panic!("out of memory");
            }

            let name = CString::new(name).unwrap();

            (*input).name = av_strdup(name.as_ptr());
            (*input).filter_ctx = context.as_mut_ptr();
            (*input).pad_idx = pad as c_int;
            (*input).next = ptr::null_mut();

            if self.inputs.is_null() {
                self.inputs = input;
            } else {
                (*self.inputs).next = input;
            }
        }

        Ok(self)
    }

    pub fn output(mut self, name: &str, pad: usize) -> Result<Self, Error> {
        unsafe {
            let mut context = self.graph.get(name).ok_or(Error::InvalidData)?;
            let output = avfilter_inout_alloc();

            if output.is_null() {
                panic!("out of memory");
            }

            let name = CString::new(name).unwrap();

            (*output).name = av_strdup(name.as_ptr());
            (*output).filter_ctx = context.as_mut_ptr();
            (*output).pad_idx = pad as c_int;
            (*output).next = ptr::null_mut();

            if self.outputs.is_null() {
                self.outputs = output;
            } else {
                (*self.outputs).next = output;
            }
        }

        Ok(self)
    }

    pub fn parse(mut self, spec: &str) -> Result<(), Error> {
        unsafe {
            let spec = CString::new(spec).unwrap();

            let result = avfilter_graph_parse_ptr(
                self.graph.as_mut_ptr(),
                spec.as_ptr(),
                &mut self.inputs,
                &mut self.outputs,
                ptr::null_mut(),
            );

            avfilter_inout_free(&mut self.inputs);
            avfilter_inout_free(&mut self.outputs);

            match result {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
</file>

<file path="patches/ffmpeg-next/src/filter/mod.rs">
pub mod flag;
pub use self::flag::Flags;

pub mod pad;
pub use self::pad::Pad;

pub mod filter;
pub use self::filter::Filter;

pub mod context;
pub use self::context::{Context, Sink, Source};

pub mod graph;
pub use self::graph::Graph;

use std::ffi::{CStr, CString};
use std::str::from_utf8_unchecked;

use ffi::*;
#[cfg(not(feature = "ffmpeg_5_0"))]
use Error;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn register_all() {
    unsafe {
        avfilter_register_all();
    }
}

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn register(filter: &Filter) -> Result<(), Error> {
    unsafe {
        match avfilter_register(filter.as_ptr() as *mut _) {
            0 => Ok(()),
            _ => Err(Error::InvalidData),
        }
    }
}

pub fn version() -> u32 {
    unsafe { avfilter_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avfilter_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avfilter_license()).to_bytes()) }
}

pub fn find(name: &str) -> Option<Filter> {
    unsafe {
        let name = CString::new(name).unwrap();
        let ptr = avfilter_get_by_name(name.as_ptr());

        if ptr.is_null() {
            None
        } else {
            Some(Filter::wrap(ptr as *mut _))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paditer() {
        #[cfg(not(feature = "ffmpeg_5_0"))]
        register_all();
        assert_eq!(
            find("overlay")
                .unwrap()
                .inputs()
                .unwrap()
                .map(|input| input.name().unwrap().to_string())
                .collect::<Vec<_>>(),
            vec!("main", "overlay")
        );
    }
}
</file>

<file path="patches/ffmpeg-next/src/filter/pad.rs">
use std::ffi::CStr;
use std::marker::PhantomData;
use std::str::from_utf8_unchecked;

use ffi::*;
use media;

pub struct Pad<'a> {
    ptr: *const AVFilterPad,
    idx: isize,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Pad<'a> {
    pub unsafe fn wrap(ptr: *const AVFilterPad, idx: isize) -> Self {
        Pad {
            ptr,
            idx,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFilterPad {
        self.ptr
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFilterPad {
        self.ptr as *mut _
    }
}

impl<'a> Pad<'a> {
    pub fn name(&self) -> Option<&str> {
        unsafe {
            let ptr = avfilter_pad_get_name(self.ptr, self.idx as i32);

            if ptr.is_null() {
                None
            } else {
                Some(from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
            }
        }
    }

    pub fn medium(&self) -> media::Type {
        unsafe { media::Type::from(avfilter_pad_get_type(self.ptr, self.idx as i32)) }
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/chapter/chapter_mut.rs">
use std::mem;
use std::ops::Deref;

use super::Chapter;
use ffi::*;
use format::context::common::Context;
use {Dictionary, DictionaryMut, Rational};

// WARNING: index refers to the offset in the chapters array (starting from 0)
// it is not necessarly equal to the id (which may start at 1)
pub struct ChapterMut<'a> {
    context: &'a mut Context,
    index: usize,

    immutable: Chapter<'a>,
}

impl<'a> ChapterMut<'a> {
    pub unsafe fn wrap(context: &mut Context, index: usize) -> ChapterMut {
        ChapterMut {
            context: mem::transmute_copy(&context),
            index,

            immutable: Chapter::wrap(mem::transmute_copy(&context), index),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVChapter {
        *(*self.context.as_mut_ptr()).chapters.add(self.index)
    }
}

impl<'a> ChapterMut<'a> {
    pub fn set_id(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).id = value as _;
        }
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }

    pub fn set_start(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).start = value;
        }
    }

    pub fn set_end(&mut self, value: i64) {
        unsafe {
            (*self.as_mut_ptr()).end = value;
        }
    }

    pub fn set_metadata<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        // dictionary.set() allocates the AVDictionary the first time a key/value is inserted
        // so we want to update the metadata dictionary afterwards
        unsafe {
            let mut dictionary = Dictionary::own(self.metadata().as_mut_ptr());
            dictionary.set(key.as_ref(), value.as_ref());
            (*self.as_mut_ptr()).metadata = dictionary.disown();
        }
    }

    pub fn metadata(&mut self) -> DictionaryMut {
        unsafe { DictionaryMut::wrap((*self.as_mut_ptr()).metadata) }
    }
}

impl<'a> Deref for ChapterMut<'a> {
    type Target = Chapter<'a>;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/chapter/chapter.rs">
use ffi::*;
use {DictionaryRef, Rational};

use format::context::common::Context;

// WARNING: index refers to the offset in the chapters array (starting from 0)
// it is not necessarly equal to the id (which may start at 1)
pub struct Chapter<'a> {
    context: &'a Context,
    index: usize,
}

impl<'a> Chapter<'a> {
    pub unsafe fn wrap(context: &Context, index: usize) -> Chapter {
        Chapter { context, index }
    }

    pub unsafe fn as_ptr(&self) -> *const AVChapter {
        *(*self.context.as_ptr()).chapters.add(self.index)
    }
}

impl<'a> Chapter<'a> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn id(&self) -> i64 {
        #[allow(clippy::unnecessary_cast)]
        unsafe {
            (*self.as_ptr()).id as i64
        }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).time_base) }
    }

    pub fn start(&self) -> i64 {
        unsafe { (*self.as_ptr()).start }
    }

    pub fn end(&self) -> i64 {
        unsafe { (*self.as_ptr()).end }
    }

    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}

impl<'a> PartialEq for Chapter<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.as_ptr() == other.as_ptr() }
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/chapter/mod.rs">
mod chapter;
pub use self::chapter::Chapter;

mod chapter_mut;
pub use self::chapter_mut::ChapterMut;
</file>

<file path="patches/ffmpeg-next/src/format/context/common.rs">
use std::fmt;
use std::mem;
use std::ptr;
use std::rc::Rc;

use super::destructor::{self, Destructor};
use ffi::*;
use libc::{c_int, c_uint};
use {media, Chapter, ChapterMut, DictionaryRef, Stream, StreamMut};

pub struct Context {
    ptr: *mut AVFormatContext,
    dtor: Rc<Destructor>,
}

unsafe impl Send for Context {}

impl Context {
    pub unsafe fn wrap(ptr: *mut AVFormatContext, mode: destructor::Mode) -> Self {
        Context {
            ptr,
            dtor: Rc::new(Destructor::new(ptr, mode)),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }

    pub unsafe fn destructor(&self) -> Rc<Destructor> {
        Rc::clone(&self.dtor)
    }
}

impl Context {
    #[inline]
    pub fn nb_streams(&self) -> u32 {
        unsafe { (*self.as_ptr()).nb_streams }
    }

    pub fn stream<'a, 'b>(&'a self, index: usize) -> Option<Stream<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_streams() as usize {
                None
            } else {
                Some(Stream::wrap(self, index))
            }
        }
    }

    pub fn stream_mut<'a, 'b>(&'a mut self, index: usize) -> Option<StreamMut<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_streams() as usize {
                None
            } else {
                Some(StreamMut::wrap(self, index))
            }
        }
    }

    pub fn streams(&self) -> StreamIter {
        StreamIter::new(self)
    }

    pub fn streams_mut(&mut self) -> StreamIterMut {
        StreamIterMut::new(self)
    }

    pub fn bit_rate(&self) -> i64 {
        unsafe { (*self.as_ptr()).bit_rate }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).duration }
    }

    #[inline]
    pub fn nb_chapters(&self) -> u32 {
        unsafe { (*self.as_ptr()).nb_chapters }
    }

    pub fn chapter<'a, 'b>(&'a self, index: usize) -> Option<Chapter<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_chapters() as usize {
                None
            } else {
                Some(Chapter::wrap(self, index))
            }
        }
    }

    pub fn chapter_mut<'a, 'b>(&'a mut self, index: usize) -> Option<ChapterMut<'b>>
    where
        'a: 'b,
    {
        unsafe {
            if index >= self.nb_chapters() as usize {
                None
            } else {
                Some(ChapterMut::wrap(self, index))
            }
        }
    }

    pub fn chapters(&self) -> ChapterIter {
        ChapterIter::new(self)
    }

    pub fn chapters_mut(&mut self) -> ChapterIterMut {
        ChapterIterMut::new(self)
    }

    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}

pub struct Best<'a> {
    context: &'a Context,

    wanted: i32,
    related: i32,
}

impl<'a> Best<'a> {
    pub unsafe fn new<'b, 'c: 'b>(context: &'c Context) -> Best<'b> {
        Best {
            context,

            wanted: -1,
            related: -1,
        }
    }

    pub fn wanted<'b>(mut self, stream: &'b Stream) -> Best<'a>
    where
        'a: 'b,
    {
        self.wanted = stream.index() as i32;
        self
    }

    pub fn related<'b>(mut self, stream: &'b Stream) -> Best<'a>
    where
        'a: 'b,
    {
        self.related = stream.index() as i32;
        self
    }

    pub fn best<'b>(self, kind: media::Type) -> Option<Stream<'b>>
    where
        'a: 'b,
    {
        unsafe {
            let decoder = ptr::null_mut();
            let index = av_find_best_stream(
                self.context.ptr,
                kind.into(),
                self.wanted as c_int,
                self.related as c_int,
                decoder,
                0,
            );

            if index >= 0 {
                Some(Stream::wrap(self.context, index as usize))
            } else {
                None
            }
        }
    }
}

pub struct StreamIter<'a> {
    context: &'a Context,
    current: c_uint,
}

impl<'a> StreamIter<'a> {
    pub fn new<'s, 'c: 's>(context: &'c Context) -> StreamIter<'s> {
        StreamIter {
            context,
            current: 0,
        }
    }
}

impl<'a> StreamIter<'a> {
    pub fn wanted<'b, 'c>(&self, stream: &'b Stream) -> Best<'c>
    where
        'a: 'b,
        'a: 'c,
    {
        unsafe { Best::new(self.context).wanted(stream) }
    }

    pub fn related<'b, 'c>(&self, stream: &'b Stream) -> Best<'c>
    where
        'a: 'b,
        'a: 'c,
    {
        unsafe { Best::new(self.context).related(stream) }
    }

    pub fn best<'b>(&self, kind: media::Type) -> Option<Stream<'b>>
    where
        'a: 'b,
    {
        unsafe { Best::new(self.context).best(kind) }
    }
}

impl<'a> Iterator for StreamIter<'a> {
    type Item = Stream<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= self.context.nb_streams() {
                return None;
            }

            self.current += 1;

            Some(Stream::wrap(self.context, (self.current - 1) as usize))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.context.nb_streams() as usize;

        (
            length - self.current as usize,
            Some(length - self.current as usize),
        )
    }
}

impl<'a> ExactSizeIterator for StreamIter<'a> {}

pub struct StreamIterMut<'a> {
    context: &'a mut Context,
    current: c_uint,
}

impl<'a> StreamIterMut<'a> {
    pub fn new<'s, 'c: 's>(context: &'c mut Context) -> StreamIterMut<'s> {
        StreamIterMut {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for StreamIterMut<'a> {
    type Item = StreamMut<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.current >= self.context.nb_streams() {
            return None;
        }
        self.current += 1;

        unsafe {
            Some(StreamMut::wrap(
                mem::transmute_copy(&self.context),
                (self.current - 1) as usize,
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.context.nb_streams() as usize;

        (
            length - self.current as usize,
            Some(length - self.current as usize),
        )
    }
}

impl<'a> ExactSizeIterator for StreamIterMut<'a> {}

pub struct ChapterIter<'a> {
    context: &'a Context,
    current: c_uint,
}

impl<'a> ChapterIter<'a> {
    pub fn new<'s, 'c: 's>(context: &'c Context) -> ChapterIter<'s> {
        ChapterIter {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for ChapterIter<'a> {
    type Item = Chapter<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= (*self.context.as_ptr()).nb_chapters {
                return None;
            }

            self.current += 1;

            Some(Chapter::wrap(self.context, (self.current - 1) as usize))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.context.as_ptr()).nb_chapters as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

impl<'a> ExactSizeIterator for ChapterIter<'a> {}

pub struct ChapterIterMut<'a> {
    context: &'a mut Context,
    current: c_uint,
}

impl<'a> ChapterIterMut<'a> {
    pub fn new<'s, 'c: 's>(context: &'c mut Context) -> ChapterIterMut<'s> {
        ChapterIterMut {
            context,
            current: 0,
        }
    }
}

impl<'a> Iterator for ChapterIterMut<'a> {
    type Item = ChapterMut<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= (*self.context.as_ptr()).nb_chapters {
                return None;
            }

            self.current += 1;

            Some(ChapterMut::wrap(
                mem::transmute_copy(&self.context),
                (self.current - 1) as usize,
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.context.as_ptr()).nb_chapters as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

impl<'a> ExactSizeIterator for ChapterIterMut<'a> {}

impl fmt::Debug for Context {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut s = fmt.debug_struct("AVFormatContext");
        s.field("bit_rate", &self.bit_rate());
        s.field("duration", &self.duration());
        s.field("nb_chapters", &self.nb_chapters());
        s.field("nb_streams", &self.nb_streams());
        s.finish()
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/context/destructor.rs">
use ffi::*;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Input,
    Output,
}

pub struct Destructor {
    ptr: *mut AVFormatContext,
    mode: Mode,
}

impl Destructor {
    pub unsafe fn new(ptr: *mut AVFormatContext, mode: Mode) -> Self {
        Destructor { ptr, mode }
    }
}

impl Drop for Destructor {
    fn drop(&mut self) {
        unsafe {
            match self.mode {
                Mode::Input => avformat_close_input(&mut self.ptr),

                Mode::Output => {
                    avio_close((*self.ptr).pb);
                    avformat_free_context(self.ptr);
                }
            }
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/context/input.rs">
use std::ffi::CString;
use std::mem;
use std::ops::{Deref, DerefMut};

use super::common::Context;
use super::destructor;
use ffi::*;
use util::range::Range;
#[cfg(not(feature = "ffmpeg_5_0"))]
use Codec;
use {format, Error, Packet, Stream};

pub struct Input {
    ptr: *mut AVFormatContext,
    ctx: Context,
}

unsafe impl Send for Input {}

impl Input {
    pub unsafe fn wrap(ptr: *mut AVFormatContext) -> Self {
        Input {
            ptr,
            ctx: Context::wrap(ptr, destructor::Mode::Input),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }
}

impl Input {
    pub fn format(&self) -> format::Input {
        // We get a clippy warning in 4.4 but not in 5.0 and newer, so we allow that cast to not complicate the code
        #[allow(clippy::unnecessary_cast)]
        unsafe {
            format::Input::wrap((*self.as_ptr()).iformat as *mut AVInputFormat)
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn video_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).video_codec;

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn audio_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).audio_codec;

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn subtitle_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).subtitle_codec;

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn data_codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).data_codec;

            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    pub fn probe_score(&self) -> i32 {
        unsafe { (*self.as_ptr()).probe_score }
    }

    pub fn packets(&mut self) -> PacketIter {
        PacketIter::new(self)
    }

    pub fn pause(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_pause(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        unsafe {
            match av_read_play(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn seek<R: Range<i64>>(&mut self, ts: i64, range: R) -> Result<(), Error> {
        unsafe {
            match avformat_seek_file(
                self.as_mut_ptr(),
                -1,
                range.start().cloned().unwrap_or(i64::min_value()),
                ts,
                range.end().cloned().unwrap_or(i64::max_value()),
                0,
            ) {
                s if s >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Deref for Input {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub struct PacketIter<'a> {
    context: &'a mut Input,
}

impl<'a> PacketIter<'a> {
    pub fn new(context: &mut Input) -> PacketIter {
        PacketIter { context }
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = (Stream<'a>, Packet);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut packet = Packet::empty();

        loop {
            match packet.read(self.context) {
                Ok(..) => unsafe {
                    return Some((
                        Stream::wrap(mem::transmute_copy(&self.context), packet.stream()),
                        packet,
                    ));
                },

                Err(Error::Eof) => return None,

                Err(..) => (),
            }
        }
    }
}

pub fn dump(ctx: &Input, index: i32, url: Option<&str>) {
    let url = url.map(|u| CString::new(u).unwrap());

    unsafe {
        av_dump_format(
            ctx.as_ptr() as *mut _,
            index,
            url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
            0,
        );
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/context/mod.rs">
pub mod destructor;
pub use self::destructor::Destructor;

pub mod input;
pub use self::input::Input;

pub mod output;
pub use self::output::Output;

#[doc(hidden)]
pub mod common;

pub enum Context {
    Input(Input),
    Output(Output),
}

unsafe impl Send for Context {}

impl Context {
    pub fn is_input(&self) -> bool {
        matches!(*self, Context::Input(..))
    }

    pub fn input(self) -> Input {
        if let Context::Input(context) = self {
            return context;
        }

        unreachable!();
    }

    pub fn is_output(&self) -> bool {
        matches!(*self, Context::Output(..))
    }

    pub fn output(self) -> Output {
        if let Context::Output(context) = self {
            return context;
        }

        unreachable!();
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/context/output.rs">
use std::ffi::CString;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::ptr;

use libc;

use super::common::Context;
use super::destructor;
use codec::traits;
use ffi::*;
use {format, ChapterMut, Dictionary, Error, Rational, StreamMut};

pub struct Output {
    ptr: *mut AVFormatContext,
    ctx: Context,
}

unsafe impl Send for Output {}

impl Output {
    pub unsafe fn wrap(ptr: *mut AVFormatContext) -> Self {
        Output {
            ptr,
            ctx: Context::wrap(ptr, destructor::Mode::Output),
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }
}

impl Output {
    pub fn format(&self) -> format::Output {
        // We get a clippy warning in 4.4 but not in 5.0 and newer, so we allow that cast to not complicate the code
        #[allow(clippy::unnecessary_cast)]
        unsafe {
            format::Output::wrap((*self.as_ptr()).oformat as *mut AVOutputFormat)
        }
    }

    pub fn write_header(&mut self) -> Result<(), Error> {
        unsafe {
            match avformat_write_header(self.as_mut_ptr(), ptr::null_mut()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn write_header_with(&mut self, options: Dictionary) -> Result<Dictionary, Error> {
        unsafe {
            let mut opts = options.disown();
            let res = avformat_write_header(self.as_mut_ptr(), &mut opts);

            match res {
                0 => Ok(Dictionary::own(opts)),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn write_trailer(&mut self) -> Result<(), Error> {
        unsafe {
            match av_write_trailer(self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn add_stream<E: traits::Encoder>(&mut self, codec: E) -> Result<StreamMut, Error> {
        unsafe {
            let codec = codec.encoder();
            let codec = codec.map_or(ptr::null(), |c| c.as_ptr());
            let ptr = avformat_new_stream(self.as_mut_ptr(), codec);

            if ptr.is_null() {
                return Err(Error::Unknown);
            }

            let index = (*self.ctx.as_ptr()).nb_streams - 1;

            Ok(StreamMut::wrap(&mut self.ctx, index as usize))
        }
    }

    pub fn add_chapter<R: Into<Rational>, S: AsRef<str>>(
        &mut self,
        id: i64,
        time_base: R,
        start: i64,
        end: i64,
        title: S,
    ) -> Result<ChapterMut, Error> {
        // avpriv_new_chapter is private (libavformat/internal.h)

        if start > end {
            return Err(Error::InvalidData);
        }

        let mut existing = None;
        for chapter in self.chapters() {
            if chapter.id() == id {
                existing = Some(chapter.index());
                break;
            }
        }

        let index = match existing {
            Some(index) => index,
            None => unsafe {
                let ptr = av_mallocz(size_of::<AVChapter>())
                    .as_mut()
                    .ok_or(Error::Bug)?;
                let mut nb_chapters = (*self.as_ptr()).nb_chapters as i32;

                // chapters array will be freed by `avformat_free_context`
                av_dynarray_add(
                    &mut (*self.as_mut_ptr()).chapters as *mut _ as *mut libc::c_void,
                    &mut nb_chapters,
                    ptr,
                );

                if nb_chapters > 0 {
                    (*self.as_mut_ptr()).nb_chapters = nb_chapters as u32;
                    let index = (*self.ctx.as_ptr()).nb_chapters - 1;
                    index as usize
                } else {
                    // failed to add the chapter
                    av_freep(ptr);
                    return Err(Error::Bug);
                }
            },
        };

        let mut chapter = self.chapter_mut(index).ok_or(Error::Bug)?;

        chapter.set_id(id);
        chapter.set_time_base(time_base);
        chapter.set_start(start);
        chapter.set_end(end);
        chapter.set_metadata("title", title);

        Ok(chapter)
    }

    pub fn set_metadata(&mut self, dictionary: Dictionary) {
        unsafe {
            (*self.as_mut_ptr()).metadata = dictionary.disown();
        }
    }
}

impl Deref for Output {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Output {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

pub fn dump(ctx: &Output, index: i32, url: Option<&str>) {
    let url = url.map(|u| CString::new(u).unwrap());

    unsafe {
        av_dump_format(
            ctx.as_ptr() as *mut _,
            index,
            url.unwrap_or_else(|| CString::new("").unwrap()).as_ptr(),
            1,
        );
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/format/flag.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const NO_FILE       = AVFMT_NOFILE;
        const NEED_NUMBER   = AVFMT_NEEDNUMBER;
        const SHOW_IDS      = AVFMT_SHOW_IDS;
        #[cfg(not(feature = "ffmpeg_4_0"))]
        const RAW_PICTURE   = AVFMT_RAWPICTURE;
        const GLOBAL_HEADER = AVFMT_GLOBALHEADER;
        const NO_TIMESTAMPS = AVFMT_NOTIMESTAMPS;
        const GENERIC_INDEX = AVFMT_GENERIC_INDEX;
        const TS_DISCONT    = AVFMT_TS_DISCONT;
        const VARIABLE_FPS  = AVFMT_VARIABLE_FPS;
        const NO_DIMENSIONS = AVFMT_NODIMENSIONS;
        const NO_STREAMS    = AVFMT_NOSTREAMS;
        const NO_BINSEARCH  = AVFMT_NOBINSEARCH;
        const NO_GENSEARCH  = AVFMT_NOGENSEARCH;
        const NO_BYTE_SEEK  = AVFMT_NO_BYTE_SEEK;
        const ALLOW_FLUSH   = AVFMT_ALLOW_FLUSH;
        const TS_NONSTRICT  = AVFMT_TS_NONSTRICT;
        const TS_NEGATIVE   = AVFMT_TS_NEGATIVE;
        const SEEK_TO_PTS   = AVFMT_SEEK_TO_PTS;
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/format/input.rs">
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub struct Input {
    ptr: *mut AVInputFormat,
}

impl Input {
    pub unsafe fn wrap(ptr: *mut AVInputFormat) -> Self {
        Input { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVInputFormat {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVInputFormat {
        self.ptr
    }
}

impl Input {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).long_name).to_bytes()) }
    }

    pub fn extensions(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).extensions;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }

    pub fn mime_types(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).mime_type;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/format/iter.rs">
use std::ptr;

use super::{Format, Input, Output};
use ffi::*;

pub struct Iter {
    input: *mut AVInputFormat,
    output: *mut AVOutputFormat,
    step: Step,
}

enum Step {
    Input,
    Output,
    Done,
}

impl Iter {
    pub fn new() -> Self {
        Iter {
            input: ptr::null_mut(),
            output: ptr::null_mut(),
            step: Step::Input,
        }
    }
}

impl Default for Iter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Iter {
    type Item = Format;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match self.step {
                Step::Input => {
                    let ptr = av_iformat_next(self.input);

                    if ptr.is_null() && !self.input.is_null() {
                        self.step = Step::Output;

                        self.next()
                    } else {
                        self.input = ptr;

                        Some(Format::Input(Input::wrap(ptr)))
                    }
                }

                Step::Output => {
                    let ptr = av_oformat_next(self.output);

                    if ptr.is_null() && !self.output.is_null() {
                        self.step = Step::Done;

                        self.next()
                    } else {
                        self.output = ptr;

                        Some(Format::Output(Output::wrap(ptr)))
                    }
                }

                Step::Done => None,
            }
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/format/mod.rs">
pub mod flag;
pub use self::flag::Flags;

mod input;
pub use self::input::Input;

mod output;
pub use self::output::Output;

#[cfg(not(feature = "ffmpeg_5_0"))]
mod iter;
#[cfg(not(feature = "ffmpeg_5_0"))]
pub use self::iter::Iter;

pub enum Format {
    Input(Input),
    Output(Output),
}

impl Format {
    pub fn name(&self) -> &str {
        match *self {
            Format::Input(ref f) => f.name(),
            Format::Output(ref f) => f.name(),
        }
    }

    pub fn description(&self) -> &str {
        match *self {
            Format::Input(ref f) => f.description(),
            Format::Output(ref f) => f.description(),
        }
    }

    pub fn extensions(&self) -> Vec<&str> {
        match *self {
            Format::Input(ref f) => f.extensions(),
            Format::Output(ref f) => f.extensions(),
        }
    }

    pub fn mime_types(&self) -> Vec<&str> {
        match *self {
            Format::Input(ref f) => f.mime_types(),
            Format::Output(ref f) => f.mime_types(),
        }
    }
}

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn list() -> Iter {
    Iter::new()
}
</file>

<file path="patches/ffmpeg-next/src/format/format/output.rs">
use std::path::Path;

use std::ffi::{CStr, CString};
use std::ptr;
use std::str::from_utf8_unchecked;

use super::Flags;
use ffi::*;
use {codec, media};

pub struct Output {
    ptr: *mut AVOutputFormat,
}

impl Output {
    pub unsafe fn wrap(ptr: *mut AVOutputFormat) -> Self {
        Output { ptr }
    }

    pub unsafe fn as_ptr(&self) -> *const AVOutputFormat {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVOutputFormat {
        self.ptr
    }
}

impl Output {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).long_name).to_bytes()) }
    }

    pub fn extensions(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).extensions;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }

    pub fn mime_types(&self) -> Vec<&str> {
        unsafe {
            let ptr = (*self.as_ptr()).mime_type;

            if ptr.is_null() {
                Vec::new()
            } else {
                from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes())
                    .split(',')
                    .collect()
            }
        }
    }

    pub fn codec<P: AsRef<Path>>(&self, path: &P, kind: media::Type) -> codec::Id {
        // XXX: use to_cstring when stable
        let path = CString::new(path.as_ref().as_os_str().to_str().unwrap()).unwrap();

        unsafe {
            codec::Id::from(av_guess_codec(
                self.as_ptr() as *mut _,
                ptr::null(),
                path.as_ptr(),
                ptr::null(),
                kind.into(),
            ))
        }
    }

    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags) }
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/stream/disposition.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Disposition: c_int {
        const DEFAULT          = AV_DISPOSITION_DEFAULT;
        const DUB              = AV_DISPOSITION_DUB;
        const ORIGINAL         = AV_DISPOSITION_ORIGINAL;
        const COMMENT          = AV_DISPOSITION_COMMENT;
        const LYRICS           = AV_DISPOSITION_LYRICS;
        const KARAOKE          = AV_DISPOSITION_KARAOKE;
        const FORCED           = AV_DISPOSITION_FORCED;
        const HEARING_IMPAIRED = AV_DISPOSITION_HEARING_IMPAIRED;
        const VISUAL_IMPAIRED  = AV_DISPOSITION_VISUAL_IMPAIRED;
        const CLEAN_EFFECTS    = AV_DISPOSITION_CLEAN_EFFECTS;
        const ATTACHED_PIC     = AV_DISPOSITION_ATTACHED_PIC;
        const CAPTIONS         = AV_DISPOSITION_CAPTIONS;
        const DESCRIPTIONS     = AV_DISPOSITION_DESCRIPTIONS;
        const METADATA         = AV_DISPOSITION_METADATA;
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/stream/mod.rs">
pub mod disposition;
pub use self::disposition::Disposition;

mod stream;
pub use self::stream::Stream;

mod stream_mut;
pub use self::stream_mut::StreamMut;
</file>

<file path="patches/ffmpeg-next/src/format/stream/stream_mut.rs">
use std::mem;
use std::ops::Deref;

use super::Stream;
use ffi::*;
use format::context::common::Context;
use {codec, Dictionary, Rational};

pub struct StreamMut<'a> {
    context: &'a mut Context,
    index: usize,

    immutable: Stream<'a>,
}

impl<'a> StreamMut<'a> {
    pub unsafe fn wrap(context: &mut Context, index: usize) -> StreamMut {
        StreamMut {
            context: mem::transmute_copy(&context),
            index,

            immutable: Stream::wrap(mem::transmute_copy(&context), index),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVStream {
        *(*self.context.as_mut_ptr()).streams.add(self.index)
    }
}

impl<'a> StreamMut<'a> {
    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }

    pub fn set_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).r_frame_rate = value.into().into();
        }
    }

    pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).avg_frame_rate = value.into().into();
        }
    }

    pub fn set_parameters<P: Into<codec::Parameters>>(&mut self, parameters: P) {
        let parameters = parameters.into();

        unsafe {
            avcodec_parameters_copy((*self.as_mut_ptr()).codecpar, parameters.as_ptr());
        }
    }

    pub fn set_metadata(&mut self, metadata: Dictionary) {
        unsafe {
            let metadata = metadata.disown();
            (*self.as_mut_ptr()).metadata = metadata;
        }
    }
}

impl<'a> Deref for StreamMut<'a> {
    type Target = Stream<'a>;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/stream/stream.rs">
use super::Disposition;
use codec::{self, packet};
use ffi::*;
use format::context::common::Context;
use libc::c_int;
use {DictionaryRef, Discard, Rational};

#[derive(Debug)]
pub struct Stream<'a> {
    context: &'a Context,
    index: usize,
}

impl<'a> Stream<'a> {
    pub unsafe fn wrap(context: &Context, index: usize) -> Stream {
        Stream { context, index }
    }

    pub unsafe fn as_ptr(&self) -> *const AVStream {
        *(*self.context.as_ptr()).streams.add(self.index)
    }
}

impl<'a> Stream<'a> {
    pub fn id(&self) -> i32 {
        unsafe { (*self.as_ptr()).id }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn codec(&self) -> codec::Context {
        unsafe { codec::Context::wrap((*self.as_ptr()).codec, Some(self.context.destructor())) }
    }

    pub fn parameters(&self) -> codec::Parameters {
        unsafe {
            codec::Parameters::wrap((*self.as_ptr()).codecpar, Some(self.context.destructor()))
        }
    }

    pub fn index(&self) -> usize {
        unsafe { (*self.as_ptr()).index as usize }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).time_base) }
    }

    pub fn start_time(&self) -> i64 {
        unsafe { (*self.as_ptr()).start_time }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).duration }
    }

    pub fn frames(&self) -> i64 {
        unsafe { (*self.as_ptr()).nb_frames }
    }

    pub fn disposition(&self) -> Disposition {
        unsafe { Disposition::from_bits_truncate((*self.as_ptr()).disposition) }
    }

    pub fn discard(&self) -> Discard {
        unsafe { Discard::from((*self.as_ptr()).discard) }
    }

    pub fn side_data(&self) -> SideDataIter {
        SideDataIter::new(self)
    }

    pub fn rate(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).r_frame_rate) }
    }

    pub fn avg_frame_rate(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).avg_frame_rate) }
    }

    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}

impl<'a> PartialEq for Stream<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.as_ptr() == other.as_ptr() }
    }
}

impl<'a> Eq for Stream<'a> {}

pub struct SideDataIter<'a> {
    stream: &'a Stream<'a>,
    current: c_int,
}

impl<'a> SideDataIter<'a> {
    pub fn new<'sd, 's: 'sd>(stream: &'s Stream) -> SideDataIter<'sd> {
        SideDataIter { stream, current: 0 }
    }
}

impl<'a> Iterator for SideDataIter<'a> {
    type Item = packet::SideData<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= (*self.stream.as_ptr()).nb_side_data {
                return None;
            }

            self.current += 1;

            Some(packet::SideData::wrap(
                (*self.stream.as_ptr())
                    .side_data
                    .offset((self.current - 1) as isize),
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.stream.as_ptr()).nb_side_data as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

impl<'a> ExactSizeIterator for SideDataIter<'a> {}
</file>

<file path="patches/ffmpeg-next/src/format/mod.rs">
pub use util::format::{pixel, Pixel};
pub use util::format::{sample, Sample};
use util::interrupt;

pub mod stream;

pub mod chapter;

pub mod context;
pub use self::context::Context;

pub mod format;
#[cfg(not(feature = "ffmpeg_5_0"))]
pub use self::format::list;
pub use self::format::{flag, Flags};
pub use self::format::{Input, Output};

pub mod network;

use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::str::from_utf8_unchecked;

use ffi::*;
use {Dictionary, Error, Format};

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn register_all() {
    unsafe {
        av_register_all();
    }
}

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn register(format: &Format) {
    match *format {
        Format::Input(ref format) => unsafe {
            av_register_input_format(format.as_ptr() as *mut _);
        },

        Format::Output(ref format) => unsafe {
            av_register_output_format(format.as_ptr() as *mut _);
        },
    }
}

pub fn version() -> u32 {
    unsafe { avformat_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avformat_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avformat_license()).to_bytes()) }
}

// XXX: use to_cstring when stable
fn from_path<P: AsRef<Path>>(path: &P) -> CString {
    CString::new(path.as_ref().as_os_str().to_str().unwrap()).unwrap()
}

// NOTE: this will be better with specialization or anonymous return types
pub fn open<P: AsRef<Path>>(path: &P, format: &Format) -> Result<Context, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);

        match *format {
            Format::Input(ref format) => match avformat_open_input(
                &mut ps,
                path.as_ptr(),
                format.as_ptr() as *mut _,
                ptr::null_mut(),
            ) {
                0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                    r if r >= 0 => Ok(Context::Input(context::Input::wrap(ps))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },

            Format::Output(ref format) => match avformat_alloc_output_context2(
                &mut ps,
                format.as_ptr() as *mut _,
                ptr::null(),
                path.as_ptr(),
            ) {
                0 => match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
                    0 => Ok(Context::Output(context::Output::wrap(ps))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },
        }
    }
}

pub fn open_with<P: AsRef<Path>>(
    path: &P,
    format: &Format,
    options: Dictionary,
) -> Result<Context, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let mut opts = options.disown();

        match *format {
            Format::Input(ref format) => {
                let res = avformat_open_input(
                    &mut ps,
                    path.as_ptr(),
                    format.as_ptr() as *mut _,
                    &mut opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                        r if r >= 0 => Ok(Context::Input(context::Input::wrap(ps))),
                        e => Err(Error::from(e)),
                    },

                    e => Err(Error::from(e)),
                }
            }

            Format::Output(ref format) => match avformat_alloc_output_context2(
                &mut ps,
                format.as_ptr() as *mut _,
                ptr::null(),
                path.as_ptr(),
            ) {
                0 => match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
                    0 => Ok(Context::Output(context::Output::wrap(ps))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },
        }
    }
}

pub fn input<P: AsRef<Path>>(path: &P) -> Result<context::Input, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);

        match avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn input_with_dictionary<P: AsRef<Path>>(
    path: &P,
    options: Dictionary,
) -> Result<context::Input, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let mut opts = options.disown();
        let res = avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), &mut opts);

        Dictionary::own(opts);

        match res {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn input_with_interrupt<P: AsRef<Path>, F>(
    path: &P,
    closure: F,
) -> Result<context::Input, Error>
where
    F: FnMut() -> bool,
{
    unsafe {
        let mut ps = avformat_alloc_context();
        let path = from_path(path);
        (*ps).interrupt_callback = interrupt::new(Box::new(closure)).interrupt;

        match avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output<P: AsRef<Path>>(path: &P) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);

        match avformat_alloc_output_context2(&mut ps, ptr::null_mut(), ptr::null(), path.as_ptr()) {
            0 => match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
                0 => Ok(context::Output::wrap(ps)),
                e => Err(Error::from(e)),
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_with<P: AsRef<Path>>(
    path: &P,
    options: Dictionary,
) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let mut opts = options.disown();

        match avformat_alloc_output_context2(&mut ps, ptr::null_mut(), ptr::null(), path.as_ptr()) {
            0 => {
                let res = avio_open2(
                    &mut (*ps).pb,
                    path.as_ptr(),
                    AVIO_FLAG_WRITE,
                    ptr::null(),
                    &mut opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => Ok(context::Output::wrap(ps)),
                    e => Err(Error::from(e)),
                }
            }

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_as<P: AsRef<Path>>(path: &P, format: &str) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let format = CString::new(format).unwrap();

        match avformat_alloc_output_context2(
            &mut ps,
            ptr::null_mut(),
            format.as_ptr(),
            path.as_ptr(),
        ) {
            0 => match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
                0 => Ok(context::Output::wrap(ps)),
                e => Err(Error::from(e)),
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_as_with<P: AsRef<Path>>(
    path: &P,
    format: &str,
    options: Dictionary,
) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let format = CString::new(format).unwrap();
        let mut opts = options.disown();

        match avformat_alloc_output_context2(
            &mut ps,
            ptr::null_mut(),
            format.as_ptr(),
            path.as_ptr(),
        ) {
            0 => {
                let res = avio_open2(
                    &mut (*ps).pb,
                    path.as_ptr(),
                    AVIO_FLAG_WRITE,
                    ptr::null(),
                    &mut opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => Ok(context::Output::wrap(ps)),
                    e => Err(Error::from(e)),
                }
            }

            e => Err(Error::from(e)),
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/format/network.rs">
use ffi::*;

pub fn init() {
    unsafe {
        avformat_network_init();
    }
}

pub fn deinit() {
    unsafe {
        avformat_network_deinit();
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/resampling/context.rs">
use std::ptr;

use super::Delay;
use ffi::*;
use libc::c_int;
use std::ffi::c_void;
use util::format;
use Dictionary;
use {frame, ChannelLayout, Error};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Definition {
    pub format: format::Sample,
    pub channel_layout: ChannelLayout,
    pub rate: u32,
}

pub struct Context {
    ptr: *mut SwrContext,

    input: Definition,
    output: Definition,
}

unsafe impl Send for Context {}

impl Context {
    #[doc(hidden)]
    pub unsafe fn as_ptr(&self) -> *const SwrContext {
        self.ptr as *const _
    }

    #[doc(hidden)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut SwrContext {
        self.ptr
    }
}

impl Context {
    /// Create a resampler with the given definitions.
    pub fn get(
        src_format: format::Sample,
        src_channel_layout: ChannelLayout,
        src_rate: u32,
        dst_format: format::Sample,
        dst_channel_layout: ChannelLayout,
        dst_rate: u32,
    ) -> Result<Self, Error> {
        Self::get_with(
            src_format,
            src_channel_layout,
            src_rate,
            dst_format,
            dst_channel_layout,
            dst_rate,
            Dictionary::new(),
        )
    }

    /// Create a resampler with the given definitions and custom options dictionary.
    pub fn get_with(
        src_format: format::Sample,
        src_channel_layout: ChannelLayout,
        src_rate: u32,
        dst_format: format::Sample,
        dst_channel_layout: ChannelLayout,
        dst_rate: u32,
        options: Dictionary,
    ) -> Result<Self, Error> {
        unsafe {
            let ptr = swr_alloc_set_opts(
                ptr::null_mut(),
                dst_channel_layout.bits() as i64,
                dst_format.into(),
                dst_rate as c_int,
                src_channel_layout.bits() as i64,
                src_format.into(),
                src_rate as c_int,
                0,
                ptr::null_mut(),
            );

            let mut opts = options.disown();
            let res = av_opt_set_dict(ptr as *mut c_void, &mut opts);
            Dictionary::own(opts);

            if res != 0 {
                return Err(Error::from(res));
            }

            if !ptr.is_null() {
                match swr_init(ptr) {
                    e if e < 0 => Err(Error::from(e)),

                    _ => Ok(Context {
                        ptr,

                        input: Definition {
                            format: src_format,
                            channel_layout: src_channel_layout,
                            rate: src_rate,
                        },

                        output: Definition {
                            format: dst_format,
                            channel_layout: dst_channel_layout,
                            rate: dst_rate,
                        },
                    }),
                }
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    /// Get the input definition.
    pub fn input(&self) -> &Definition {
        &self.input
    }

    /// Get the output definition.
    pub fn output(&self) -> &Definition {
        &self.output
    }

    /// Get the remaining delay.
    pub fn delay(&self) -> Option<Delay> {
        unsafe {
            match swr_get_delay(self.as_ptr() as *mut _, 1) {
                0 => None,
                _ => Some(Delay::from(self)),
            }
        }
    }

    /// Run the resampler from the given input to the given output.
    ///
    /// When there are internal frames to process it will return `Ok(Some(Delay { .. }))`.
    pub fn run(
        &mut self,
        input: &frame::Audio,
        output: &mut frame::Audio,
    ) -> Result<Option<Delay>, Error> {
        unsafe {
            (*output.as_mut_ptr()).sample_rate = self.output.rate as i32;
        }

        unsafe {
            if output.is_empty() {
                output.alloc(
                    self.output.format,
                    input.samples(),
                    self.output.channel_layout,
                );
            }

            match swr_convert_frame(self.as_mut_ptr(), output.as_mut_ptr(), input.as_ptr()) {
                0 => Ok(self.delay()),

                e => Err(Error::from(e)),
            }
        }
    }

    /// Convert one of the remaining internal frames.
    ///
    /// When there are no more internal frames `Ok(None)` will be returned.
    pub fn flush(&mut self, output: &mut frame::Audio) -> Result<Option<Delay>, Error> {
        unsafe {
            (*output.as_mut_ptr()).sample_rate = self.output.rate as i32;
        }

        unsafe {
            match swr_convert_frame(self.as_mut_ptr(), output.as_mut_ptr(), ptr::null()) {
                0 => Ok(self.delay()),

                e => Err(Error::from(e)),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            swr_free(&mut self.as_mut_ptr());
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/resampling/delay.rs">
use super::Context;
use ffi::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Delay {
    pub seconds: i64,
    pub milliseconds: i64,
    pub input: i64,
    pub output: i64,
}

impl Delay {
    pub fn from(context: &Context) -> Self {
        unsafe {
            Delay {
                seconds: swr_get_delay(context.as_ptr() as *mut _, 1),
                milliseconds: swr_get_delay(context.as_ptr() as *mut _, 1000),
                input: swr_get_delay(context.as_ptr() as *mut _, i64::from(context.input().rate)),
                output: swr_get_delay(context.as_ptr() as *mut _, i64::from(context.output().rate)),
            }
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/resampling/dither.rs">
use ffi::SwrDitherType::*;
use ffi::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Dither {
    None,
    Rectangular,
    Triangular,
    TriangularHighPass,

    NoiseShapingLipshitz,
    NoiseShapingFWeighted,
    NoiseShapingModifiedEWeighted,
    NoiseShapingImprovedEWeighted,
    NoiseShapingShibata,
    NoiseShapingLowShibata,
    NoiseShapingHighShibata,
}

impl From<SwrDitherType> for Dither {
    fn from(value: SwrDitherType) -> Dither {
        match value {
            SWR_DITHER_NONE => Dither::None,
            SWR_DITHER_RECTANGULAR => Dither::Rectangular,
            SWR_DITHER_TRIANGULAR => Dither::Triangular,
            SWR_DITHER_TRIANGULAR_HIGHPASS => Dither::TriangularHighPass,

            SWR_DITHER_NS => Dither::None,
            SWR_DITHER_NS_LIPSHITZ => Dither::NoiseShapingLipshitz,
            SWR_DITHER_NS_F_WEIGHTED => Dither::NoiseShapingFWeighted,
            SWR_DITHER_NS_MODIFIED_E_WEIGHTED => Dither::NoiseShapingModifiedEWeighted,
            SWR_DITHER_NS_IMPROVED_E_WEIGHTED => Dither::NoiseShapingImprovedEWeighted,
            SWR_DITHER_NS_SHIBATA => Dither::NoiseShapingShibata,
            SWR_DITHER_NS_LOW_SHIBATA => Dither::NoiseShapingLowShibata,
            SWR_DITHER_NS_HIGH_SHIBATA => Dither::NoiseShapingHighShibata,
            SWR_DITHER_NB => Dither::None,
        }
    }
}

impl From<Dither> for SwrDitherType {
    fn from(value: Dither) -> SwrDitherType {
        match value {
            Dither::None => SWR_DITHER_NONE,
            Dither::Rectangular => SWR_DITHER_RECTANGULAR,
            Dither::Triangular => SWR_DITHER_TRIANGULAR,
            Dither::TriangularHighPass => SWR_DITHER_TRIANGULAR_HIGHPASS,

            Dither::NoiseShapingLipshitz => SWR_DITHER_NS_LIPSHITZ,
            Dither::NoiseShapingFWeighted => SWR_DITHER_NS_F_WEIGHTED,
            Dither::NoiseShapingModifiedEWeighted => SWR_DITHER_NS_MODIFIED_E_WEIGHTED,
            Dither::NoiseShapingImprovedEWeighted => SWR_DITHER_NS_IMPROVED_E_WEIGHTED,
            Dither::NoiseShapingShibata => SWR_DITHER_NS_SHIBATA,
            Dither::NoiseShapingLowShibata => SWR_DITHER_NS_LOW_SHIBATA,
            Dither::NoiseShapingHighShibata => SWR_DITHER_NS_HIGH_SHIBATA,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/resampling/engine.rs">
use ffi::*;
use sys::SwrEngine::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Engine {
    Software,
    SoundExchange,
}

impl From<SwrEngine> for Engine {
    fn from(value: SwrEngine) -> Engine {
        match value {
            SWR_ENGINE_SWR => Engine::Software,
            SWR_ENGINE_SOXR => Engine::SoundExchange,
            SWR_ENGINE_NB => Engine::Software,
        }
    }
}

impl From<Engine> for SwrEngine {
    fn from(value: Engine) -> SwrEngine {
        match value {
            Engine::Software => SWR_ENGINE_SWR,
            Engine::SoundExchange => SWR_ENGINE_SOXR,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/resampling/extensions.rs">
use super::Context;
use util::format;
use {decoder, frame, ChannelLayout, Error};

impl frame::Audio {
    #[inline]
    pub fn resampler(
        &self,
        format: format::Sample,
        channel_layout: ChannelLayout,
        rate: u32,
    ) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.channel_layout(),
            unsafe { (*self.as_ptr()).sample_rate as u32 },
            format,
            channel_layout,
            rate,
        )
    }
}

impl decoder::Audio {
    #[inline]
    pub fn resampler(
        &self,
        format: format::Sample,
        channel_layout: ChannelLayout,
        rate: u32,
    ) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.channel_layout(),
            self.rate(),
            format,
            channel_layout,
            rate,
        )
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/resampling/filter.rs">
use ffi::SwrFilterType::*;
use ffi::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Filter {
    Cubic,
    BlackmanNuttall,
    Kaiser,
}

impl From<SwrFilterType> for Filter {
    fn from(value: SwrFilterType) -> Filter {
        match value {
            SWR_FILTER_TYPE_CUBIC => Filter::Cubic,
            SWR_FILTER_TYPE_BLACKMAN_NUTTALL => Filter::BlackmanNuttall,
            SWR_FILTER_TYPE_KAISER => Filter::Kaiser,
        }
    }
}

impl From<Filter> for SwrFilterType {
    fn from(value: Filter) -> SwrFilterType {
        match value {
            Filter::Cubic => SWR_FILTER_TYPE_CUBIC,
            Filter::BlackmanNuttall => SWR_FILTER_TYPE_BLACKMAN_NUTTALL,
            Filter::Kaiser => SWR_FILTER_TYPE_KAISER,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/resampling/flag.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const FORCE = SWR_FLAG_RESAMPLE;
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/resampling/mod.rs">
pub mod flag;
pub use self::flag::Flags;

pub mod dither;
pub use self::dither::Dither;

pub mod engine;
pub use self::engine::Engine;

pub mod filter;
pub use self::filter::Filter;

pub mod delay;
pub use self::delay::Delay;

pub mod context;
pub use self::context::Context;

mod extensions;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub fn version() -> u32 {
    unsafe { swresample_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(swresample_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(swresample_license()).to_bytes()) }
}
</file>

<file path="patches/ffmpeg-next/src/software/scaling/color_space.rs">
use ffi::*;
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ColorSpace {
    Default,

    ITU709,
    FCC,
    ITU601,
    ITU624,
    SMPTE170M,
    SMPTE240M,
}

impl From<c_int> for ColorSpace {
    fn from(value: c_int) -> ColorSpace {
        match value {
            SWS_CS_ITU709 => ColorSpace::ITU709,
            SWS_CS_FCC => ColorSpace::FCC,
            SWS_CS_DEFAULT => ColorSpace::Default,
            SWS_CS_SMPTE240M => ColorSpace::SMPTE240M,

            _ => ColorSpace::Default,
        }
    }
}

impl From<ColorSpace> for c_int {
    fn from(value: ColorSpace) -> c_int {
        match value {
            ColorSpace::Default => SWS_CS_DEFAULT,
            ColorSpace::ITU709 => SWS_CS_ITU709,
            ColorSpace::FCC => SWS_CS_FCC,
            ColorSpace::ITU601 => SWS_CS_ITU601,
            ColorSpace::ITU624 => SWS_CS_ITU624,
            ColorSpace::SMPTE170M => SWS_CS_SMPTE170M,
            ColorSpace::SMPTE240M => SWS_CS_SMPTE240M,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/scaling/context.rs">
use std::ptr;

use super::Flags;
use ffi::*;
use libc::c_int;
use util::format;
use {frame, Error};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Definition {
    pub format: format::Pixel,
    pub width: u32,
    pub height: u32,
}

pub struct Context {
    ptr: *mut SwsContext,

    input: Definition,
    output: Definition,
}

impl Context {
    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const SwsContext {
        self.ptr as *const _
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut SwsContext {
        self.ptr
    }
}

impl Context {
    pub fn get(
        src_format: format::Pixel,
        src_w: u32,
        src_h: u32,
        dst_format: format::Pixel,
        dst_w: u32,
        dst_h: u32,
        flags: Flags,
    ) -> Result<Self, Error> {
        unsafe {
            let ptr = sws_getContext(
                src_w as c_int,
                src_h as c_int,
                src_format.into(),
                dst_w as c_int,
                dst_h as c_int,
                dst_format.into(),
                flags.bits(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if !ptr.is_null() {
                Ok(Context {
                    ptr,

                    input: Definition {
                        format: src_format,
                        width: src_w,
                        height: src_h,
                    },

                    output: Definition {
                        format: dst_format,
                        width: dst_w,
                        height: dst_h,
                    },
                })
            } else {
                Err(Error::InvalidData)
            }
        }
    }

    pub fn cached(
        &mut self,
        src_format: format::Pixel,
        src_w: u32,
        src_h: u32,
        dst_format: format::Pixel,
        dst_w: u32,
        dst_h: u32,
        flags: Flags,
    ) {
        self.input = Definition {
            format: src_format,
            width: src_w,
            height: src_h,
        };

        self.output = Definition {
            format: dst_format,
            width: dst_w,
            height: dst_h,
        };

        unsafe {
            self.ptr = sws_getCachedContext(
                self.as_mut_ptr(),
                src_w as c_int,
                src_h as c_int,
                src_format.into(),
                dst_w as c_int,
                dst_h as c_int,
                dst_format.into(),
                flags.bits(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null(),
            );
        }
    }

    #[inline]
    pub fn input(&self) -> &Definition {
        &self.input
    }

    #[inline]
    pub fn output(&self) -> &Definition {
        &self.output
    }

    pub fn run(&mut self, input: &frame::Video, output: &mut frame::Video) -> Result<(), Error> {
        if input.format() != self.input.format
            || input.width() != self.input.width
            || input.height() != self.input.height
        {
            return Err(Error::InputChanged);
        }

        unsafe {
            if output.is_empty() {
                output.alloc(self.output.format, self.output.width, self.output.height);
            }
        }

        if output.format() != self.output.format
            || output.width() != self.output.width
            || output.height() != self.output.height
        {
            return Err(Error::OutputChanged);
        }

        unsafe {
            sws_scale(
                self.as_mut_ptr(),
                (*input.as_ptr()).data.as_ptr() as *const *const _,
                (*input.as_ptr()).linesize.as_ptr() as *const _,
                0,
                self.input.height as c_int,
                (*output.as_mut_ptr()).data.as_ptr(),
                (*output.as_mut_ptr()).linesize.as_ptr() as *mut _,
            );
        }

        Ok(())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            sws_freeContext(self.as_mut_ptr());
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/scaling/extensions.rs">
use super::{Context, Flags};
use util::format;
#[cfg(not(feature = "ffmpeg_5_0"))]
use Picture;
use {decoder, frame, Error};

#[cfg(not(feature = "ffmpeg_5_0"))]
impl<'a> Picture<'a> {
    #[inline]
    pub fn scaler(&self, width: u32, height: u32, flags: Flags) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            self.format(),
            width,
            height,
            flags,
        )
    }

    #[inline]
    pub fn converter(&self, format: format::Pixel) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            format,
            self.width(),
            self.height(),
            Flags::FAST_BILINEAR,
        )
    }
}

impl frame::Video {
    #[inline]
    pub fn scaler(&self, width: u32, height: u32, flags: Flags) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            self.format(),
            width,
            height,
            flags,
        )
    }

    #[inline]
    pub fn converter(&self, format: format::Pixel) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            format,
            self.width(),
            self.height(),
            Flags::FAST_BILINEAR,
        )
    }
}

impl decoder::Video {
    #[inline]
    pub fn scaler(&self, width: u32, height: u32, flags: Flags) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            self.format(),
            width,
            height,
            flags,
        )
    }

    #[inline]
    pub fn converter(&self, format: format::Pixel) -> Result<Context, Error> {
        Context::get(
            self.format(),
            self.width(),
            self.height(),
            format,
            self.width(),
            self.height(),
            Flags::FAST_BILINEAR,
        )
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/scaling/filter.rs">
use super::Vector;
use ffi::*;

pub struct Filter {
    ptr: *mut SwsFilter,
}

impl Filter {
    pub unsafe fn as_ptr(&self) -> *const SwsFilter {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut SwsFilter {
        self.ptr
    }
}

impl Filter {
    pub fn get(
        luma_g_blur: f32,
        chroma_g_blur: f32,
        luma_sharpen: f32,
        chroma_sharpen: f32,
        chroma_h_shift: f32,
        chroma_v_shift: f32,
    ) -> Self {
        unsafe {
            Filter {
                ptr: sws_getDefaultFilter(
                    luma_g_blur,
                    chroma_g_blur,
                    luma_sharpen,
                    chroma_sharpen,
                    chroma_h_shift,
                    chroma_v_shift,
                    0,
                ),
            }
        }
    }

    pub fn new() -> Self {
        Self::get(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    pub fn luma_horizontal(&self) -> Vector {
        unsafe { Vector::wrap((*self.as_ptr()).lumH) }
    }

    pub fn luma_horizontal_mut(&mut self) -> Vector {
        unsafe { Vector::wrap((*self.as_mut_ptr()).lumH) }
    }

    pub fn luma_vertical(&self) -> Vector {
        unsafe { Vector::wrap((*self.as_ptr()).lumV) }
    }

    pub fn luma_vertical_mut(&mut self) -> Vector {
        unsafe { Vector::wrap((*self.as_mut_ptr()).lumV) }
    }

    pub fn chroma_horizontal(&self) -> Vector {
        unsafe { Vector::wrap((*self.as_ptr()).lumV) }
    }

    pub fn chroma_horizontal_mut(&mut self) -> Vector {
        unsafe { Vector::wrap((*self.as_mut_ptr()).lumV) }
    }

    pub fn chroma_vertical(&self) -> Vector {
        unsafe { Vector::wrap((*self.as_ptr()).lumV) }
    }

    pub fn chroma_vertical_mut(&mut self) -> Vector {
        unsafe { Vector::wrap((*self.as_mut_ptr()).lumV) }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe {
            sws_freeFilter(self.as_mut_ptr());
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/scaling/flag.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const FAST_BILINEAR        = SWS_FAST_BILINEAR;
        const BILINEAR             = SWS_BILINEAR;
        const BICUBIC              = SWS_BICUBIC;
        const X                    = SWS_X;
        const POINT                = SWS_POINT;
        const AREA                 = SWS_AREA;
        const BICUBLIN             = SWS_BICUBLIN;
        const GAUSS                = SWS_GAUSS;
        const SINC                 = SWS_SINC;
        const LANCZOS              = SWS_LANCZOS;
        const SPLINE               = SWS_SPLINE;
        const SRC_V_CHR_DROP_MASK  = SWS_SRC_V_CHR_DROP_MASK;
        const SRC_V_CHR_DROP_SHIFT = SWS_SRC_V_CHR_DROP_SHIFT;
        const PARAM_DEFAULT        = SWS_PARAM_DEFAULT;
        const PRINT_INFO           = SWS_PRINT_INFO;
        const FULL_CHR_H_INT       = SWS_FULL_CHR_H_INT;
        const FULL_CHR_H_INP       = SWS_FULL_CHR_H_INP;
        const DIRECT_BGR           = SWS_DIRECT_BGR;
        const ACCURATE_RND         = SWS_ACCURATE_RND;
        const BITEXACT             = SWS_BITEXACT;
        const ERROR_DIFFUSION      = SWS_ERROR_DIFFUSION;
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/scaling/mod.rs">
pub mod flag;
pub use self::flag::Flags;

pub mod color_space;
pub use self::color_space::ColorSpace;

pub mod support;

pub mod vector;
pub use self::vector::Vector;

pub mod filter;
pub use self::filter::Filter;

pub mod context;
pub use self::context::Context;

mod extensions;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub fn version() -> u32 {
    unsafe { swscale_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(swscale_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(swscale_license()).to_bytes()) }
}
</file>

<file path="patches/ffmpeg-next/src/software/scaling/support.rs">
use ffi::*;
use util::format;

pub fn input(format: format::Pixel) -> bool {
    unsafe { sws_isSupportedInput(format.into()) != 0 }
}

pub fn output(format: format::Pixel) -> bool {
    unsafe { sws_isSupportedOutput(format.into()) != 0 }
}

pub fn endianness_conversion(format: format::Pixel) -> bool {
    unsafe { sws_isSupportedEndiannessConversion(format.into()) != 0 }
}
</file>

<file path="patches/ffmpeg-next/src/software/scaling/vector.rs">
use std::marker::PhantomData;
use std::slice;

use ffi::*;
use libc::{c_double, c_int};

pub struct Vector<'a> {
    ptr: *mut SwsVector,

    _own: bool,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Vector<'a> {
    pub unsafe fn wrap(ptr: *mut SwsVector) -> Self {
        Vector {
            ptr,
            _own: false,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const SwsVector {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut SwsVector {
        self.ptr
    }
}

impl<'a> Vector<'a> {
    pub fn new(length: usize) -> Self {
        unsafe {
            Vector {
                ptr: sws_allocVec(length as c_int),
                _own: true,
                _marker: PhantomData,
            }
        }
    }

    pub fn gaussian(variance: f64, quality: f64) -> Self {
        unsafe {
            Vector {
                ptr: sws_getGaussianVec(variance as c_double, quality as c_double),
                _own: true,
                _marker: PhantomData,
            }
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn value(value: f64, length: usize) -> Self {
        unsafe {
            Vector {
                ptr: sws_getConstVec(value as c_double, length as c_int),
                _own: true,
                _marker: PhantomData,
            }
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn identity() -> Self {
        unsafe {
            Vector {
                ptr: sws_getIdentityVec(),
                _own: true,
                _marker: PhantomData,
            }
        }
    }

    pub fn scale(&mut self, scalar: f64) {
        unsafe {
            sws_scaleVec(self.as_mut_ptr(), scalar as c_double);
        }
    }

    pub fn normalize(&mut self, height: f64) {
        unsafe {
            sws_normalizeVec(self.as_mut_ptr(), height as c_double);
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn conv(&mut self, other: &Vector) {
        unsafe {
            sws_convVec(self.as_mut_ptr(), other.as_ptr() as *mut _);
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn add(&mut self, other: &Vector) {
        unsafe {
            sws_addVec(self.as_mut_ptr(), other.as_ptr() as *mut _);
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn sub(&mut self, other: &Vector) {
        unsafe {
            sws_subVec(self.as_mut_ptr(), other.as_ptr() as *mut _);
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn shift(&mut self, value: usize) {
        unsafe {
            sws_shiftVec(self.as_mut_ptr(), value as c_int);
        }
    }

    pub fn coefficients(&self) -> &[f64] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).coeff, (*self.as_ptr()).length as usize) }
    }

    pub fn coefficients_mut(&self) -> &[f64] {
        unsafe {
            slice::from_raw_parts_mut((*self.as_ptr()).coeff, (*self.as_ptr()).length as usize)
        }
    }
}

#[cfg(not(feature = "ffmpeg_5_0"))]
impl<'a> Clone for Vector<'a> {
    fn clone(&self) -> Self {
        unsafe {
            Vector {
                ptr: sws_cloneVec(self.as_ptr() as *mut _),
                _own: true,
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> Drop for Vector<'a> {
    fn drop(&mut self) {
        unsafe {
            if self._own {
                sws_freeVec(self.as_mut_ptr());
            }
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/software/mod.rs">
#[cfg(feature = "software-scaling")]
pub mod scaling;

#[cfg(feature = "software-scaling")]
#[inline]
pub fn scaler(
    format: ::format::Pixel,
    flags: scaling::Flags,
    (in_width, in_height): (u32, u32),
    (out_width, out_height): (u32, u32),
) -> Result<scaling::Context, ::Error> {
    scaling::Context::get(
        format, in_width, in_height, format, out_width, out_height, flags,
    )
}

#[cfg(feature = "software-scaling")]
#[inline]
pub fn converter(
    (width, height): (u32, u32),
    input: ::format::Pixel,
    output: ::format::Pixel,
) -> Result<scaling::Context, ::Error> {
    scaling::Context::get(
        input,
        width,
        height,
        output,
        width,
        height,
        scaling::flag::Flags::FAST_BILINEAR,
    )
}

#[cfg(feature = "software-resampling")]
pub mod resampling;

#[cfg(feature = "software-resampling")]
#[inline]
pub fn resampler(
    (in_format, in_layout, in_rate): (::format::Sample, ::ChannelLayout, u32),
    (out_format, out_layout, out_rate): (::format::Sample, ::ChannelLayout, u32),
) -> Result<resampling::Context, ::Error> {
    resampling::Context::get(
        in_format, in_layout, in_rate, out_format, out_layout, out_rate,
    )
}
</file>

<file path="patches/ffmpeg-next/src/util/chroma/location.rs">
use ffi::AVChromaLocation::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Location {
    Unspecified,
    Left,
    Center,
    TopLeft,
    Top,
    BottomLeft,
    Bottom,
}

impl From<AVChromaLocation> for Location {
    fn from(value: AVChromaLocation) -> Self {
        match value {
            AVCHROMA_LOC_UNSPECIFIED => Location::Unspecified,
            AVCHROMA_LOC_LEFT => Location::Left,
            AVCHROMA_LOC_CENTER => Location::Center,
            AVCHROMA_LOC_TOPLEFT => Location::TopLeft,
            AVCHROMA_LOC_TOP => Location::Top,
            AVCHROMA_LOC_BOTTOMLEFT => Location::BottomLeft,
            AVCHROMA_LOC_BOTTOM => Location::Bottom,
            AVCHROMA_LOC_NB => Location::Unspecified,
        }
    }
}

impl From<Location> for AVChromaLocation {
    fn from(value: Location) -> AVChromaLocation {
        match value {
            Location::Unspecified => AVCHROMA_LOC_UNSPECIFIED,
            Location::Left => AVCHROMA_LOC_LEFT,
            Location::Center => AVCHROMA_LOC_CENTER,
            Location::TopLeft => AVCHROMA_LOC_TOPLEFT,
            Location::Top => AVCHROMA_LOC_TOP,
            Location::BottomLeft => AVCHROMA_LOC_BOTTOMLEFT,
            Location::Bottom => AVCHROMA_LOC_BOTTOM,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/chroma/mod.rs">
pub mod location;
pub use self::location::Location;
</file>

<file path="patches/ffmpeg-next/src/util/color/mod.rs">
pub mod range;
pub use self::range::Range;

pub mod space;
pub use self::space::Space;

pub mod primaries;
pub use self::primaries::Primaries;

pub mod transfer_characteristic;
pub use self::transfer_characteristic::TransferCharacteristic;
</file>

<file path="patches/ffmpeg-next/src/util/color/primaries.rs">
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVColorPrimaries::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Primaries {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    BT470M,

    BT470BG,
    SMPTE170M,
    SMPTE240M,
    Film,
    BT2020,

    SMPTE428,
    SMPTE431,
    SMPTE432,
    #[cfg(not(feature = "ffmpeg_4_3"))]
    JEDEC_P22,
    #[cfg(feature = "ffmpeg_4_3")]
    EBU3213,
}

impl Primaries {
    #[cfg(feature = "ffmpeg_4_3")]
    pub const JEDEC_P22: Primaries = Primaries::EBU3213;

    pub fn name(&self) -> Option<&'static str> {
        if *self == Primaries::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_primaries_name((*self).into());
            ptr.as_ref()
                .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorPrimaries> for Primaries {
    fn from(value: AVColorPrimaries) -> Primaries {
        match value {
            AVCOL_PRI_RESERVED0 => Primaries::Reserved0,
            AVCOL_PRI_BT709 => Primaries::BT709,
            AVCOL_PRI_UNSPECIFIED => Primaries::Unspecified,
            AVCOL_PRI_RESERVED => Primaries::Reserved,
            AVCOL_PRI_BT470M => Primaries::BT470M,

            AVCOL_PRI_BT470BG => Primaries::BT470BG,
            AVCOL_PRI_SMPTE170M => Primaries::SMPTE170M,
            AVCOL_PRI_SMPTE240M => Primaries::SMPTE240M,
            AVCOL_PRI_FILM => Primaries::Film,
            AVCOL_PRI_BT2020 => Primaries::BT2020,
            AVCOL_PRI_NB => Primaries::Reserved0,

            AVCOL_PRI_SMPTE428 => Primaries::SMPTE428,
            AVCOL_PRI_SMPTE431 => Primaries::SMPTE431,
            AVCOL_PRI_SMPTE432 => Primaries::SMPTE432,
            #[cfg(not(feature = "ffmpeg_4_3"))]
            AVCOL_PRI_JEDEC_P22 => Primaries::JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            AVCOL_PRI_EBU3213 => Primaries::EBU3213,
        }
    }
}

impl From<Primaries> for AVColorPrimaries {
    fn from(value: Primaries) -> AVColorPrimaries {
        match value {
            Primaries::Reserved0 => AVCOL_PRI_RESERVED0,
            Primaries::BT709 => AVCOL_PRI_BT709,
            Primaries::Unspecified => AVCOL_PRI_UNSPECIFIED,
            Primaries::Reserved => AVCOL_PRI_RESERVED,
            Primaries::BT470M => AVCOL_PRI_BT470M,

            Primaries::BT470BG => AVCOL_PRI_BT470BG,
            Primaries::SMPTE170M => AVCOL_PRI_SMPTE170M,
            Primaries::SMPTE240M => AVCOL_PRI_SMPTE240M,
            Primaries::Film => AVCOL_PRI_FILM,
            Primaries::BT2020 => AVCOL_PRI_BT2020,

            Primaries::SMPTE428 => AVCOL_PRI_SMPTE428,
            Primaries::SMPTE431 => AVCOL_PRI_SMPTE431,
            Primaries::SMPTE432 => AVCOL_PRI_SMPTE432,
            #[cfg(not(feature = "ffmpeg_4_3"))]
            Primaries::JEDEC_P22 => AVCOL_PRI_JEDEC_P22,
            #[cfg(feature = "ffmpeg_4_3")]
            Primaries::EBU3213 => AVCOL_PRI_EBU3213,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/color/range.rs">
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVColorRange::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Range {
    Unspecified,
    MPEG,
    JPEG,
}

impl Range {
    pub fn name(&self) -> Option<&'static str> {
        if *self == Range::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_range_name((*self).into());
            ptr.as_ref()
                .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorRange> for Range {
    fn from(value: AVColorRange) -> Self {
        match value {
            AVCOL_RANGE_UNSPECIFIED => Range::Unspecified,
            AVCOL_RANGE_MPEG => Range::MPEG,
            AVCOL_RANGE_JPEG => Range::JPEG,
            AVCOL_RANGE_NB => Range::Unspecified,
        }
    }
}

impl From<Range> for AVColorRange {
    fn from(value: Range) -> AVColorRange {
        match value {
            Range::Unspecified => AVCOL_RANGE_UNSPECIFIED,
            Range::MPEG => AVCOL_RANGE_MPEG,
            Range::JPEG => AVCOL_RANGE_JPEG,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/color/space.rs">
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVColorSpace::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Space {
    RGB,
    BT709,
    Unspecified,
    Reserved,
    FCC,
    BT470BG,
    SMPTE170M,
    SMPTE240M,
    YCGCO,
    BT2020NCL,
    BT2020CL,
    SMPTE2085,

    ChromaDerivedNCL,
    ChromaDerivedCL,
    ICTCP,
}

impl Space {
    pub const YCOCG: Space = Space::YCGCO;

    pub fn name(&self) -> Option<&'static str> {
        if *self == Space::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_space_name((*self).into());
            ptr.as_ref()
                .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorSpace> for Space {
    fn from(value: AVColorSpace) -> Self {
        match value {
            AVCOL_SPC_RGB => Space::RGB,
            AVCOL_SPC_BT709 => Space::BT709,
            AVCOL_SPC_UNSPECIFIED => Space::Unspecified,
            AVCOL_SPC_RESERVED => Space::Reserved,
            AVCOL_SPC_FCC => Space::FCC,
            AVCOL_SPC_BT470BG => Space::BT470BG,
            AVCOL_SPC_SMPTE170M => Space::SMPTE170M,
            AVCOL_SPC_SMPTE240M => Space::SMPTE240M,
            AVCOL_SPC_YCGCO => Space::YCGCO,
            AVCOL_SPC_BT2020_NCL => Space::BT2020NCL,
            AVCOL_SPC_BT2020_CL => Space::BT2020CL,
            AVCOL_SPC_SMPTE2085 => Space::SMPTE2085,
            AVCOL_SPC_NB => Space::Unspecified,

            AVCOL_SPC_CHROMA_DERIVED_NCL => Space::ChromaDerivedNCL,
            AVCOL_SPC_CHROMA_DERIVED_CL => Space::ChromaDerivedCL,
            AVCOL_SPC_ICTCP => Space::ICTCP,
        }
    }
}

impl From<Space> for AVColorSpace {
    fn from(value: Space) -> AVColorSpace {
        match value {
            Space::RGB => AVCOL_SPC_RGB,
            Space::BT709 => AVCOL_SPC_BT709,
            Space::Unspecified => AVCOL_SPC_UNSPECIFIED,
            Space::Reserved => AVCOL_SPC_RESERVED,
            Space::FCC => AVCOL_SPC_FCC,
            Space::BT470BG => AVCOL_SPC_BT470BG,
            Space::SMPTE170M => AVCOL_SPC_SMPTE170M,
            Space::SMPTE240M => AVCOL_SPC_SMPTE240M,
            Space::YCGCO => AVCOL_SPC_YCGCO,
            Space::BT2020NCL => AVCOL_SPC_BT2020_NCL,
            Space::BT2020CL => AVCOL_SPC_BT2020_CL,
            Space::SMPTE2085 => AVCOL_SPC_SMPTE2085,

            Space::ChromaDerivedNCL => AVCOL_SPC_CHROMA_DERIVED_NCL,
            Space::ChromaDerivedCL => AVCOL_SPC_CHROMA_DERIVED_CL,
            Space::ICTCP => AVCOL_SPC_ICTCP,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/color/transfer_characteristic.rs">
use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::AVColorTransferCharacteristic::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TransferCharacteristic {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    GAMMA22,
    GAMMA28,
    SMPTE170M,
    SMPTE240M,
    Linear,
    Log,
    LogSqrt,
    IEC61966_2_4,
    BT1361_ECG,
    IEC61966_2_1,
    BT2020_10,
    BT2020_12,
    SMPTE2084,
    SMPTE428,
    ARIB_STD_B67,
}

impl TransferCharacteristic {
    pub fn name(&self) -> Option<&'static str> {
        if *self == TransferCharacteristic::Unspecified {
            return None;
        }
        unsafe {
            let ptr = av_color_transfer_name((*self).into());
            ptr.as_ref()
                .map(|ptr| from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()))
        }
    }
}

impl From<AVColorTransferCharacteristic> for TransferCharacteristic {
    fn from(value: AVColorTransferCharacteristic) -> TransferCharacteristic {
        match value {
            AVCOL_TRC_RESERVED0 => TransferCharacteristic::Reserved0,
            AVCOL_TRC_BT709 => TransferCharacteristic::BT709,
            AVCOL_TRC_UNSPECIFIED => TransferCharacteristic::Unspecified,
            AVCOL_TRC_RESERVED => TransferCharacteristic::Reserved,
            AVCOL_TRC_GAMMA22 => TransferCharacteristic::GAMMA22,
            AVCOL_TRC_GAMMA28 => TransferCharacteristic::GAMMA28,
            AVCOL_TRC_SMPTE170M => TransferCharacteristic::SMPTE170M,
            AVCOL_TRC_SMPTE240M => TransferCharacteristic::SMPTE240M,
            AVCOL_TRC_LINEAR => TransferCharacteristic::Linear,
            AVCOL_TRC_LOG => TransferCharacteristic::Log,
            AVCOL_TRC_LOG_SQRT => TransferCharacteristic::LogSqrt,
            AVCOL_TRC_IEC61966_2_4 => TransferCharacteristic::IEC61966_2_4,
            AVCOL_TRC_BT1361_ECG => TransferCharacteristic::BT1361_ECG,
            AVCOL_TRC_IEC61966_2_1 => TransferCharacteristic::IEC61966_2_1,
            AVCOL_TRC_BT2020_10 => TransferCharacteristic::BT2020_10,
            AVCOL_TRC_BT2020_12 => TransferCharacteristic::BT2020_12,
            AVCOL_TRC_NB => TransferCharacteristic::Reserved0,
            AVCOL_TRC_SMPTE2084 => TransferCharacteristic::SMPTE2084,
            AVCOL_TRC_SMPTE428 => TransferCharacteristic::SMPTE428,
            AVCOL_TRC_ARIB_STD_B67 => TransferCharacteristic::ARIB_STD_B67,
        }
    }
}

impl From<TransferCharacteristic> for AVColorTransferCharacteristic {
    fn from(value: TransferCharacteristic) -> AVColorTransferCharacteristic {
        match value {
            TransferCharacteristic::Reserved0 => AVCOL_TRC_RESERVED0,
            TransferCharacteristic::BT709 => AVCOL_TRC_BT709,
            TransferCharacteristic::Unspecified => AVCOL_TRC_UNSPECIFIED,
            TransferCharacteristic::Reserved => AVCOL_TRC_RESERVED,
            TransferCharacteristic::GAMMA22 => AVCOL_TRC_GAMMA22,
            TransferCharacteristic::GAMMA28 => AVCOL_TRC_GAMMA28,
            TransferCharacteristic::SMPTE170M => AVCOL_TRC_SMPTE170M,
            TransferCharacteristic::SMPTE240M => AVCOL_TRC_SMPTE240M,
            TransferCharacteristic::Linear => AVCOL_TRC_LINEAR,
            TransferCharacteristic::Log => AVCOL_TRC_LOG,
            TransferCharacteristic::LogSqrt => AVCOL_TRC_LOG_SQRT,
            TransferCharacteristic::IEC61966_2_4 => AVCOL_TRC_IEC61966_2_4,
            TransferCharacteristic::BT1361_ECG => AVCOL_TRC_BT1361_ECG,
            TransferCharacteristic::IEC61966_2_1 => AVCOL_TRC_IEC61966_2_1,
            TransferCharacteristic::BT2020_10 => AVCOL_TRC_BT2020_10,
            TransferCharacteristic::BT2020_12 => AVCOL_TRC_BT2020_12,
            TransferCharacteristic::SMPTE2084 => AVCOL_TRC_SMPTE2084,
            TransferCharacteristic::SMPTE428 => AVCOL_TRC_SMPTE428,
            TransferCharacteristic::ARIB_STD_B67 => AVCOL_TRC_ARIB_STD_B67,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/dictionary/immutable.rs">
use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::ptr;
use std::str::from_utf8_unchecked;

use super::{Iter, Owned};
use ffi::*;

pub struct Ref<'a> {
    ptr: *const AVDictionary,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Ref<'a> {
    pub unsafe fn wrap(ptr: *const AVDictionary) -> Self {
        Ref {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVDictionary {
        self.ptr
    }
}

impl<'a> Ref<'a> {
    pub fn get(&'a self, key: &str) -> Option<&'a str> {
        unsafe {
            let key = CString::new(key).unwrap();
            let entry = av_dict_get(self.as_ptr(), key.as_ptr(), ptr::null_mut(), 0);

            if entry.is_null() {
                None
            } else {
                Some(from_utf8_unchecked(
                    CStr::from_ptr((*entry).value).to_bytes(),
                ))
            }
        }
    }

    pub fn iter(&self) -> Iter {
        unsafe { Iter::new(self.as_ptr()) }
    }

    pub fn to_owned<'b>(&self) -> Owned<'b> {
        self.iter().collect()
    }
}

impl<'a> IntoIterator for &'a Ref<'a> {
    type Item = (&'a str, &'a str);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> fmt::Debug for Ref<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_map().entries(self.iter()).finish()
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/dictionary/iter.rs">
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr;
use std::str::from_utf8_unchecked;

use ffi::*;

pub struct Iter<'a> {
    ptr: *const AVDictionary,
    cur: *mut AVDictionaryEntry,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Iter<'a> {
    pub fn new(dictionary: *const AVDictionary) -> Self {
        Iter {
            ptr: dictionary,
            cur: ptr::null_mut(),

            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            let empty = CString::new("").unwrap();
            let entry = av_dict_get(self.ptr, empty.as_ptr(), self.cur, AV_DICT_IGNORE_SUFFIX);

            if !entry.is_null() {
                let key = from_utf8_unchecked(CStr::from_ptr((*entry).key).to_bytes());
                let val = from_utf8_unchecked(CStr::from_ptr((*entry).value).to_bytes());

                self.cur = entry;

                Some((key, val))
            } else {
                None
            }
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/dictionary/mod.rs">
mod immutable;
pub use self::immutable::Ref;

mod mutable;
pub use self::mutable::Ref as Mut;

mod owned;
pub use self::owned::Owned;

mod iter;
pub use self::iter::Iter;

#[macro_export]
macro_rules! dict {
	( $($key:expr => $value:expr),* $(,)*) => ({
			let mut dict = ::ffmpeg::Dictionary::new();

			$(
				dict.set($key, $value);
			)*

			dict
		}
	);
}
</file>

<file path="patches/ffmpeg-next/src/util/dictionary/mutable.rs">
use std::ffi::CString;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

use super::immutable;
use ffi::*;

pub struct Ref<'a> {
    ptr: *mut AVDictionary,
    imm: immutable::Ref<'a>,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Ref<'a> {
    pub unsafe fn wrap(ptr: *mut AVDictionary) -> Self {
        Ref {
            ptr,
            imm: immutable::Ref::wrap(ptr),
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut AVDictionary {
        self.ptr
    }
}

impl<'a> Ref<'a> {
    pub fn set(&mut self, key: &str, value: &str) {
        unsafe {
            let key = CString::new(key).unwrap();
            let value = CString::new(value).unwrap();
            let mut ptr = self.as_mut_ptr();

            if av_dict_set(&mut ptr, key.as_ptr(), value.as_ptr(), 0) < 0 {
                panic!("out of memory");
            }

            self.ptr = ptr;
            self.imm = immutable::Ref::wrap(ptr);
        }
    }
}

impl<'a> Deref for Ref<'a> {
    type Target = immutable::Ref<'a>;

    fn deref(&self) -> &Self::Target {
        &self.imm
    }
}

impl<'a> fmt::Debug for Ref<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.imm.fmt(fmt)
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/dictionary/owned.rs">
use std::fmt;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::ptr;

use super::mutable;
use ffi::*;

pub struct Owned<'a> {
    inner: mutable::Ref<'a>,
}

impl<'a> Default for Owned<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Owned<'a> {
    pub unsafe fn own(ptr: *mut AVDictionary) -> Self {
        Owned {
            inner: mutable::Ref::wrap(ptr),
        }
    }

    pub unsafe fn disown(mut self) -> *mut AVDictionary {
        let result = self.inner.as_mut_ptr();
        self.inner = mutable::Ref::wrap(ptr::null_mut());

        result
    }
}

impl<'a> Owned<'a> {
    pub fn new() -> Self {
        unsafe {
            Owned {
                inner: mutable::Ref::wrap(ptr::null_mut()),
            }
        }
    }
}

impl<'a, 'b> FromIterator<(&'b str, &'b str)> for Owned<'a> {
    fn from_iter<T: IntoIterator<Item = (&'b str, &'b str)>>(iterator: T) -> Self {
        let mut result = Owned::new();

        for (key, value) in iterator {
            result.set(key, value);
        }

        result
    }
}

impl<'a, 'b> FromIterator<&'b (&'b str, &'b str)> for Owned<'a> {
    fn from_iter<T: IntoIterator<Item = &'b (&'b str, &'b str)>>(iterator: T) -> Self {
        let mut result = Owned::new();

        for &(key, value) in iterator {
            result.set(key, value);
        }

        result
    }
}

impl<'a> FromIterator<(String, String)> for Owned<'a> {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iterator: T) -> Self {
        let mut result = Owned::new();

        for (key, value) in iterator {
            result.set(&key, &value);
        }

        result
    }
}

impl<'a, 'b> FromIterator<&'b (String, String)> for Owned<'a> {
    fn from_iter<T: IntoIterator<Item = &'b (String, String)>>(iterator: T) -> Self {
        let mut result = Owned::new();

        for (key, value) in iterator {
            result.set(key, value);
        }

        result
    }
}

impl<'a> Deref for Owned<'a> {
    type Target = mutable::Ref<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for Owned<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> Clone for Owned<'a> {
    fn clone(&self) -> Self {
        let mut dictionary = Owned::new();
        dictionary.clone_from(self);

        dictionary
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            let mut ptr = self.as_mut_ptr();
            av_dict_copy(&mut ptr, source.as_ptr(), 0);
            self.inner = mutable::Ref::wrap(ptr);
        }
    }
}

impl<'a> Drop for Owned<'a> {
    fn drop(&mut self) {
        unsafe {
            av_dict_free(&mut self.inner.as_mut_ptr());
        }
    }
}

impl<'a> fmt::Debug for Owned<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(fmt)
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/format/mod.rs">
pub mod sample;
pub use self::sample::Sample;

pub mod pixel;
pub use self::pixel::Pixel;
</file>

<file path="patches/ffmpeg-next/src/util/format/pixel.rs">
use std::error;
use std::ffi::{CStr, CString, NulError};
use std::fmt;
use std::str::{from_utf8_unchecked, FromStr};

use ffi::AVPixelFormat::*;
use ffi::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Pixel {
    None,

    YUV420P,
    YUYV422,
    RGB24,
    BGR24,
    YUV422P,
    YUV444P,
    YUV410P,
    YUV411P,
    GRAY8,
    MonoWhite,
    MonoBlack,
    PAL8,
    YUVJ420P,
    YUVJ422P,
    YUVJ444P,
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    XVMC_MPEG2_MC,
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    XVMC_MPEG2_IDCT,
    UYVY422,
    UYYVYY411,
    BGR8,
    BGR4,
    BGR4_BYTE,
    RGB8,
    RGB4,
    RGB4_BYTE,
    NV12,
    NV21,

    ARGB,
    RGBA,
    ABGR,
    BGRA,

    GRAY16BE,
    GRAY16LE,
    YUV440P,
    YUVJ440P,
    YUVA420P,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_H264,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG1,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG2,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_WMV3,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_VC1,
    RGB48BE,
    RGB48LE,

    RGB565BE,
    RGB565LE,
    RGB555BE,
    RGB555LE,

    BGR565BE,
    BGR565LE,
    BGR555BE,
    BGR555LE,

    #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
    VAAPI_MOCO,
    #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
    VAAPI_IDCT,
    #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
    VAAPI_VLD,
    #[cfg(any(not(feature = "ff_api_vaapi"), feature = "ffmpeg_5_0"))]
    VAAPI,

    YUV420P16LE,
    YUV420P16BE,
    YUV422P16LE,
    YUV422P16BE,
    YUV444P16LE,
    YUV444P16BE,
    #[cfg(feature = "ff_api_vdpau")]
    VDPAU_MPEG4,
    DXVA2_VLD,

    RGB444LE,
    RGB444BE,
    BGR444LE,
    BGR444BE,
    YA8,

    BGR48BE,
    BGR48LE,

    YUV420P9BE,
    YUV420P9LE,
    YUV420P10BE,
    YUV420P10LE,
    YUV422P10BE,
    YUV422P10LE,
    YUV444P9BE,
    YUV444P9LE,
    YUV444P10BE,
    YUV444P10LE,
    YUV422P9BE,
    YUV422P9LE,
    #[cfg(not(feature = "ffmpeg_4_0"))]
    VDA_VLD,

    GBRP,
    GBRP9BE,
    GBRP9LE,
    GBRP10BE,
    GBRP10LE,
    GBRP16BE,
    GBRP16LE,

    YUVA420P9BE,
    YUVA420P9LE,
    YUVA422P9BE,
    YUVA422P9LE,
    YUVA444P9BE,
    YUVA444P9LE,
    YUVA420P10BE,
    YUVA420P10LE,
    YUVA422P10BE,
    YUVA422P10LE,
    YUVA444P10BE,
    YUVA444P10LE,
    YUVA420P16BE,
    YUVA420P16LE,
    YUVA422P16BE,
    YUVA422P16LE,
    YUVA444P16BE,
    YUVA444P16LE,

    VDPAU,

    XYZ12LE,
    XYZ12BE,
    NV16,
    NV20LE,
    NV20BE,

    RGBA64BE,
    RGBA64LE,
    BGRA64BE,
    BGRA64LE,

    YVYU422,

    #[cfg(not(feature = "ffmpeg_4_0"))]
    VDA,

    YA16BE,
    YA16LE,

    QSV,
    MMAL,

    D3D11VA_VLD,

    CUDA,

    ZRGB,
    RGBZ,
    ZBGR,
    BGRZ,
    YUVA444P,
    YUVA422P,

    YUV420P12BE,
    YUV420P12LE,
    YUV420P14BE,
    YUV420P14LE,
    YUV422P12BE,
    YUV422P12LE,
    YUV422P14BE,
    YUV422P14LE,
    YUV444P12BE,
    YUV444P12LE,
    YUV444P14BE,
    YUV444P14LE,
    GBRP12BE,
    GBRP12LE,
    GBRP14BE,
    GBRP14LE,
    GBRAP,
    GBRAP16BE,
    GBRAP16LE,
    YUVJ411P,

    BAYER_BGGR8,
    BAYER_RGGB8,
    BAYER_GBRG8,
    BAYER_GRBG8,
    BAYER_BGGR16LE,
    BAYER_BGGR16BE,
    BAYER_RGGB16LE,
    BAYER_RGGB16BE,
    BAYER_GBRG16LE,
    BAYER_GBRG16BE,
    BAYER_GRBG16LE,
    BAYER_GRBG16BE,

    YUV440P10LE,
    YUV440P10BE,
    YUV440P12LE,
    YUV440P12BE,
    AYUV64LE,
    AYUV64BE,

    VIDEOTOOLBOX,

    // --- defaults
    #[cfg(feature = "ffmpeg_4_0")]
    XVMC,

    RGB32,
    RGB32_1,
    BGR32,
    BGR32_1,
    ZRGB32,
    ZBGR32,

    GRAY16,
    YA16,
    RGB48,
    RGB565,
    RGB555,
    RGB444,
    BGR48,
    BGR565,
    BGR555,
    BGR444,

    YUV420P9,
    YUV422P9,
    YUV444P9,
    YUV420P10,
    YUV422P10,
    YUV440P10,
    YUV444P10,
    YUV420P12,
    YUV422P12,
    YUV440P12,
    YUV444P12,
    YUV420P14,
    YUV422P14,
    YUV444P14,
    YUV420P16,
    YUV422P16,
    YUV444P16,

    GBRP9,
    GBRP10,
    GBRP12,
    GBRP14,
    GBRP16,
    GBRAP16,

    BAYER_BGGR16,
    BAYER_RGGB16,
    BAYER_GBRG16,
    BAYER_GRBG16,

    YUVA420P9,
    YUVA422P9,
    YUVA444P9,
    YUVA420P10,
    YUVA422P10,
    YUVA444P10,
    YUVA420P16,
    YUVA422P16,
    YUVA444P16,

    XYZ12,
    NV20,
    AYUV64,

    P010LE,
    P010BE,
    GBRAP12BE,
    GBRAP12LE,
    GBRAP10LE,
    GBRAP10BE,
    MEDIACODEC,
    GRAY12BE,
    GRAY12LE,
    GRAY10BE,
    GRAY10LE,
    P016LE,
    P016BE,

    D3D11,
    GRAY9BE,
    GRAY9LE,
    GBRPF32BE,
    GBRPF32LE,
    GBRAPF32BE,
    GBRAPF32LE,
    DRM_PRIME,

    #[cfg(feature = "ffmpeg_4_0")]
    OPENCL,

    #[cfg(feature = "ffmpeg_4_1")]
    GRAY14BE,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAY14LE,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAYF32BE,
    #[cfg(feature = "ffmpeg_4_1")]
    GRAYF32LE,

    #[cfg(feature = "ffmpeg_4_2")]
    YUVA422P12BE,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA422P12LE,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA444P12BE,
    #[cfg(feature = "ffmpeg_4_2")]
    YUVA444P12LE,
    #[cfg(feature = "ffmpeg_4_2")]
    NV24,
    #[cfg(feature = "ffmpeg_4_2")]
    NV42,

    #[cfg(feature = "ffmpeg_4_3")]
    VULKAN,
    #[cfg(feature = "ffmpeg_4_3")]
    Y210BE,
    #[cfg(feature = "ffmpeg_4_3")]
    Y210LE,

    #[cfg(feature = "ffmpeg_4_4")]
    X2RGB10LE,
    #[cfg(feature = "ffmpeg_4_4")]
    X2RGB10BE,

    #[cfg(feature = "ffmpeg_5_0")]
    X2BGR10LE,
    #[cfg(feature = "ffmpeg_5_0")]
    X2BGR10BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P210BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P210LE,
    #[cfg(feature = "ffmpeg_5_0")]
    P410BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P410LE,
    #[cfg(feature = "ffmpeg_5_0")]
    P216BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P216LE,
    #[cfg(feature = "ffmpeg_5_0")]
    P416BE,
    #[cfg(feature = "ffmpeg_5_0")]
    P416LE,

    #[cfg(feature = "ffmpeg_6_0")]
    VUYA,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF16BE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF16LE,
    #[cfg(feature = "ffmpeg_6_0")]
    VUYX,
    #[cfg(feature = "ffmpeg_6_0")]
    P012LE,
    #[cfg(feature = "ffmpeg_6_0")]
    P012BE,
    #[cfg(feature = "ffmpeg_6_0")]
    Y212BE,
    #[cfg(feature = "ffmpeg_6_0")]
    Y212LE,
    #[cfg(feature = "ffmpeg_6_0")]
    XV30BE,
    #[cfg(feature = "ffmpeg_6_0")]
    XV30LE,
    #[cfg(feature = "ffmpeg_6_0")]
    XV36BE,
    #[cfg(feature = "ffmpeg_6_0")]
    XV36LE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBF32BE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBF32LE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF32BE,
    #[cfg(feature = "ffmpeg_6_0")]
    RGBAF32LE,

    #[cfg(feature = "ffmpeg_6_1")]
    P212BE,
    #[cfg(feature = "ffmpeg_6_1")]
    P212LE,
    #[cfg(feature = "ffmpeg_6_1")]
    P412BE,
    #[cfg(feature = "ffmpeg_6_1")]
    P412LE,
    #[cfg(feature = "ffmpeg_6_1")]
    GBRAP14BE,
    #[cfg(feature = "ffmpeg_6_1")]
    GBRAP14LE,

    #[cfg(feature = "rpi")]
    SAND128,
    #[cfg(feature = "rpi")]
    SAND64_10,
    #[cfg(feature = "rpi")]
    SAND64_16,
    #[cfg(feature = "rpi")]
    RPI4_8,
    #[cfg(feature = "rpi")]
    RPI4_10,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Descriptor {
    ptr: *const AVPixFmtDescriptor,
}

unsafe impl Send for Descriptor {}
unsafe impl Sync for Descriptor {}

impl Pixel {
    pub const Y400A: Pixel = Pixel::YA8;
    pub const GRAY8A: Pixel = Pixel::YA8;
    pub const GBR24P: Pixel = Pixel::GBRP;
    #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
    pub const XVMC: Pixel = Pixel::XVMC_MPEG2_IDCT;

    pub fn descriptor(self) -> Option<Descriptor> {
        unsafe {
            let ptr = av_pix_fmt_desc_get(self.into());

            ptr.as_ref().map(|ptr| Descriptor { ptr })
        }
    }
}

impl Descriptor {
    pub fn as_ptr(self) -> *const AVPixFmtDescriptor {
        self.ptr
    }

    pub fn name(self) -> &'static str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn nb_components(self) -> u8 {
        unsafe { (*self.as_ptr()).nb_components }
    }

    pub fn log2_chroma_w(self) -> u8 {
        unsafe { (*self.as_ptr()).log2_chroma_w }
    }

    pub fn log2_chroma_h(self) -> u8 {
        unsafe { (*self.as_ptr()).log2_chroma_h }
    }
}

impl From<AVPixelFormat> for Pixel {
    #[inline]
    fn from(value: AVPixelFormat) -> Self {
        match value {
            AV_PIX_FMT_NONE => Pixel::None,

            AV_PIX_FMT_YUV420P => Pixel::YUV420P,
            AV_PIX_FMT_YUYV422 => Pixel::YUYV422,
            AV_PIX_FMT_RGB24 => Pixel::RGB24,
            AV_PIX_FMT_BGR24 => Pixel::BGR24,
            AV_PIX_FMT_YUV422P => Pixel::YUV422P,
            AV_PIX_FMT_YUV444P => Pixel::YUV444P,
            AV_PIX_FMT_YUV410P => Pixel::YUV410P,
            AV_PIX_FMT_YUV411P => Pixel::YUV411P,
            AV_PIX_FMT_GRAY8 => Pixel::GRAY8,
            AV_PIX_FMT_MONOWHITE => Pixel::MonoWhite,
            AV_PIX_FMT_MONOBLACK => Pixel::MonoBlack,
            AV_PIX_FMT_PAL8 => Pixel::PAL8,
            AV_PIX_FMT_YUVJ420P => Pixel::YUVJ420P,
            AV_PIX_FMT_YUVJ422P => Pixel::YUVJ422P,
            AV_PIX_FMT_YUVJ444P => Pixel::YUVJ444P,
            #[cfg(feature = "ffmpeg_4_0")]
            AV_PIX_FMT_XVMC => Pixel::XVMC,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            AV_PIX_FMT_XVMC_MPEG2_MC => Pixel::XVMC_MPEG2_MC,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            AV_PIX_FMT_XVMC_MPEG2_IDCT => Pixel::XVMC_MPEG2_IDCT,
            AV_PIX_FMT_UYVY422 => Pixel::UYVY422,
            AV_PIX_FMT_UYYVYY411 => Pixel::UYYVYY411,
            AV_PIX_FMT_BGR8 => Pixel::BGR8,
            AV_PIX_FMT_BGR4 => Pixel::BGR4,
            AV_PIX_FMT_BGR4_BYTE => Pixel::BGR4_BYTE,
            AV_PIX_FMT_RGB8 => Pixel::RGB8,
            AV_PIX_FMT_RGB4 => Pixel::RGB4,
            AV_PIX_FMT_RGB4_BYTE => Pixel::RGB4_BYTE,
            AV_PIX_FMT_NV12 => Pixel::NV12,
            AV_PIX_FMT_NV21 => Pixel::NV21,

            AV_PIX_FMT_ARGB => Pixel::ARGB,
            AV_PIX_FMT_RGBA => Pixel::RGBA,
            AV_PIX_FMT_ABGR => Pixel::ABGR,
            AV_PIX_FMT_BGRA => Pixel::BGRA,

            AV_PIX_FMT_GRAY16BE => Pixel::GRAY16BE,
            AV_PIX_FMT_GRAY16LE => Pixel::GRAY16LE,
            AV_PIX_FMT_YUV440P => Pixel::YUV440P,
            AV_PIX_FMT_YUVJ440P => Pixel::YUVJ440P,
            AV_PIX_FMT_YUVA420P => Pixel::YUVA420P,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_H264 => Pixel::VDPAU_H264,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_MPEG1 => Pixel::VDPAU_MPEG1,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_MPEG2 => Pixel::VDPAU_MPEG2,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_WMV3 => Pixel::VDPAU_WMV3,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_VC1 => Pixel::VDPAU_VC1,
            AV_PIX_FMT_RGB48BE => Pixel::RGB48BE,
            AV_PIX_FMT_RGB48LE => Pixel::RGB48LE,

            AV_PIX_FMT_RGB565BE => Pixel::RGB565BE,
            AV_PIX_FMT_RGB565LE => Pixel::RGB565LE,
            AV_PIX_FMT_RGB555BE => Pixel::RGB555BE,
            AV_PIX_FMT_RGB555LE => Pixel::RGB555LE,

            AV_PIX_FMT_BGR565BE => Pixel::BGR565BE,
            AV_PIX_FMT_BGR565LE => Pixel::BGR565LE,
            AV_PIX_FMT_BGR555BE => Pixel::BGR555BE,
            AV_PIX_FMT_BGR555LE => Pixel::BGR555LE,

            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            AV_PIX_FMT_VAAPI_MOCO => Pixel::VAAPI_MOCO,
            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            AV_PIX_FMT_VAAPI_IDCT => Pixel::VAAPI_IDCT,
            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            AV_PIX_FMT_VAAPI_VLD => Pixel::VAAPI_VLD,
            #[cfg(any(not(feature = "ff_api_vaapi"), feature = "ffmpeg_5_0"))]
            AV_PIX_FMT_VAAPI => Pixel::VAAPI,

            AV_PIX_FMT_YUV420P16LE => Pixel::YUV420P16LE,
            AV_PIX_FMT_YUV420P16BE => Pixel::YUV420P16BE,
            AV_PIX_FMT_YUV422P16LE => Pixel::YUV422P16LE,
            AV_PIX_FMT_YUV422P16BE => Pixel::YUV422P16BE,
            AV_PIX_FMT_YUV444P16LE => Pixel::YUV444P16LE,
            AV_PIX_FMT_YUV444P16BE => Pixel::YUV444P16BE,
            #[cfg(feature = "ff_api_vdpau")]
            AV_PIX_FMT_VDPAU_MPEG4 => Pixel::VDPAU_MPEG4,
            AV_PIX_FMT_DXVA2_VLD => Pixel::DXVA2_VLD,

            AV_PIX_FMT_RGB444LE => Pixel::RGB444LE,
            AV_PIX_FMT_RGB444BE => Pixel::RGB444BE,
            AV_PIX_FMT_BGR444LE => Pixel::BGR444LE,
            AV_PIX_FMT_BGR444BE => Pixel::BGR444BE,
            AV_PIX_FMT_YA8 => Pixel::YA8,

            AV_PIX_FMT_BGR48BE => Pixel::BGR48BE,
            AV_PIX_FMT_BGR48LE => Pixel::BGR48LE,

            AV_PIX_FMT_YUV420P9BE => Pixel::YUV420P9BE,
            AV_PIX_FMT_YUV420P9LE => Pixel::YUV420P9LE,
            AV_PIX_FMT_YUV420P10BE => Pixel::YUV420P10BE,
            AV_PIX_FMT_YUV420P10LE => Pixel::YUV420P10LE,
            AV_PIX_FMT_YUV422P10BE => Pixel::YUV422P10BE,
            AV_PIX_FMT_YUV422P10LE => Pixel::YUV422P10LE,
            AV_PIX_FMT_YUV444P9BE => Pixel::YUV444P9BE,
            AV_PIX_FMT_YUV444P9LE => Pixel::YUV444P9LE,
            AV_PIX_FMT_YUV444P10BE => Pixel::YUV444P10BE,
            AV_PIX_FMT_YUV444P10LE => Pixel::YUV444P10LE,
            AV_PIX_FMT_YUV422P9BE => Pixel::YUV422P9BE,
            AV_PIX_FMT_YUV422P9LE => Pixel::YUV422P9LE,
            #[cfg(not(feature = "ffmpeg_4_0"))]
            AV_PIX_FMT_VDA_VLD => Pixel::VDA_VLD,

            AV_PIX_FMT_GBRP => Pixel::GBRP,
            AV_PIX_FMT_GBRP9BE => Pixel::GBRP9BE,
            AV_PIX_FMT_GBRP9LE => Pixel::GBRP9LE,
            AV_PIX_FMT_GBRP10BE => Pixel::GBRP10BE,
            AV_PIX_FMT_GBRP10LE => Pixel::GBRP10LE,
            AV_PIX_FMT_GBRP16BE => Pixel::GBRP16BE,
            AV_PIX_FMT_GBRP16LE => Pixel::GBRP16LE,

            AV_PIX_FMT_YUVA420P9BE => Pixel::YUVA420P9BE,
            AV_PIX_FMT_YUVA420P9LE => Pixel::YUVA420P9LE,
            AV_PIX_FMT_YUVA422P9BE => Pixel::YUVA422P9BE,
            AV_PIX_FMT_YUVA422P9LE => Pixel::YUVA422P9LE,
            AV_PIX_FMT_YUVA444P9BE => Pixel::YUVA444P9BE,
            AV_PIX_FMT_YUVA444P9LE => Pixel::YUVA444P9LE,
            AV_PIX_FMT_YUVA420P10BE => Pixel::YUVA420P10BE,
            AV_PIX_FMT_YUVA420P10LE => Pixel::YUVA420P10LE,
            AV_PIX_FMT_YUVA422P10BE => Pixel::YUVA422P10BE,
            AV_PIX_FMT_YUVA422P10LE => Pixel::YUVA422P10LE,
            AV_PIX_FMT_YUVA444P10BE => Pixel::YUVA444P10BE,
            AV_PIX_FMT_YUVA444P10LE => Pixel::YUVA444P10LE,
            AV_PIX_FMT_YUVA420P16BE => Pixel::YUVA420P16BE,
            AV_PIX_FMT_YUVA420P16LE => Pixel::YUVA420P16LE,
            AV_PIX_FMT_YUVA422P16BE => Pixel::YUVA422P16BE,
            AV_PIX_FMT_YUVA422P16LE => Pixel::YUVA422P16LE,
            AV_PIX_FMT_YUVA444P16BE => Pixel::YUVA444P16BE,
            AV_PIX_FMT_YUVA444P16LE => Pixel::YUVA444P16LE,

            AV_PIX_FMT_VDPAU => Pixel::VDPAU,

            AV_PIX_FMT_XYZ12LE => Pixel::XYZ12LE,
            AV_PIX_FMT_XYZ12BE => Pixel::XYZ12BE,
            AV_PIX_FMT_NV16 => Pixel::NV16,
            AV_PIX_FMT_NV20LE => Pixel::NV20LE,
            AV_PIX_FMT_NV20BE => Pixel::NV20BE,

            AV_PIX_FMT_RGBA64BE => Pixel::RGBA64BE,
            AV_PIX_FMT_RGBA64LE => Pixel::RGBA64LE,
            AV_PIX_FMT_BGRA64BE => Pixel::BGRA64BE,
            AV_PIX_FMT_BGRA64LE => Pixel::BGRA64LE,

            AV_PIX_FMT_YVYU422 => Pixel::YVYU422,

            #[cfg(not(feature = "ffmpeg_4_0"))]
            AV_PIX_FMT_VDA => Pixel::VDA,

            AV_PIX_FMT_YA16BE => Pixel::YA16BE,
            AV_PIX_FMT_YA16LE => Pixel::YA16LE,

            AV_PIX_FMT_QSV => Pixel::QSV,
            AV_PIX_FMT_MMAL => Pixel::MMAL,

            AV_PIX_FMT_D3D11VA_VLD => Pixel::D3D11VA_VLD,

            AV_PIX_FMT_CUDA => Pixel::CUDA,

            AV_PIX_FMT_0RGB => Pixel::ZRGB,
            AV_PIX_FMT_RGB0 => Pixel::RGBZ,
            AV_PIX_FMT_0BGR => Pixel::ZBGR,
            AV_PIX_FMT_BGR0 => Pixel::BGRZ,
            AV_PIX_FMT_YUVA444P => Pixel::YUVA444P,
            AV_PIX_FMT_YUVA422P => Pixel::YUVA422P,

            AV_PIX_FMT_YUV420P12BE => Pixel::YUV420P12BE,
            AV_PIX_FMT_YUV420P12LE => Pixel::YUV420P12LE,
            AV_PIX_FMT_YUV420P14BE => Pixel::YUV420P14BE,
            AV_PIX_FMT_YUV420P14LE => Pixel::YUV420P14LE,
            AV_PIX_FMT_YUV422P12BE => Pixel::YUV422P12BE,
            AV_PIX_FMT_YUV422P12LE => Pixel::YUV422P12LE,
            AV_PIX_FMT_YUV422P14BE => Pixel::YUV422P14BE,
            AV_PIX_FMT_YUV422P14LE => Pixel::YUV422P14LE,
            AV_PIX_FMT_YUV444P12BE => Pixel::YUV444P12BE,
            AV_PIX_FMT_YUV444P12LE => Pixel::YUV444P12LE,
            AV_PIX_FMT_YUV444P14BE => Pixel::YUV444P14BE,
            AV_PIX_FMT_YUV444P14LE => Pixel::YUV444P14LE,
            AV_PIX_FMT_GBRP12BE => Pixel::GBRP12BE,
            AV_PIX_FMT_GBRP12LE => Pixel::GBRP12LE,
            AV_PIX_FMT_GBRP14BE => Pixel::GBRP14BE,
            AV_PIX_FMT_GBRP14LE => Pixel::GBRP14LE,
            AV_PIX_FMT_GBRAP => Pixel::GBRAP,
            AV_PIX_FMT_GBRAP16BE => Pixel::GBRAP16BE,
            AV_PIX_FMT_GBRAP16LE => Pixel::GBRAP16LE,
            AV_PIX_FMT_YUVJ411P => Pixel::YUVJ411P,

            AV_PIX_FMT_BAYER_BGGR8 => Pixel::BAYER_BGGR8,
            AV_PIX_FMT_BAYER_RGGB8 => Pixel::BAYER_RGGB8,
            AV_PIX_FMT_BAYER_GBRG8 => Pixel::BAYER_GBRG8,
            AV_PIX_FMT_BAYER_GRBG8 => Pixel::BAYER_GRBG8,
            AV_PIX_FMT_BAYER_BGGR16LE => Pixel::BAYER_BGGR16LE,
            AV_PIX_FMT_BAYER_BGGR16BE => Pixel::BAYER_BGGR16BE,
            AV_PIX_FMT_BAYER_RGGB16LE => Pixel::BAYER_RGGB16LE,
            AV_PIX_FMT_BAYER_RGGB16BE => Pixel::BAYER_RGGB16BE,
            AV_PIX_FMT_BAYER_GBRG16LE => Pixel::BAYER_GBRG16LE,
            AV_PIX_FMT_BAYER_GBRG16BE => Pixel::BAYER_GBRG16BE,
            AV_PIX_FMT_BAYER_GRBG16LE => Pixel::BAYER_GRBG16LE,
            AV_PIX_FMT_BAYER_GRBG16BE => Pixel::BAYER_GRBG16BE,

            AV_PIX_FMT_YUV440P10LE => Pixel::YUV440P10LE,
            AV_PIX_FMT_YUV440P10BE => Pixel::YUV440P10BE,
            AV_PIX_FMT_YUV440P12LE => Pixel::YUV440P12LE,
            AV_PIX_FMT_YUV440P12BE => Pixel::YUV440P12BE,
            AV_PIX_FMT_AYUV64LE => Pixel::AYUV64LE,
            AV_PIX_FMT_AYUV64BE => Pixel::AYUV64BE,

            AV_PIX_FMT_VIDEOTOOLBOX => Pixel::VIDEOTOOLBOX,

            AV_PIX_FMT_P010LE => Pixel::P010LE,
            AV_PIX_FMT_P010BE => Pixel::P010BE,
            AV_PIX_FMT_GBRAP12BE => Pixel::GBRAP12BE,
            AV_PIX_FMT_GBRAP12LE => Pixel::GBRAP12LE,
            AV_PIX_FMT_GBRAP10LE => Pixel::GBRAP10LE,
            AV_PIX_FMT_GBRAP10BE => Pixel::GBRAP10BE,
            AV_PIX_FMT_MEDIACODEC => Pixel::MEDIACODEC,
            AV_PIX_FMT_GRAY12BE => Pixel::GRAY12BE,
            AV_PIX_FMT_GRAY12LE => Pixel::GRAY12LE,
            AV_PIX_FMT_GRAY10BE => Pixel::GRAY10BE,
            AV_PIX_FMT_GRAY10LE => Pixel::GRAY10LE,
            AV_PIX_FMT_P016LE => Pixel::P016LE,
            AV_PIX_FMT_P016BE => Pixel::P016BE,

            AV_PIX_FMT_NB => Pixel::None,

            AV_PIX_FMT_D3D11 => Pixel::D3D11,
            AV_PIX_FMT_GRAY9BE => Pixel::GRAY9BE,
            AV_PIX_FMT_GRAY9LE => Pixel::GRAY9LE,
            AV_PIX_FMT_GBRPF32BE => Pixel::GBRPF32BE,
            AV_PIX_FMT_GBRPF32LE => Pixel::GBRPF32LE,
            AV_PIX_FMT_GBRAPF32BE => Pixel::GBRAPF32BE,
            AV_PIX_FMT_GBRAPF32LE => Pixel::GBRAPF32LE,
            AV_PIX_FMT_DRM_PRIME => Pixel::DRM_PRIME,

            #[cfg(feature = "ffmpeg_4_0")]
            AV_PIX_FMT_OPENCL => Pixel::OPENCL,

            #[cfg(feature = "ffmpeg_4_1")]
            AV_PIX_FMT_GRAY14BE => Pixel::GRAY14BE,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_PIX_FMT_GRAY14LE => Pixel::GRAY14LE,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_PIX_FMT_GRAYF32BE => Pixel::GRAYF32BE,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_PIX_FMT_GRAYF32LE => Pixel::GRAYF32LE,

            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_YUVA422P12BE => Pixel::YUVA422P12BE,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_YUVA422P12LE => Pixel::YUVA422P12LE,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_YUVA444P12BE => Pixel::YUVA444P12BE,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_YUVA444P12LE => Pixel::YUVA444P12LE,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_NV24 => Pixel::NV24,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_PIX_FMT_NV42 => Pixel::NV42,

            #[cfg(feature = "ffmpeg_4_3")]
            AV_PIX_FMT_VULKAN => Pixel::VULKAN,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_PIX_FMT_Y210BE => Pixel::Y210BE,
            #[cfg(feature = "ffmpeg_4_3")]
            AV_PIX_FMT_Y210LE => Pixel::Y210LE,

            #[cfg(feature = "ffmpeg_4_4")]
            AV_PIX_FMT_X2RGB10LE => Pixel::X2RGB10LE,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_PIX_FMT_X2RGB10BE => Pixel::X2RGB10BE,

            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_X2BGR10LE => Pixel::X2BGR10LE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_X2BGR10BE => Pixel::X2BGR10BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_P210BE => Pixel::P210BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_P210LE => Pixel::P210LE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_P410BE => Pixel::P410BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_P410LE => Pixel::P410LE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_P216BE => Pixel::P216BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_P216LE => Pixel::P216LE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_P416BE => Pixel::P416BE,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_PIX_FMT_P416LE => Pixel::P416LE,

            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_VUYA => Pixel::VUYA,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_RGBAF16BE => Pixel::RGBAF16BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_RGBAF16LE => Pixel::RGBAF16LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_VUYX => Pixel::VUYX,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_P012LE => Pixel::P012LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_P012BE => Pixel::P012BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_Y212BE => Pixel::Y212BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_Y212LE => Pixel::Y212LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_XV30BE => Pixel::XV30BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_XV30LE => Pixel::XV30LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_XV36BE => Pixel::XV36BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_XV36LE => Pixel::XV36LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_RGBF32BE => Pixel::RGBF32BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_RGBF32LE => Pixel::RGBF32LE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_RGBAF32BE => Pixel::RGBAF32BE,
            #[cfg(feature = "ffmpeg_6_0")]
            AV_PIX_FMT_RGBAF32LE => Pixel::RGBAF32LE,

            #[cfg(feature = "ffmpeg_6_1")]
            AV_PIX_FMT_P212BE => Pixel::P212BE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_PIX_FMT_P212LE => Pixel::P212LE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_PIX_FMT_P412BE => Pixel::P412BE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_PIX_FMT_P412LE => Pixel::P412LE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_PIX_FMT_GBRAP14BE => Pixel::GBRAP14BE,
            #[cfg(feature = "ffmpeg_6_1")]
            AV_PIX_FMT_GBRAP14LE => Pixel::GBRAP14LE,

            #[cfg(feature = "rpi")]
            AV_PIX_FMT_SAND128 => Pixel::SAND128,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_SAND64_10 => Pixel::SAND64_10,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_SAND64_16 => Pixel::SAND64_16,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_RPI4_8 => Pixel::RPI4_8,
            #[cfg(feature = "rpi")]
            AV_PIX_FMT_RPI4_10 => Pixel::RPI4_10,
        }
    }
}

impl From<Pixel> for AVPixelFormat {
    #[inline]
    fn from(value: Pixel) -> AVPixelFormat {
        match value {
            Pixel::None => AV_PIX_FMT_NONE,

            Pixel::YUV420P => AV_PIX_FMT_YUV420P,
            Pixel::YUYV422 => AV_PIX_FMT_YUYV422,
            Pixel::RGB24 => AV_PIX_FMT_RGB24,
            Pixel::BGR24 => AV_PIX_FMT_BGR24,
            Pixel::YUV422P => AV_PIX_FMT_YUV422P,
            Pixel::YUV444P => AV_PIX_FMT_YUV444P,
            Pixel::YUV410P => AV_PIX_FMT_YUV410P,
            Pixel::YUV411P => AV_PIX_FMT_YUV411P,
            Pixel::GRAY8 => AV_PIX_FMT_GRAY8,
            Pixel::MonoWhite => AV_PIX_FMT_MONOWHITE,
            Pixel::MonoBlack => AV_PIX_FMT_MONOBLACK,
            Pixel::PAL8 => AV_PIX_FMT_PAL8,
            Pixel::YUVJ420P => AV_PIX_FMT_YUVJ420P,
            Pixel::YUVJ422P => AV_PIX_FMT_YUVJ422P,
            Pixel::YUVJ444P => AV_PIX_FMT_YUVJ444P,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            Pixel::XVMC_MPEG2_MC => AV_PIX_FMT_XVMC_MPEG2_MC,
            #[cfg(all(feature = "ff_api_xvmc", not(feature = "ffmpeg_5_0")))]
            Pixel::XVMC_MPEG2_IDCT => AV_PIX_FMT_XVMC_MPEG2_IDCT,
            Pixel::UYVY422 => AV_PIX_FMT_UYVY422,
            Pixel::UYYVYY411 => AV_PIX_FMT_UYYVYY411,
            Pixel::BGR8 => AV_PIX_FMT_BGR8,
            Pixel::BGR4 => AV_PIX_FMT_BGR4,
            Pixel::BGR4_BYTE => AV_PIX_FMT_BGR4_BYTE,
            Pixel::RGB8 => AV_PIX_FMT_RGB8,
            Pixel::RGB4 => AV_PIX_FMT_RGB4,
            Pixel::RGB4_BYTE => AV_PIX_FMT_RGB4_BYTE,
            Pixel::NV12 => AV_PIX_FMT_NV12,
            Pixel::NV21 => AV_PIX_FMT_NV21,

            Pixel::ARGB => AV_PIX_FMT_ARGB,
            Pixel::RGBA => AV_PIX_FMT_RGBA,
            Pixel::ABGR => AV_PIX_FMT_ABGR,
            Pixel::BGRA => AV_PIX_FMT_BGRA,

            Pixel::GRAY16BE => AV_PIX_FMT_GRAY16BE,
            Pixel::GRAY16LE => AV_PIX_FMT_GRAY16LE,
            Pixel::YUV440P => AV_PIX_FMT_YUV440P,
            Pixel::YUVJ440P => AV_PIX_FMT_YUVJ440P,
            Pixel::YUVA420P => AV_PIX_FMT_YUVA420P,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_H264 => AV_PIX_FMT_VDPAU_H264,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_MPEG1 => AV_PIX_FMT_VDPAU_MPEG1,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_MPEG2 => AV_PIX_FMT_VDPAU_MPEG2,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_WMV3 => AV_PIX_FMT_VDPAU_WMV3,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_VC1 => AV_PIX_FMT_VDPAU_VC1,
            Pixel::RGB48BE => AV_PIX_FMT_RGB48BE,
            Pixel::RGB48LE => AV_PIX_FMT_RGB48LE,

            Pixel::RGB565BE => AV_PIX_FMT_RGB565BE,
            Pixel::RGB565LE => AV_PIX_FMT_RGB565LE,
            Pixel::RGB555BE => AV_PIX_FMT_RGB555BE,
            Pixel::RGB555LE => AV_PIX_FMT_RGB555LE,

            Pixel::BGR565BE => AV_PIX_FMT_BGR565BE,
            Pixel::BGR565LE => AV_PIX_FMT_BGR565LE,
            Pixel::BGR555BE => AV_PIX_FMT_BGR555BE,
            Pixel::BGR555LE => AV_PIX_FMT_BGR555LE,

            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            Pixel::VAAPI_MOCO => AV_PIX_FMT_VAAPI_MOCO,
            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            Pixel::VAAPI_IDCT => AV_PIX_FMT_VAAPI_IDCT,
            #[cfg(all(feature = "ff_api_vaapi", not(feature = "ffmpeg_5_0")))]
            Pixel::VAAPI_VLD => AV_PIX_FMT_VAAPI_VLD,
            #[cfg(not(feature = "ff_api_vaapi"))]
            Pixel::VAAPI => AV_PIX_FMT_VAAPI,

            Pixel::YUV420P16LE => AV_PIX_FMT_YUV420P16LE,
            Pixel::YUV420P16BE => AV_PIX_FMT_YUV420P16BE,
            Pixel::YUV422P16LE => AV_PIX_FMT_YUV422P16LE,
            Pixel::YUV422P16BE => AV_PIX_FMT_YUV422P16BE,
            Pixel::YUV444P16LE => AV_PIX_FMT_YUV444P16LE,
            Pixel::YUV444P16BE => AV_PIX_FMT_YUV444P16BE,
            #[cfg(feature = "ff_api_vdpau")]
            Pixel::VDPAU_MPEG4 => AV_PIX_FMT_VDPAU_MPEG4,
            Pixel::DXVA2_VLD => AV_PIX_FMT_DXVA2_VLD,

            Pixel::RGB444LE => AV_PIX_FMT_RGB444LE,
            Pixel::RGB444BE => AV_PIX_FMT_RGB444BE,
            Pixel::BGR444LE => AV_PIX_FMT_BGR444LE,
            Pixel::BGR444BE => AV_PIX_FMT_BGR444BE,
            Pixel::YA8 => AV_PIX_FMT_YA8,

            Pixel::BGR48BE => AV_PIX_FMT_BGR48BE,
            Pixel::BGR48LE => AV_PIX_FMT_BGR48LE,

            Pixel::YUV420P9BE => AV_PIX_FMT_YUV420P9BE,
            Pixel::YUV420P9LE => AV_PIX_FMT_YUV420P9LE,
            Pixel::YUV420P10BE => AV_PIX_FMT_YUV420P10BE,
            Pixel::YUV420P10LE => AV_PIX_FMT_YUV420P10LE,
            Pixel::YUV422P10BE => AV_PIX_FMT_YUV422P10BE,
            Pixel::YUV422P10LE => AV_PIX_FMT_YUV422P10LE,
            Pixel::YUV444P9BE => AV_PIX_FMT_YUV444P9BE,
            Pixel::YUV444P9LE => AV_PIX_FMT_YUV444P9LE,
            Pixel::YUV444P10BE => AV_PIX_FMT_YUV444P10BE,
            Pixel::YUV444P10LE => AV_PIX_FMT_YUV444P10LE,
            Pixel::YUV422P9BE => AV_PIX_FMT_YUV422P9BE,
            Pixel::YUV422P9LE => AV_PIX_FMT_YUV422P9LE,
            #[cfg(not(feature = "ffmpeg_4_0"))]
            Pixel::VDA_VLD => AV_PIX_FMT_VDA_VLD,

            Pixel::GBRP => AV_PIX_FMT_GBRP,
            Pixel::GBRP9BE => AV_PIX_FMT_GBRP9BE,
            Pixel::GBRP9LE => AV_PIX_FMT_GBRP9LE,
            Pixel::GBRP10BE => AV_PIX_FMT_GBRP10BE,
            Pixel::GBRP10LE => AV_PIX_FMT_GBRP10LE,
            Pixel::GBRP16BE => AV_PIX_FMT_GBRP16BE,
            Pixel::GBRP16LE => AV_PIX_FMT_GBRP16LE,

            Pixel::YUVA420P9BE => AV_PIX_FMT_YUVA420P9BE,
            Pixel::YUVA420P9LE => AV_PIX_FMT_YUVA420P9LE,
            Pixel::YUVA422P9BE => AV_PIX_FMT_YUVA422P9BE,
            Pixel::YUVA422P9LE => AV_PIX_FMT_YUVA422P9LE,
            Pixel::YUVA444P9BE => AV_PIX_FMT_YUVA444P9BE,
            Pixel::YUVA444P9LE => AV_PIX_FMT_YUVA444P9LE,
            Pixel::YUVA420P10BE => AV_PIX_FMT_YUVA420P10BE,
            Pixel::YUVA420P10LE => AV_PIX_FMT_YUVA420P10LE,
            Pixel::YUVA422P10BE => AV_PIX_FMT_YUVA422P10BE,
            Pixel::YUVA422P10LE => AV_PIX_FMT_YUVA422P10LE,
            Pixel::YUVA444P10BE => AV_PIX_FMT_YUVA444P10BE,
            Pixel::YUVA444P10LE => AV_PIX_FMT_YUVA444P10LE,
            Pixel::YUVA420P16BE => AV_PIX_FMT_YUVA420P16BE,
            Pixel::YUVA420P16LE => AV_PIX_FMT_YUVA420P16LE,
            Pixel::YUVA422P16BE => AV_PIX_FMT_YUVA422P16BE,
            Pixel::YUVA422P16LE => AV_PIX_FMT_YUVA422P16LE,
            Pixel::YUVA444P16BE => AV_PIX_FMT_YUVA444P16BE,
            Pixel::YUVA444P16LE => AV_PIX_FMT_YUVA444P16LE,

            Pixel::VDPAU => AV_PIX_FMT_VDPAU,

            Pixel::XYZ12LE => AV_PIX_FMT_XYZ12LE,
            Pixel::XYZ12BE => AV_PIX_FMT_XYZ12BE,
            Pixel::NV16 => AV_PIX_FMT_NV16,
            Pixel::NV20LE => AV_PIX_FMT_NV20LE,
            Pixel::NV20BE => AV_PIX_FMT_NV20BE,

            Pixel::RGBA64BE => AV_PIX_FMT_RGBA64BE,
            Pixel::RGBA64LE => AV_PIX_FMT_RGBA64LE,
            Pixel::BGRA64BE => AV_PIX_FMT_BGRA64BE,
            Pixel::BGRA64LE => AV_PIX_FMT_BGRA64LE,

            Pixel::YVYU422 => AV_PIX_FMT_YVYU422,

            #[cfg(not(feature = "ffmpeg_4_0"))]
            Pixel::VDA => AV_PIX_FMT_VDA,

            Pixel::YA16BE => AV_PIX_FMT_YA16BE,
            Pixel::YA16LE => AV_PIX_FMT_YA16LE,

            Pixel::QSV => AV_PIX_FMT_QSV,
            Pixel::MMAL => AV_PIX_FMT_MMAL,

            Pixel::D3D11VA_VLD => AV_PIX_FMT_D3D11VA_VLD,

            Pixel::CUDA => AV_PIX_FMT_CUDA,

            Pixel::ZRGB => AV_PIX_FMT_0RGB,
            Pixel::RGBZ => AV_PIX_FMT_RGB0,
            Pixel::ZBGR => AV_PIX_FMT_0BGR,
            Pixel::BGRZ => AV_PIX_FMT_BGR0,
            Pixel::YUVA444P => AV_PIX_FMT_YUVA444P,
            Pixel::YUVA422P => AV_PIX_FMT_YUVA422P,

            Pixel::YUV420P12BE => AV_PIX_FMT_YUV420P12BE,
            Pixel::YUV420P12LE => AV_PIX_FMT_YUV420P12LE,
            Pixel::YUV420P14BE => AV_PIX_FMT_YUV420P14BE,
            Pixel::YUV420P14LE => AV_PIX_FMT_YUV420P14LE,
            Pixel::YUV422P12BE => AV_PIX_FMT_YUV422P12BE,
            Pixel::YUV422P12LE => AV_PIX_FMT_YUV422P12LE,
            Pixel::YUV422P14BE => AV_PIX_FMT_YUV422P14BE,
            Pixel::YUV422P14LE => AV_PIX_FMT_YUV422P14LE,
            Pixel::YUV444P12BE => AV_PIX_FMT_YUV444P12BE,
            Pixel::YUV444P12LE => AV_PIX_FMT_YUV444P12LE,
            Pixel::YUV444P14BE => AV_PIX_FMT_YUV444P14BE,
            Pixel::YUV444P14LE => AV_PIX_FMT_YUV444P14LE,
            Pixel::GBRP12BE => AV_PIX_FMT_GBRP12BE,
            Pixel::GBRP12LE => AV_PIX_FMT_GBRP12LE,
            Pixel::GBRP14BE => AV_PIX_FMT_GBRP14BE,
            Pixel::GBRP14LE => AV_PIX_FMT_GBRP14LE,
            Pixel::GBRAP => AV_PIX_FMT_GBRAP,
            Pixel::GBRAP16BE => AV_PIX_FMT_GBRAP16BE,
            Pixel::GBRAP16LE => AV_PIX_FMT_GBRAP16LE,
            Pixel::YUVJ411P => AV_PIX_FMT_YUVJ411P,

            Pixel::BAYER_BGGR8 => AV_PIX_FMT_BAYER_BGGR8,
            Pixel::BAYER_RGGB8 => AV_PIX_FMT_BAYER_RGGB8,
            Pixel::BAYER_GBRG8 => AV_PIX_FMT_BAYER_GBRG8,
            Pixel::BAYER_GRBG8 => AV_PIX_FMT_BAYER_GRBG8,
            Pixel::BAYER_BGGR16LE => AV_PIX_FMT_BAYER_BGGR16LE,
            Pixel::BAYER_BGGR16BE => AV_PIX_FMT_BAYER_BGGR16BE,
            Pixel::BAYER_RGGB16LE => AV_PIX_FMT_BAYER_RGGB16LE,
            Pixel::BAYER_RGGB16BE => AV_PIX_FMT_BAYER_RGGB16BE,
            Pixel::BAYER_GBRG16LE => AV_PIX_FMT_BAYER_GBRG16LE,
            Pixel::BAYER_GBRG16BE => AV_PIX_FMT_BAYER_GBRG16BE,
            Pixel::BAYER_GRBG16LE => AV_PIX_FMT_BAYER_GRBG16LE,
            Pixel::BAYER_GRBG16BE => AV_PIX_FMT_BAYER_GRBG16BE,

            Pixel::YUV440P10LE => AV_PIX_FMT_YUV440P10LE,
            Pixel::YUV440P10BE => AV_PIX_FMT_YUV440P10BE,
            Pixel::YUV440P12LE => AV_PIX_FMT_YUV440P12LE,
            Pixel::YUV440P12BE => AV_PIX_FMT_YUV440P12BE,
            Pixel::AYUV64LE => AV_PIX_FMT_AYUV64LE,
            Pixel::AYUV64BE => AV_PIX_FMT_AYUV64BE,

            Pixel::VIDEOTOOLBOX => AV_PIX_FMT_VIDEOTOOLBOX,

            // --- defaults
            #[cfg(feature = "ffmpeg_4_0")]
            Pixel::XVMC => AV_PIX_FMT_XVMC,

            Pixel::RGB32 => AV_PIX_FMT_RGB32,
            Pixel::RGB32_1 => AV_PIX_FMT_RGB32_1,
            Pixel::BGR32 => AV_PIX_FMT_BGR32,
            Pixel::BGR32_1 => AV_PIX_FMT_BGR32_1,
            Pixel::ZRGB32 => AV_PIX_FMT_0RGB32,
            Pixel::ZBGR32 => AV_PIX_FMT_0BGR32,

            Pixel::GRAY16 => AV_PIX_FMT_GRAY16,
            Pixel::YA16 => AV_PIX_FMT_YA16,
            Pixel::RGB48 => AV_PIX_FMT_RGB48,
            Pixel::RGB565 => AV_PIX_FMT_RGB565,
            Pixel::RGB555 => AV_PIX_FMT_RGB555,
            Pixel::RGB444 => AV_PIX_FMT_RGB444,
            Pixel::BGR48 => AV_PIX_FMT_BGR48,
            Pixel::BGR565 => AV_PIX_FMT_BGR565,
            Pixel::BGR555 => AV_PIX_FMT_BGR555,
            Pixel::BGR444 => AV_PIX_FMT_BGR444,

            Pixel::YUV420P9 => AV_PIX_FMT_YUV420P9,
            Pixel::YUV422P9 => AV_PIX_FMT_YUV422P9,
            Pixel::YUV444P9 => AV_PIX_FMT_YUV444P9,
            Pixel::YUV420P10 => AV_PIX_FMT_YUV420P10,
            Pixel::YUV422P10 => AV_PIX_FMT_YUV422P10,
            Pixel::YUV440P10 => AV_PIX_FMT_YUV440P10,
            Pixel::YUV444P10 => AV_PIX_FMT_YUV444P10,
            Pixel::YUV420P12 => AV_PIX_FMT_YUV420P12,
            Pixel::YUV422P12 => AV_PIX_FMT_YUV422P12,
            Pixel::YUV440P12 => AV_PIX_FMT_YUV440P12,
            Pixel::YUV444P12 => AV_PIX_FMT_YUV444P12,
            Pixel::YUV420P14 => AV_PIX_FMT_YUV420P14,
            Pixel::YUV422P14 => AV_PIX_FMT_YUV422P14,
            Pixel::YUV444P14 => AV_PIX_FMT_YUV444P14,
            Pixel::YUV420P16 => AV_PIX_FMT_YUV420P16,
            Pixel::YUV422P16 => AV_PIX_FMT_YUV422P16,
            Pixel::YUV444P16 => AV_PIX_FMT_YUV444P16,

            Pixel::GBRP9 => AV_PIX_FMT_GBRP9,
            Pixel::GBRP10 => AV_PIX_FMT_GBRP10,
            Pixel::GBRP12 => AV_PIX_FMT_GBRP12,
            Pixel::GBRP14 => AV_PIX_FMT_GBRP14,
            Pixel::GBRP16 => AV_PIX_FMT_GBRP16,
            Pixel::GBRAP16 => AV_PIX_FMT_GBRAP16,

            Pixel::BAYER_BGGR16 => AV_PIX_FMT_BAYER_BGGR16,
            Pixel::BAYER_RGGB16 => AV_PIX_FMT_BAYER_RGGB16,
            Pixel::BAYER_GBRG16 => AV_PIX_FMT_BAYER_GBRG16,
            Pixel::BAYER_GRBG16 => AV_PIX_FMT_BAYER_GRBG16,

            Pixel::YUVA420P9 => AV_PIX_FMT_YUVA420P9,
            Pixel::YUVA422P9 => AV_PIX_FMT_YUVA422P9,
            Pixel::YUVA444P9 => AV_PIX_FMT_YUVA444P9,
            Pixel::YUVA420P10 => AV_PIX_FMT_YUVA420P10,
            Pixel::YUVA422P10 => AV_PIX_FMT_YUVA422P10,
            Pixel::YUVA444P10 => AV_PIX_FMT_YUVA444P10,
            Pixel::YUVA420P16 => AV_PIX_FMT_YUVA420P16,
            Pixel::YUVA422P16 => AV_PIX_FMT_YUVA422P16,
            Pixel::YUVA444P16 => AV_PIX_FMT_YUVA444P16,

            Pixel::XYZ12 => AV_PIX_FMT_XYZ12,
            Pixel::NV20 => AV_PIX_FMT_NV20,
            Pixel::AYUV64 => AV_PIX_FMT_AYUV64,

            Pixel::P010LE => AV_PIX_FMT_P010LE,
            Pixel::P010BE => AV_PIX_FMT_P010BE,
            Pixel::GBRAP12BE => AV_PIX_FMT_GBRAP12BE,
            Pixel::GBRAP12LE => AV_PIX_FMT_GBRAP12LE,
            Pixel::GBRAP10LE => AV_PIX_FMT_GBRAP10LE,
            Pixel::GBRAP10BE => AV_PIX_FMT_GBRAP10BE,
            Pixel::MEDIACODEC => AV_PIX_FMT_MEDIACODEC,
            Pixel::GRAY12BE => AV_PIX_FMT_GRAY12BE,
            Pixel::GRAY12LE => AV_PIX_FMT_GRAY12LE,
            Pixel::GRAY10BE => AV_PIX_FMT_GRAY10BE,
            Pixel::GRAY10LE => AV_PIX_FMT_GRAY10LE,
            Pixel::P016LE => AV_PIX_FMT_P016LE,
            Pixel::P016BE => AV_PIX_FMT_P016BE,

            Pixel::D3D11 => AV_PIX_FMT_D3D11,
            Pixel::GRAY9BE => AV_PIX_FMT_GRAY9BE,
            Pixel::GRAY9LE => AV_PIX_FMT_GRAY9LE,
            Pixel::GBRPF32BE => AV_PIX_FMT_GBRPF32BE,
            Pixel::GBRPF32LE => AV_PIX_FMT_GBRPF32LE,
            Pixel::GBRAPF32BE => AV_PIX_FMT_GBRAPF32BE,
            Pixel::GBRAPF32LE => AV_PIX_FMT_GBRAPF32LE,
            Pixel::DRM_PRIME => AV_PIX_FMT_DRM_PRIME,

            #[cfg(feature = "ffmpeg_4_0")]
            Pixel::OPENCL => AV_PIX_FMT_OPENCL,

            #[cfg(feature = "ffmpeg_4_1")]
            Pixel::GRAY14BE => AV_PIX_FMT_GRAY14BE,
            #[cfg(feature = "ffmpeg_4_1")]
            Pixel::GRAY14LE => AV_PIX_FMT_GRAY14LE,
            #[cfg(feature = "ffmpeg_4_1")]
            Pixel::GRAYF32BE => AV_PIX_FMT_GRAYF32BE,
            #[cfg(feature = "ffmpeg_4_1")]
            Pixel::GRAYF32LE => AV_PIX_FMT_GRAYF32LE,

            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::YUVA422P12BE => AV_PIX_FMT_YUVA422P12BE,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::YUVA422P12LE => AV_PIX_FMT_YUVA422P12LE,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::YUVA444P12BE => AV_PIX_FMT_YUVA444P12BE,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::YUVA444P12LE => AV_PIX_FMT_YUVA444P12LE,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::NV24 => AV_PIX_FMT_NV24,
            #[cfg(feature = "ffmpeg_4_2")]
            Pixel::NV42 => AV_PIX_FMT_NV42,

            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::VULKAN => AV_PIX_FMT_VULKAN,
            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::Y210BE => AV_PIX_FMT_Y210BE,
            #[cfg(feature = "ffmpeg_4_3")]
            Pixel::Y210LE => AV_PIX_FMT_Y210LE,

            #[cfg(feature = "ffmpeg_4_4")]
            Pixel::X2RGB10LE => AV_PIX_FMT_X2RGB10LE,
            #[cfg(feature = "ffmpeg_4_4")]
            Pixel::X2RGB10BE => AV_PIX_FMT_X2RGB10BE,

            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::X2BGR10LE => AV_PIX_FMT_X2BGR10LE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::X2BGR10BE => AV_PIX_FMT_X2BGR10BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P210BE => AV_PIX_FMT_P210BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P210LE => AV_PIX_FMT_P210LE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P410BE => AV_PIX_FMT_P410BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P410LE => AV_PIX_FMT_P410LE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P216BE => AV_PIX_FMT_P216BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P216LE => AV_PIX_FMT_P216LE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P416BE => AV_PIX_FMT_P416BE,
            #[cfg(feature = "ffmpeg_5_0")]
            Pixel::P416LE => AV_PIX_FMT_P416LE,

            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::VUYA => AV_PIX_FMT_VUYA,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBAF16BE => AV_PIX_FMT_RGBAF16BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBAF16LE => AV_PIX_FMT_RGBAF16LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::VUYX => AV_PIX_FMT_VUYX,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::P012LE => AV_PIX_FMT_P012LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::P012BE => AV_PIX_FMT_P012BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::Y212BE => AV_PIX_FMT_Y212BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::Y212LE => AV_PIX_FMT_Y212LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::XV30BE => AV_PIX_FMT_XV30BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::XV30LE => AV_PIX_FMT_XV30LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::XV36BE => AV_PIX_FMT_XV36BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::XV36LE => AV_PIX_FMT_XV36LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBF32BE => AV_PIX_FMT_RGBF32BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBF32LE => AV_PIX_FMT_RGBF32LE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBAF32BE => AV_PIX_FMT_RGBAF32BE,
            #[cfg(feature = "ffmpeg_6_0")]
            Pixel::RGBAF32LE => AV_PIX_FMT_RGBAF32LE,

            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::P212BE => AV_PIX_FMT_P212BE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::P212LE => AV_PIX_FMT_P212LE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::P412BE => AV_PIX_FMT_P412BE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::P412LE => AV_PIX_FMT_P412LE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::GBRAP14BE => AV_PIX_FMT_GBRAP14BE,
            #[cfg(feature = "ffmpeg_6_1")]
            Pixel::GBRAP14LE => AV_PIX_FMT_GBRAP14LE,

            #[cfg(feature = "rpi")]
            Pixel::SAND128 => AV_PIX_FMT_SAND128,
            #[cfg(feature = "rpi")]
            Pixel::SAND64_10 => AV_PIX_FMT_SAND64_10,
            #[cfg(feature = "rpi")]
            Pixel::SAND64_16 => AV_PIX_FMT_SAND64_16,
            #[cfg(feature = "rpi")]
            Pixel::RPI4_8 => AV_PIX_FMT_RPI4_8,
            #[cfg(feature = "rpi")]
            Pixel::RPI4_10 => AV_PIX_FMT_RPI4_10,
        }
    }
}

#[derive(Debug)]
pub enum ParsePixelError {
    NulError(NulError),
    UnknownFormat,
}

impl fmt::Display for ParsePixelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsePixelError::NulError(ref e) => e.fmt(f),
            ParsePixelError::UnknownFormat => write!(f, "unknown pixel format"),
        }
    }
}

impl error::Error for ParsePixelError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ParsePixelError::NulError(ref e) => Some(e),
            ParsePixelError::UnknownFormat => None,
        }
    }
}

impl From<NulError> for ParsePixelError {
    fn from(x: NulError) -> ParsePixelError {
        ParsePixelError::NulError(x)
    }
}

impl FromStr for Pixel {
    type Err = ParsePixelError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Pixel, ParsePixelError> {
        let cstring = CString::new(s)?;
        let format = unsafe { av_get_pix_fmt(cstring.as_ptr()) }.into();

        if format == Pixel::None {
            Err(ParsePixelError::UnknownFormat)
        } else {
            Ok(format)
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/format/sample.rs">
use std::ffi::{CStr, CString};
use std::ops::Index;
use std::ptr;
use std::slice;
use std::str::from_utf8_unchecked;

use ffi::AVSampleFormat::*;
use ffi::*;
use libc::{c_int, c_void};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Sample {
    None,

    U8(Type),
    I16(Type),
    I32(Type),
    I64(Type),
    F32(Type),
    F64(Type),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
    Packed,
    Planar,
}

impl Sample {
    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr(av_get_sample_fmt_name((*self).into())).to_bytes())
        }
    }

    #[inline]
    pub fn packed(&self) -> Self {
        unsafe { Sample::from(av_get_packed_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn planar(&self) -> Self {
        unsafe { Sample::from(av_get_planar_sample_fmt((*self).into())) }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        unsafe { av_sample_fmt_is_planar((*self).into()) == 1 }
    }

    #[inline]
    pub fn is_packed(&self) -> bool {
        !self.is_planar()
    }

    #[inline]
    pub fn bytes(&self) -> usize {
        unsafe { av_get_bytes_per_sample((*self).into()) as usize }
    }

    #[inline]
    pub fn buffer(&self, channels: u16, samples: usize, align: bool) -> Buffer {
        Buffer::new(*self, channels, samples, align)
    }
}

impl From<AVSampleFormat> for Sample {
    #[inline]
    fn from(value: AVSampleFormat) -> Self {
        match value {
            AV_SAMPLE_FMT_NONE => Sample::None,

            AV_SAMPLE_FMT_U8 => Sample::U8(Type::Packed),
            AV_SAMPLE_FMT_S16 => Sample::I16(Type::Packed),
            AV_SAMPLE_FMT_S32 => Sample::I32(Type::Packed),
            AV_SAMPLE_FMT_S64 => Sample::I64(Type::Packed),
            AV_SAMPLE_FMT_FLT => Sample::F32(Type::Packed),
            AV_SAMPLE_FMT_DBL => Sample::F64(Type::Packed),

            AV_SAMPLE_FMT_U8P => Sample::U8(Type::Planar),
            AV_SAMPLE_FMT_S16P => Sample::I16(Type::Planar),
            AV_SAMPLE_FMT_S32P => Sample::I32(Type::Planar),
            AV_SAMPLE_FMT_S64P => Sample::I64(Type::Planar),
            AV_SAMPLE_FMT_FLTP => Sample::F32(Type::Planar),
            AV_SAMPLE_FMT_DBLP => Sample::F64(Type::Planar),

            AV_SAMPLE_FMT_NB => Sample::None,
        }
    }
}

impl From<&'static str> for Sample {
    #[inline]
    fn from(value: &'static str) -> Self {
        unsafe {
            let value = CString::new(value).unwrap();

            Sample::from(av_get_sample_fmt(value.as_ptr()))
        }
    }
}

impl From<Sample> for AVSampleFormat {
    #[inline]
    fn from(value: Sample) -> AVSampleFormat {
        match value {
            Sample::None => AV_SAMPLE_FMT_NONE,

            Sample::U8(Type::Packed) => AV_SAMPLE_FMT_U8,
            Sample::I16(Type::Packed) => AV_SAMPLE_FMT_S16,
            Sample::I32(Type::Packed) => AV_SAMPLE_FMT_S32,
            Sample::I64(Type::Packed) => AV_SAMPLE_FMT_S64,
            Sample::F32(Type::Packed) => AV_SAMPLE_FMT_FLT,
            Sample::F64(Type::Packed) => AV_SAMPLE_FMT_DBL,

            Sample::U8(Type::Planar) => AV_SAMPLE_FMT_U8P,
            Sample::I16(Type::Planar) => AV_SAMPLE_FMT_S16P,
            Sample::I32(Type::Planar) => AV_SAMPLE_FMT_S32P,
            Sample::I64(Type::Planar) => AV_SAMPLE_FMT_S64P,
            Sample::F32(Type::Planar) => AV_SAMPLE_FMT_FLTP,
            Sample::F64(Type::Planar) => AV_SAMPLE_FMT_DBLP,
        }
    }
}

pub struct Buffer {
    pub format: Sample,
    pub channels: u16,
    pub samples: usize,
    pub align: bool,

    buffer: *mut *mut u8,
    size: c_int,
}

impl Buffer {
    #[inline]
    pub fn size(format: Sample, channels: u16, samples: usize, align: bool) -> usize {
        unsafe {
            av_samples_get_buffer_size(
                ptr::null_mut(),
                i32::from(channels),
                samples as c_int,
                format.into(),
                !align as c_int,
            ) as usize
        }
    }

    #[inline]
    pub fn new(format: Sample, channels: u16, samples: usize, align: bool) -> Self {
        unsafe {
            let mut buf = Buffer {
                format,
                channels,
                samples,
                align,

                buffer: ptr::null_mut(),
                size: 0,
            };

            av_samples_alloc_array_and_samples(
                &mut buf.buffer,
                &mut buf.size,
                i32::from(channels),
                samples as c_int,
                format.into(),
                !align as c_int,
            );

            buf
        }
    }
}

impl Index<usize> for Buffer {
    type Output = [u8];

    #[inline]
    fn index(&self, index: usize) -> &[u8] {
        if index >= self.samples {
            panic!("out of bounds");
        }

        unsafe { slice::from_raw_parts(*self.buffer.add(index), self.size as usize) }
    }
}

impl Clone for Buffer {
    #[inline]
    fn clone(&self) -> Self {
        let mut buf = Buffer::new(self.format, self.channels, self.samples, self.align);
        buf.clone_from(self);

        buf
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        unsafe {
            av_samples_copy(
                self.buffer,
                source.buffer as *const *mut u8,
                0,
                0,
                source.samples as c_int,
                i32::from(source.channels),
                source.format.into(),
            );
        }
    }
}

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            av_freep(self.buffer as *mut c_void);
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/frame/audio.rs">
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

use super::Frame;
use ffi::*;
use libc::{c_int, c_ulonglong};
use util::format;
use ChannelLayout;

#[derive(PartialEq, Eq)]
pub struct Audio(Frame);

impl Audio {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrame) -> Self {
        Audio(Frame::wrap(ptr))
    }

    #[inline]
    pub unsafe fn alloc(&mut self, format: format::Sample, samples: usize, layout: ChannelLayout) {
        self.set_format(format);
        self.set_samples(samples);
        self.set_channel_layout(layout);

        av_frame_get_buffer(self.as_mut_ptr(), 0);
    }
}

impl Audio {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe { Audio(Frame::empty()) }
    }

    #[inline]
    pub fn new(format: format::Sample, samples: usize, layout: ChannelLayout) -> Self {
        unsafe {
            let mut frame = Audio::empty();
            frame.alloc(format, samples, layout);

            frame
        }
    }

    #[inline]
    pub fn format(&self) -> format::Sample {
        unsafe {
            if (*self.as_ptr()).format == -1 {
                format::Sample::None
            } else {
                format::Sample::from(mem::transmute::<_, AVSampleFormat>((*self.as_ptr()).format))
            }
        }
    }

    #[inline]
    pub fn set_format(&mut self, value: format::Sample) {
        unsafe {
            (*self.as_mut_ptr()).format = mem::transmute::<AVSampleFormat, c_int>(value.into());
        }
    }

    #[inline]
    pub fn channel_layout(&self) -> ChannelLayout {
        unsafe { ChannelLayout::from_bits_truncate((*self.as_ptr()).channel_layout as c_ulonglong) }
    }

    #[inline]
    pub fn set_channel_layout(&mut self, value: ChannelLayout) {
        unsafe { (*self.as_mut_ptr()).channel_layout = value.bits() }
    }

    #[inline]
    pub fn channels(&self) -> u16 {
        unsafe { (*self.as_ptr()).channels as u16 }
    }

    #[inline]
    pub fn set_channels(&mut self, value: u16) {
        unsafe {
            (*self.as_mut_ptr()).channels = i32::from(value);
        }
    }

    #[inline]
    pub fn rate(&self) -> u32 {
        unsafe { (*self.as_ptr()).sample_rate as u32 }
    }

    #[inline]
    pub fn set_rate(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).sample_rate = value as c_int;
        }
    }

    #[inline]
    pub fn samples(&self) -> usize {
        unsafe { (*self.as_ptr()).nb_samples as usize }
    }

    #[inline]
    pub fn set_samples(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).nb_samples = value as c_int;
        }
    }

    #[inline]
    pub fn is_planar(&self) -> bool {
        self.format().is_planar()
    }

    #[inline]
    pub fn is_packed(&self) -> bool {
        self.format().is_packed()
    }

    #[inline]
    pub fn planes(&self) -> usize {
        unsafe {
            if (*self.as_ptr()).linesize[0] == 0 {
                return 0;
            }
        }

        if self.is_packed() {
            1
        } else {
            self.channels() as usize
        }
    }

    #[inline]
    pub fn plane<T: Sample>(&self, index: usize) -> &[T] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        if !<T as Sample>::is_valid(self.format(), self.channels()) {
            panic!("unsupported type");
        }

        unsafe { slice::from_raw_parts((*self.as_ptr()).data[index] as *const T, self.samples()) }
    }

    #[inline]
    pub fn plane_mut<T: Sample>(&mut self, index: usize) -> &mut [T] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        if !<T as Sample>::is_valid(self.format(), self.channels()) {
            panic!("unsupported type");
        }

        unsafe {
            slice::from_raw_parts_mut((*self.as_mut_ptr()).data[index] as *mut T, self.samples())
        }
    }

    #[inline]
    pub fn data(&self, index: usize) -> &[u8] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe {
            slice::from_raw_parts(
                (*self.as_ptr()).data[index],
                (*self.as_ptr()).linesize[index] as usize,
            )
        }
    }

    #[inline]
    pub fn data_mut(&mut self, index: usize) -> &mut [u8] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe {
            slice::from_raw_parts_mut(
                (*self.as_mut_ptr()).data[index],
                (*self.as_ptr()).linesize[index] as usize,
            )
        }
    }
}

impl Deref for Audio {
    type Target = Frame;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl ::std::fmt::Debug for Audio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str("ffmpeg::frame::Audio { ")?;
        f.write_str(&format!("format: {:?}, ", self.format()))?;
        f.write_str(&format!("channels: {:?}, ", self.channels()))?;
        f.write_str(&format!("rate: {:?}, ", self.rate()))?;
        f.write_str(&format!("samples: {:?} ", self.samples()))?;
        f.write_str("}")
    }
}

impl Clone for Audio {
    fn clone(&self) -> Self {
        let mut cloned = Audio::new(self.format(), self.samples(), self.channel_layout());
        cloned.clone_from(self);

        cloned
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            av_frame_copy(self.as_mut_ptr(), source.as_ptr());
            av_frame_copy_props(self.as_mut_ptr(), source.as_ptr());
        }
    }
}

impl From<Frame> for Audio {
    fn from(frame: Frame) -> Self {
        Audio(frame)
    }
}

pub unsafe trait Sample {
    fn is_valid(format: format::Sample, channels: u16) -> bool;
}

unsafe impl Sample for u8 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::U8(..))
    }
}

unsafe impl Sample for (u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (u8, u8, u8, u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::U8(format::sample::Type::Packed)
    }
}

unsafe impl Sample for i16 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::I16(..))
    }
}

unsafe impl Sample for (i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i16, i16, i16, i16, i16, i16, i16) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::I16(format::sample::Type::Packed)
    }
}

unsafe impl Sample for i32 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::I32(..))
    }
}

unsafe impl Sample for (i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (i32, i32, i32, i32, i32, i32, i32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::I32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for f32 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::F32(..))
    }
}

unsafe impl Sample for (f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f32, f32, f32, f32, f32, f32, f32) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::F32(format::sample::Type::Packed)
    }
}

unsafe impl Sample for f64 {
    #[inline(always)]
    fn is_valid(format: format::Sample, _channels: u16) -> bool {
        matches!(format, format::Sample::F64(..))
    }
}

unsafe impl Sample for (f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 2 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 3 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 4 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 5 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 6 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}

unsafe impl Sample for (f64, f64, f64, f64, f64, f64, f64) {
    #[inline(always)]
    fn is_valid(format: format::Sample, channels: u16) -> bool {
        channels == 7 && format == format::Sample::F64(format::sample::Type::Packed)
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/frame/flag.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const CORRUPT = AV_FRAME_FLAG_CORRUPT;
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/frame/mod.rs">
pub mod side_data;
pub use self::side_data::SideData;

pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod flag;
pub use self::flag::Flags;

use ffi::*;
use {Dictionary, DictionaryRef};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Packet {
    pub duration: i64,
    pub position: i64,
    pub size: usize,

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub pts: i64,
    pub dts: i64,
}

#[derive(PartialEq, Eq)]
pub struct Frame {
    ptr: *mut AVFrame,

    _own: bool,
}

unsafe impl Send for Frame {}
unsafe impl Sync for Frame {}

impl Frame {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrame) -> Self {
        Frame { ptr, _own: false }
    }

    #[inline(always)]
    pub unsafe fn empty() -> Self {
        Frame {
            ptr: av_frame_alloc(),
            _own: true,
        }
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const AVFrame {
        self.ptr as *const _
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFrame {
        self.ptr
    }

    #[inline(always)]
    pub unsafe fn is_empty(&self) -> bool {
        (*self.as_ptr()).data[0].is_null()
    }
}

impl Frame {
    #[inline]
    pub fn is_key(&self) -> bool {
        unsafe { (*self.as_ptr()).key_frame == 1 }
    }

    #[inline]
    pub fn is_corrupt(&self) -> bool {
        self.flags().contains(Flags::CORRUPT)
    }

    #[inline]
    pub fn packet(&self) -> Packet {
        unsafe {
            Packet {
                duration: (*self.as_ptr()).pkt_duration,
                position: (*self.as_ptr()).pkt_pos,
                size: (*self.as_ptr()).pkt_size as usize,

                #[cfg(not(feature = "ffmpeg_5_0"))]
                pts: (*self.as_ptr()).pkt_pts,
                dts: (*self.as_ptr()).pkt_dts,
            }
        }
    }

    #[inline]
    pub fn pts(&self) -> Option<i64> {
        unsafe {
            match (*self.as_ptr()).pts {
                AV_NOPTS_VALUE => None,
                pts => Some(pts),
            }
        }
    }

    #[inline]
    pub fn set_pts(&mut self, value: Option<i64>) {
        unsafe {
            (*self.as_mut_ptr()).pts = value.unwrap_or(AV_NOPTS_VALUE);
        }
    }

    #[inline]
    pub fn timestamp(&self) -> Option<i64> {
        unsafe {
            match (*self.as_ptr()).best_effort_timestamp {
                AV_NOPTS_VALUE => None,
                t => Some(t),
            }
        }
    }

    #[inline]
    pub fn quality(&self) -> usize {
        unsafe { (*self.as_ptr()).quality as usize }
    }

    #[inline]
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags) }
    }

    #[inline]
    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }

    #[inline]
    pub fn set_metadata(&mut self, value: Dictionary) {
        unsafe { (*self.as_mut_ptr()).metadata = value.disown() }
    }

    #[inline]
    pub fn side_data(&self, kind: side_data::Type) -> Option<SideData> {
        unsafe {
            let ptr = av_frame_get_side_data(self.as_ptr(), kind.into());

            if ptr.is_null() {
                None
            } else {
                Some(SideData::wrap(ptr))
            }
        }
    }

    #[inline]
    pub fn new_side_data(&mut self, kind: side_data::Type, size: usize) -> Option<SideData> {
        unsafe {
            let ptr = av_frame_new_side_data(self.as_mut_ptr(), kind.into(), size as _);

            if ptr.is_null() {
                None
            } else {
                Some(SideData::wrap(ptr))
            }
        }
    }

    #[inline]
    pub fn remove_side_data(&mut self, kind: side_data::Type) {
        unsafe {
            av_frame_remove_side_data(self.as_mut_ptr(), kind.into());
        }
    }
}

impl Drop for Frame {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            av_frame_free(&mut self.as_mut_ptr());
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/frame/side_data.rs">
use std::ffi::CStr;
use std::marker::PhantomData;
use std::slice;
use std::str::from_utf8_unchecked;

use super::Frame;
use ffi::AVFrameSideDataType::*;
use ffi::*;
use DictionaryRef;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
    PanScan,
    A53CC,
    Stereo3D,
    MatrixEncoding,
    DownMixInfo,
    ReplayGain,
    DisplayMatrix,
    AFD,
    MotionVectors,
    SkipSamples,
    AudioServiceType,
    MasteringDisplayMetadata,
    GOPTimecode,
    Spherical,

    ContentLightLevel,
    IccProfile,

    #[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
    QPTableProperties,
    #[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
    QPTableData,

    #[cfg(feature = "ffmpeg_4_1")]
    S12M_TIMECODE,

    #[cfg(feature = "ffmpeg_4_2")]
    DYNAMIC_HDR_PLUS,
    #[cfg(feature = "ffmpeg_4_2")]
    REGIONS_OF_INTEREST,

    #[cfg(feature = "ffmpeg_4_3")]
    VIDEO_ENC_PARAMS,

    #[cfg(feature = "ffmpeg_4_4")]
    SEI_UNREGISTERED,
    #[cfg(feature = "ffmpeg_4_4")]
    FILM_GRAIN_PARAMS,

    #[cfg(feature = "ffmpeg_5_0")]
    DETECTION_BBOXES,
    #[cfg(feature = "ffmpeg_5_0")]
    DOVI_RPU_BUFFER,
    #[cfg(feature = "ffmpeg_5_0")]
    DOVI_METADATA,

    #[cfg(feature = "ffmpeg_5_1")]
    DYNAMIC_HDR_VIVID,

    #[cfg(feature = "ffmpeg_6_0")]
    AMBIENT_VIEWING_ENVIRONMENT,

    #[cfg(feature = "ffmpeg_6_1")]
    VIDEO_HINT,
}

impl Type {
    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr(av_frame_side_data_name((*self).into())).to_bytes())
        }
    }
}

impl From<AVFrameSideDataType> for Type {
    #[inline(always)]
    fn from(value: AVFrameSideDataType) -> Self {
        match value {
            AV_FRAME_DATA_PANSCAN => Type::PanScan,
            AV_FRAME_DATA_A53_CC => Type::A53CC,
            AV_FRAME_DATA_STEREO3D => Type::Stereo3D,
            AV_FRAME_DATA_MATRIXENCODING => Type::MatrixEncoding,
            AV_FRAME_DATA_DOWNMIX_INFO => Type::DownMixInfo,
            AV_FRAME_DATA_REPLAYGAIN => Type::ReplayGain,
            AV_FRAME_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
            AV_FRAME_DATA_AFD => Type::AFD,
            AV_FRAME_DATA_MOTION_VECTORS => Type::MotionVectors,
            AV_FRAME_DATA_SKIP_SAMPLES => Type::SkipSamples,
            AV_FRAME_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV_FRAME_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV_FRAME_DATA_GOP_TIMECODE => Type::GOPTimecode,
            AV_FRAME_DATA_SPHERICAL => Type::Spherical,

            AV_FRAME_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV_FRAME_DATA_ICC_PROFILE => Type::IccProfile,

            #[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
            AV_FRAME_DATA_QP_TABLE_PROPERTIES => Type::QPTableProperties,
            #[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
            AV_FRAME_DATA_QP_TABLE_DATA => Type::QPTableData,
            #[cfg(feature = "ffmpeg_4_1")]
            AV_FRAME_DATA_S12M_TIMECODE => Type::S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_4_2")]
            AV_FRAME_DATA_DYNAMIC_HDR_PLUS => Type::DYNAMIC_HDR_PLUS,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_FRAME_DATA_REGIONS_OF_INTEREST => Type::REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            AV_FRAME_DATA_VIDEO_ENC_PARAMS => Type::VIDEO_ENC_PARAMS,

            #[cfg(feature = "ffmpeg_4_4")]
            AV_FRAME_DATA_SEI_UNREGISTERED => Type::SEI_UNREGISTERED,
            #[cfg(feature = "ffmpeg_4_4")]
            AV_FRAME_DATA_FILM_GRAIN_PARAMS => Type::FILM_GRAIN_PARAMS,

            #[cfg(feature = "ffmpeg_5_0")]
            AV_FRAME_DATA_DETECTION_BBOXES => Type::DETECTION_BBOXES,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_FRAME_DATA_DOVI_RPU_BUFFER => Type::DOVI_RPU_BUFFER,
            #[cfg(feature = "ffmpeg_5_0")]
            AV_FRAME_DATA_DOVI_METADATA => Type::DOVI_METADATA,

            #[cfg(feature = "ffmpeg_5_1")]
            AV_FRAME_DATA_DYNAMIC_HDR_VIVID => Type::DYNAMIC_HDR_VIVID,

            #[cfg(feature = "ffmpeg_6_0")]
            AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT => Type::AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_6_1")]
            AV_FRAME_DATA_VIDEO_HINT => Type::VIDEO_HINT,

            // FFmpeg 7.x added new variants not covered by ffmpeg-next 6.1
            #[allow(unreachable_patterns)]
            _ => Type::PanScan,
        }
    }
}

impl From<Type> for AVFrameSideDataType {
    #[inline(always)]
    fn from(value: Type) -> AVFrameSideDataType {
        match value {
            Type::PanScan => AV_FRAME_DATA_PANSCAN,
            Type::A53CC => AV_FRAME_DATA_A53_CC,
            Type::Stereo3D => AV_FRAME_DATA_STEREO3D,
            Type::MatrixEncoding => AV_FRAME_DATA_MATRIXENCODING,
            Type::DownMixInfo => AV_FRAME_DATA_DOWNMIX_INFO,
            Type::ReplayGain => AV_FRAME_DATA_REPLAYGAIN,
            Type::DisplayMatrix => AV_FRAME_DATA_DISPLAYMATRIX,
            Type::AFD => AV_FRAME_DATA_AFD,
            Type::MotionVectors => AV_FRAME_DATA_MOTION_VECTORS,
            Type::SkipSamples => AV_FRAME_DATA_SKIP_SAMPLES,
            Type::AudioServiceType => AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
            Type::MasteringDisplayMetadata => AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
            Type::GOPTimecode => AV_FRAME_DATA_GOP_TIMECODE,
            Type::Spherical => AV_FRAME_DATA_SPHERICAL,

            Type::ContentLightLevel => AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
            Type::IccProfile => AV_FRAME_DATA_ICC_PROFILE,

            #[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
            Type::QPTableProperties => AV_FRAME_DATA_QP_TABLE_PROPERTIES,
            #[cfg(all(feature = "ffmpeg_4_0", not(feature = "ffmpeg_5_0")))]
            Type::QPTableData => AV_FRAME_DATA_QP_TABLE_DATA,
            #[cfg(feature = "ffmpeg_4_1")]
            Type::S12M_TIMECODE => AV_FRAME_DATA_S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_4_2")]
            Type::DYNAMIC_HDR_PLUS => AV_FRAME_DATA_DYNAMIC_HDR_PLUS,
            #[cfg(feature = "ffmpeg_4_2")]
            Type::REGIONS_OF_INTEREST => AV_FRAME_DATA_REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::VIDEO_ENC_PARAMS => AV_FRAME_DATA_VIDEO_ENC_PARAMS,

            #[cfg(feature = "ffmpeg_4_4")]
            Type::SEI_UNREGISTERED => AV_FRAME_DATA_SEI_UNREGISTERED,
            #[cfg(feature = "ffmpeg_4_4")]
            Type::FILM_GRAIN_PARAMS => AV_FRAME_DATA_FILM_GRAIN_PARAMS,

            #[cfg(feature = "ffmpeg_5_0")]
            Type::DETECTION_BBOXES => AV_FRAME_DATA_DETECTION_BBOXES,
            #[cfg(feature = "ffmpeg_5_0")]
            Type::DOVI_RPU_BUFFER => AV_FRAME_DATA_DOVI_RPU_BUFFER,
            #[cfg(feature = "ffmpeg_5_0")]
            Type::DOVI_METADATA => AV_FRAME_DATA_DOVI_METADATA,

            #[cfg(feature = "ffmpeg_5_1")]
            Type::DYNAMIC_HDR_VIVID => AV_FRAME_DATA_DYNAMIC_HDR_VIVID,

            #[cfg(feature = "ffmpeg_6_0")]
            Type::AMBIENT_VIEWING_ENVIRONMENT => AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT,

            #[cfg(feature = "ffmpeg_6_1")]
            Type::VIDEO_HINT => AV_FRAME_DATA_VIDEO_HINT,
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVFrameSideData,

    _marker: PhantomData<&'a Frame>,
}

impl<'a> SideData<'a> {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrameSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const AVFrameSideData {
        self.ptr as *const _
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFrameSideData {
        self.ptr
    }
}

impl<'a> SideData<'a> {
    #[inline]
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        #[allow(clippy::unnecessary_cast)]
        unsafe {
            slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize)
        }
    }

    #[inline]
    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/frame/video.rs">
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

use super::Frame;
use color;
use ffi::*;
use libc::c_int;
use picture;
use util::chroma;
use util::format;
use Rational;

#[derive(PartialEq, Eq)]
pub struct Video(Frame);

impl Video {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrame) -> Self {
        Video(Frame::wrap(ptr))
    }

    #[inline]
    pub unsafe fn alloc(&mut self, format: format::Pixel, width: u32, height: u32) {
        self.set_format(format);
        self.set_width(width);
        self.set_height(height);

        av_frame_get_buffer(self.as_mut_ptr(), 32);
    }
}

impl Video {
    #[inline(always)]
    pub fn empty() -> Self {
        unsafe { Video(Frame::empty()) }
    }

    #[inline]
    pub fn new(format: format::Pixel, width: u32, height: u32) -> Self {
        unsafe {
            let mut frame = Video::empty();
            frame.alloc(format, width, height);

            frame
        }
    }

    #[inline]
    pub fn format(&self) -> format::Pixel {
        unsafe {
            if (*self.as_ptr()).format == -1 {
                format::Pixel::None
            } else {
                format::Pixel::from(mem::transmute::<_, AVPixelFormat>((*self.as_ptr()).format))
            }
        }
    }

    #[inline]
    pub fn set_format(&mut self, value: format::Pixel) {
        unsafe {
            (*self.as_mut_ptr()).format = mem::transmute::<AVPixelFormat, c_int>(value.into());
        }
    }

    #[inline]
    pub fn kind(&self) -> picture::Type {
        unsafe { picture::Type::from((*self.as_ptr()).pict_type) }
    }

    #[inline]
    pub fn set_kind(&mut self, value: picture::Type) {
        unsafe {
            (*self.as_mut_ptr()).pict_type = value.into();
        }
    }

    #[inline]
    pub fn is_interlaced(&self) -> bool {
        unsafe { (*self.as_ptr()).interlaced_frame != 0 }
    }

    #[inline]
    pub fn is_top_first(&self) -> bool {
        unsafe { (*self.as_ptr()).top_field_first != 0 }
    }

    #[inline]
    pub fn has_palette_changed(&self) -> bool {
        unsafe { (*self.as_ptr()).palette_has_changed != 0 }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        unsafe { (*self.as_ptr()).width as u32 }
    }

    #[inline]
    pub fn set_width(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).width = value as c_int;
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        unsafe { (*self.as_ptr()).height as u32 }
    }

    #[inline]
    pub fn set_height(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).height = value as c_int;
        }
    }

    #[inline]
    pub fn color_space(&self) -> color::Space {
        unsafe { color::Space::from((*self.as_ptr()).colorspace) }
    }

    #[inline]
    pub fn set_color_space(&mut self, value: color::Space) {
        unsafe {
            (*self.as_mut_ptr()).colorspace = value.into();
        }
    }

    #[inline]
    pub fn color_range(&self) -> color::Range {
        unsafe { color::Range::from((*self.as_ptr()).color_range) }
    }

    #[inline]
    pub fn set_color_range(&mut self, value: color::Range) {
        unsafe {
            (*self.as_mut_ptr()).color_range = value.into();
        }
    }

    #[inline]
    pub fn color_primaries(&self) -> color::Primaries {
        unsafe { color::Primaries::from((*self.as_ptr()).color_primaries) }
    }

    #[inline]
    pub fn set_color_primaries(&mut self, value: color::Primaries) {
        unsafe {
            (*self.as_mut_ptr()).color_primaries = value.into();
        }
    }

    #[inline]
    pub fn color_transfer_characteristic(&self) -> color::TransferCharacteristic {
        unsafe { color::TransferCharacteristic::from((*self.as_ptr()).color_trc) }
    }

    #[inline]
    pub fn set_color_transfer_characteristic(&mut self, value: color::TransferCharacteristic) {
        unsafe {
            (*self.as_mut_ptr()).color_trc = value.into();
        }
    }

    #[inline]
    pub fn chroma_location(&self) -> chroma::Location {
        unsafe { chroma::Location::from((*self.as_ptr()).chroma_location) }
    }

    #[inline]
    pub fn aspect_ratio(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).sample_aspect_ratio) }
    }

    #[inline]
    pub fn coded_number(&self) -> usize {
        unsafe { (*self.as_ptr()).coded_picture_number as usize }
    }

    #[inline]
    pub fn display_number(&self) -> usize {
        unsafe { (*self.as_ptr()).display_picture_number as usize }
    }

    #[inline]
    pub fn repeat(&self) -> f64 {
        unsafe { f64::from((*self.as_ptr()).repeat_pict) }
    }

    #[inline]
    pub fn stride(&self, index: usize) -> usize {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe { (*self.as_ptr()).linesize[index] as usize }
    }

    #[inline]
    pub fn planes(&self) -> usize {
        for i in 0..8 {
            unsafe {
                if (*self.as_ptr()).linesize[i] == 0 {
                    return i;
                }
            }
        }

        8
    }

    #[inline]
    pub fn plane_width(&self, index: usize) -> u32 {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        // Logic taken from image_get_linesize().
        if index != 1 && index != 2 {
            return self.width();
        }

        if let Some(desc) = self.format().descriptor() {
            let s = desc.log2_chroma_w();
            (self.width() + (1 << s) - 1) >> s
        } else {
            self.width()
        }
    }

    #[inline]
    pub fn plane_height(&self, index: usize) -> u32 {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        // Logic taken from av_image_fill_pointers().
        if index != 1 && index != 2 {
            return self.height();
        }

        if let Some(desc) = self.format().descriptor() {
            let s = desc.log2_chroma_h();
            (self.height() + (1 << s) - 1) >> s
        } else {
            self.height()
        }
    }

    #[inline]
    pub fn plane<T: Component>(&self, index: usize) -> &[T] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        if !<T as Component>::is_valid(self.format()) {
            panic!("unsupported type");
        }

        unsafe {
            slice::from_raw_parts(
                (*self.as_ptr()).data[index] as *const T,
                self.stride(index) * self.plane_height(index) as usize / mem::size_of::<T>(),
            )
        }
    }

    #[inline]
    pub fn plane_mut<T: Component>(&mut self, index: usize) -> &mut [T] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        if !<T as Component>::is_valid(self.format()) {
            panic!("unsupported type");
        }

        unsafe {
            slice::from_raw_parts_mut(
                (*self.as_mut_ptr()).data[index] as *mut T,
                self.stride(index) * self.plane_height(index) as usize / mem::size_of::<T>(),
            )
        }
    }

    #[inline]
    pub fn data(&self, index: usize) -> &[u8] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe {
            slice::from_raw_parts(
                (*self.as_ptr()).data[index],
                self.stride(index) * self.plane_height(index) as usize,
            )
        }
    }

    #[inline]
    pub fn data_mut(&mut self, index: usize) -> &mut [u8] {
        if index >= self.planes() {
            panic!("out of bounds");
        }

        unsafe {
            slice::from_raw_parts_mut(
                (*self.as_mut_ptr()).data[index],
                self.stride(index) * self.plane_height(index) as usize,
            )
        }
    }
}

impl Deref for Video {
    type Target = Frame;

    #[inline]
    fn deref(&self) -> &Frame {
        &self.0
    }
}

impl DerefMut for Video {
    #[inline]
    fn deref_mut(&mut self) -> &mut Frame {
        &mut self.0
    }
}

impl Clone for Video {
    #[inline]
    fn clone(&self) -> Self {
        let mut cloned = Video::new(self.format(), self.width(), self.height());
        cloned.clone_from(self);

        cloned
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        unsafe {
            av_frame_copy(self.as_mut_ptr(), source.as_ptr());
            av_frame_copy_props(self.as_mut_ptr(), source.as_ptr());
        }
    }
}

impl From<Frame> for Video {
    #[inline]
    fn from(frame: Frame) -> Self {
        Video(frame)
    }
}

pub unsafe trait Component {
    fn is_valid(format: format::Pixel) -> bool;
}

#[cfg(feature = "image")]
unsafe impl Component for ::image::Luma<u8> {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::GRAY8
    }
}

#[cfg(feature = "image")]
unsafe impl Component for ::image::Rgb<u8> {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGB24
    }
}

#[cfg(feature = "image")]
unsafe impl Component for ::image::Rgba<u8> {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGBA
    }
}

unsafe impl Component for [u8; 3] {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGB24 || format == format::Pixel::BGR24
    }
}

unsafe impl Component for (u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGB24 || format == format::Pixel::BGR24
    }
}

unsafe impl Component for [u8; 4] {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGBA
            || format == format::Pixel::BGRA
            || format == format::Pixel::ARGB
            || format == format::Pixel::ABGR
            || format == format::Pixel::RGBZ
            || format == format::Pixel::BGRZ
            || format == format::Pixel::ZRGB
            || format == format::Pixel::ZBGR
    }
}

unsafe impl Component for (u8, u8, u8, u8) {
    #[inline(always)]
    fn is_valid(format: format::Pixel) -> bool {
        format == format::Pixel::RGBA
            || format == format::Pixel::BGRA
            || format == format::Pixel::ARGB
            || format == format::Pixel::ABGR
            || format == format::Pixel::RGBZ
            || format == format::Pixel::BGRZ
            || format == format::Pixel::ZRGB
            || format == format::Pixel::ZBGR
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/log/flag.rs">
use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        const SKIP_REPEATED = AV_LOG_SKIP_REPEATED;
        const PRINT_LEVEL = AV_LOG_PRINT_LEVEL;
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/log/level.rs">
use std::convert::TryFrom;

use ffi::*;
use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Level {
    Quiet,
    Panic,
    Fatal,
    Error,
    Warning,
    Info,
    Verbose,
    Debug,
    Trace,
}

pub struct LevelError;

impl TryFrom<c_int> for Level {
    type Error = &'static str;

    fn try_from(value: c_int) -> Result<Self, &'static str> {
        match value {
            AV_LOG_QUIET => Ok(Level::Quiet),
            AV_LOG_PANIC => Ok(Level::Panic),
            AV_LOG_FATAL => Ok(Level::Fatal),
            AV_LOG_ERROR => Ok(Level::Error),
            AV_LOG_WARNING => Ok(Level::Warning),
            AV_LOG_INFO => Ok(Level::Info),
            AV_LOG_VERBOSE => Ok(Level::Verbose),
            AV_LOG_DEBUG => Ok(Level::Debug),
            AV_LOG_TRACE => Ok(Level::Trace),
            _ => Err("illegal log level"),
        }
    }
}

impl From<Level> for c_int {
    fn from(value: Level) -> c_int {
        match value {
            Level::Quiet => AV_LOG_QUIET,
            Level::Panic => AV_LOG_PANIC,
            Level::Fatal => AV_LOG_FATAL,
            Level::Error => AV_LOG_ERROR,
            Level::Warning => AV_LOG_WARNING,
            Level::Info => AV_LOG_INFO,
            Level::Verbose => AV_LOG_VERBOSE,
            Level::Debug => AV_LOG_DEBUG,
            Level::Trace => AV_LOG_TRACE,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/log/mod.rs">
pub mod level;
pub use self::level::Level;

pub mod flag;
pub use self::flag::Flags;

use ffi::*;
use std::convert::TryInto;

pub fn set_level(value: Level) {
    unsafe { av_log_set_level(value.into()) }
}

pub fn get_level() -> Result<Level, &'static str> {
    unsafe { av_log_get_level().try_into() }
}

pub fn set_flags(value: Flags) {
    unsafe { av_log_set_flags(value.bits()) }
}

pub fn get_flags() -> Flags {
    unsafe { Flags::from_bits_truncate(av_log_get_flags()) }
}
</file>

<file path="patches/ffmpeg-next/src/util/mathematics/mod.rs">
pub mod rounding;
pub use self::rounding::Rounding;

pub mod rescale;
pub use self::rescale::Rescale;
</file>

<file path="patches/ffmpeg-next/src/util/mathematics/rescale.rs">
use ffi::*;
use {Rational, Rounding};

pub const TIME_BASE: Rational = Rational(AV_TIME_BASE_Q.num, AV_TIME_BASE_Q.den);

pub trait Rescale {
    fn rescale<S, D>(&self, source: S, destination: D) -> i64
    where
        S: Into<Rational>,
        D: Into<Rational>;

    fn rescale_with<S, D>(&self, source: S, destination: D, rounding: Rounding) -> i64
    where
        S: Into<Rational>,
        D: Into<Rational>;
}

impl<T: Into<i64> + Clone> Rescale for T {
    fn rescale<S, D>(&self, source: S, destination: D) -> i64
    where
        S: Into<Rational>,
        D: Into<Rational>,
    {
        unsafe {
            av_rescale_q(
                self.clone().into(),
                source.into().into(),
                destination.into().into(),
            )
        }
    }

    fn rescale_with<S, D>(&self, source: S, destination: D, rounding: Rounding) -> i64
    where
        S: Into<Rational>,
        D: Into<Rational>,
    {
        unsafe {
            av_rescale_q_rnd(
                self.clone().into(),
                source.into().into(),
                destination.into().into(),
                rounding.into(),
            )
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/mathematics/rounding.rs">
use ffi::AVRounding::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Rounding {
    Zero,
    Infinity,
    Down,
    Up,
    NearInfinity,
    PassMinMax,
}

impl From<AVRounding> for Rounding {
    #[inline(always)]
    fn from(value: AVRounding) -> Self {
        match value {
            AV_ROUND_ZERO => Rounding::Zero,
            AV_ROUND_INF => Rounding::Infinity,
            AV_ROUND_DOWN => Rounding::Down,
            AV_ROUND_UP => Rounding::Up,
            AV_ROUND_NEAR_INF => Rounding::NearInfinity,
            AV_ROUND_PASS_MINMAX => Rounding::PassMinMax,
        }
    }
}

impl From<Rounding> for AVRounding {
    #[inline(always)]
    fn from(value: Rounding) -> AVRounding {
        match value {
            Rounding::Zero => AV_ROUND_ZERO,
            Rounding::Infinity => AV_ROUND_INF,
            Rounding::Down => AV_ROUND_DOWN,
            Rounding::Up => AV_ROUND_UP,
            Rounding::NearInfinity => AV_ROUND_NEAR_INF,
            Rounding::PassMinMax => AV_ROUND_PASS_MINMAX,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/option/mod.rs">
mod traits;
pub use self::traits::{Gettable, Iterable, Settable, Target};

use ffi::AVOptionType::*;
use ffi::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Type {
    Flags,
    Int,
    Int64,
    Double,
    Float,
    String,
    Rational,
    Binary,
    Dictionary,
    Constant,

    ImageSize,
    PixelFormat,
    SampleFormat,
    VideoRate,
    Duration,
    Color,
    ChannelLayout,
    c_ulong,
    bool,
}

impl From<AVOptionType> for Type {
    fn from(value: AVOptionType) -> Self {
        match value {
            AV_OPT_TYPE_FLAGS => Type::Flags,
            AV_OPT_TYPE_INT => Type::Int,
            AV_OPT_TYPE_INT64 => Type::Int64,
            AV_OPT_TYPE_DOUBLE => Type::Double,
            AV_OPT_TYPE_FLOAT => Type::Float,
            AV_OPT_TYPE_STRING => Type::String,
            AV_OPT_TYPE_RATIONAL => Type::Rational,
            AV_OPT_TYPE_BINARY => Type::Binary,
            AV_OPT_TYPE_DICT => Type::Dictionary,
            AV_OPT_TYPE_CONST => Type::Constant,
            AV_OPT_TYPE_UINT64 => Type::c_ulong,
            AV_OPT_TYPE_BOOL => Type::bool,

            AV_OPT_TYPE_IMAGE_SIZE => Type::ImageSize,
            AV_OPT_TYPE_PIXEL_FMT => Type::PixelFormat,
            AV_OPT_TYPE_SAMPLE_FMT => Type::SampleFormat,
            AV_OPT_TYPE_VIDEO_RATE => Type::VideoRate,
            AV_OPT_TYPE_DURATION => Type::Duration,
            AV_OPT_TYPE_COLOR => Type::Color,
            AV_OPT_TYPE_CHANNEL_LAYOUT => Type::ChannelLayout,
            #[cfg(feature = "ffmpeg_5_1")]
            AV_OPT_TYPE_CHLAYOUT => Type::ChannelLayout,
        }
    }
}

impl From<Type> for AVOptionType {
    fn from(value: Type) -> AVOptionType {
        match value {
            Type::Flags => AV_OPT_TYPE_FLAGS,
            Type::Int => AV_OPT_TYPE_INT,
            Type::Int64 => AV_OPT_TYPE_INT64,
            Type::Double => AV_OPT_TYPE_DOUBLE,
            Type::Float => AV_OPT_TYPE_FLOAT,
            Type::String => AV_OPT_TYPE_STRING,
            Type::Rational => AV_OPT_TYPE_RATIONAL,
            Type::Binary => AV_OPT_TYPE_BINARY,
            Type::Dictionary => AV_OPT_TYPE_DICT,
            Type::Constant => AV_OPT_TYPE_CONST,
            Type::c_ulong => AV_OPT_TYPE_UINT64,
            Type::bool => AV_OPT_TYPE_BOOL,

            Type::ImageSize => AV_OPT_TYPE_IMAGE_SIZE,
            Type::PixelFormat => AV_OPT_TYPE_PIXEL_FMT,
            Type::SampleFormat => AV_OPT_TYPE_SAMPLE_FMT,
            Type::VideoRate => AV_OPT_TYPE_VIDEO_RATE,
            Type::Duration => AV_OPT_TYPE_DURATION,
            Type::Color => AV_OPT_TYPE_COLOR,
            Type::ChannelLayout => AV_OPT_TYPE_CHANNEL_LAYOUT,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/option/traits.rs">
//! NOTE: this will be much better once specialization comes

use std::ffi::CString;
use std::mem;

use ffi::*;
use libc::{c_int, c_void};
use util::format;
use {ChannelLayout, Error, Rational};

macro_rules! check {
    ($expr:expr) => {
        match $expr {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    };
}

pub unsafe trait Target {
    fn as_ptr(&self) -> *const c_void;
    fn as_mut_ptr(&mut self) -> *mut c_void;
}

pub trait Settable: Target {
    fn set<T: 'static>(&mut self, name: &str, value: &T) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_bin(
                self.as_mut_ptr(),
                name.as_ptr(),
                value as *const _ as *const _,
                mem::size_of::<T>() as c_int,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();
            let value = CString::new(value).unwrap();

            check!(av_opt_set(
                self.as_mut_ptr(),
                name.as_ptr(),
                value.as_ptr(),
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_int(&mut self, name: &str, value: i64) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_int(
                self.as_mut_ptr(),
                name.as_ptr(),
                value,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_double(&mut self, name: &str, value: f64) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_double(
                self.as_mut_ptr(),
                name.as_ptr(),
                value,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_rational<T: Into<Rational>>(&mut self, name: &str, value: T) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_q(
                self.as_mut_ptr(),
                name.as_ptr(),
                value.into().into(),
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_image_size(&mut self, name: &str, w: u32, h: u32) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_image_size(
                self.as_mut_ptr(),
                name.as_ptr(),
                w as c_int,
                h as c_int,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_pixel_format(&mut self, name: &str, format: format::Pixel) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_pixel_fmt(
                self.as_mut_ptr(),
                name.as_ptr(),
                format.into(),
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_sample_format(&mut self, name: &str, format: format::Sample) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_sample_fmt(
                self.as_mut_ptr(),
                name.as_ptr(),
                format.into(),
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }

    fn set_channel_layout(&mut self, name: &str, layout: ChannelLayout) -> Result<(), Error> {
        unsafe {
            let name = CString::new(name).unwrap();

            check!(av_opt_set_channel_layout(
                self.as_mut_ptr(),
                name.as_ptr(),
                layout.bits() as i64,
                AV_OPT_SEARCH_CHILDREN
            ))
        }
    }
}

pub trait Gettable: Target {}

pub trait Iterable: Target {}
</file>

<file path="patches/ffmpeg-next/src/util/channel_layout.rs">
use ffi::*;
use libc::c_ulonglong;

bitflags! {
    pub struct ChannelLayout: c_ulonglong {
        const FRONT_LEFT            = AV_CH_FRONT_LEFT;
        const FRONT_RIGHT           = AV_CH_FRONT_RIGHT;
        const FRONT_CENTER          = AV_CH_FRONT_CENTER;
        const LOW_FREQUENCY         = AV_CH_LOW_FREQUENCY;
        const BACK_LEFT             = AV_CH_BACK_LEFT;
        const BACK_RIGHT            = AV_CH_BACK_RIGHT;
        const FRONT_LEFT_OF_CENTER  = AV_CH_FRONT_LEFT_OF_CENTER;
        const FRONT_RIGHT_OF_CENTER = AV_CH_FRONT_RIGHT_OF_CENTER;
        const BACK_CENTER           = AV_CH_BACK_CENTER;
        const SIDE_LEFT             = AV_CH_SIDE_LEFT;
        const SIDE_RIGHT            = AV_CH_SIDE_RIGHT;
        const TOP_CENTER            = AV_CH_TOP_CENTER;
        const TOP_FRONT_LEFT        = AV_CH_TOP_FRONT_LEFT;
        const TOP_FRONT_CENTER      = AV_CH_TOP_FRONT_CENTER;
        const TOP_FRONT_RIGHT       = AV_CH_TOP_FRONT_RIGHT;
        const TOP_BACK_LEFT         = AV_CH_TOP_BACK_LEFT;
        const TOP_BACK_CENTER       = AV_CH_TOP_BACK_CENTER;
        const TOP_BACK_RIGHT        = AV_CH_TOP_BACK_RIGHT;
        const STEREO_LEFT           = AV_CH_STEREO_LEFT;
        const STEREO_RIGHT          = AV_CH_STEREO_RIGHT;
        const WIDE_LEFT             = AV_CH_WIDE_LEFT;
        const WIDE_RIGHT            = AV_CH_WIDE_RIGHT;
        const SURROUND_DIRECT_LEFT  = AV_CH_SURROUND_DIRECT_LEFT;
        const SURROUND_DIRECT_RIGHT = AV_CH_SURROUND_DIRECT_RIGHT;
        const LOW_FREQUENCY_2       = AV_CH_LOW_FREQUENCY_2;
        const NATIVE                = AV_CH_LAYOUT_NATIVE;

        const MONO               = AV_CH_LAYOUT_MONO;
        const STEREO             = AV_CH_LAYOUT_STEREO;
        const _2POINT1           = AV_CH_LAYOUT_2POINT1;
        const _2_1               = AV_CH_LAYOUT_2_1;
        const SURROUND           = AV_CH_LAYOUT_SURROUND;
        const _3POINT1           = AV_CH_LAYOUT_3POINT1;
        const _4POINT0           = AV_CH_LAYOUT_4POINT0;
        const _4POINT1           = AV_CH_LAYOUT_4POINT1;
        const _2_2               = AV_CH_LAYOUT_2_2;
        const QUAD               = AV_CH_LAYOUT_QUAD;
        const _5POINT0           = AV_CH_LAYOUT_5POINT0;
        const _5POINT1           = AV_CH_LAYOUT_5POINT1;
        const _5POINT0_BACK      = AV_CH_LAYOUT_5POINT0_BACK;
        const _5POINT1_BACK      = AV_CH_LAYOUT_5POINT1_BACK;
        const _6POINT0           = AV_CH_LAYOUT_6POINT0;
        const _6POINT0_FRONT     = AV_CH_LAYOUT_6POINT0_FRONT;
        const HEXAGONAL          = AV_CH_LAYOUT_HEXAGONAL;
        const _6POINT1           = AV_CH_LAYOUT_6POINT1;
        const _6POINT1_BACK      = AV_CH_LAYOUT_6POINT1_BACK;
        const _6POINT1_FRONT     = AV_CH_LAYOUT_6POINT1_FRONT;
        const _7POINT0           = AV_CH_LAYOUT_7POINT0;
        const _7POINT0_FRONT     = AV_CH_LAYOUT_7POINT0_FRONT;
        const _7POINT1           = AV_CH_LAYOUT_7POINT1;
        const _7POINT1_WIDE      = AV_CH_LAYOUT_7POINT1_WIDE;
        const _7POINT1_WIDE_BACK = AV_CH_LAYOUT_7POINT1_WIDE_BACK;
        const OCTAGONAL          = AV_CH_LAYOUT_OCTAGONAL;
        const HEXADECAGONAL      = AV_CH_LAYOUT_HEXADECAGONAL;
        const STEREO_DOWNMIX     = AV_CH_LAYOUT_STEREO_DOWNMIX;

        #[cfg(feature = "ffmpeg_6_1")]
        const _3POINT1POINT2      = AV_CH_LAYOUT_3POINT1POINT2;
        #[cfg(feature = "ffmpeg_6_1")]
        const _5POINT1POINT2_BACK = AV_CH_LAYOUT_5POINT1POINT2_BACK;
        #[cfg(feature = "ffmpeg_6_1")]
        const _5POINT1POINT4_BACK = AV_CH_LAYOUT_5POINT1POINT4_BACK;
        #[cfg(feature = "ffmpeg_6_1")]
        const _7POINT1POINT2      = AV_CH_LAYOUT_7POINT1POINT2;
        #[cfg(feature = "ffmpeg_6_1")]
        const _7POINT1POINT4_BACK = AV_CH_LAYOUT_7POINT1POINT4_BACK;
    }
}

impl ChannelLayout {
    #[inline]
    pub fn channels(&self) -> i32 {
        unsafe { av_get_channel_layout_nb_channels(self.bits()) }
    }

    pub fn default(number: i32) -> ChannelLayout {
        unsafe {
            ChannelLayout::from_bits_truncate(av_get_default_channel_layout(number) as c_ulonglong)
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/error.rs">
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::io;
use std::str::from_utf8_unchecked;

use ffi::*;
use libc::{c_char, c_int};

// Export POSIX error codes so that users can do something like
//
//   if error == (Error::Other { errno: EAGAIN }) {
//       ...
//   }
pub use libc::{
    E2BIG, EACCES, EADDRINUSE, EADDRNOTAVAIL, EAFNOSUPPORT, EAGAIN, EALREADY, EBADF, EBADMSG,
    EBUSY, ECANCELED, ECHILD, ECONNABORTED, ECONNREFUSED, ECONNRESET, EDEADLK, EDESTADDRREQ, EDOM,
    EEXIST, EFAULT, EFBIG, EHOSTUNREACH, EIDRM, EILSEQ, EINPROGRESS, EINTR, EINVAL, EIO, EISCONN,
    EISDIR, ELOOP, EMFILE, EMLINK, EMSGSIZE, ENAMETOOLONG, ENETDOWN, ENETRESET, ENETUNREACH,
    ENFILE, ENOBUFS, ENODATA, ENODEV, ENOENT, ENOEXEC, ENOLCK, ENOLINK, ENOMEM, ENOMSG,
    ENOPROTOOPT, ENOSPC, ENOSR, ENOSTR, ENOSYS, ENOTCONN, ENOTDIR, ENOTEMPTY, ENOTRECOVERABLE,
    ENOTSOCK, ENOTSUP, ENOTTY, ENXIO, EOPNOTSUPP, EOVERFLOW, EOWNERDEAD, EPERM, EPIPE, EPROTO,
    EPROTONOSUPPORT, EPROTOTYPE, ERANGE, EROFS, ESPIPE, ESRCH, ETIME, ETIMEDOUT, ETXTBSY,
    EWOULDBLOCK, EXDEV,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Error {
    Bug,
    Bug2,
    Unknown,
    Experimental,
    BufferTooSmall,
    Eof,
    Exit,
    External,
    InvalidData,
    PatchWelcome,

    InputChanged,
    OutputChanged,

    BsfNotFound,
    DecoderNotFound,
    DemuxerNotFound,
    EncoderNotFound,
    OptionNotFound,
    MuxerNotFound,
    FilterNotFound,
    ProtocolNotFound,
    StreamNotFound,

    HttpBadRequest,
    HttpUnauthorized,
    HttpForbidden,
    HttpNotFound,
    HttpOther4xx,
    HttpServerError,

    /// For AVERROR(e) wrapping POSIX error codes, e.g. AVERROR(EAGAIN).
    Other {
        errno: c_int,
    },
}

impl From<c_int> for Error {
    fn from(value: c_int) -> Error {
        match value {
            AVERROR_BSF_NOT_FOUND => Error::BsfNotFound,
            AVERROR_BUG => Error::Bug,
            AVERROR_BUFFER_TOO_SMALL => Error::BufferTooSmall,
            AVERROR_DECODER_NOT_FOUND => Error::DecoderNotFound,
            AVERROR_DEMUXER_NOT_FOUND => Error::DemuxerNotFound,
            AVERROR_ENCODER_NOT_FOUND => Error::EncoderNotFound,
            AVERROR_EOF => Error::Eof,
            AVERROR_EXIT => Error::Exit,
            AVERROR_EXTERNAL => Error::External,
            AVERROR_FILTER_NOT_FOUND => Error::FilterNotFound,
            AVERROR_INVALIDDATA => Error::InvalidData,
            AVERROR_MUXER_NOT_FOUND => Error::MuxerNotFound,
            AVERROR_OPTION_NOT_FOUND => Error::OptionNotFound,
            AVERROR_PATCHWELCOME => Error::PatchWelcome,
            AVERROR_PROTOCOL_NOT_FOUND => Error::ProtocolNotFound,
            AVERROR_STREAM_NOT_FOUND => Error::StreamNotFound,
            AVERROR_BUG2 => Error::Bug2,
            AVERROR_UNKNOWN => Error::Unknown,
            AVERROR_EXPERIMENTAL => Error::Experimental,
            AVERROR_INPUT_CHANGED => Error::InputChanged,
            AVERROR_OUTPUT_CHANGED => Error::OutputChanged,
            AVERROR_HTTP_BAD_REQUEST => Error::HttpBadRequest,
            AVERROR_HTTP_UNAUTHORIZED => Error::HttpUnauthorized,
            AVERROR_HTTP_FORBIDDEN => Error::HttpForbidden,
            AVERROR_HTTP_NOT_FOUND => Error::HttpNotFound,
            AVERROR_HTTP_OTHER_4XX => Error::HttpOther4xx,
            AVERROR_HTTP_SERVER_ERROR => Error::HttpServerError,
            e => Error::Other {
                errno: AVUNERROR(e),
            },
        }
    }
}

impl From<Error> for c_int {
    fn from(value: Error) -> c_int {
        match value {
            Error::BsfNotFound => AVERROR_BSF_NOT_FOUND,
            Error::Bug => AVERROR_BUG,
            Error::BufferTooSmall => AVERROR_BUFFER_TOO_SMALL,
            Error::DecoderNotFound => AVERROR_DECODER_NOT_FOUND,
            Error::DemuxerNotFound => AVERROR_DEMUXER_NOT_FOUND,
            Error::EncoderNotFound => AVERROR_ENCODER_NOT_FOUND,
            Error::Eof => AVERROR_EOF,
            Error::Exit => AVERROR_EXIT,
            Error::External => AVERROR_EXTERNAL,
            Error::FilterNotFound => AVERROR_FILTER_NOT_FOUND,
            Error::InvalidData => AVERROR_INVALIDDATA,
            Error::MuxerNotFound => AVERROR_MUXER_NOT_FOUND,
            Error::OptionNotFound => AVERROR_OPTION_NOT_FOUND,
            Error::PatchWelcome => AVERROR_PATCHWELCOME,
            Error::ProtocolNotFound => AVERROR_PROTOCOL_NOT_FOUND,
            Error::StreamNotFound => AVERROR_STREAM_NOT_FOUND,
            Error::Bug2 => AVERROR_BUG2,
            Error::Unknown => AVERROR_UNKNOWN,
            Error::Experimental => AVERROR_EXPERIMENTAL,
            Error::InputChanged => AVERROR_INPUT_CHANGED,
            Error::OutputChanged => AVERROR_OUTPUT_CHANGED,
            Error::HttpBadRequest => AVERROR_HTTP_BAD_REQUEST,
            Error::HttpUnauthorized => AVERROR_HTTP_UNAUTHORIZED,
            Error::HttpForbidden => AVERROR_HTTP_FORBIDDEN,
            Error::HttpNotFound => AVERROR_HTTP_NOT_FOUND,
            Error::HttpOther4xx => AVERROR_HTTP_OTHER_4XX,
            Error::HttpServerError => AVERROR_HTTP_SERVER_ERROR,
            Error::Other { errno } => AVERROR(errno),
        }
    }
}

impl error::Error for Error {}

impl From<Error> for io::Error {
    fn from(value: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(unsafe {
            from_utf8_unchecked(
                CStr::from_ptr(match *self {
                    Error::Other { errno } => libc::strerror(errno),
                    _ => STRINGS[index(self)].as_ptr(),
                })
                .to_bytes(),
            )
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("ffmpeg::Error(")?;
        f.write_str(&format!("{}: ", AVUNERROR((*self).into())))?;
        fmt::Display::fmt(self, f)?;
        f.write_str(")")
    }
}

#[inline(always)]
fn index(error: &Error) -> usize {
    match *error {
        Error::BsfNotFound => 0,
        Error::Bug => 1,
        Error::BufferTooSmall => 2,
        Error::DecoderNotFound => 3,
        Error::DemuxerNotFound => 4,
        Error::EncoderNotFound => 5,
        Error::Eof => 6,
        Error::Exit => 7,
        Error::External => 8,
        Error::FilterNotFound => 9,
        Error::InvalidData => 10,
        Error::MuxerNotFound => 11,
        Error::OptionNotFound => 12,
        Error::PatchWelcome => 13,
        Error::ProtocolNotFound => 14,
        Error::StreamNotFound => 15,
        Error::Bug2 => 16,
        Error::Unknown => 17,
        Error::Experimental => 18,
        Error::InputChanged => 19,
        Error::OutputChanged => 20,
        Error::HttpBadRequest => 21,
        Error::HttpUnauthorized => 22,
        Error::HttpForbidden => 23,
        Error::HttpNotFound => 24,
        Error::HttpOther4xx => 25,
        Error::HttpServerError => 26,
        Error::Other { errno: _ } => (-1isize) as usize,
    }
}

// XXX: the length has to be synced with the number of errors
static mut STRINGS: [[c_char; AV_ERROR_MAX_STRING_SIZE]; 27] = [[0; AV_ERROR_MAX_STRING_SIZE]; 27];

pub fn register_all() {
    unsafe {
        av_strerror(
            Error::Bug.into(),
            STRINGS[index(&Error::Bug)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::Bug2.into(),
            STRINGS[index(&Error::Bug2)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::Unknown.into(),
            STRINGS[index(&Error::Unknown)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::Experimental.into(),
            STRINGS[index(&Error::Experimental)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::BufferTooSmall.into(),
            STRINGS[index(&Error::BufferTooSmall)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::Eof.into(),
            STRINGS[index(&Error::Eof)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::Exit.into(),
            STRINGS[index(&Error::Exit)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::External.into(),
            STRINGS[index(&Error::External)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::InvalidData.into(),
            STRINGS[index(&Error::InvalidData)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::PatchWelcome.into(),
            STRINGS[index(&Error::PatchWelcome)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );

        av_strerror(
            Error::InputChanged.into(),
            STRINGS[index(&Error::InputChanged)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::OutputChanged.into(),
            STRINGS[index(&Error::OutputChanged)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );

        av_strerror(
            Error::BsfNotFound.into(),
            STRINGS[index(&Error::BsfNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::DecoderNotFound.into(),
            STRINGS[index(&Error::DecoderNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::DemuxerNotFound.into(),
            STRINGS[index(&Error::DemuxerNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::EncoderNotFound.into(),
            STRINGS[index(&Error::EncoderNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::OptionNotFound.into(),
            STRINGS[index(&Error::OptionNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::MuxerNotFound.into(),
            STRINGS[index(&Error::MuxerNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::FilterNotFound.into(),
            STRINGS[index(&Error::FilterNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::ProtocolNotFound.into(),
            STRINGS[index(&Error::ProtocolNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::StreamNotFound.into(),
            STRINGS[index(&Error::StreamNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );

        av_strerror(
            Error::HttpBadRequest.into(),
            STRINGS[index(&Error::HttpBadRequest)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::HttpUnauthorized.into(),
            STRINGS[index(&Error::HttpUnauthorized)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::HttpForbidden.into(),
            STRINGS[index(&Error::HttpForbidden)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::HttpNotFound.into(),
            STRINGS[index(&Error::HttpNotFound)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::HttpOther4xx.into(),
            STRINGS[index(&Error::HttpOther4xx)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
        av_strerror(
            Error::HttpServerError.into(),
            STRINGS[index(&Error::HttpServerError)].as_mut_ptr(),
            AV_ERROR_MAX_STRING_SIZE,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_roundtrip() {
        assert_eq!(Into::<c_int>::into(Error::from(AVERROR_EOF)), AVERROR_EOF);
        assert_eq!(
            Into::<c_int>::into(Error::from(AVERROR(EAGAIN))),
            AVERROR(EAGAIN)
        );
        assert_eq!(Error::from(AVERROR(EAGAIN)), Error::Other { errno: EAGAIN });
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn test_posix_error_string() {
        assert_eq!(
            Error::from(AVERROR(EAGAIN)).to_string(),
            "Resource temporarily unavailable"
        )
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/interrupt.rs">
use std::panic;
use std::process;

use ffi::*;
use libc::{c_int, c_void};

pub struct Interrupt {
    pub interrupt: AVIOInterruptCB,
}

extern "C" fn callback<F>(opaque: *mut c_void) -> c_int
where
    F: FnMut() -> bool,
{
    // Clippy suggests to remove &mut, but it doesn't compile then (move occurs because value has type `F`, which does not implement the `Copy` trait)
    #[allow(clippy::needless_borrow)]
    match panic::catch_unwind(|| (unsafe { &mut *(opaque as *mut F) })()) {
        Ok(ret) => ret as c_int,
        Err(_) => process::abort(),
    }
}

pub fn new<F>(opaque: Box<F>) -> Interrupt
where
    F: FnMut() -> bool,
{
    let interrupt_cb = AVIOInterruptCB {
        callback: Some(callback::<F>),
        opaque: Box::into_raw(opaque) as *mut c_void,
    };
    Interrupt {
        interrupt: interrupt_cb,
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/media.rs">
use ffi::AVMediaType::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    Unknown,
    Video,
    Audio,
    Data,
    Subtitle,
    Attachment,
}

impl From<AVMediaType> for Type {
    #[inline(always)]
    fn from(value: AVMediaType) -> Self {
        match value {
            AVMEDIA_TYPE_UNKNOWN => Type::Unknown,
            AVMEDIA_TYPE_VIDEO => Type::Video,
            AVMEDIA_TYPE_AUDIO => Type::Audio,
            AVMEDIA_TYPE_DATA => Type::Data,
            AVMEDIA_TYPE_SUBTITLE => Type::Subtitle,
            AVMEDIA_TYPE_ATTACHMENT => Type::Attachment,
            AVMEDIA_TYPE_NB => Type::Unknown,
        }
    }
}

impl From<Type> for AVMediaType {
    #[inline(always)]
    fn from(value: Type) -> AVMediaType {
        match value {
            Type::Unknown => AVMEDIA_TYPE_UNKNOWN,
            Type::Video => AVMEDIA_TYPE_VIDEO,
            Type::Audio => AVMEDIA_TYPE_AUDIO,
            Type::Data => AVMEDIA_TYPE_DATA,
            Type::Subtitle => AVMEDIA_TYPE_SUBTITLE,
            Type::Attachment => AVMEDIA_TYPE_ATTACHMENT,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/mod.rs">
#[macro_use]
pub mod dictionary;
pub mod channel_layout;
pub mod chroma;
pub mod color;
pub mod error;
pub mod format;
pub mod frame;
pub mod interrupt;
pub mod log;
pub mod mathematics;
pub mod media;
pub mod option;
pub mod picture;
pub mod range;
pub mod rational;
pub mod time;

use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use ffi::*;

#[inline(always)]
pub fn version() -> u32 {
    unsafe { avutil_version() }
}

#[inline(always)]
pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avutil_configuration()).to_bytes()) }
}

#[inline(always)]
pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avutil_license()).to_bytes()) }
}
</file>

<file path="patches/ffmpeg-next/src/util/picture.rs">
use ffi::AVPictureType::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    I,
    P,
    B,
    S,
    SI,
    SP,
    BI,
}

impl From<AVPictureType> for Type {
    #[inline(always)]
    fn from(value: AVPictureType) -> Type {
        match value {
            AV_PICTURE_TYPE_NONE => Type::None,
            AV_PICTURE_TYPE_I => Type::I,
            AV_PICTURE_TYPE_P => Type::P,
            AV_PICTURE_TYPE_B => Type::B,
            AV_PICTURE_TYPE_S => Type::S,
            AV_PICTURE_TYPE_SI => Type::SI,
            AV_PICTURE_TYPE_SP => Type::SP,
            AV_PICTURE_TYPE_BI => Type::BI,
        }
    }
}

impl From<Type> for AVPictureType {
    #[inline(always)]
    fn from(value: Type) -> AVPictureType {
        match value {
            Type::None => AV_PICTURE_TYPE_NONE,
            Type::I => AV_PICTURE_TYPE_I,
            Type::P => AV_PICTURE_TYPE_P,
            Type::B => AV_PICTURE_TYPE_B,
            Type::S => AV_PICTURE_TYPE_S,
            Type::SI => AV_PICTURE_TYPE_SI,
            Type::SP => AV_PICTURE_TYPE_SP,
            Type::BI => AV_PICTURE_TYPE_BI,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/range.rs">
use std::ops;

pub trait Range<T> {
    fn start(&self) -> Option<&T> {
        None
    }

    fn end(&self) -> Option<&T> {
        None
    }
}

impl<T> Range<T> for ops::Range<T> {
    fn start(&self) -> Option<&T> {
        Some(&self.start)
    }

    fn end(&self) -> Option<&T> {
        Some(&self.end)
    }
}

impl<T> Range<T> for ops::RangeTo<T> {
    fn end(&self) -> Option<&T> {
        Some(&self.end)
    }
}

impl<T> Range<T> for ops::RangeFrom<T> {
    fn start(&self) -> Option<&T> {
        Some(&self.start)
    }
}

impl<T> Range<T> for ops::RangeFull {}
</file>

<file path="patches/ffmpeg-next/src/util/rational.rs">
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use ffi::*;
use libc::c_int;

#[derive(Copy, Clone)]
pub struct Rational(pub i32, pub i32);

impl Rational {
    #[inline]
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Rational(numerator, denominator)
    }

    #[inline]
    pub fn numerator(&self) -> i32 {
        self.0
    }

    #[inline]
    pub fn denominator(&self) -> i32 {
        self.1
    }

    #[inline]
    pub fn reduce(&self) -> Rational {
        match self.reduce_with_limit(i32::max_value()) {
            Ok(r) => r,
            Err(r) => r,
        }
    }

    #[inline]
    pub fn reduce_with_limit(&self, max: i32) -> Result<Rational, Rational> {
        unsafe {
            let mut dst_num: c_int = 0;
            let mut dst_den: c_int = 0;

            let exact = av_reduce(
                &mut dst_num,
                &mut dst_den,
                i64::from(self.numerator()),
                i64::from(self.denominator()),
                i64::from(max),
            );

            if exact == 1 {
                Ok(Rational(dst_num, dst_den))
            } else {
                Err(Rational(dst_num, dst_den))
            }
        }
    }

    #[inline]
    pub fn invert(&self) -> Rational {
        unsafe { Rational::from(av_inv_q((*self).into())) }
    }
}

impl From<AVRational> for Rational {
    #[inline]
    fn from(value: AVRational) -> Rational {
        Rational(value.num, value.den)
    }
}

impl From<Rational> for AVRational {
    #[inline]
    fn from(value: Rational) -> AVRational {
        AVRational {
            num: value.0,
            den: value.1,
        }
    }
}

impl From<f64> for Rational {
    #[inline]
    fn from(value: f64) -> Rational {
        unsafe { Rational::from(av_d2q(value, c_int::max_value())) }
    }
}

impl From<Rational> for f64 {
    #[inline]
    fn from(value: Rational) -> f64 {
        unsafe { av_q2d(value.into()) }
    }
}

impl From<Rational> for u32 {
    #[inline]
    fn from(value: Rational) -> u32 {
        unsafe { av_q2intfloat(value.into()) }
    }
}

impl From<(i32, i32)> for Rational {
    fn from((num, den): (i32, i32)) -> Rational {
        Rational::new(num, den)
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        if self.0 == other.0 && self.1 == other.1 {
            return true;
        }

        let a = self.reduce();
        let b = other.reduce();

        if a.0 == b.0 && a.1 == b.1 {
            return true;
        }

        false
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            match av_cmp_q((*self).into(), (*other).into()) {
                0 => Some(Ordering::Equal),
                1 => Some(Ordering::Greater),
                -1 => Some(Ordering::Less),

                _ => None,
            }
        }
    }
}

impl Add for Rational {
    type Output = Rational;

    #[inline]
    fn add(self, other: Rational) -> Rational {
        unsafe { Rational::from(av_add_q(self.into(), other.into())) }
    }
}

impl Sub for Rational {
    type Output = Rational;

    #[inline]
    fn sub(self, other: Rational) -> Rational {
        unsafe { Rational::from(av_sub_q(self.into(), other.into())) }
    }
}

impl Mul for Rational {
    type Output = Rational;

    #[inline]
    fn mul(self, other: Rational) -> Rational {
        unsafe { Rational::from(av_mul_q(self.into(), other.into())) }
    }
}

impl Div for Rational {
    type Output = Rational;

    #[inline]
    fn div(self, other: Rational) -> Rational {
        unsafe { Rational::from(av_div_q(self.into(), other.into())) }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!("{}/{}", self.numerator(), self.denominator()))
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!(
            "Rational({}/{})",
            self.numerator(),
            self.denominator()
        ))
    }
}

#[inline]
pub fn nearer(q: Rational, q1: Rational, q2: Rational) -> Ordering {
    unsafe {
        match av_nearer_q(q.into(), q1.into(), q2.into()) {
            1 => Ordering::Greater,
            -1 => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/util/rounding.rs">

</file>

<file path="patches/ffmpeg-next/src/util/time.rs">
use ffi::*;
use Error;

#[inline(always)]
pub fn current() -> i64 {
    unsafe { av_gettime() }
}

#[inline(always)]
pub fn relative() -> i64 {
    unsafe { av_gettime_relative() }
}

#[inline(always)]
pub fn is_monotonic() -> bool {
    unsafe { av_gettime_relative_is_monotonic() != 0 }
}

#[inline(always)]
pub fn sleep(usec: u32) -> Result<(), Error> {
    unsafe {
        match av_usleep(usec) {
            0 => Ok(()),
            e => Err(Error::from(e)),
        }
    }
}
</file>

<file path="patches/ffmpeg-next/src/lib.rs">
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate bitflags;
pub extern crate ffmpeg_sys_next as sys;
#[cfg(feature = "image")]
extern crate image;
extern crate libc;

pub use sys as ffi;

#[macro_use]
pub mod util;
pub use util::channel_layout::{self, ChannelLayout};
pub use util::chroma;
pub use util::color;
pub use util::dictionary;
pub use util::dictionary::Mut as DictionaryMut;
pub use util::dictionary::Owned as Dictionary;
pub use util::dictionary::Ref as DictionaryRef;
pub use util::error::{self, Error};
pub use util::frame::{self, Frame};
pub use util::log;
pub use util::mathematics::{self, rescale, Rescale, Rounding};
pub use util::media;
pub use util::option;
pub use util::picture;
pub use util::rational::{self, Rational};
pub use util::time;

#[cfg(feature = "format")]
pub mod format;
#[cfg(feature = "format")]
pub use format::chapter::{Chapter, ChapterMut};
#[cfg(feature = "format")]
pub use format::format::Format;
#[cfg(feature = "format")]
pub use format::stream::{Stream, StreamMut};

#[cfg(feature = "codec")]
pub mod codec;
#[cfg(feature = "codec")]
pub use codec::audio_service::AudioService;
#[cfg(feature = "codec")]
pub use codec::codec::Codec;
#[cfg(feature = "codec")]
pub use codec::discard::Discard;
#[cfg(feature = "codec")]
pub use codec::field_order::FieldOrder;
#[cfg(feature = "codec")]
pub use codec::packet::{self, Packet};
#[cfg(all(feature = "codec", not(feature = "ffmpeg_5_0")))]
pub use codec::picture::Picture;
#[cfg(feature = "codec")]
pub use codec::subtitle::{self, Subtitle};
#[cfg(feature = "codec")]
pub use codec::threading;
#[cfg(feature = "codec")]
pub use codec::{decoder, encoder};

#[cfg(feature = "device")]
pub mod device;

#[cfg(feature = "filter")]
pub mod filter;
#[cfg(feature = "filter")]
pub use filter::Filter;

pub mod software;

fn init_error() {
    util::error::register_all();
}

#[cfg(all(feature = "format", not(feature = "ffmpeg_5_0")))]
fn init_format() {
    format::register_all();
}

#[cfg(not(feature = "format"))]
fn init_format() {}

#[cfg(feature = "device")]
fn init_device() {
    device::register_all();
}

#[cfg(not(feature = "device"))]
fn init_device() {}

#[cfg(all(feature = "filter", not(feature = "ffmpeg_5_0")))]
fn init_filter() {
    filter::register_all();
}

#[cfg(not(feature = "filter"))]
fn init_filter() {}

#[cfg_attr(
    any(feature = "ffmpeg4", feature = "ffmpeg41", feature = "ffmpeg42"),
    deprecated(
        note = "features ffmpeg4/ffmpeg41/ffmpeg42/ffmpeg43 are now auto-detected \
        and will be removed in a future version"
    )
)]
pub fn init() -> Result<(), Error> {
    init_error();
    #[cfg(not(feature = "ffmpeg_5_0"))]
    init_format();
    init_device();
    #[cfg(not(feature = "ffmpeg_5_0"))]
    init_filter();

    Ok(())
}
</file>

<file path="patches/ffmpeg-next/.cargo_vcs_info.json">
{
  "git": {
    "sha1": "c53730e7abe0e37d65d3f1e13dd8b28fd826a64c"
  },
  "path_in_vcs": ""
}
</file>

<file path="patches/ffmpeg-next/.cargo-ok">
{"v":1}
</file>

<file path="patches/ffmpeg-next/.gitignore">
# Rust files
target
Cargo.lock

# Vim temporary files
*.swp
*.swo
*.swn
</file>

<file path="patches/ffmpeg-next/build.rs">
use std::env;

fn main() {
    for (name, _value) in env::vars() {
        if name.starts_with("DEP_FFMPEG_") {
            println!(
                r#"cargo:rustc-cfg=feature="{}""#,
                name["DEP_FFMPEG_".len()..name.len()].to_lowercase()
            );
        }
    }
}
</file>

<file path="patches/ffmpeg-next/Cargo.toml">
# THIS FILE IS AUTOMATICALLY GENERATED BY CARGO
#
# When uploading crates to the registry Cargo will automatically
# "normalize" Cargo.toml files for maximal compatibility
# with all versions of Cargo and also rewrite `path` dependencies
# to registry (e.g., crates.io) dependencies.
#
# If you are reading this file be aware that the original Cargo.toml
# will likely look very different (and much more reasonable).
# See Cargo.toml.orig for the original contents.

[package]
name = "ffmpeg-next"
version = "6.1.1"
authors = [
    "meh. <meh@schizofreni.co>",
    "Zhiming Wang <i@zhimingwang.org>",
]
build = "build.rs"
description = "Safe FFmpeg wrapper (FFmpeg 4 compatible fork of the ffmpeg crate)"
homepage = "https://github.com/zmwangx/rust-ffmpeg#readme"
documentation = "https://docs.rs/ffmpeg-next"
readme = "README.md"
keywords = [
    "ffmpeg",
    "multimedia",
    "video",
    "audio",
]
categories = ["multimedia"]
license = "WTFPL"
repository = "https://github.com/zmwangx/rust-ffmpeg"

[dependencies.bitflags]
version = "1.2"

[dependencies.ffmpeg-sys-next]
version = "6.1.0"
default-features = false

[dependencies.image]
version = "0.23"
optional = true

[dependencies.libc]
version = "0.2"

[features]
build = [
    "static",
    "ffmpeg-sys-next/build",
]
build-lib-aacplus = ["ffmpeg-sys-next/build-lib-aacplus"]
build-lib-ass = ["ffmpeg-sys-next/build-lib-ass"]
build-lib-avs = ["ffmpeg-sys-next/build-lib-avs"]
build-lib-celt = ["ffmpeg-sys-next/build-lib-celt"]
build-lib-dav1d = ["ffmpeg-sys-next/build-lib-dav1d"]
build-lib-dcadec = ["ffmpeg-sys-next/build-lib-dcadec"]
build-lib-faac = ["ffmpeg-sys-next/build-lib-faac"]
build-lib-fdk-aac = ["ffmpeg-sys-next/build-lib-fdk-aac"]
build-lib-fontconfig = ["ffmpeg-sys-next/build-lib-fontconfig"]
build-lib-freebidi = ["ffmpeg-sys-next/build-lib-freebidi"]
build-lib-freetype = ["ffmpeg-sys-next/build-lib-freetype"]
build-lib-frei0r = ["ffmpeg-sys-next/build-lib-frei0r"]
build-lib-gnutls = ["ffmpeg-sys-next/build-lib-gnutls"]
build-lib-gsm = ["ffmpeg-sys-next/build-lib-gsm"]
build-lib-ilbc = ["ffmpeg-sys-next/build-lib-ilbc"]
build-lib-kvazaar = ["ffmpeg-sys-next/build-lib-kvazaar"]
build-lib-ladspa = ["ffmpeg-sys-next/build-lib-ladspa"]
build-lib-mp3lame = ["ffmpeg-sys-next/build-lib-mp3lame"]
build-lib-opencore-amrnb = ["ffmpeg-sys-next/build-lib-opencore-amrnb"]
build-lib-opencore-amrwb = ["ffmpeg-sys-next/build-lib-opencore-amrwb"]
build-lib-opencv = ["ffmpeg-sys-next/build-lib-opencv"]
build-lib-openh264 = ["ffmpeg-sys-next/build-lib-openh264"]
build-lib-openjpeg = ["ffmpeg-sys-next/build-lib-openjpeg"]
build-lib-openssl = ["ffmpeg-sys-next/build-lib-openssl"]
build-lib-opus = ["ffmpeg-sys-next/build-lib-opus"]
build-lib-schroedinger = ["ffmpeg-sys-next/build-lib-schroedinger"]
build-lib-shine = ["ffmpeg-sys-next/build-lib-shine"]
build-lib-smbclient = ["ffmpeg-sys-next/build-lib-smbclient"]
build-lib-snappy = ["ffmpeg-sys-next/build-lib-snappy"]
build-lib-speex = ["ffmpeg-sys-next/build-lib-speex"]
build-lib-ssh = ["ffmpeg-sys-next/build-lib-ssh"]
build-lib-stagefright-h264 = ["ffmpeg-sys-next/build-lib-stagefright-h264"]
build-lib-theora = ["ffmpeg-sys-next/build-lib-theora"]
build-lib-twolame = ["ffmpeg-sys-next/build-lib-twolame"]
build-lib-utvideo = ["ffmpeg-sys-next/build-lib-utvideo"]
build-lib-vmaf = ["ffmpeg-sys-next/build-lib-vmaf"]
build-lib-vo-aacenc = ["ffmpeg-sys-next/build-lib-vo-aacenc"]
build-lib-vo-amrwbenc = ["ffmpeg-sys-next/build-lib-vo-amrwbenc"]
build-lib-vorbis = ["ffmpeg-sys-next/build-lib-vorbis"]
build-lib-vpx = ["ffmpeg-sys-next/build-lib-vpx"]
build-lib-wavpack = ["ffmpeg-sys-next/build-lib-wavpack"]
build-lib-webp = ["ffmpeg-sys-next/build-lib-webp"]
build-lib-x264 = ["ffmpeg-sys-next/build-lib-x264"]
build-lib-x265 = ["ffmpeg-sys-next/build-lib-x265"]
build-lib-xvid = ["ffmpeg-sys-next/build-lib-xvid"]
build-license-gpl = ["ffmpeg-sys-next/build-license-gpl"]
build-license-nonfree = ["ffmpeg-sys-next/build-license-nonfree"]
build-license-version3 = ["ffmpeg-sys-next/build-license-version3"]
build-pic = ["ffmpeg-sys-next/build-pic"]
build-zlib = ["ffmpeg-sys-next/build-zlib"]
codec = ["ffmpeg-sys-next/avcodec"]
default = [
    "codec",
    "device",
    "filter",
    "format",
    "software-resampling",
    "software-scaling",
]
device = [
    "ffmpeg-sys-next/avdevice",
    "format",
]
ffmpeg4 = []
ffmpeg41 = []
ffmpeg42 = []
ffmpeg43 = []
filter = ["ffmpeg-sys-next/avfilter"]
format = [
    "ffmpeg-sys-next/avformat",
    "codec",
]
postprocessing = ["ffmpeg-sys-next/postproc"]
resampling = ["ffmpeg-sys-next/avresample"]
rpi = []
software-resampling = ["ffmpeg-sys-next/swresample"]
software-scaling = [
    "ffmpeg-sys-next/swscale",
    "codec",
]
static = ["ffmpeg-sys-next/static"]
</file>

<file path="patches/ffmpeg-next/CHANGELOG.md">
5.0.0
-----

- Introduce conditional compilation flags to preserve functions that are
  removed from ffmpeg 5.0 and onwards.
- Fix examples so they are using the ffmpeg-sanctionned way of doing
  things. More specifically, AVStream.codec has been removed, and the
  correct way of getting the codec from a stream is to use
  Context::from_parameters(stream.parameters()) and then that context's
  encoder / decoder.

4.4.0
-----

- crate: `ffmpeg43` feature flag (noop since 4.3.4) has been dropped from default features.

- codec: deprecate APIs based on deprecated (since FFmpeg 3.1) `avcodec_decode_video2()` / `avcodec_decode_audio4()` / `avcodec_encode_video2()` /`avcodec_encode_audio2()` -- `decoder::Video::decode()`, `decode::Audio::decode()`, `encoder::Video::encode()` and `encoder::Audio::decode()`. Users should migrate to `send_packet()` / `send_eof()`, `receive_frame()`, `send_frame()` / `send_eof()`, and `receive_packet()` APIs instead, which are based on the modern send/receive APIs. See [documentation in `libavcodec/avcodec.h`](https://github.com/FFmpeg/FFmpeg/blob/n4.3.1/libavcodec/avcodec.h#L84-L196) for details. (#28)

- codec: fix signature of `Packet::write_interleaved`; previously `Result<bool, Error>`, now `Result<(), Error>`. (#25)

4.3.8
-----
- software::resampling: add Context::get_with for specifying additional options. (#41)

4.3.7
-----

- codec:  fix codec description potential null ptr issue. (#36)

4.3.6
-----

- util: fix Windows compatibility due to unavailable errnos. (#30)

4.3.5
-----

- util: add `util::log` module to expose FFmpeg's logging facilities.

- filter: add method `Source::close()` to expose `av_buffersrc_close`. (#23)

- codec: add new encoding/decoding APIs `send_frame()` / `send_eof()`, `receive_packet()` to `encoder::{Audio, Video}` and `send_packet()` / `send_eof()`, `receive_frame()` to `decoder::{Audio, Video}` based on modern send/receive APIs (instead of `avcodec_decode_video2()` / `avcodec_decode_audio4()` / `avcodec_encode_video2()` /`avcodec_encode_audio2()` which have been deprecated since FFmpeg 3.1). Users should consider switching to the new APIs. See [documentation in `libavcodec/avcodec.h`](https://github.com/FFmpeg/FFmpeg/blob/n4.3.1/libavcodec/avcodec.h#L84-L196) for details. (#28)

- util: introduce new `Error` variant `Error::Other { errno }` for wrapped POSIX error codes (see the `AVERROR` macro in `libavutil/error.h`), and reexport common POSIX error codes under `util::error`. (#24)

4.3.4
-----

- crate: FFmpeg version detection is now automatic, obseleting feature flags `ffmpeg4`, `ffmpeg41`, `ffmpeg42` and `ffmpeg43`. The flags are kept as noop for now, will be removed in 5.0.
</file>

<file path="patches/ffmpeg-next/LICENSE">
DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                    Version 2, December 2004

 Everyone is permitted to copy and distribute verbatim or modified
 copies of this license document, and changing it is allowed as long
 as the name is changed.

            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. You just DO WHAT THE FUCK YOU WANT TO.
</file>

<file path="patches/ffmpeg-next/README.md">
[![crates.io](https://img.shields.io/crates/v/ffmpeg-next.svg)](https://crates.io/crates/ffmpeg-next)
[![docs.rs](https://docs.rs/ffmpeg-next/badge.svg)](https://docs.rs/ffmpeg-next/)
[![build](https://github.com/zmwangx/rust-ffmpeg/workflows/build/badge.svg)](https://github.com/zmwangx/rust-ffmpeg/actions)

This is a fork of the abandoned [ffmpeg](https://crates.io/crates/ffmpeg) crate by [meh.](https://github.com/meh/rust-ffmpeg).

Currently supported FFmpeg versions: 3.4.x through 4.4.x.

Build instructions can be found on the [wiki](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building).

Documentation:

- [docs.rs](https://docs.rs/ffmpeg-next/);
- [FFmpeg user manual](https://ffmpeg.org/ffmpeg-all.html);
- [FFmpeg Doxygen](https://ffmpeg.org/doxygen/trunk/).

*Note on upgrading to v4.3.4 or later: v4.3.4 introduced automatic FFmpeg version detection, obsoleting feature flags `ffmpeg4`, `ffmpeg41`, `ffmpeg42` and `ffmpeg43`. If you manually specify any of these features, now is the time to remove them; if you use `ffmpeg43` through the `default` feature, it's still on for backward-compatibility but it has turned into a no-op, and you don't need to do anything. Deprecation plan: `ffmpeg43` will be dropped from default features come 4.4, and all these features will be removed come 5.0.*

*See [CHANGELOG.md](CHANGELOG.md) for other information on version upgrades.*

A word on versioning: major and minor versions of this crate track major and minor versions of FFmpeg, e.g. 4.2.x of this crate has been updated to support the 4.2.x series of FFmpeg. Patch level is reserved for changes to this crate and does not track FFmpeg patch versions. Since we can only freely bump the patch level, versioning of this crate differs from semver: minor versions may behave like semver major versions and introduce backward-incompatible changes; patch versions may behave like semver minor versions and introduce new APIs. Please peg the version you use accordingly.

**Please realize that this crate is in maintenance-only mode for the most part.** Which means I'll try my best to ensure the crate compiles against all release branches of FFmpeg 3.4 and later (only the latest patch release of each release branch is officially supported) and fix reported bugs, but if a new FFmpeg version brings new APIs that require significant effort to port to Rust, you might have to send me a PR (and just to be clear, I can't really guarantee I'll have the time to review). Any PR to improve existing API is unlikely to be merged, unfortunately.

🤝 **If you have significant, demonstrable experience in Rust and multimedia-related programming, please let me know, I'll be more than happy to invite you as a collaborator.** 🤝
</file>

<file path="src/app/playback.rs">
//! Utilidades de temporización de repintado durante la reproducción.
//!
//! Separado del `impl DiffPlayerApp` para poder testear la matemática de `Duration`
//! sin arrancar egui. El reproductor acorta el intervalo entre frames cuando hay
//! audio activo para reducir underruns en rodio.

use crate::types::VideoFrame;
use std::time::Duration;

/// Máximo tiempo entre repintados cuando hay sink de audio (ms).
pub const REPINT_AUDIO_MAX_MS: u64 = 8;
/// Máximo cuando no hay audio activo en ese camino (ms).
pub const REPINT_IDLE_MAX_MS: u64 = 100;

/// Calcula el retardo hasta el siguiente repintado alineado al siguiente frame de vídeo.
#[must_use]
pub fn next_frame_repaint_delay(fps: f64, current_pts: f64, max_delay_ms: u64) -> Duration {
    if fps <= 0.0 {
        return Duration::from_millis(1);
    }
    let next_frame_pts = (current_pts * fps).ceil() / fps;
    let delay_secs = (next_frame_pts - current_pts).max(0.0);
    Duration::from_secs_f64(delay_secs).clamp(
        Duration::from_millis(1),
        Duration::from_millis(max_delay_ms),
    )
}

/// Pure function to select the best frame for the current clock given the tolerance.
/// It takes the currently cached frame (if any) and an iterator of incoming frames.
/// Returns `(best_frame_to_render, next_frame_to_cache)`.
pub fn select_best_frame<I>(
    mut current_candidate: Option<VideoFrame>,
    incoming_frames: I,
    current_pts: f64,
    pts_tolerance: f64,
) -> (Option<VideoFrame>, Option<VideoFrame>)
where
    I: Iterator<Item = VideoFrame>,
{
    let mut best_frame = current_candidate.take();

    if let Some(ref bf) = best_frame {
        if bf.pts > current_pts + pts_tolerance {
            // The cached next_frame is still in the future, keep waiting.
            return (None, best_frame);
        }
    }

    for frame in incoming_frames {
        if frame.pts <= current_pts + pts_tolerance {
            best_frame = Some(frame);
        } else {
            return (best_frame, Some(frame));
        }
    }

    (best_frame, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_clamped_to_max() {
        let d = next_frame_repaint_delay(25.0, 0.0, 8);
        assert!(d <= Duration::from_millis(8));
    }

    #[test]
    fn select_best_frame_future_cached() {
        let cached = Some(VideoFrame {
            pts: 100.0,
            width: 0,
            height: 0,
            rgba_data: std::sync::Arc::new([]),
        });
        let incoming = vec![];
        let (best, next) = select_best_frame(cached.clone(), incoming.into_iter(), 50.0, 10.0);
        // The cached frame is at 100 > 60 (current_pts 50 + tolerance 10). It should be kept as next.
        assert!(best.is_none());
        assert_eq!(next.unwrap().pts, 100.0);
    }

    #[test]
    fn select_best_frame_consumes_and_advances() {
        let cached = Some(VideoFrame {
            pts: 40.0,
            width: 0,
            height: 0,
            rgba_data: std::sync::Arc::new([]),
        });
        let incoming = vec![
            VideoFrame {
                pts: 50.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
            VideoFrame {
                pts: 60.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
            VideoFrame {
                pts: 70.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
        ];

        let (best, next) = select_best_frame(cached, incoming.into_iter(), 50.0, 10.0);
        // Best should be the one at 60 (50 + 10 tolerance), next should be 70.
        assert_eq!(best.unwrap().pts, 60.0);
        assert_eq!(next.unwrap().pts, 70.0);
    }

    #[test]
    fn select_best_frame_exhausts_incoming() {
        let cached = None;
        let incoming = vec![
            VideoFrame {
                pts: 10.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
            VideoFrame {
                pts: 20.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
        ];

        let (best, next) = select_best_frame(cached, incoming.into_iter(), 50.0, 10.0);
        // It consumes all, best is the last one (20), none pending.
        assert_eq!(best.unwrap().pts, 20.0);
        assert!(next.is_none());
    }
}
</file>

<file path="src/app/proxy_bridge.rs">
//! Puente entre la generación de proxy EXR (`crate::proxy`) y la carga en un canal del reproductor.
//!
//! Tras FFmpeg, el archivo resultante vive en la carpeta temporal con nombre fijo [`PROXY_VIDEO_FILENAME`].
//! `DiffPlayerApp` lo abre con el mismo flujo que un vídeo normal.

use std::path::{Path, PathBuf};

/// Ruta al vídeo proxy dentro de un directorio temporal de una ejecución de proxy.
#[must_use]
pub fn proxy_video_path(temp_dir: &Path) -> PathBuf {
    temp_dir.join(crate::proxy::PROXY_VIDEO_FILENAME)
}
</file>

<file path="src/ui/design.rs">
//! Design tokens shared across native UI panels.

use egui::Color32;

use crate::types::Language;

/// Short ES / EN / Quenya lookup for menu copy (extend as i18n grows).
pub fn tr(
    lang: Language,
    es: &'static str,
    en: &'static str,
    quenya: &'static str,
) -> &'static str {
    match lang {
        Language::Es => es,
        Language::En => en,
        Language::Quenya => quenya,
    }
}

pub const FONT_TITLE: f32 = 17.0;
pub const FONT_SUBTITLE: f32 = 12.0;
pub const FONT_LABEL: f32 = 11.0;
pub const FONT_VALUE: f32 = 11.0;
/// Monospace data (timecode, frame counters).
pub const FONT_MONO: f32 = 11.0;
pub const FONT_MONO_SMALL: f32 = 10.0;

/// Primary accent (timeline playhead, highlights) — keep aligned with info panel branding.
pub const ACCENT_PRIMARY: Color32 = Color32::from_rgb(80, 160, 230);

pub const TIMELINE_HEIGHT: f32 = 44.0;

pub fn dialog_ok(lang: Language) -> &'static str {
    tr(lang, "Aceptar", "OK", "Ná")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Language;

    #[test]
    fn tr_selects_branch_by_language() {
        assert_eq!(tr(Language::Es, "es", "en", "qy"), "es");
        assert_eq!(tr(Language::En, "es", "en", "qy"), "en");
        assert_eq!(tr(Language::Quenya, "es", "en", "qy"), "qy");
    }
}
</file>

<file path="src/ui/i18n.rs">
//! Textos de interfaz agrupados por dominio (algoritmos de diferencia, temas, etc.).

use crate::types::{DiffMode, Language, Theme};
use crate::ui::design::tr;

/// Etiqueta corta para modos de diferencia en combos y barras laterales.
pub fn diff_mode_label(lang: Language, mode: DiffMode) -> &'static str {
    match mode {
        DiffMode::LegacyAbs => tr(lang, "Legado (abs)", "Legacy (abs)", "Yestë (abs)"),
        DiffMode::AbsLinear => tr(lang, "Lineal", "Linear", "Lina"),
        DiffMode::AbsSqrt => tr(lang, "Raíz", "Sqrt", "Súrt"),
        DiffMode::SignedDiverging => tr(lang, "Divergente signado", "Signed diverging", "Haina"),
        DiffMode::None => tr(lang, "—", "—", "—"),
    }
}

/// Nombres mostrados en el submenú de temas (nombres de paletas reconocibles; mismo texto en todos los idiomas).
/// Al añadir un valor en [`Theme`](crate::types::Theme), actualizar esta lista.
pub const THEME_MENU_CHOICES: &[(Theme, &'static str)] = &[
    (Theme::Dark, "Dark"),
    (Theme::Light, "Light"),
    (Theme::Rust, "Rust"),
    (Theme::SolarizedDark, "Solarized Dark"),
    (Theme::SolarizedLight, "Solarized Light"),
    (Theme::Dracula, "Dracula"),
    (Theme::Gruvbox, "Gruvbox"),
    (Theme::Nord, "Nord"),
    (Theme::Monokai, "Monokai"),
    (Theme::OneDark, "One Dark"),
    (Theme::OneLight, "One Light"),
    (Theme::Catppuccin, "Catppuccin"),
    (Theme::TokyoNight, "Tokyo Night"),
    (Theme::NightOwl, "Night Owl"),
    (Theme::Ayc, "Ayc"),
    (Theme::MaterialDesign, "Material Design"),
    (Theme::Everforest, "Everforest"),
    (Theme::TomorrowNight, "Tomorrow Night"),
    (Theme::RosePine, "Rose Pine"),
    (Theme::SynthWave84, "SynthWave '84"),
    (Theme::Nordic, "Nordic"),
    (Theme::OceanicNext, "Oceanic Next"),
    (Theme::Palenight, "Palenight"),
    (Theme::Powerlevel10k, "Powerlevel10k"),
    (Theme::Snazzy, "Snazzy"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_mode_label_spanish() {
        assert_eq!(
            diff_mode_label(Language::Es, DiffMode::LegacyAbs),
            "Legado (abs)"
        );
        assert_eq!(diff_mode_label(Language::Es, DiffMode::AbsLinear), "Lineal");
        assert_eq!(diff_mode_label(Language::Es, DiffMode::AbsSqrt), "Raíz");
        assert_eq!(
            diff_mode_label(Language::Es, DiffMode::SignedDiverging),
            "Divergente signado"
        );
        assert_eq!(diff_mode_label(Language::Es, DiffMode::None), "—");
    }

    #[test]
    fn diff_mode_label_english() {
        assert_eq!(
            diff_mode_label(Language::En, DiffMode::LegacyAbs),
            "Legacy (abs)"
        );
        assert_eq!(
            diff_mode_label(Language::En, DiffMode::SignedDiverging),
            "Signed diverging"
        );
    }

    #[test]
    fn diff_mode_label_quenya() {
        assert_eq!(
            diff_mode_label(Language::Quenya, DiffMode::AbsLinear),
            "Lina"
        );
        assert_eq!(
            diff_mode_label(Language::Quenya, DiffMode::SignedDiverging),
            "Haina"
        );
    }

    #[test]
    fn theme_menu_matches_theme_variant_count() {
        assert_eq!(
            THEME_MENU_CHOICES.len(),
            25,
            "actualizar THEME_MENU_CHOICES si cambia Theme en types.rs"
        );
        let mut seen = [false; 25];
        for (i, (t, _)) in THEME_MENU_CHOICES.iter().enumerate() {
            let idx = *t as usize;
            assert!(idx < 25, "índice Theme fuera de rango: {t:?} -> {idx}");
            assert!(
                !seen[idx],
                "variante Theme duplicada en THEME_MENU_CHOICES: {t:?} (pos {i})"
            );
            seen[idx] = true;
        }
    }
}
</file>

<file path="src/ui/vu_meter.rs">
use crate::app::DiffPlayerApp;
use egui::{Color32, FontId, Pos2, Rect, Rounding, Stroke, Vec2};

// ─── Peak Hold State for True Peak ──────────────────────────────────────────
use std::sync::Mutex;

#[derive(Clone, Copy)]
struct TpState {
    peak: f32, // Peak hold for True Peak marker
    age: f32,
    clip: bool,
    ppm_level: f32, // Fast PPM level for the left bar
}

impl Default for TpState {
    fn default() -> Self {
        Self {
            peak: -60.0,
            age: 0.0,
            clip: false,
            ppm_level: 0.0,
        }
    }
}

impl TpState {
    fn update(&mut self, tp: f32, dt: f32) {
        const HOLD_SEC: f32 = 2.0;
        const FALL_RATE: f32 = 20.0; // dB/s

        // tp is in linear amplitude! Convert to dB
        let tp_db = if tp <= 0.00001 {
            -60.0
        } else {
            20.0 * tp.log10()
        };

        // 1) Peak hold update
        if tp_db > self.peak {
            self.peak = tp_db;
            self.age = 0.0;
        } else {
            self.age += dt;
            if self.age > HOLD_SEC {
                self.peak -= FALL_RATE * dt;
            }
        }
        self.peak = self.peak.max(-60.0);

        // 2) PPM Ballistics for the fast bar (Linear Domain)
        const TAU_ATTACK: f32 = 0.006; // Fast attack ~10ms
        const TAU_RELEASE: f32 = 1.0; // Slow release ~8.6 dB/s
        let tau = if tp > self.ppm_level {
            TAU_ATTACK
        } else {
            TAU_RELEASE
        };
        let alpha = (-dt / tau).exp();
        self.ppm_level = tp + (self.ppm_level - tp) * alpha;

        // Clip detection
        if tp_db >= -1.0 {
            self.clip = true;
        }
    }
}

static TP_STATE: Mutex<[[TpState; 2]; 2]> = Mutex::new(
    [[TpState {
        peak: -60.0,
        age: 0.0,
        clip: false,
        ppm_level: 0.0,
    }; 2]; 2],
);

pub fn reset_meter_state(ch_idx: usize) {
    if let Ok(mut state) = TP_STATE.lock() {
        if ch_idx < 2 {
            state[ch_idx][0] = TpState::default();
            state[ch_idx][1] = TpState::default();
        }
    }
}

// ─── Window entry point ──────────────────────────────────────────────────────
pub fn show_vu_meter_window(ctx: &egui::Context, app: &mut DiffPlayerApp) {
    if !app.view().show_vu_meter {
        return;
    }

    let dt = ctx.input(|i| i.stable_dt).min(0.1_f32);
    ctx.request_repaint();

    let (ch_idx, ch_label, loudness) = if !app.view().mute_a {
        (0usize, "CHANNEL A - HYBRID METER", app.view().loudness_a)
    } else if !app.view().mute_b {
        (1usize, "CHANNEL B - HYBRID METER", app.view().loudness_b)
    } else {
        (0, "— MUTED —", Default::default())
    };

    let (tp_l, tp_r) = {
        let mut state = TP_STATE.lock().unwrap();
        // Since we want the fast bar to represent the overall peak, we can just use the max of L/R
        // or we could show L/R in separate bars. But we only have two bars: Left=PPM, Right=Momentary LUFS.
        // We will just process both channels, and use the max for the PPM bar, or mix them.
        state[ch_idx][0].update(loudness.true_peak[0] as f32, dt);
        state[ch_idx][1].update(loudness.true_peak[1] as f32, dt);
        (state[ch_idx][0], state[ch_idx][1])
    };

    let mut open = true;
    egui::Window::new("Loudness & Peak Meter")
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .default_pos(Pos2::new(
            ctx.screen_rect().width() - 320.0,
            ctx.screen_rect().height() - 600.0,
        ))
        .frame(
            egui::Frame::none()
                .fill(Color32::from_rgb(12, 12, 16))
                .inner_margin(egui::Margin::same(12.0))
                .stroke(Stroke::new(1.5, Color32::from_rgb(50, 50, 68)))
                .rounding(Rounding::same(6.0)),
        )
        .show(ctx, |ui| {
            draw_ebu_panel(ui, ch_idx, ch_label, &loudness, tp_l, tp_r);
        });

    if !open {
        app.view_mut().show_vu_meter = false;
    }
}

// ─── Drawing ─────────────────────────────────────────────────────────────────
fn lufs_to_t(lufs: f32) -> f32 {
    const MIN: f32 = -54.0;
    const MAX: f32 = 9.0;
    ((lufs - MIN) / (MAX - MIN)).clamp(0.0, 1.0)
}

fn lufs_color(lufs: f32, lit: bool) -> Color32 {
    let (r, g, b) = if lufs >= -14.0 {
        (255, 40, 40)
    } else if lufs >= -20.0 {
        (255, 180, 40)
    } else if lufs >= -26.0 {
        (40, 220, 80)
    } else {
        (30, 140, 180)
    };
    if lit {
        Color32::from_rgb(r, g, b)
    } else {
        Color32::from_rgb(
            (r as f32 * 0.08) as u8,
            (g as f32 * 0.08) as u8,
            (b as f32 * 0.08) as u8,
        )
    }
}

fn ppm_color(db: f32, lit: bool) -> Color32 {
    let (r, g, b) = if db >= -1.0 {
        (255, 40, 40)
    } else if db >= -9.0 {
        (255, 180, 40)
    } else {
        (40, 220, 80)
    };
    if lit {
        Color32::from_rgb(r, g, b)
    } else {
        Color32::from_rgb(
            (r as f32 * 0.08) as u8,
            (g as f32 * 0.08) as u8,
            (b as f32 * 0.08) as u8,
        )
    }
}

fn draw_ebu_panel(
    ui: &mut egui::Ui,
    ch_idx: usize,
    ch_label: &str,
    loudness: &crate::app::LoudnessResult,
    tp_l: TpState,
    tp_r: TpState,
) {
    const NUM_LEDS: usize = 48;
    const LED_W: f32 = 64.0;
    const LED_H: f32 = 8.0;
    const GAP: f32 = 2.0;
    const GUTTER: f32 = 16.0;
    const SCALE_W: f32 = 40.0;
    const OUTER_PAD: f32 = 20.0;

    // The scale applies to both bars conceptually but physically they are different units
    const MARKS: &[f32] = &[0.0, -9.0, -14.0, -18.0, -23.0, -30.0, -40.0, -54.0];

    let col_h = (LED_H + GAP) * NUM_LEDS as f32;
    let header_h = 36.0;
    let footer_h = 96.0;
    let total_h = header_h + col_h + footer_h;
    let total_w = OUTER_PAD + LED_W + GUTTER + SCALE_W + GUTTER + LED_W + OUTER_PAD;

    // ── Header ────────────────────────────────────────────────────────────────
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("⬛  HYBRID METER  ⬛")
                .font(FontId::monospace(11.0))
                .color(Color32::from_rgb(160, 160, 190))
                .strong(),
        );
        ui.label(
            egui::RichText::new(ch_label)
                .font(FontId::monospace(10.0))
                .color(Color32::from_rgb(80, 190, 255)),
        );
        ui.add_space(4.0);
    });

    let (resp, painter) = ui.allocate_painter(Vec2::new(total_w, total_h), egui::Sense::hover());
    let origin = resp.rect.min;

    let col_l_x = origin.x + OUTER_PAD;
    let scale_x = col_l_x + LED_W + GUTTER;
    let col_r_x = scale_x + SCALE_W + GUTTER;
    let leds_top = origin.y + header_h;

    let db_to_y = |db: f32| -> f32 {
        let t = 1.0 - lufs_to_t(db);
        leds_top + t * col_h
    };

    let bg = Rect::from_min_size(Pos2::new(origin.x, leds_top), Vec2::new(total_w, col_h));
    painter.rect_filled(bg, Rounding::same(4.0), Color32::from_rgb(6, 6, 10));
    painter.rect_stroke(
        bg,
        Rounding::same(4.0),
        Stroke::new(1.0, Color32::from_rgb(35, 35, 50)),
    );

    // Left Column: PPM (True Peak Fast)
    let ppm_val_l_db = if tp_l.ppm_level <= 0.00001 {
        -60.0
    } else {
        20.0 * tp_l.ppm_level.log10()
    };
    let ppm_val_r_db = if tp_r.ppm_level <= 0.00001 {
        -60.0
    } else {
        20.0 * tp_r.ppm_level.log10()
    };

    // Right Column: Momentary LUFS
    let m_lufs = loudness.momentary as f32;

    for i in 0..NUM_LEDS {
        let t = i as f32 / (NUM_LEDS - 1) as f32;
        let led_val = egui::lerp(-54.0f32..=9.0, 1.0 - t);
        let y = leds_top + t * col_h;

        let rect_l_l =
            Rect::from_min_size(Pos2::new(col_l_x, y), Vec2::new(LED_W / 2.0 - 1.0, LED_H));
        let rect_l_r = Rect::from_min_size(
            Pos2::new(col_l_x + LED_W / 2.0 + 1.0, y),
            Vec2::new(LED_W / 2.0 - 1.0, LED_H),
        );
        let rect_r = Rect::from_min_size(Pos2::new(col_r_x, y), Vec2::new(LED_W, LED_H));

        // Draw PPM L/R on left, LUFS on right
        painter.rect_filled(
            rect_l_l,
            Rounding::same(1.0),
            ppm_color(led_val, ppm_val_l_db >= led_val),
        );
        painter.rect_filled(
            rect_l_r,
            Rounding::same(1.0),
            ppm_color(led_val, ppm_val_r_db >= led_val),
        );
        painter.rect_filled(
            rect_r,
            Rounding::same(1.5),
            lufs_color(led_val, m_lufs >= led_val),
        );
    }

    // Target reference line at -23 (Target LUFS)
    let ref_y = db_to_y(-23.0) + LED_H / 2.0;
    painter.line_segment(
        [
            Pos2::new(col_r_x - 4.0, ref_y),
            Pos2::new(col_r_x + LED_W + 4.0, ref_y),
        ],
        Stroke::new(2.0, Color32::from_rgb(0, 255, 255)),
    );

    // Peak limit line at -1 dBTP (Left column)
    let tp_ref_y = db_to_y(-1.0) + LED_H / 2.0;
    painter.line_segment(
        [
            Pos2::new(col_l_x - 4.0, tp_ref_y),
            Pos2::new(col_l_x + LED_W + 4.0, tp_ref_y),
        ],
        Stroke::new(2.0, Color32::from_rgb(255, 50, 50)),
    );

    // ── True Peak markers ────────────────
    let draw_tp_marker = |x: f32, w: f32, peak_db: f32| {
        if peak_db <= -53.5 {
            return;
        }
        let py = db_to_y(peak_db);
        let mrect = Rect::from_min_size(Pos2::new(x, py), Vec2::new(w, 3.0));
        let mut color = Color32::from_rgb(255, 100, 100);
        if peak_db >= -1.0 {
            color = Color32::from_rgb(255, 0, 0);
        }
        painter.rect_filled(mrect, Rounding::same(0.0), color);
        painter.rect_stroke(mrect, Rounding::same(0.0), Stroke::new(1.0, Color32::WHITE));
    };
    draw_tp_marker(col_l_x, LED_W / 2.0 - 1.0, tp_l.peak);
    draw_tp_marker(col_l_x + LED_W / 2.0 + 1.0, LED_W / 2.0 - 1.0, tp_r.peak);

    // ── Centre Scale ──────────────────────────────────────────────────────────
    for &db in MARKS {
        let y = db_to_y(db) + LED_H / 2.0;
        painter.line_segment(
            [Pos2::new(scale_x, y), Pos2::new(scale_x + 5.0, y)],
            Stroke::new(1.0, Color32::from_rgb(70, 70, 90)),
        );
        painter.line_segment(
            [
                Pos2::new(scale_x + SCALE_W - 5.0, y),
                Pos2::new(scale_x + SCALE_W, y),
            ],
            Stroke::new(1.0, Color32::from_rgb(70, 70, 90)),
        );
        let label = format!("{:3.0}", db);
        painter.text(
            Pos2::new(scale_x + SCALE_W / 2.0, y),
            egui::Align2::CENTER_CENTER,
            label,
            FontId::monospace(9.5),
            if db == -23.0 {
                Color32::from_rgb(0, 255, 255)
            } else {
                Color32::from_rgb(150, 150, 170)
            },
        );
    }

    // ── CLIP indicators ──────────────────────────────────────────────────────
    let clip_y = origin.y + 2.0;
    let clip_h = 14.0;

    // Left clip (True Peak)
    let is_clip = tp_l.clip || tp_r.clip;
    let cr_l = Rect::from_min_size(Pos2::new(col_l_x, clip_y), Vec2::new(LED_W, clip_h));

    // Make clip indicator clickable
    let clip_resp = ui.interact(
        cr_l,
        ui.id().with(format!("clip_{}", ch_idx)),
        egui::Sense::click(),
    );
    if clip_resp.clicked() {
        if let Ok(mut state) = TP_STATE.lock() {
            state[ch_idx][0].clip = false;
            state[ch_idx][1].clip = false;
        }
    }

    let (fill, text_col) = if is_clip {
        (Color32::from_rgb(220, 20, 20), Color32::WHITE)
    } else {
        (Color32::from_rgb(25, 10, 10), Color32::from_rgb(70, 30, 30))
    };
    if clip_resp.hovered() {
        painter.rect_filled(cr_l, Rounding::same(2.0), Color32::from_rgb(100, 30, 30));
    } else {
        painter.rect_filled(cr_l, Rounding::same(2.0), fill);
    }
    painter.rect_stroke(
        cr_l,
        Rounding::same(2.0),
        Stroke::new(0.5, Color32::from_rgb(80, 30, 30)),
    );
    painter.text(
        cr_l.center(),
        egui::Align2::CENTER_CENTER,
        "OVER",
        FontId::monospace(7.5),
        text_col,
    );

    // ── Headers ───────────────────────────────────────────────────────────────
    let header_label_y = origin.y + 18.0;
    painter.text(
        Pos2::new(col_l_x + LED_W / 2.0, header_label_y),
        egui::Align2::CENTER_CENTER,
        "PPM",
        FontId::monospace(12.0),
        Color32::from_rgb(255, 120, 120),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W / 2.0, header_label_y),
        egui::Align2::CENTER_CENTER,
        "LUFS",
        FontId::monospace(12.0),
        Color32::from_rgb(120, 190, 255),
    );

    // ── Footer: numeric readouts ──────────────────────────────────────────────
    let mut footer_y = leds_top + col_h + 8.0;

    let fmt_val = |v: f32| -> String {
        if !v.is_finite() || v <= -119.5 {
            " -∞  ".into()
        } else {
            format!("{:5.1}", v)
        }
    };

    let max_ppm = ppm_val_l_db.max(ppm_val_r_db);
    painter.text(
        Pos2::new(col_l_x + LED_W / 2.0, footer_y),
        egui::Align2::CENTER_TOP,
        fmt_val(max_ppm),
        FontId::monospace(12.0),
        ppm_color(max_ppm, true),
    );
    painter.text(
        Pos2::new(scale_x + SCALE_W / 2.0, footer_y),
        egui::Align2::CENTER_TOP,
        "dB/LU",
        FontId::monospace(9.0),
        Color32::from_rgb(70, 70, 90),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W / 2.0, footer_y),
        egui::Align2::CENTER_TOP,
        fmt_val(m_lufs),
        FontId::monospace(12.0),
        lufs_color(m_lufs, true),
    );

    footer_y += 20.0;
    // Short-term & Integrated
    let s_lufs = loudness.short_term as f32;
    let i_lufs = loudness.integrated as f32;
    painter.text(
        Pos2::new(col_l_x, footer_y),
        egui::Align2::LEFT_TOP,
        "Short-term",
        FontId::monospace(11.0),
        Color32::from_rgb(150, 150, 170),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W, footer_y),
        egui::Align2::RIGHT_TOP,
        format!("{} LUFS", fmt_val(s_lufs)),
        FontId::monospace(12.0),
        if (s_lufs + 23.0).abs() <= 2.0 {
            Color32::from_rgb(50, 255, 100)
        } else {
            Color32::from_rgb(255, 200, 50)
        },
    );

    footer_y += 16.0;
    painter.text(
        Pos2::new(col_l_x, footer_y),
        egui::Align2::LEFT_TOP,
        "Integrated",
        FontId::monospace(11.0),
        Color32::from_rgb(150, 150, 170),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W, footer_y),
        egui::Align2::RIGHT_TOP,
        format!("{} LUFS", fmt_val(i_lufs)),
        FontId::monospace(12.0),
        if (i_lufs + 23.0).abs() <= 1.0 {
            Color32::from_rgb(50, 255, 100)
        } else {
            Color32::from_rgb(255, 200, 50)
        },
    );

    footer_y += 16.0;
    // True Peak Max
    let max_tp = tp_l.peak.max(tp_r.peak);
    painter.text(
        Pos2::new(col_l_x, footer_y),
        egui::Align2::LEFT_TOP,
        "True Peak",
        FontId::monospace(11.0),
        Color32::from_rgb(150, 150, 170),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W, footer_y),
        egui::Align2::RIGHT_TOP,
        format!("{} dBTP", fmt_val(max_tp)),
        FontId::monospace(12.0),
        if max_tp >= -1.0 {
            Color32::from_rgb(255, 50, 50)
        } else {
            Color32::from_rgb(50, 200, 255)
        },
    );
}
</file>

<file path="LICENSE">
MIT License

Copyright (c) 2026 DiffPlayerQC

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</file>

<file path="PLANTILLAS.txt">
================================================================================
                    PLANTILLAS PARA REPORTES EN diffplayerqc
================================================================================


================================================================================
                            PLANTILLA: FEATURE REQUEST
================================================================================

Título: [Descripción breve de la funcionalidad deseada]

---

## Descripción
[Explica qué funcionalidad te gustaría que se agregara y por qué la necesitas]

## Beneficio
[Describe cómo esta funcionalidad mejoraría la experiencia o utilidad del proyecto]

## Contexto
[Proporciona ejemplos de casos de uso o contexto adicional si es relevante]

## Posible Solución (Opcional)
[Si tienes ideas sobre cómo podría implementarse, compártelas aquí]

## Alternativas Consideradas (Opcional)
[Si hay otras formas de resolver el problema, menciónalas]

## Información Adicional
Plataforma: [Windows/Linux/macOS]
Versión: [Si aplica]
Otros detalles: [Cualquier información adicional relevante]


================================================================================
                            PLANTILLA: ISSUE (BUG REPORT)
================================================================================

Título: [Descripción breve del problema]

---

## Descripción del Bug
[Describe el problema que encontraste de forma clara y concisa]

## Pasos para Reproducir
1. [Primer paso]
2. [Segundo paso]
3. [Continúa...]

## Comportamiento Esperado
[Describe qué debería pasar]

## Comportamiento Actual
[Describe qué sucede en cambio]

## Screenshots/Logs (Opcional)
[Adjunta capturas de pantalla, errores de consola o logs relevantes si es posible]

## Información del Entorno
- SO: [Windows/Linux/macOS y versión]
- Versión del Proyecto: [Versión o rama]
- Hardware Relevante: [GPU, CPU u otro hardware relevante si aplica]
- Dependencias: [Versiones de dependencias relevantes si aplica]

## Contexto Adicional
[Información adicional que pueda ser útil para entender el problema]

## Archivos Afectados (Opcional)
[Archivos o módulos que crees que están relacionados con el problema]


================================================================================
                        NOTAS PARA AMBAS PLANTILLAS
================================================================================

✓ ANTES DE ENVIAR:
  • Verifica que no haya un issue o feature request similar ya reportado
  • Sé específico y detallado
  • Proporciona contexto suficiente
  • Usa títulos descriptivos y claros
  • Revisa la ortografía y gramática

✓ INFORMACIÓN ÚTIL:
  • Incluye versiones exactas cuando sea relevante
  • Adjunta logs, screenshots o ejemplos de código si es posible
  • Describe ambiente: SO, versión, hardware especial
  • Indica si el problema es reproducible consistentemente

✓ MANTENIMIENTO:
  • Los reportes incompletos pueden ser cerrados sin explicación
  • Se espera que los reportadores participen en la resolución del problema
  • Proporciona feedback si se sugiere una solución

================================================================================
</file>

<file path=".cargo/config.toml">
[target.x86_64-pc-windows-gnu]
linker = "C:/msys64/ucrt64/bin/gcc.exe"
ar = "C:/msys64/ucrt64/bin/ar.exe"

[env]
PKG_CONFIG_PATH = "/opt/homebrew/opt/ffmpeg@7/lib/pkgconfig"
</file>

<file path="src/proxy.rs">
//! Generación de vídeo proxy a partir de secuencias EXR (FFmpeg externo, FFV1 sin pérdidas).
//!
//! Escribe una lista concat (`exr_list.txt`), ordena los EXR por nombre de fichero y deja
//! el resultado en `proxy.mkv` dentro del directorio temporal indicado.

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use parking_lot::Mutex;

/// Concat list filename and output video filename used in the temp dir.
pub const PROXY_VIDEO_FILENAME: &str = "proxy.mkv";
const EXR_LIST_FILENAME: &str = "exr_list.txt";

use crate::error::AppError;

pub fn validate_ffmpeg_binary() -> Result<PathBuf, AppError> {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::FfmpegNotFound
            } else {
                AppError::Io(e)
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(AppError::FfmpegCommandFailed {
            status: output.status.code(),
            stderr,
        });
    }

    Ok(PathBuf::from("ffmpeg"))
}

/// Ordena rutas EXR por nombre de fichero (mismo criterio que `ls` lexicográfico en el nombre).
fn sort_exr_paths_by_file_name(paths: &mut [PathBuf]) {
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
}

/// Collect and sort EXR paths: from a directory (list .exr inside) or from an existing list.
fn collect_exr_paths(source: ProxySource) -> Vec<PathBuf> {
    let mut paths = match source {
        ProxySource::Directory(ref dir) => {
            let mut out = Vec::new();
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension()
                        .map(|e| {
                            e.to_str()
                                .map(|s| s.eq_ignore_ascii_case("exr"))
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
                    {
                        out.push(p);
                    }
                }
            }
            out
        }
        ProxySource::Files(ref list) => list
            .iter()
            .filter(|p| {
                p.extension()
                    .map(|e| {
                        e.to_str()
                            .map(|s| s.eq_ignore_ascii_case("exr"))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .cloned()
            .collect(),
    };
    sort_exr_paths_by_file_name(&mut paths);
    paths
}

#[derive(Clone)]
enum ProxySource {
    Directory(PathBuf),
    Files(Vec<PathBuf>),
}

/// Default duration per still frame in the concat list (24 fps). Required for image/EXR inputs
/// or FFmpeg emits a near-zero duration stream and players only see a handful of frames.
const CONCAT_FRAME_DURATION_SECS: &str = "0.041666666666666664";

/// Write FFmpeg concat list (ffconcat version 1.0 + file + duration per EXR).
fn write_exr_concat_list(dir: &std::path::Path, exr_paths: &[PathBuf]) -> std::io::Result<PathBuf> {
    let list_path = dir.join(EXR_LIST_FILENAME);
    let mut content = String::new();
    content.push_str("ffconcat version 1.0\n");
    for p in exr_paths {
        let abs = p.canonicalize().unwrap_or_else(|_| p.clone());
        let s = abs.to_string_lossy().replace('\\', "/");
        content.push_str("file '");
        content.push_str(&s);
        content.push_str("'\n");
        content.push_str("duration ");
        content.push_str(CONCAT_FRAME_DURATION_SECS);
        content.push('\n');
    }
    // Concat demuxer ignores duration on the last segment; repeat last file so the prior duration applies.
    if let Some(last) = exr_paths.last() {
        let abs = last.canonicalize().unwrap_or_else(|_| last.clone());
        let s = abs.to_string_lossy().replace('\\', "/");
        content.push_str("file '");
        content.push_str(&s);
        content.push_str("'\n");
    }
    std::fs::write(&list_path, &content)?;
    Ok(list_path)
}

/// Run EXR sequence → single video proxy in a background thread.
/// Output: dst_dir/proxy.mkv — FFV1 lossless, 1080p height, keyframe every frame (-g 1).
/// Progress is updated by parsing FFmpeg stderr for "frame= N".
fn run_exr_to_video_proxy_in_background(
    source: ProxySource,
    dst_dir: PathBuf,
    progress: Arc<Mutex<f32>>,
    running: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
) {
    if running.load(Ordering::Relaxed) {
        return;
    }

    if let Err(e) = std::fs::create_dir_all(&dst_dir) {
        log::warn!("Failed to create proxy dir {:?}: {}", dst_dir, e);
    }

    let exr_paths = collect_exr_paths(source.clone());
    let total = exr_paths.len();
    if total == 0 {
        running.store(false, Ordering::Relaxed);
        return;
    }

    let list_path = match write_exr_concat_list(&dst_dir, &exr_paths) {
        Ok(p) => p,
        Err(e) => {
            let msg = format!("Failed to write EXR list: {}", e);
            log::error!("{}", msg);
            *error.lock() = Some(msg);
            running.store(false, Ordering::Relaxed);
            return;
        }
    };

    let output_path = dst_dir.join(PROXY_VIDEO_FILENAME);

    running.store(true, Ordering::Relaxed);
    *progress.lock() = 0.0;

    const FFMPEG_SCALE: &str = "scale=-1:1080";
    const FFMPEG_GOP: &str = "1";
    const FFMPEG_LEVEL: &str = "3";
    const FFMPEG_PIX_FMT: &str = "yuv420p";

    thread::spawn(move || {
        // FFV1: lossless. -g 1: keyframe every frame. scale=-1:1080: 1080p height. -an: no audio.
        let mut child = match Command::new("ffmpeg")
            .arg("-y")
            .args(["-f", "concat"])
            .args(["-safe", "0"])
            .args(["-i", list_path.to_string_lossy().as_ref()])
            .args(["-vf", FFMPEG_SCALE])
            .args(["-c:v", "ffv1"])
            .args(["-g", FFMPEG_GOP])
            .args(["-level", FFMPEG_LEVEL])
            .args(["-pix_fmt", FFMPEG_PIX_FMT])
            .arg("-an")
            .arg(output_path.as_os_str())
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("Failed to spawn ffmpeg: {}", e);
                log::error!("{}", msg);
                *error.lock() = Some(msg);
                running.store(false, Ordering::Relaxed);
                return;
            }
        };

        let Some(stderr) = child.stderr.take() else {
            let msg = "ffmpeg stderr was not piped as expected".to_string();
            log::error!("{}", msg);
            *error.lock() = Some(msg);
            running.store(false, Ordering::Relaxed);
            return;
        };
        let reader = BufReader::new(stderr);
        // Parse lines like "frame=  123 fps=..." to update progress
        for line in reader.lines().flatten() {
            if let Some(frame_str) = line.split_whitespace().find(|s| s.starts_with("frame=")) {
                if let Some(num_str) = frame_str.strip_prefix("frame=") {
                    let num_str = num_str.trim();
                    if let Ok(n) = num_str.parse::<u64>() {
                        let p = (n as f32 / total as f32).min(1.0);
                        *progress.lock() = p;
                    }
                }
            }
        }

        match child.wait() {
            Ok(status) if !status.success() => {
                let msg = format!("FFmpeg failed with status: {}", status);
                log::warn!("{}", msg);
                *error.lock() = Some(msg);
            }
            Err(e) => {
                let msg = format!("Failed to wait on ffmpeg child process: {}", e);
                log::warn!("{}", msg);
                *error.lock() = Some(msg);
            }
            _ => {}
        }
        *progress.lock() = 1.0;
        running.store(false, Ordering::Relaxed);
    });
}

/// Start proxy generation from a directory (list EXR inside). Output: dst_dir/proxy.mkv.
pub fn run_from_directory_in_background(
    src_dir: PathBuf,
    dst_dir: PathBuf,
    progress: Arc<Mutex<f32>>,
    running: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
) {
    run_exr_to_video_proxy_in_background(
        ProxySource::Directory(src_dir),
        dst_dir,
        progress,
        running,
        error,
    );
}

/// Start proxy generation from an explicit list of files. Output: dst_dir/proxy.mkv.
pub fn run_from_files_in_background(
    exr_paths: Vec<PathBuf>,
    dst_dir: PathBuf,
    progress: Arc<Mutex<f32>>,
    running: Arc<AtomicBool>,
    error: Arc<Mutex<Option<String>>>,
) {
    run_exr_to_video_proxy_in_background(
        ProxySource::Files(exr_paths),
        dst_dir,
        progress,
        running,
        error,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn exr_paths_sorted_by_file_name() {
        let mut paths = vec![
            PathBuf::from("/seq/frame_010.exr"),
            PathBuf::from("/seq/frame_2.exr"),
            PathBuf::from("/seq/frame_001.exr"),
        ];
        sort_exr_paths_by_file_name(&mut paths);
        assert!(
            paths[0].to_string_lossy().contains("001"),
            "expected lexicographic order by file name"
        );
        assert!(paths[1].to_string_lossy().contains("010"));
        assert!(paths[2].to_string_lossy().contains("2"));
    }
}
</file>

<file path="src/trace_log.rs">
//! Log de trazas legible por sesión (un fichero por arranque, nombre con fecha/hora).

use chrono::{Datelike, Timelike};
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

static TRACE: Mutex<Option<File>> = Mutex::new(None);

/// Initialize the trace log. Creates a file named `yyyy_mm_dd_hh_mm_ss_Diff_start.log`
/// in the given directory (e.g. CARGO_MANIFEST_DIR or logs/). Call once at startup.
pub fn init(log_dir: &std::path::Path) -> std::io::Result<()> {
    let now = chrono::Local::now();
    let name = format!(
        "{:04}_{:02}_{:02}_{:02}_{:02}_{:02}_Diff_start.log",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    let path = log_dir.join(name);
    let file = File::create(path)?;
    *TRACE.lock().unwrap() = Some(file);
    Ok(())
}

/// Write a line to the trace log: `[yyyy-mm-dd HH:MM:SS.mmm] msg`
pub fn log(msg: &str) {
    if let Ok(mut guard) = TRACE.lock() {
        if let Some(ref mut f) = *guard {
            let now = chrono::Local::now();
            let line = format!("[{}] {}\n", now.format("%Y-%m-%d %H:%M:%S%.3f"), msg);
            if let Err(e) = f.write_all(line.as_bytes()) {
                eprintln!("Failed to write trace log: {}", e);
            }
            if let Err(e) = f.flush() {
                eprintln!("Failed to flush trace log: {}", e);
            }
        }
    }
}
</file>

<file path="shaders/compare.wgsl">
// DiffPlayerQC — shader de comparación A/B (cortina, diff, heatmap, lado a lado).
// Debe coincidir con `ShaderUniforms` en `src/renderer.rs` (tamaños y orden de campos).

// ---------------------------------------------------------------------------
//  Uniform buffer (must match ShaderUniforms in renderer.rs)
// ---------------------------------------------------------------------------
struct Uniforms {
    split_pos:        f32,
    mode:             u32,
    diff_mode:        u32,
    amplifier:        f32,
    zoom:             f32,
    pan_u:            f32,
    pan_v:            f32,
    scale_u:          f32,
    scale_v:          f32,
    bg_r:             f32,
    bg_g:             f32,
    bg_b:             f32,
    split_horizontal: u32,
}

@group(0) @binding(0) var tex_a:   texture_2d<f32>;
@group(0) @binding(1) var tex_b:   texture_2d<f32>;
@group(0) @binding(2) var samp:    sampler;
@group(0) @binding(3) var<uniform> u: Uniforms;

// ---------------------------------------------------------------------------
//  Vertex stage — generates a fullscreen triangle from vertex index
//  (no vertex buffer required)
// ---------------------------------------------------------------------------
struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0)       uv:  vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vert_idx: u32) -> VertexOut {
    // Fullscreen triangle trick: three hard-coded clip-space positions
    var positions = array<vec2<f32>, 3>(
        vec2(-1.0, -3.0),
        vec2(-1.0,  1.0),
        vec2( 3.0,  1.0),
    );
    var uvs = array<vec2<f32>, 3>(
        vec2(0.0, 2.0),
        vec2(0.0, 0.0),
        vec2(2.0, 0.0),
    );

    var out: VertexOut;
    out.pos = vec4(positions[vert_idx], 0.0, 1.0);
    out.uv  = uvs[vert_idx];
    return out;
}

// ---------------------------------------------------------------------------
//  Fragment stage — the comparison logic
// ---------------------------------------------------------------------------

/// Apply zoom and pan to a raw UV coordinate.
fn zoom_pan_uv(raw_uv: vec2<f32>) -> vec2<f32> {
    // Zoom around centre (0.5, 0.5)
    var centred = raw_uv - vec2(0.5, 0.5);
    
    // Apply aspect ratio scale (letterboxing)
    centred.x = centred.x * u.scale_u;
    centred.y = centred.y * u.scale_v;
    
    let zoomed  = centred / u.zoom;
    // Apply pan offset (pan_u, pan_v are in UV space)
    return zoomed + vec2(0.5 + u.pan_u, 0.5 + u.pan_v);
}

/// Map a scalar intensity (0–1) to heatmap color.
/// 0.0 = black, 0.25 = blue, 0.5 = yellow, 0.75 = orange, 1.0 = red
fn heatmap_color(t: f32) -> vec3<f32> {
    let c = clamp(t, 0.0, 1.0);
    // Gradient: black → dark-blue → yellow → red
    let r = smoothstep(0.4, 0.8, c);
    let g = 1.0 - smoothstep(0.5, 1.0, c) * (1.0 - smoothstep(0.0, 0.4, c));
    let b = smoothstep(0.0, 0.25, c) * (1.0 - smoothstep(0.25, 0.6, c));
    return vec3(r, g, b);
}

// Computes the configured difference mode (0=Legacy, 1=Linear, 2=Sqrt, 3=Signed)
fn compute_difference(col_a: vec3<f32>, col_b: vec3<f32>) -> vec3<f32> {
    var diff: vec3<f32>;
    if u.diff_mode == 0u {
        // LegacyAbs: saturate(abs(A-B) * 2)
        diff = clamp(abs(col_a - col_b) * 2.0, vec3(0.0), vec3(1.0));
    } else if u.diff_mode == 1u {
        // AbsLinear: saturate(abs(A-B) * AMP)
        diff = clamp(abs(col_a - col_b) * u.amplifier, vec3(0.0), vec3(1.0));
    } else if u.diff_mode == 2u {
        // AbsSqrt: sqrt(saturate(abs(A-B) * AMP))
        diff = sqrt(clamp(abs(col_a - col_b) * u.amplifier, vec3(0.0), vec3(1.0)));
    } else {
        // SignedDiverging
        let mag = sqrt(clamp(abs(col_a - col_b) * u.amplifier, vec3(0.0), vec3(1.0)));
        let is_positive = step(vec3(0.0), col_a - col_b);
        diff = mix(vec3(0.5) - mag * 0.5, vec3(0.5) + mag * 0.5, is_positive);
    }
    return diff;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // Transform UV with zoom/pan
    let uv = zoom_pan_uv(in.uv);

    // If UV is out of [0,1] range due to pan, show a dark border
    let border = step(0.0, uv.x) * step(uv.x, 1.0) * step(0.0, uv.y) * step(uv.y, 1.0);

    let col_a = textureSample(tex_a, samp, uv);
    let col_b = textureSample(tex_b, samp, uv);

    var out_color: vec4<f32>;

    let line_half_w = 0.0015;
    // Curtain orientation: 0 = vertical (split on X), 1 = horizontal (split on Y)
    let on_left = select(in.uv.x < u.split_pos, in.uv.y < u.split_pos, u.split_horizontal == 1u);
    let in_line = select(
        abs(in.uv.x - u.split_pos) < line_half_w,
        abs(in.uv.y - u.split_pos) < line_half_w,
        u.split_horizontal == 1u
    );

    if u.mode == 0u {
        // ── 0: Split-Screen (curtain) ──────────────────────────────────────
        let base = select(col_b, col_a, on_left);
        out_color = select(base, vec4(1.0, 1.0, 0.0, 1.0), in_line);
    } else if u.mode == 1u {
        // ── 1: Absolute Difference ─────────────────────────────────────────
        let diff = compute_difference(col_a.rgb, col_b.rgb);
        let base = select(vec4(diff, 1.0), col_a, on_left);
        out_color = select(base, vec4(1.0, 1.0, 0.0, 1.0), in_line);
    } else if u.mode == 2u {
        // ── 2: Heatmap QC ──────────────────────────────────────────────────
        let diff_vec = abs(col_a.rgb - col_b.rgb);
        // Perceptual luminance weight
        let intensity = dot(diff_vec, vec3(0.2126, 0.7152, 0.0722)) * u.amplifier;
        let heat      = heatmap_color(intensity);
        let base = select(vec4(heat, 1.0), col_a, on_left);
        out_color = select(base, vec4(1.0, 1.0, 0.0, 1.0), in_line);
    } else {
        // ── 3: Side-by-Side ────────────────────────────────────────────────
        // Left half shows tex_a scaled to hit 0..1 in x
        // Right half shows tex_b scaled to hit 0..1 in x
        let is_left_half = in.uv.x < 0.5;
        
        var sbs_uv = in.uv;
        if is_left_half {
            sbs_uv.x = sbs_uv.x * 2.0;
        } else {
            sbs_uv.x = (sbs_uv.x - 0.5) * 2.0;
        }
        
        sbs_uv = zoom_pan_uv(sbs_uv);
        
        let sbs_col_a = textureSample(tex_a, samp, sbs_uv);
        let sbs_col_b = textureSample(tex_b, samp, sbs_uv);
        
        var right_side: vec4<f32>;
        if u.diff_mode == 4u {
            right_side = sbs_col_b;
        } else {
            right_side = vec4(compute_difference(sbs_col_a.rgb, sbs_col_b.rgb), 1.0);
        }
        
        let base = select(right_side, sbs_col_a, is_left_half);
        
        // Draw a line down the middle
        let center_line_w = 0.0015;
        let is_center = abs(in.uv.x - 0.5) < center_line_w;
        out_color = select(base, vec4(1.0, 1.0, 0.0, 1.0), is_center);
        
        // Disable outer border clipping for side-by-side mode 
        // because we manually handle the UV scaling and we don't want the 0.5 split clipping it.
        // Instead, we just check if the transformed sbs_uv is out of bounds [0, 1].
        let sbs_border = step(0.0, sbs_uv.x) * step(sbs_uv.x, 1.0) * step(0.0, sbs_uv.y) * step(sbs_uv.y, 1.0);
        let bg = vec3(u.bg_r, u.bg_g, u.bg_b);
        return vec4(mix(bg, out_color.rgb, sbs_border), 1.0);
    }

    // Mix with background color (out of video UV range)
    let bg = vec3(u.bg_r, u.bg_g, u.bg_b);
    return vec4(mix(bg, out_color.rgb, border), 1.0);
}
</file>

<file path="src/app/mod.rs">
//! Aplicación egui/eframe: estado global, bucle `update`, decoders, audio y proxy EXR.
//!
//! Submódulos: [`playback`] (temporización de repintado), [`proxy_bridge`] (ruta al proxy.mkv).
//! Ver `docs/ARQUITECTURA.md` en el repositorio para el flujo completo.

mod playback;
mod proxy_bridge;

use crossbeam_channel::{Receiver, Sender};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::decoder;
use crate::renderer::{RenderCallback, ShaderUniforms, VideoRenderer};
use crate::types::{
    AudioFrame, Channel, ColorMetadata, CompareMode, DecoderCommand, DiffMode, Language,
    PlaybackState, SafeZoneMode, VideoFrame,
};
use rodio::{OutputStream, Sink};
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
//  View state — zoom / pan / mode / sliders
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewState {
    pub mode: CompareMode,
    pub diff_mode: DiffMode,
    pub lang: Language,
    pub theme: crate::types::Theme,
    pub show_hud: bool,
    /// Show left sidebar (video metadata / info).
    pub show_left_panel: bool,
    /// Show right sidebar (mode + audio controls).
    pub show_right_panel: bool,
    pub split_pos: f32,
    pub screenshot_dir: Option<PathBuf>,
    pub amplifier: f32,
    pub zoom: f32,
    pub pan_u: f32,
    pub pan_v: f32,
    pub last_psnr: Option<f64>,
    pub canvas_bg_color: [f32; 3],
    pub show_clean_feed_window: bool,
    pub show_vu_meter: bool,
    /// Canvas rect in egui screen-space (for coordinate transform)
    #[serde(skip, default = "default_rect")]
    pub canvas_rect: egui::Rect,
    pub mute_a: bool,
    pub mute_b: bool,
    pub vol_a: f32,
    pub vol_b: f32,
    #[serde(default = "default_true")]
    pub loop_playback: bool,
    /// Split curtain orientation: false = vertical (X), true = horizontal (Y).
    pub split_horizontal: bool,
    /// Safe zone overlay: None, TV (EBU R95), or Social (9:16).
    pub safe_zone: crate::types::SafeZoneMode,
    /// EBU R128 loudness metrics for channel A (not persisted).
    #[serde(skip, default)]
    pub loudness_a: LoudnessResult,
    /// EBU R128 loudness metrics for channel B (not persisted).
    #[serde(skip, default)]
    pub loudness_b: LoudnessResult,
    /// Saved state for loop_playback when Youlean is opened.
    #[serde(skip)]
    pub saved_loop_playback: Option<bool>,
    /// Delay playback slightly to give external apps time to start (e.g. Youlean).
    #[serde(skip)]
    pub pending_play_after_delay: Option<std::time::Instant>,
}

#[derive(Debug, Clone, Copy)]
pub struct LoudnessResult {
    pub momentary: f64,
    pub short_term: f64,
    pub integrated: f64,
    pub true_peak: [f64; 2],
}

impl Default for LoudnessResult {
    fn default() -> Self {
        Self {
            momentary: -120.0,
            short_term: -120.0,
            integrated: -120.0,
            true_peak: [0.0; 2],
        }
    }
}

impl ViewState {
    pub fn config_path() -> Option<PathBuf> {
        directories::ProjectDirs::from("com", "diffplayerqc", "diffplayerqc")
            .map(|proj| proj.config_dir().join("config.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            log::info!("Loading config from: {:?}", path);
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(mut loaded) = serde_json::from_str::<Self>(&content) {
                    log::info!("Config loaded successfully");
                    // Ensure screenshot_dir exists or fall back to desktop
                    if let Some(dir) = &loaded.screenshot_dir {
                        if !dir.exists() {
                            loaded.screenshot_dir = directories::UserDirs::new()
                                .and_then(|d| d.desktop_dir().map(|p| p.to_path_buf()));
                        }
                    }
                    return loaded;
                } else {
                    log::warn!("Failed to parse config.json, using defaults");
                }
            } else {
                log::info!(
                    "No existing config.json found at {:?}, using defaults",
                    path
                );
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            log::info!("Saving config to: {:?}", path);
            if let Some(parent) = path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    log::warn!("Failed to create config parent directory: {}", e);
                }
            }
            if let Ok(content) = serde_json::to_string_pretty(self) {
                let tmp_path = path.with_extension("json.tmp");
                if let Err(e) = std::fs::write(&tmp_path, &content) {
                    log::error!("Failed to write config.json.tmp: {e}");
                    return;
                }
                if let Err(e) = std::fs::rename(&tmp_path, &path) {
                    log::error!("Failed to rename config.json.tmp to config.json: {e}");
                } else {
                    log::info!("Config saved successfully ({} bytes)", content.len());
                }
            } else {
                log::error!("Failed to serialize ViewState to JSON");
            }
        }
    }
}

impl Default for ViewState {
    fn default() -> Self {
        let desk_dir =
            directories::UserDirs::new().and_then(|d| d.desktop_dir().map(|p| p.to_path_buf()));
        Self {
            mode: CompareMode::SplitScreen,
            diff_mode: DiffMode::AbsLinear,
            lang: Language::Es,
            theme: crate::types::Theme::Dark,
            show_hud: true,
            show_left_panel: true,
            show_right_panel: true,
            split_pos: 0.5,
            screenshot_dir: desk_dir,
            amplifier: 5.0,
            zoom: 1.0,
            pan_u: 0.0,
            pan_v: 0.0,
            last_psnr: None,
            canvas_bg_color: [0.0, 0.0, 0.0],
            show_clean_feed_window: false,
            show_vu_meter: false,
            canvas_rect: egui::Rect::NOTHING,
            mute_a: true,
            mute_b: true,
            vol_a: 1.0,
            vol_b: 1.0,
            loop_playback: true,
            split_horizontal: false,
            safe_zone: crate::types::SafeZoneMode::None,
            loudness_a: LoudnessResult::default(),
            loudness_b: LoudnessResult::default(),
            saved_loop_playback: None,
            pending_play_after_delay: None,
        }
    }
}

// ---------------------------------------------------------------------------
//  Per-channel decoder handle
// ---------------------------------------------------------------------------

struct DecoderHandle {
    cmd_tx: Sender<DecoderCommand>,
    frame_rx: Receiver<VideoFrame>,
    audio_rx: Receiver<AudioFrame>,
    last_frame: Option<VideoFrame>,
    next_frame: Option<VideoFrame>,
    meta: ColorMetadata,
    path: String,
}

// ---------------------------------------------------------------------------
//  Main application struct
// ---------------------------------------------------------------------------

pub struct DiffPlayerApp {
    decoder_a: Option<DecoderHandle>,
    decoder_b: Option<DecoderHandle>,

    view: ViewState,
    playback: PlaybackState,
    pub session: crate::types::SessionState,

    renderer: Arc<Mutex<VideoRenderer>>,

    drag_start: Option<(egui::Pos2, f32, f32)>,
    dragging_split: bool,
    drag_drop_hover_pos: Option<egui::Pos2>,

    // DE VUELTA AL ORIGINAL
    _audio_stream: Option<OutputStream>,
    sink_a: Option<Sink>,
    sink_b: Option<Sink>,

    // EBU R128 analyzers: Option<(analyzer, channels, sample_rate)>
    pub ebu_a: Option<(ebur128::EbuR128, u32, u32)>,
    pub ebu_b: Option<(ebur128::EbuR128, u32, u32)>,

    error_title: Option<String>,
    error_message: Option<String>,
    last_step_time: f64,

    /// Request viewport focus/visible for first N frames (macOS window not showing workaround).
    focus_visible_frames_left: u32,

    /// Incremented each frame. Frame 0 skips all Wgpu work so the window can appear on macOS.
    frame_count: u64,

    /// Deferred play/pause toggle (Space): process at start of next update to avoid re-entrancy deadlock.
    pending_play_pause_toggle: bool,

    /// Deferred key action: process at start of next update to avoid re-entrancy/deadlock when called from ctx.input().
    pending_key_action: PendingKeyAction,

    /// Proxy generation: progress 0.0..=1.0.
    proxy_progress: Arc<Mutex<f32>>,
    proxy_error: Arc<Mutex<Option<String>>>,
    /// Proxy generation: true while background thread is running.
    proxy_running: Arc<AtomicBool>,
    /// Temp directory for current proxy run (PNGs + concat); cleared when run finishes or new run starts.
    proxy_temp_dir: Option<PathBuf>,
    /// Channel to load the proxy sequence into when generation finishes.
    proxy_target_channel: Option<Channel>,
    /// All proxy temp dirs to remove on exit.
    proxy_temp_dirs: Vec<PathBuf>,
}

/// Key actions deferred from ctx.input() to start of update() to avoid re-entrancy on macOS.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PendingKeyAction {
    None,
    StepFwd,
    StepBck,
    Seek(f64),
    CycleMode,
    SideBySide,
    SplitPos0,
    SplitPos1,
    ToggleHud,
    Zoom(f32),
    ResetZoomPan,
    SwapVideos,
    SetLoopIn,
    SetLoopOut,
    ClearLoopRange,
    ToggleLoopRange,
    AddMarker,
}

impl Default for PendingKeyAction {
    fn default() -> Self {
        PendingKeyAction::None
    }
}

impl DiffPlayerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        // Limpiar proxies temporales de sesiones anteriores por si hubo crasheo.
        let proxies_base_dir = std::env::temp_dir().join("diffplayerqc_proxies");
        if proxies_base_dir.exists() {
            if let Err(e) = std::fs::remove_dir_all(&proxies_base_dir) {
                log::warn!("Failed to clean up old proxies directory at startup: {}", e);
            } else {
                log::info!("Cleaned up old proxies directory at startup.");
            }
        }

        let render_state = match cc.wgpu_render_state.as_ref() {
            Some(rs) => rs,
            None => panic!("Wgpu render state missing"),
        };

        let target_format = render_state.target_format;
        let renderer = Arc::new(Mutex::new(VideoRenderer::new(
            &render_state.device,
            target_format,
        )));

        let view = ViewState::load();
        crate::ui::theme::apply_theme(&cc.egui_ctx, view.theme);

        // INICIALIZACIÓN DIRECTA (¡Ya no hay dark_light que moleste!)

        log::info!("Inicializando Audio en el hilo principal...");

        let (audio_stream, sink_a, sink_b) = match rodio::OutputStream::try_default() {
            Ok((stream, handle)) => {
                let s_a = rodio::Sink::try_new(&handle).ok();
                let s_b = rodio::Sink::try_new(&handle).ok();
                if let (Some(sa), Some(sb)) = (&s_a, &s_b) {
                    sa.set_volume(0.0);
                    sb.set_volume(0.0);
                }
                (Some(stream), s_a, s_b)
            }
            Err(e) => {
                log::error!("Error al inicializar audio: {}", e);
                (None, None, None)
            }
        };
        if sink_a.is_some() && sink_b.is_some() {
            log::info!("Audio inicializado correctamente (canales A y B).");
        }

        let mut target_sample_rate = 44100;
        let mut target_channels = 2;
        {
            use rodio::cpal::traits::{DeviceTrait, HostTrait};
            if let Some(device) = rodio::cpal::default_host().default_output_device() {
                if let Ok(config) = device.default_output_config() {
                    target_sample_rate = config.sample_rate().0;
                    target_channels = config.channels();
                    log::info!(
                        "Device default audio config: {} Hz, {} channels",
                        target_sample_rate,
                        target_channels
                    );
                }
            }
        }

        crate::trace_log::log("App initialized");

        Self {
            decoder_a: None,
            decoder_b: None,
            view,
            playback: PlaybackState {
                target_sample_rate,
                target_channels,
                ..Default::default()
            },
            session: crate::types::SessionState::default(),
            renderer,
            drag_start: None,
            dragging_split: false,
            drag_drop_hover_pos: None,

            _audio_stream: audio_stream,
            sink_a,
            sink_b,

            ebu_a: None,
            ebu_b: None,

            error_title: None,
            error_message: None,
            last_step_time: 0.0,
            focus_visible_frames_left: 15,
            frame_count: 0,
            pending_play_pause_toggle: false,
            pending_key_action: PendingKeyAction::None,

            proxy_progress: Arc::new(Mutex::new(0.0)),
            proxy_error: Arc::new(Mutex::new(None)),
            proxy_running: Arc::new(AtomicBool::new(false)),
            proxy_temp_dir: None,
            proxy_target_channel: None,
            proxy_temp_dirs: Vec::new(),
        }
    }

    /// Start EXR→PNG proxy generation from a directory (lists .exr inside). When done, loads sequence into `channel`.
    pub fn start_proxy_from_exr_input_dir(
        &mut self,
        src_dir: PathBuf,
        channel: Channel,
        _ctx: &egui::Context,
    ) {
        if self.proxy_running() {
            return;
        }
        if let Err(e) = crate::proxy::validate_ffmpeg_binary() {
            log::error!("FFmpeg validation failed: {}", e);
            self.error_title = Some("Dependencia requerida: FFmpeg".to_string());
            self.error_message = Some(e.to_string());
            return;
        }
        let name = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis().to_string())
            .unwrap_or_else(|_| "proxy".to_string());
        let temp_dir = std::env::temp_dir().join("diffplayerqc_proxies").join(name);
        if let Err(e) = std::fs::create_dir_all(&temp_dir) {
            log::error!("Failed to create proxy temp dir: {e}");
            return;
        }
        self.proxy_temp_dir = Some(temp_dir.clone());
        self.proxy_target_channel = Some(channel);
        *self.proxy_progress.lock() = 0.0;
        *self.proxy_error.lock() = None;

        crate::proxy::run_from_directory_in_background(
            src_dir,
            temp_dir,
            Arc::clone(&self.proxy_progress),
            Arc::clone(&self.proxy_running),
            Arc::clone(&self.proxy_error),
        );
    }

    /// Start EXR→PNG proxy generation from a list of EXR file paths. When done, loads sequence into `channel`.
    pub fn start_proxy_from_exr_input_files(
        &mut self,
        exr_paths: Vec<PathBuf>,
        channel: Channel,
        _ctx: &egui::Context,
    ) {
        if self.proxy_running() || exr_paths.is_empty() {
            return;
        }
        if let Err(e) = crate::proxy::validate_ffmpeg_binary() {
            log::error!("FFmpeg validation failed: {}", e);
            self.error_title = Some("Dependencia requerida: FFmpeg".to_string());
            self.error_message = Some(e.to_string());
            return;
        }
        let name = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis().to_string())
            .unwrap_or_else(|_| "proxy".to_string());
        let temp_dir = std::env::temp_dir().join("diffplayerqc_proxies").join(name);
        if let Err(e) = std::fs::create_dir_all(&temp_dir) {
            log::error!("Failed to create proxy temp dir: {e}");
            return;
        }
        self.proxy_temp_dir = Some(temp_dir.clone());
        self.proxy_target_channel = Some(channel);
        *self.proxy_progress.lock() = 0.0;
        *self.proxy_error.lock() = None;

        crate::proxy::run_from_files_in_background(
            exr_paths,
            temp_dir,
            Arc::clone(&self.proxy_progress),
            Arc::clone(&self.proxy_running),
            Arc::clone(&self.proxy_error),
        );
    }

    /// True if proxy generation is currently running.
    pub fn proxy_running(&self) -> bool {
        self.proxy_running.load(Ordering::Relaxed)
    }

    /// Current proxy progress 0.0..=1.0.
    pub fn proxy_progress(&self) -> f32 {
        *self.proxy_progress.lock()
    }

    // -----------------------------------------------------------------------
    //  Session Save / Load
    // -----------------------------------------------------------------------
    pub fn save_session(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("DiffPlayerQC Session", &["dpqc"])
            .save_file()
        {
            if let Ok(json) = serde_json::to_string_pretty(&self.session) {
                if let Err(e) = std::fs::write(&path, json) {
                    log::error!("Failed to save session: {}", e);
                }
            }
        }
    }

    pub fn load_session(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("DiffPlayerQC Session", &["dpqc"])
            .pick_file()
        {
            if let Ok(json) = std::fs::read_to_string(&path) {
                if let Ok(session) = serde_json::from_str::<crate::types::SessionState>(&json) {
                    self.session = session;
                    // Optional: load videos if they exist
                    if let Some(p) = &self.session.video_a_path {
                        self.open_video_from_path(p.clone(), Channel::A, ctx);
                    }
                    if let Some(p) = &self.session.video_b_path {
                        self.open_video_from_path(p.clone(), Channel::B, ctx);
                    }
                } else {
                    log::error!("Failed to parse session file");
                }
            }
        }
    }

    pub fn export_csv(&self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("CSV file", &["csv"])
            .save_file()
        {
            let mut wtr = match csv::Writer::from_path(&path) {
                Ok(w) => w,
                Err(e) => {
                    log::error!("Failed to create CSV writer: {}", e);
                    return;
                }
            };

            let fps = self.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
            let res = self
                .decoder_a_meta()
                .map(|m| format!("{}x{}", m.width, m.height))
                .unwrap_or_else(|| "Unknown".to_string());

            let _ = wtr.write_record(&["TC", "Tipo (Nota)", "Texto", "Resolución", "FPS"]);
            for marker in &self.session.markers {
                let tc = crate::ui::markers::format_timecode(marker.pts, fps);
                let _ = wtr.write_record(&[&tc, "Marker", &marker.note, &res, &fps.to_string()]);
            }

            if let Err(e) = wtr.flush() {
                log::error!("Failed to flush CSV: {}", e);
            }
        }
    }

    // -----------------------------------------------------------------------
    //  Open a video file for one channel
    // -----------------------------------------------------------------------
    fn open_video(&mut self, chan: Channel, ctx: &egui::Context) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter(
                "Video",
                &[
                    "mp4", "mov", "mxf", "mkv", "avi", "prores", "mts", "mpg", "mpeg", "ts",
                ],
            )
            .add_filter("All files", &["*"])
            .pick_file()
        else {
            return;
        };

        let path_str = path.to_string_lossy().to_string();
        self.open_video_from_path(path_str, chan, ctx);
    }

    /// Load a video from a filesystem path into the given channel, replacing any existing video.
    pub fn open_video_from_path(&mut self, path_str: String, chan: Channel, ctx: &egui::Context) {
        match decoder::spawn_decoder(
            &path_str,
            self.playback.target_sample_rate as i32,
            self.playback.target_channels as i32,
        ) {
            Ok((cmd_tx, frame_rx, audio_rx, meta)) => {
                let handle = DecoderHandle {
                    cmd_tx,
                    frame_rx,
                    audio_rx,
                    last_frame: None,
                    next_frame: None,
                    meta,
                    path: path_str.clone(),
                };

                match chan {
                    Channel::A => {
                        // Stop old decoder if any
                        if let Some(old) = self.decoder_a.take() {
                            if let Err(e) = old.cmd_tx.send(DecoderCommand::Stop) {
                                log::debug!("Decoder cmd channel closed: {}", e);
                            }
                        }
                        if let Some(sink) = &self.sink_a {
                            sink.clear();
                        }
                        self.playback.duration_a = handle.meta.duration_secs;
                        self.session.video_a_path = Some(path_str.clone());
                        self.decoder_a = Some(handle);
                        self.do_seek(0.0, ctx);
                        // No need for repaint here as do_seek handles it
                    }
                    Channel::B => {
                        if let Some(old) = self.decoder_b.take() {
                            if let Err(e) = old.cmd_tx.send(DecoderCommand::Stop) {
                                log::debug!("Decoder cmd channel closed: {}", e);
                            }
                        }
                        if let Some(sink) = &self.sink_b {
                            sink.clear();
                        }
                        self.playback.duration_b = handle.meta.duration_secs;
                        self.session.video_b_path = Some(path_str.clone());
                        self.decoder_b = Some(handle);
                        crate::ui::vu_meter::reset_meter_state(1);
                        self.do_seek(0.0, ctx);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to open video: {e}");
            }
        }
    }

    // -----------------------------------------------------------------------
    //  Drain frame channels and upload new frames to GPU
    // -----------------------------------------------------------------------
    /// Drain at most one frame per decoder that is at or before current_pts (master clock).
    /// Future frames stay in next_frame or channel; no blind turbo-drain, no clock resync.
    fn drain_frames(&mut self, render_state: &egui_wgpu::RenderState) -> bool {
        let device = &render_state.device;
        let queue = &render_state.queue;
        let current_pts = self.playback.current_pts;
        let is_playing = self.playback.is_playing;
        let mut repainted = false;
        const PTS_TOLERANCE: f64 = 0.005;

        let mut process_dec = |dec: &mut DecoderHandle, is_a: bool| {
            if !is_playing {
                // Paused: show latest frame we have (step/seek)
                let mut latest = dec.next_frame.take();
                for f in dec.frame_rx.try_iter() {
                    latest = Some(f);
                }
                if let Some(frame) = latest {
                    let (w, h) = (frame.width, frame.height);
                    let data = frame.rgba_data.clone();
                    {
                        let mut rend = self.renderer.lock();
                        if is_a {
                            rend.update_texture_a(device, queue, &data, w, h);
                        } else {
                            rend.update_texture_b(device, queue, &data, w, h);
                        }
                    }
                    dec.last_frame = Some(frame.clone());
                    repainted = true;
                }
                return;
            }

            // Playing: Catch-up / resync logic
            let (best_frame, next_frame) = playback::select_best_frame(
                dec.next_frame.take(),
                dec.frame_rx.try_iter(),
                current_pts,
                PTS_TOLERANCE,
            );
            dec.next_frame = next_frame;

            if let Some(frame) = best_frame {
                let (w, h) = (frame.width, frame.height);
                let data = frame.rgba_data.clone();
                {
                    let mut rend = self.renderer.lock();
                    if is_a {
                        rend.update_texture_a(device, queue, &data, w, h);
                    } else {
                        rend.update_texture_b(device, queue, &data, w, h);
                    }
                }
                dec.last_frame = Some(frame.clone());
                repainted = true;
            }
        };

        if let Some(dec) = &mut self.decoder_a {
            process_dec(dec, true);
        }
        if let Some(dec) = &mut self.decoder_b {
            process_dec(dec, false);
        }

        if !is_playing {
            // When paused, current_pts tracks the last shown frame (from last_frame)
            if let Some(dec) = &self.decoder_a {
                if let Some(ref f) = dec.last_frame {
                    self.playback.current_pts = f.pts;
                }
            }
            if let Some(dec) = &self.decoder_b {
                if let Some(ref f) = dec.last_frame {
                    self.playback.current_pts = self.playback.current_pts.max(f.pts);
                }
            }
        }

        repainted
    }

    // -----------------------------------------------------------------------
    //  Sync uniform buffer from current view state
    // ---------------------------------------------------------------------------
    fn sync_uniforms(&self) {
        let (mut scale_u, mut scale_v) = (1.0, 1.0);

        let mut canvas_w = self.view.canvas_rect.width();
        let canvas_h = self.view.canvas_rect.height();

        if self.view.mode == CompareMode::SideBySide {
            canvas_w /= 2.0;
        }

        if canvas_w > 0.0 && canvas_h > 0.0 {
            let mut vid_w: f32 = 0.0;
            let mut vid_h: f32 = 0.0;
            if let Some(meta) = self.decoder_a_meta() {
                vid_w = vid_w.max(meta.width as f32);
                vid_h = vid_h.max(meta.height as f32);
            }
            if let Some(meta) = self.decoder_b_meta() {
                vid_w = vid_w.max(meta.width as f32);
                vid_h = vid_h.max(meta.height as f32);
            }

            if vid_w > 0.0 && vid_h > 0.0 {
                let canvas_aspect = canvas_w / canvas_h;
                let video_aspect = vid_w / vid_h;

                if canvas_aspect > video_aspect {
                    // Window is wider than video (pillarbox)
                    scale_u = canvas_aspect / video_aspect;
                } else {
                    // Window is taller than video (letterbox)
                    scale_v = video_aspect / canvas_aspect;
                }
            }
        }

        let mut rend = self.renderer.lock();
        rend.uniforms = ShaderUniforms {
            split_pos: self.view.split_pos,
            mode: self.view.mode as u32,
            diff_mode: self.view.diff_mode as u32,
            amplifier: self.view.amplifier,
            zoom: self.view.zoom,
            pan_u: self.view.pan_u,
            pan_v: self.view.pan_v,
            scale_u,
            scale_v,
            bg_color: self.view.canvas_bg_color,
            split_horizontal: if self.view.split_horizontal { 1 } else { 0 },
        };
    }

    // -----------------------------------------------------------------------
    //  Send seek command to both decoders
    // -----------------------------------------------------------------------
    fn seek_both(&self, pts: f64, ctx: &egui::Context) {
        if let Some(dec) = &self.decoder_a {
            if let Err(e) = dec.cmd_tx.send(DecoderCommand::Seek(pts)) {
                log::debug!("Decoder cmd channel closed: {}", e);
            }
        }
        if let Some(dec) = &self.decoder_b {
            if let Err(e) = dec.cmd_tx.send(DecoderCommand::Seek(pts)) {
                log::debug!("Decoder cmd channel closed: {}", e);
            }
        }
        ctx.request_repaint();
    }

    fn play_both(&mut self, ctx: &egui::Context) {
        crate::trace_log::log("Play");
        self.playback.is_playing = true;
        self.playback.playback_start_instant = Some(Instant::now());
        self.playback.playback_start_pts = self.playback.current_pts;
        if let Some(s) = &self.sink_a {
            s.play();
        }
        if let Some(s) = &self.sink_b {
            s.play();
        }
        if let Some(dec) = &self.decoder_a {
            if let Err(e) = dec.cmd_tx.send(DecoderCommand::Play) {
                log::debug!("Decoder cmd channel closed: {}", e);
            }
        }
        if let Some(dec) = &self.decoder_b {
            if let Err(e) = dec.cmd_tx.send(DecoderCommand::Play) {
                log::debug!("Decoder cmd channel closed: {}", e);
            }
        }
        ctx.request_repaint();
    }

    fn pause_both(&mut self, ctx: &egui::Context) {
        crate::trace_log::log("Pause");
        self.playback.is_playing = false;
        self.playback.playback_start_instant = None;
        if let Some(s) = &self.sink_a {
            s.pause();
        }
        if let Some(s) = &self.sink_b {
            s.pause();
        }
        if let Some(dec) = &self.decoder_a {
            if let Err(e) = dec.cmd_tx.send(DecoderCommand::Pause) {
                log::debug!("Decoder cmd channel closed: {}", e);
            }
        }
        if let Some(dec) = &self.decoder_b {
            if let Err(e) = dec.cmd_tx.send(DecoderCommand::Pause) {
                log::debug!("Decoder cmd channel closed: {}", e);
            }
        }
        ctx.request_repaint();
    }

    fn step_forward(&self, ctx: &egui::Context) {
        if let Some(dec) = &self.decoder_a {
            if let Err(e) = dec.cmd_tx.send(DecoderCommand::StepForward) {
                log::debug!("Decoder cmd channel closed: {}", e);
            }
        }
        if let Some(dec) = &self.decoder_b {
            if let Err(e) = dec.cmd_tx.send(DecoderCommand::StepForward) {
                log::debug!("Decoder cmd channel closed: {}", e);
            }
        }
        ctx.request_repaint();
    }

    pub fn swap_videos(&mut self, ctx: &egui::Context) {
        self.swap_videos_inner(ctx);
    }
    fn swap_videos_inner(&mut self, ctx: &egui::Context) {
        std::mem::swap(&mut self.decoder_a, &mut self.decoder_b);
        std::mem::swap(&mut self.playback.duration_a, &mut self.playback.duration_b);
        std::mem::swap(&mut self.view.mute_a, &mut self.view.mute_b);
        std::mem::swap(&mut self.view.vol_a, &mut self.view.vol_b);
        std::mem::swap(&mut self.sink_a, &mut self.sink_b);
        // Force rendering buffers to swap next frame
        if let Some(dec) = &mut self.decoder_a {
            dec.next_frame = dec.frame_rx.try_recv().ok();
        }
        if let Some(dec) = &mut self.decoder_b {
            dec.next_frame = dec.frame_rx.try_recv().ok();
        }
        ctx.request_repaint();
    }

    pub fn set_loop_in(&mut self) {
        self.playback.loop_in = Some(self.playback.current_pts);
        if let Some(out_pts) = self.playback.loop_out {
            if self.playback.loop_in.unwrap() >= out_pts {
                self.playback.loop_out = None;
            }
        }
    }

    pub fn set_loop_out(&mut self) {
        self.playback.loop_out = Some(self.playback.current_pts);
        if let Some(in_pts) = self.playback.loop_in {
            if self.playback.loop_out.unwrap() <= in_pts {
                self.playback.loop_in = None;
            }
        }
    }

    pub fn toggle_loop_range(&mut self) {
        self.playback.loop_range_active = !self.playback.loop_range_active;
        if self.playback.loop_range_active {
            self.view.loop_playback = false;
        }
    }

    fn complete_proxy_if_ready(&mut self, ctx: &egui::Context) {
        if self.proxy_running()
            || self.proxy_target_channel.is_none()
            || self.proxy_temp_dir.is_none()
        {
            return;
        }
        let dir = self.proxy_temp_dir.take().unwrap();
        let channel = self.proxy_target_channel.take().unwrap();

        let err = self.proxy_error.lock().take();
        if let Some(msg) = err {
            self.error_title = Some("Proxy Generation Failed".to_string());
            self.error_message = Some(msg);
            let _ = std::fs::remove_dir_all(&dir);
            return;
        }

        let proxy_video = proxy_bridge::proxy_video_path(&dir);
        if proxy_video.exists() {
            self.proxy_temp_dirs.push(dir);
            let path_str = proxy_video.to_string_lossy().to_string();
            self.open_video_from_path(path_str, channel, ctx);
        } else {
            self.error_title = Some("Proxy Generation Failed".to_string());
            self.error_message =
                Some("Proxy video file was not found after FFmpeg execution.".to_string());
            let _ = std::fs::remove_dir_all(&dir);
        }
    }

    fn update_master_clock_and_repaint(&mut self, ctx: &egui::Context, is_first_frame: bool) {
        if !self.playback.is_playing {
            return;
        }
        if let Some(start) = self.playback.playback_start_instant {
            let elapsed = start.elapsed().as_secs_f64();
            self.playback.current_pts = self.playback.playback_start_pts + elapsed;

            // --- Protect against decoder stall (Issue 3) ---
            let mut min_rendered_pts = f64::MAX;
            if let Some(ref dec) = self.decoder_a {
                if let Some(ref f) = dec.last_frame {
                    min_rendered_pts = min_rendered_pts.min(f.pts);
                }
            }
            if let Some(ref dec) = self.decoder_b {
                if let Some(ref f) = dec.last_frame {
                    min_rendered_pts = min_rendered_pts.min(f.pts);
                }
            }
            if min_rendered_pts < f64::MAX {
                let drift = self.playback.current_pts - min_rendered_pts;
                let max_drift = 0.1; // 100ms tolerance
                if drift > max_drift {
                    // Pull back the master clock to not outpace the decoders
                    self.playback.current_pts = min_rendered_pts + max_drift;
                    // Re-anchor the clock so it doesn't jump back and forth
                    self.playback.playback_start_pts = self.playback.current_pts;
                    self.playback.playback_start_instant = Some(std::time::Instant::now());
                }
            }
            // -----------------------------------------------

            let max_duration = self.playback.duration_a.max(self.playback.duration_b);

            if self.playback.loop_range_active {
                if let (Some(in_pts), Some(out_pts)) =
                    (self.playback.loop_in, self.playback.loop_out)
                {
                    if self.playback.current_pts >= out_pts {
                        self.do_seek(in_pts, ctx);
                    }
                }
            } else if max_duration > 0.0 {
                if self.playback.current_pts >= max_duration {
                    if self.view.loop_playback {
                        self.do_seek(0.0, ctx);
                    } else {
                        self.playback.current_pts = max_duration;
                        self.playback.is_playing = false;
                        self.playback.playback_start_instant = None;
                    }
                } else {
                    self.playback.current_pts = self.playback.current_pts.clamp(0.0, max_duration);
                }
            }
        }
        // Repintado: intervalo corto con audio activo (rodio) para evitar underruns.
        if !is_first_frame {
            let fps = self
                .decoder_a_meta()
                .or_else(|| self.decoder_b_meta())
                .map(|m| m.fps)
                .unwrap_or(25.0);
            let max_delay_ms = if self.sink_a.is_some() || self.sink_b.is_some() {
                playback::REPINT_AUDIO_MAX_MS
            } else {
                playback::REPINT_IDLE_MAX_MS
            };
            if fps > 0.0 {
                let delay = playback::next_frame_repaint_delay(
                    fps,
                    self.playback.current_pts,
                    max_delay_ms,
                );
                ctx.request_repaint_after(delay);
            } else {
                ctx.request_repaint();
            }
        }
    }

    fn drain_audio_and_update_levels(&mut self) {
        use ebur128::{EbuR128, Mode};

        // If playback is paused, let the levels slowly decay in the UI if needed,
        if !self.playback.is_playing {
            self.view.loudness_a.true_peak = [0.0, 0.0];
            self.view.loudness_b.true_peak = [0.0, 0.0];
            return;
        }

        // ── Channel A ────────────────────────────────────────────────────────
        let mut tp_a = [0.0f64, 0.0f64];
        if let Some(dec) = &mut self.decoder_a {
            if let Some(sink) = &self.sink_a {
                while let Ok(audio) = dec.audio_rx.try_recv() {
                    let channels = audio.channels as u32;
                    let sample_rate = audio.sample_rate as u32;

                    // Initialize or reset analyzer if format changes
                    if self.ebu_a.is_none()
                        || self.ebu_a.as_ref().unwrap().1 != channels
                        || self.ebu_a.as_ref().unwrap().2 != sample_rate
                    {
                        if let Ok(ebu) = EbuR128::new(
                            channels,
                            sample_rate,
                            Mode::I | Mode::M | Mode::S | Mode::TRUE_PEAK,
                        ) {
                            self.ebu_a = Some((ebu, channels, sample_rate));
                        }
                    }

                    if let Some((ebu, stored_ch, stored_rate)) = &mut self.ebu_a {
                        // For mono, we might need to duplicate to stereo for proper loudness calculation
                        // but ebur128 handles channel mapping if initialized correctly.
                        if channels == 1 {
                            // Interleave mono into stereo
                            let mut stereo = Vec::with_capacity(audio.samples.len() * 2);
                            for &s in &audio.samples {
                                stereo.push(s);
                                stereo.push(s);
                            }
                            // If we duplicated to stereo, ensure ebu analyzer is 2 channels
                            if *stored_ch != 2 {
                                if let Ok(new_ebu) = EbuR128::new(
                                    2,
                                    sample_rate,
                                    Mode::I | Mode::M | Mode::S | Mode::TRUE_PEAK,
                                ) {
                                    *ebu = new_ebu;
                                    *stored_ch = 2;
                                    *stored_rate = sample_rate;
                                }
                            }
                            if let Err(e) = ebu.add_frames_f32(&stereo) {
                                log::error!("Failed to add audio frames to Youlean/ebur128: {}", e);
                            }
                        } else {
                            if let Err(e) = ebu.add_frames_f32(&audio.samples) {
                                log::error!("Failed to add audio frames to Youlean/ebur128: {}", e);
                            }
                        }

                        let chs = *stored_ch;
                        tp_a[0] = tp_a[0].max(ebu.prev_true_peak(0).unwrap_or(0.0));
                        tp_a[1] = tp_a[1].max(if chs > 1 {
                            ebu.prev_true_peak(1).unwrap_or(0.0)
                        } else {
                            ebu.prev_true_peak(0).unwrap_or(0.0)
                        });

                        self.view.loudness_a = LoudnessResult {
                            momentary: ebu.loudness_momentary().unwrap_or(-120.0),
                            short_term: ebu.loudness_shortterm().unwrap_or(-120.0),
                            integrated: ebu.loudness_global().unwrap_or(-120.0),
                            true_peak: tp_a,
                        };
                    }

                    sink.append(rodio::buffer::SamplesBuffer::new(
                        audio.channels,
                        audio.sample_rate,
                        audio.samples,
                    ));
                }
            }
        }

        // ── Channel B ────────────────────────────────────────────────────────
        let mut tp_b = [0.0f64, 0.0f64];
        if let Some(dec) = &mut self.decoder_b {
            if let Some(sink) = &self.sink_b {
                while let Ok(audio) = dec.audio_rx.try_recv() {
                    let channels = audio.channels as u32;
                    let sample_rate = audio.sample_rate as u32;

                    if self.ebu_b.is_none()
                        || self.ebu_b.as_ref().unwrap().1 != channels
                        || self.ebu_b.as_ref().unwrap().2 != sample_rate
                    {
                        if let Ok(ebu) = EbuR128::new(
                            channels,
                            sample_rate,
                            Mode::I | Mode::M | Mode::S | Mode::TRUE_PEAK,
                        ) {
                            self.ebu_b = Some((ebu, channels, sample_rate));
                        }
                    }

                    if let Some((ebu, stored_ch, stored_rate)) = &mut self.ebu_b {
                        if channels == 1 {
                            let mut stereo = Vec::with_capacity(audio.samples.len() * 2);
                            for &s in &audio.samples {
                                stereo.push(s);
                                stereo.push(s);
                            }
                            if *stored_ch != 2 {
                                if let Ok(new_ebu) = EbuR128::new(
                                    2,
                                    sample_rate,
                                    Mode::I | Mode::M | Mode::S | Mode::TRUE_PEAK,
                                ) {
                                    *ebu = new_ebu;
                                    *stored_ch = 2;
                                    *stored_rate = sample_rate;
                                }
                            }
                            if let Err(e) = ebu.add_frames_f32(&stereo) {
                                log::error!("Failed to add audio frames to Youlean/ebur128: {}", e);
                            }
                        } else {
                            if let Err(e) = ebu.add_frames_f32(&audio.samples) {
                                log::error!("Failed to add audio frames to Youlean/ebur128: {}", e);
                            }
                        }

                        let chs = *stored_ch;
                        tp_b[0] = tp_b[0].max(ebu.prev_true_peak(0).unwrap_or(0.0));
                        tp_b[1] = tp_b[1].max(if chs > 1 {
                            ebu.prev_true_peak(1).unwrap_or(0.0)
                        } else {
                            ebu.prev_true_peak(0).unwrap_or(0.0)
                        });

                        self.view.loudness_b = LoudnessResult {
                            momentary: ebu.loudness_momentary().unwrap_or(-120.0),
                            short_term: ebu.loudness_shortterm().unwrap_or(-120.0),
                            integrated: ebu.loudness_global().unwrap_or(-120.0),
                            true_peak: tp_b,
                        };
                    }

                    sink.append(rodio::buffer::SamplesBuffer::new(
                        audio.channels,
                        audio.sample_rate,
                        audio.samples,
                    ));
                }
            }
        }
    }

    fn apply_sink_volumes(&mut self) {
        if let Some(sink) = &self.sink_a {
            if self.view.mute_a {
                sink.set_volume(0.0);
            } else {
                sink.set_volume(1.0); // Hardcoded fixed volume
            }
        }
        if let Some(sink) = &self.sink_b {
            if self.view.mute_b {
                sink.set_volume(0.0);
            } else {
                sink.set_volume(1.0); // Hardcoded fixed volume
            }
        }
    }

    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        if ctx.wants_keyboard_input() {
            return;
        }
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.pending_play_pause_toggle = true;
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                self.pending_key_action = PendingKeyAction::StepFwd;
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.pending_key_action = PendingKeyAction::StepBck;
            }
            if i.key_pressed(egui::Key::Home) {
                self.pending_key_action = PendingKeyAction::Seek(0.0);
            }
            if i.key_pressed(egui::Key::Y) {
                self.pending_key_action = PendingKeyAction::CycleMode;
            }
            if i.key_pressed(egui::Key::L) {
                if i.modifiers.shift {
                    self.pending_key_action = PendingKeyAction::ToggleLoopRange;
                } else {
                    self.pending_key_action = PendingKeyAction::SideBySide;
                }
            }
            if i.key_pressed(egui::Key::Num1) {
                self.pending_key_action = PendingKeyAction::SplitPos0;
            }
            if i.key_pressed(egui::Key::Num2) {
                self.pending_key_action = PendingKeyAction::SplitPos1;
            }
            if i.key_pressed(egui::Key::Num3) {
                self.pending_key_action = PendingKeyAction::ToggleHud;
            }
            if i.key_pressed(egui::Key::Num4) {
                self.pending_key_action = PendingKeyAction::Zoom(1.0);
            }
            if i.key_pressed(egui::Key::Num5) {
                self.pending_key_action = PendingKeyAction::Zoom(0.5);
            }
            if i.key_pressed(egui::Key::Num6) {
                self.pending_key_action = PendingKeyAction::Zoom(1.0);
            }
            if i.key_pressed(egui::Key::Num7) {
                self.pending_key_action = PendingKeyAction::Zoom(2.0);
            }
            if i.key_pressed(egui::Key::Num8) {
                self.pending_key_action = PendingKeyAction::Zoom(4.0);
            }
            if i.key_pressed(egui::Key::Num9) {
                self.pending_key_action = PendingKeyAction::Zoom(8.0);
            }
            if i.key_pressed(egui::Key::F) {
                log::trace!("Key 'F': xcap OS-native capture");
                let dir_for_thread = self.view.screenshot_dir.clone();

                std::thread::spawn(move || {
                    let mut success = false;
                    log::trace!("xcap: scanning OS windows");
                    if let Ok(windows) = xcap::Window::all() {
                        for window in windows {
                            if let Ok(title) = window.title() {
                                if title.contains("Production Media")
                                    || title.contains("Diferencial")
                                {
                                    log::trace!("xcap: window -> {}", title);
                                    if let Ok(img_buf) = window.capture_image() {
                                        if let Some(dir) = dir_for_thread.as_ref() {
                                            let timestamp =
                                                chrono::Local::now().format("%Y%m%d_%H%M%S");
                                            let filename = format!("WPP_QC_{timestamp}.png");
                                            let path = dir.join(filename);
                                            log::trace!("xcap: writing PNG to {:?}", path);

                                            if let Err(e) = img_buf.save(&path) {
                                                log::error!("xcap disk write error: {}", e);
                                            } else {
                                                log::trace!("xcap: screenshot saved");
                                                success = true;
                                            }
                                        }
                                    } else {
                                        log::error!("xcap failed to read window buffer");
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    if !success {
                        log::error!("xcap: target WPP window not found or capture failed");
                    }
                });
            }
            if i.key_pressed(egui::Key::R) {
                self.pending_key_action = PendingKeyAction::ResetZoomPan;
            }
            if i.key_pressed(egui::Key::S) {
                self.pending_key_action = PendingKeyAction::SwapVideos;
            }
            if i.key_pressed(egui::Key::I) {
                self.pending_key_action = PendingKeyAction::SetLoopIn;
            }
            if i.key_pressed(egui::Key::O) {
                self.pending_key_action = PendingKeyAction::SetLoopOut;
            }
            if i.key_pressed(egui::Key::X) && i.modifiers.shift {
                self.pending_key_action = PendingKeyAction::ClearLoopRange;
            }
            if i.key_pressed(egui::Key::M) {
                self.pending_key_action = PendingKeyAction::AddMarker;
            }

            let now = i.time;
            let repeat_delay = 0.25;
            let repeat_interval = 0.05;

            if i.key_down(egui::Key::ArrowRight) {
                if i.key_pressed(egui::Key::ArrowRight)
                    || (now - self.last_step_time) > repeat_interval
                {
                    let delay_ok = (now - self.last_step_time) > repeat_delay;
                    if i.key_pressed(egui::Key::ArrowRight) || delay_ok {
                        self.pending_key_action = PendingKeyAction::StepFwd;
                        self.last_step_time = now;
                    }
                }
            } else if i.key_down(egui::Key::ArrowLeft) {
                if i.key_pressed(egui::Key::ArrowLeft)
                    || (now - self.last_step_time) > repeat_interval
                {
                    let delay_ok = (now - self.last_step_time) > repeat_delay;
                    if i.key_pressed(egui::Key::ArrowLeft) || delay_ok {
                        self.pending_key_action = PendingKeyAction::StepBck;
                        self.last_step_time = now;
                    }
                }
            } else {
                self.last_step_time = 0.0;
            }
        });
    }

    fn show_hud_panels(&mut self, ctx: &egui::Context, is_first_frame: bool) {
        if !self.view.show_hud || is_first_frame {
            return;
        }
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            crate::ui::controls::show_menu_bar(ui, self);
        });
        if self.view.show_left_panel {
            egui::SidePanel::left("info_panel")
                .resizable(true)
                .default_width(260.0)
                .min_width(200.0)
                .max_width(340.0)
                .show(ctx, |ui| {
                    crate::ui::info_panel::show(ui, self);
                });
        }
        if self.view.show_right_panel {
            egui::SidePanel::right("audio_panel")
                .resizable(true)
                .default_width(110.0)
                .min_width(90.0)
                .max_width(220.0)
                .show(ctx, |ui| {
                    crate::ui::controls::show_audio_panel(ui, self);
                });
        }
        egui::TopBottomPanel::bottom("timeline").show(ctx, |ui| {
            crate::ui::timeline::show(ui, self);
        });
    }

    fn show_clean_feed_viewport(&mut self, ctx: &egui::Context) {
        if !self.view.show_clean_feed_window {
            return;
        }
        let mut show = self.view.show_clean_feed_window;
        let renderer_clone = Arc::clone(&self.renderer);
        let title = crate::ui::controls::clean_feed_window_title(self.view.lang);

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("clean_feed_viewport"),
            egui::ViewportBuilder::default()
                .with_title(title)
                .with_inner_size([1280.0, 720.0])
                .with_always_on_top(),
            |ctx, _class| {
                if ctx.input(|i| i.viewport().close_requested()) {
                    show = false;
                }

                let fps = self
                    .decoder_a_meta()
                    .map(|m| m.fps)
                    .filter(|f| *f > 0.0)
                    .unwrap_or(24.0);
                let overlay_text = crate::ui::controls::clean_feed_overlay_text(
                    self.view.lang,
                    self.view.mode,
                    self.view.split_pos,
                    self.playback.current_pts,
                    fps,
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    let available = ui.available_rect_before_wrap();
                    ui.allocate_rect(available, egui::Sense::hover());
                    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                        available,
                        RenderCallback {
                            renderer: renderer_clone.clone(),
                        },
                    ));

                    let text_pos = available.min + egui::vec2(20.0, 20.0);
                    let galley = ui.painter().layout_no_wrap(
                        overlay_text,
                        egui::FontId::proportional(22.0),
                        egui::Color32::WHITE,
                    );
                    let bg_rect = galley.rect.translate(text_pos.to_vec2()).expand(6.0);
                    ui.painter()
                        .rect_filled(bg_rect, 4.0, egui::Color32::from_black_alpha(150));
                    ui.painter().galley(text_pos, galley, egui::Color32::WHITE);
                });
            },
        );

        self.view.show_clean_feed_window = show;
    }

    fn show_proxy_progress_window(&mut self, ctx: &egui::Context) {
        if !self.proxy_running() {
            return;
        }
        let progress = self.proxy_progress();
        let cap = crate::ui::controls::proxy_loading_caption(self.view.lang);
        egui::Window::new(cap)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(12.0);
                    ui.label(cap);
                    ui.add_space(8.0);
                    ui.add(egui::ProgressBar::new(progress.clamp(0.0, 1.0)).show_percentage());
                    ui.add_space(12.0);
                });
            });
    }

    fn show_error_modal_if_any(&mut self, ctx: &egui::Context) {
        let (title, msg) = match (&self.error_title, &self.error_message) {
            (Some(t), Some(m)) => (t.clone(), m.clone()),
            _ => return,
        };
        let lang = self.view.lang;
        let mut open = true;
        egui::Window::new(
            egui::RichText::new(&title)
                .color(egui::Color32::from_rgb(255, 100, 100))
                .strong(),
        )
        .collapsible(false)
        .resizable(false)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.screen_rect().center())
        .fixed_size(egui::vec2(400.0, 150.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.label(egui::RichText::new(&msg).size(15.0));
                ui.add_space(25.0);
                let ok = crate::ui::design::dialog_ok(lang);
                if ui
                    .button(egui::RichText::new(format!("   {ok}   ")).strong())
                    .clicked()
                {
                    open = false;
                }
                ui.add_space(10.0);
            });
        });
        if !open {
            self.error_title = None;
            self.error_message = None;
        }
    }
}

impl eframe::App for DiffPlayerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let is_first_frame = self.frame_count == 0;
        if self.frame_count == 0 {
            self.frame_count = 1;
        } else {
            self.frame_count = self.frame_count.saturating_add(1);
        }
        // macOS: repeatedly send Focus + Visible for first frames so window appears and comes to front.
        if self.focus_visible_frames_left > 0 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            self.focus_visible_frames_left -= 1;
        }
        // Process deferred play/pause toggle (Space) so we don't run play_both/pause_both from inside ctx.input() (avoids re-entrancy/deadlock on macOS).
        if self.pending_play_pause_toggle {
            self.pending_play_pause_toggle = false;
            if self.playback.is_playing {
                self.pause_both(ctx);
            } else {
                self.play_both(ctx);
            }
        }

        if let Some(target) = self.view.pending_play_after_delay {
            if std::time::Instant::now() >= target {
                self.view.pending_play_after_delay = None;
                if !self.playback.is_playing {
                    self.play_both(ctx);
                }
            } else {
                ctx.request_repaint(); // Keep repainting until the time is reached
            }
        }
        // Process deferred key action (arrows, Home, Y, L, Num1–9, R, S) so we never call ctx or decoder from inside ctx.input().
        match std::mem::take(&mut self.pending_key_action) {
            PendingKeyAction::None => {}
            PendingKeyAction::StepFwd => self.do_step_fwd_inner(ctx),
            PendingKeyAction::StepBck => self.do_step_bck_inner(ctx),
            PendingKeyAction::Seek(t) => self.do_seek_inner(t, ctx),
            PendingKeyAction::CycleMode => {
                self.view.mode = match self.view.mode {
                    CompareMode::SplitScreen => CompareMode::AbsDiff,
                    CompareMode::AbsDiff => CompareMode::Heatmap,
                    CompareMode::Heatmap => CompareMode::SideBySide,
                    CompareMode::SideBySide => CompareMode::SplitScreen,
                };
                ctx.request_repaint();
            }
            PendingKeyAction::SideBySide => {
                self.view.mode = CompareMode::SideBySide;
                ctx.request_repaint();
            }
            PendingKeyAction::SplitPos0 => {
                self.view.mode = CompareMode::SplitScreen;
                self.view.split_pos = if self.view.split_pos < 0.05 { 0.5 } else { 0.0 };
                ctx.request_repaint();
            }
            PendingKeyAction::SplitPos1 => {
                self.view.mode = CompareMode::SplitScreen;
                self.view.split_pos = if self.view.split_pos > 0.95 { 0.5 } else { 1.0 };
                ctx.request_repaint();
            }
            PendingKeyAction::ToggleHud => {
                self.view.show_hud = !self.view.show_hud;
            }
            PendingKeyAction::Zoom(z) => {
                self.view.zoom = z;
            }
            PendingKeyAction::ResetZoomPan => {
                self.view.zoom = 1.0;
                self.view.pan_u = 0.0;
                self.view.pan_v = 0.0;
            }
            PendingKeyAction::SwapVideos => self.swap_videos_inner(ctx),
            PendingKeyAction::SetLoopIn => self.set_loop_in(),
            PendingKeyAction::SetLoopOut => self.set_loop_out(),
            PendingKeyAction::ToggleLoopRange => self.toggle_loop_range(),
            PendingKeyAction::ClearLoopRange => {
                self.playback.loop_in = None;
                self.playback.loop_out = None;
                self.playback.loop_range_active = false;
            }
            PendingKeyAction::AddMarker => {
                let m = crate::types::Marker {
                    pts: self.playback.current_pts,
                    note: "New Marker".to_string(),
                    color: [0.0, 0.7, 1.0],
                    channel_hint: None,
                };
                self.session.markers.push(m);
            }
        }
        // When proxy generation just finished: load proxy.mkv into the target channel.
        self.complete_proxy_if_ready(ctx);
        log::trace!("App::update() tick");
        // First frame: don't request repaint so macOS can finish present.
        // Later: only schedule repaint when playing (request_repaint_after); when paused, input triggers repaint.

        // Master clock and repaint cadence.
        self.update_master_clock_and_repaint(ctx, is_first_frame);
        // Keep UI updating while proxy generation is running
        if self.proxy_running() {
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        // Skip Wgpu work on first frame so window can appear on macOS (avoids first-frame block).
        if !is_first_frame {
            if let Some(rs) = frame.wgpu_render_state() {
                if self.drain_frames(rs) {
                    ctx.request_repaint();
                }
            }
            self.sync_uniforms();
        }
        self.drain_audio_and_update_levels();
        self.apply_sink_volumes();

        // ── Handle screenshot events ────────────────────────────────────────
        let events = ctx.input(|i| i.raw.events.clone());
        // (Wgpu Screenshot event listener removed to use OS-native xcap instead)
        for _event in events {
            // Processing other events...
        }

        self.handle_keyboard_input(ctx);

        // ── UI Overlay conditionally rendered ───────────────────────────────
        // Skip HUD on first frame (macOS Metal): minimal first frame so present can complete.
        self.show_hud_panels(ctx, is_first_frame);

        // ── Central canvas ──────────────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            show_canvas(ui, self, frame);
        });

        self.show_clean_feed_viewport(ctx);
        self.show_proxy_progress_window(ctx);
        crate::ui::vu_meter::show_vu_meter_window(ctx, self);
        crate::ui::markers::show(ctx, self);
        self.show_error_modal_if_any(ctx);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.view.save();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        log::info!("Application exiting, triggering final save");
        self.view.save();
        for dir in &self.proxy_temp_dirs {
            if let Err(e) = std::fs::remove_dir_all(dir) {
                log::warn!("Failed to remove proxy temp dir {:?}: {}", dir, e);
            } else {
                log::info!("Removed proxy temp dir: {:?}", dir);
            }
        }
        self.proxy_temp_dirs.clear();
        if let Some(ref dir) = self.proxy_temp_dir {
            if let Err(e) = std::fs::remove_dir_all(dir) {
                log::warn!("Failed to remove proxy temp dir {:?}: {}", dir, e);
            }
            self.proxy_temp_dir = None;
        }
    }
}

// ---------------------------------------------------------------------------
//  Video canvas with zoom / pan interaction
// ---------------------------------------------------------------------------

fn show_canvas(ui: &mut egui::Ui, app: &mut DiffPlayerApp, _frame: &mut eframe::Frame) {
    let available = ui.available_rect_before_wrap();
    app.view.canvas_rect = available;

    let response = ui.allocate_rect(available, egui::Sense::click_and_drag());

    // -- Mouse wheel zoom ---------------------------------------------------
    let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
    if response.hovered() && scroll_delta != 0.0 {
        let zoom_factor = if scroll_delta > 0.0 {
            1.1f32
        } else {
            1.0 / 1.1
        };
        app.view.zoom = (app.view.zoom * zoom_factor).clamp(0.25, 32.0);
    }

    // -- Drag to pan OR drag split line (Available in all modes) -------------
    // Pan is only active when zoomed in (zoom > 1.0). At fit-to-frame only the
    // split divider can be dragged. Split line is vertical or horizontal per split_horizontal.
    if response.drag_started() {
        let pos = response.interact_pointer_pos().unwrap_or_default();
        let near_split = if app.view.split_horizontal {
            let split_y = available.top() + app.view.split_pos * available.height();
            (pos.y - split_y).abs() < 15.0
        } else {
            let split_x = available.left() + app.view.split_pos * available.width();
            (pos.x - split_x).abs() < 15.0
        };
        if near_split {
            app.dragging_split = true;
        } else {
            app.dragging_split = false;
            if app.view.zoom > 1.0 {
                app.drag_start = Some((pos, app.view.pan_u, app.view.pan_v));
            }
        }
    }

    if response.dragged() {
        if app.dragging_split {
            let pos = response.interact_pointer_pos().unwrap_or_default();
            if app.view.split_horizontal {
                let relative_y = (pos.y - available.top()) / available.height();
                app.view.split_pos = relative_y.clamp(0.0, 1.0);
            } else {
                let relative_x = (pos.x - available.left()) / available.width();
                app.view.split_pos = relative_x.clamp(0.0, 1.0);
            }
            ui.ctx().request_repaint();
        } else if let Some((start_pos, start_pu, start_pv)) = app.drag_start {
            let delta = response.interact_pointer_pos().unwrap_or_default() - start_pos;
            let uv_delta_u = -delta.x / available.width() / app.view.zoom;
            let uv_delta_v = -delta.y / available.height() / app.view.zoom;
            app.view.pan_u = (start_pu + uv_delta_u).clamp(-0.5, 0.5);
            app.view.pan_v = (start_pv + uv_delta_v).clamp(-0.5, 0.5);
            ui.ctx().request_repaint();
        }
    }

    if response.drag_stopped() {
        app.drag_start = None;
        app.dragging_split = false;
    }

    // -- Cursor hint for dragging split (Available in all modes) ------------
    if let Some(ptr) = ui.ctx().pointer_hover_pos() {
        let near_split = if app.view.split_horizontal {
            let split_y = available.top() + app.view.split_pos * available.height();
            available.contains(ptr) && (ptr.y - split_y).abs() < 10.0
        } else {
            let split_x = available.left() + app.view.split_pos * available.width();
            available.contains(ptr) && (ptr.x - split_x).abs() < 10.0
        };
        if near_split {
            ui.ctx().set_cursor_icon(if app.view.split_horizontal {
                egui::CursorIcon::ResizeVertical
            } else {
                egui::CursorIcon::ResizeHorizontal
            });
        }
    }

    // -- Double-click to reset zoom -----------------------------------------
    if response.double_clicked() {
        app.view.zoom = 1.0;
        app.view.pan_u = 0.0;
        app.view.pan_v = 0.0;
    }

    // -- Draw the wgpu render callback into this rect ----------------------
    // Skip on first frame so macOS window can appear (first Wgpu present can block).
    if app.frame_count > 1 {
        let renderer_clone = Arc::clone(&app.renderer);
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            available,
            RenderCallback {
                renderer: renderer_clone,
            },
        ));
    } else {
        ui.painter()
            .rect_filled(available, 0.0, egui::Color32::from_rgb(0, 0, 0));
    }

    // -- Safe zones overlay (video_rect + zoom/pan) -------------------------
    // In SideBySide mode draw on both halves (A left, B right); otherwise once on full canvas.
    if app.view.safe_zone != SafeZoneMode::None {
        let zoom = app.view.zoom;
        let visible_left = 0.5 - 0.5 / zoom + app.view.pan_u;
        let visible_right = 0.5 + 0.5 / zoom + app.view.pan_u;
        let visible_top = 0.5 - 0.5 / zoom + app.view.pan_v;
        let visible_bottom = 0.5 + 0.5 / zoom + app.view.pan_v;

        let draw_safe_zones = |container: egui::Rect, vw: f32, vh: f32| {
            let cw = container.width();
            let ch = container.height();
            let video_aspect = vw / vh;
            let container_aspect = cw / ch;
            let video_rect = if video_aspect >= container_aspect {
                let h = cw / video_aspect;
                let top = container.center().y - h * 0.5;
                egui::Rect::from_min_max(
                    egui::Pos2::new(container.left(), top),
                    egui::Pos2::new(container.right(), top + h),
                )
            } else {
                let w = ch * video_aspect;
                let left = container.center().x - w * 0.5;
                egui::Rect::from_min_max(
                    egui::Pos2::new(left, container.top()),
                    egui::Pos2::new(left + w, container.bottom()),
                )
            };
            let uv_to_screen = |u: f32, v: f32| {
                let x = video_rect.left()
                    + (u - visible_left) / (visible_right - visible_left) * video_rect.width();
                let y = video_rect.top()
                    + (v - visible_top) / (visible_bottom - visible_top) * video_rect.height();
                egui::Pos2::new(x, y)
            };

            match app.view.safe_zone {
                SafeZoneMode::None => {}
                SafeZoneMode::TvEbu => {
                    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 200, 255));
                    let action_min = uv_to_screen(0.035, 0.035);
                    let action_max = uv_to_screen(0.965, 0.965);
                    let action_rect = egui::Rect::from_min_max(action_min, action_max);
                    ui.painter().rect_stroke(action_rect, 0.0, stroke);
                    let title_min = uv_to_screen(0.10, 0.05);
                    let title_max = uv_to_screen(0.90, 0.95);
                    let title_rect = egui::Rect::from_min_max(title_min, title_max);
                    ui.painter().rect_stroke(title_rect, 0.0, stroke);
                    let center = uv_to_screen(0.5, 0.5);
                    let cross_half = 10.0;
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(center.x - cross_half, center.y),
                            egui::Pos2::new(center.x + cross_half, center.y),
                        ],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(center.x, center.y - cross_half),
                            egui::Pos2::new(center.x, center.y + cross_half),
                        ],
                        stroke,
                    );
                }
                SafeZoneMode::Social => {
                    let danger_fill = egui::Color32::from_black_alpha(150);
                    let top_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.0), uv_to_screen(1.0, 0.15));
                    let bottom_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.78), uv_to_screen(1.0, 1.0));
                    let right_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.85, 0.0), uv_to_screen(1.0, 1.0));
                    let left_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.0), uv_to_screen(0.05, 1.0));
                    ui.painter().rect_filled(top_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(bottom_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(right_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(left_danger, 0.0, danger_fill);
                    let safe_min = uv_to_screen(0.05, 0.15);
                    let safe_max = uv_to_screen(0.85, 0.78);
                    let safe_rect = egui::Rect::from_min_max(safe_min, safe_max);
                    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 200, 0));
                    ui.painter().rect_stroke(safe_rect, 0.0, stroke);
                }
            }
        };

        if app.view.mode == CompareMode::SideBySide {
            let mid_x = available.center().x;
            let left_rect =
                egui::Rect::from_min_max(available.min, egui::pos2(mid_x, available.max.y));
            let right_rect =
                egui::Rect::from_min_max(egui::pos2(mid_x, available.min.y), available.max);
            let (vw_a, vh_a) = app
                .decoder_a_meta()
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            let (vw_b, vh_b) = app
                .decoder_b_meta()
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            draw_safe_zones(left_rect, vw_a, vh_a);
            draw_safe_zones(right_rect, vw_b, vh_b);
        } else {
            let (vw, vh) = app
                .decoder_a_meta()
                .or_else(|| app.decoder_b_meta())
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            draw_safe_zones(available, vw, vh);
        }
    }

    // -- OS file drag-and-drop handling ------------------------------------
    let hovered_files = ui.ctx().input(|i| i.raw.hovered_files.clone());
    let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());

    // IMPORTANT: Handle the actual drop FIRST, before we potentially clear
    // drag_drop_hover_pos in the else branch below. On the drop frame,
    // hovered_files is already empty but drag_drop_hover_pos still holds
    // the last valid cursor position from the previous frame.
    if !dropped_files.is_empty() {
        // Collect paths for EXR or video handling
        let paths: Vec<PathBuf> = dropped_files
            .iter()
            .filter_map(|f| f.path.as_ref().map(PathBuf::from))
            .collect();

        // EXR: single directory -> proxy from folder; all .exr files -> proxy from list. Target channel from drop position.
        let mid_x = available.center().x;
        let hover_x = app
            .drag_drop_hover_pos
            .or_else(|| ui.ctx().pointer_hover_pos())
            .unwrap_or(available.center())
            .x;
        let target_chan = if hover_x < mid_x {
            crate::types::Channel::A
        } else {
            crate::types::Channel::B
        };
        if paths.len() == 1 && paths[0].is_dir() {
            app.start_proxy_from_exr_input_dir(paths[0].clone(), target_chan, ui.ctx());
            app.drag_drop_hover_pos = None;
            return;
        }
        let all_exr = !paths.is_empty()
            && paths.iter().all(|p| {
                p.extension()
                    .map(|e| {
                        e.to_str()
                            .map(|s| s.eq_ignore_ascii_case("exr"))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            });
        if all_exr {
            app.start_proxy_from_exr_input_files(paths, target_chan, ui.ctx());
            app.drag_drop_hover_pos = None;
            return;
        }

        // Video handling
        let valid_extensions = [
            "mp4", "mov", "mxf", "mkv", "avi", "prores", "mts", "mpg", "mpeg", "ts",
        ];
        let mut valid_paths = Vec::new();
        let mut invalid_files = Vec::new();

        for path in &paths {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if valid_extensions.contains(&ext.as_str()) {
                valid_paths.push(path.to_string_lossy().to_string());
            } else {
                invalid_files.push(
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }

        if !invalid_files.is_empty() {
            app.error_title = Some("Formato no soportado".to_string());
            app.error_message = Some(format!(
                "Los siguientes archivos no son formatos soportados:\n{}",
                invalid_files.join(", ")
            ));
        } else if valid_paths.len() > 2 {
            app.error_title = Some("Máximo 2 videos".to_string());
            app.error_message =
                Some("Solo puedes arrastrar un máximo de 2 videos a la vez.".to_string());
        } else if valid_paths.len() == 2 {
            valid_paths.sort(); // A goes to Slot A, B goes to Slot B alphabetically
            app.open_video_a_from_path(valid_paths[0].clone(), ui.ctx());
            app.open_video_b_from_path(valid_paths[1].clone(), ui.ctx());
        } else if !valid_paths.is_empty() {
            let mid_x = available.center().x;
            let hover_x = app
                .drag_drop_hover_pos
                .or_else(|| ui.ctx().pointer_hover_pos())
                .unwrap_or(available.center())
                .x;
            if hover_x < mid_x {
                app.open_video_a_from_path(valid_paths[0].clone(), ui.ctx());
            } else {
                app.open_video_b_from_path(valid_paths[0].clone(), ui.ctx());
            }
        }

        app.drag_drop_hover_pos = None;
    } else if !hovered_files.is_empty() {
        // Files are being dragged over — update position and draw overlay
        if let Some(ptr) = ui.ctx().pointer_hover_pos() {
            app.drag_drop_hover_pos = Some(ptr);
        }

        let mid_x = available.center().x;
        let hover_x = app.drag_drop_hover_pos.map(|p| p.x).unwrap_or(mid_x);
        let targeting_a = hover_x < mid_x;

        let (a_alpha, b_alpha) = if targeting_a {
            (80u8, 30u8)
        } else {
            (30u8, 80u8)
        };

        let left_rect = egui::Rect::from_min_max(available.min, egui::pos2(mid_x, available.max.y));
        let right_rect =
            egui::Rect::from_min_max(egui::pos2(mid_x, available.min.y), available.max);

        ui.painter().rect_filled(
            left_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(80, 180, 100, a_alpha),
        );
        ui.painter().rect_filled(
            right_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(80, 130, 220, b_alpha),
        );

        let is_es = app.view.lang == Language::Es;
        let label_a = if is_es {
            "Soltar aquí → VIDEO A"
        } else {
            "Drop here → VIDEO A"
        };
        let label_b = if is_es {
            "Soltar aquí → VIDEO B"
        } else {
            "Drop here → VIDEO B"
        };
        ui.painter().text(
            left_rect.center(),
            egui::Align2::CENTER_CENTER,
            label_a,
            egui::FontId::proportional(22.0),
            egui::Color32::from_rgba_premultiplied(220, 255, 220, 230),
        );
        ui.painter().text(
            right_rect.center(),
            egui::Align2::CENTER_CENTER,
            label_b,
            egui::FontId::proportional(22.0),
            egui::Color32::from_rgba_premultiplied(200, 220, 255, 230),
        );
        ui.painter().vline(
            mid_x,
            available.y_range(),
            egui::Stroke::new(
                2.0,
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 120),
            ),
        );

        ui.ctx().request_repaint();
    } else {
        // Nothing dragged — clear stored position
        app.drag_drop_hover_pos = None;
    }

    // -- Overlay: "No video" message when nothing is loaded ----------------
    let has_a = app.decoder_a.is_some();
    let has_b = app.decoder_b.is_some();
    if !has_a || !has_b {
        let center = available.center();
        let is_es = app.view.lang == Language::Es;
        let text = if !has_a && !has_b {
            if is_es {
                "Abre el Vídeo A y el Vídeo B para empezar la comparación"
            } else {
                "Open Video A and Video B to begin comparison"
            }
        } else if !has_a {
            if is_es {
                "Abre el Vídeo A  ←  (panel izquierdo)"
            } else {
                "Open Video A  ←  (left panel)"
            }
        } else {
            if is_es {
                "Abre el Vídeo B  →  (panel izquierdo)"
            } else {
                "Open Video B  →  (left panel)"
            }
        };
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(20.0),
            ui.visuals().text_color().gamma_multiply(0.5),
        );
    }

    // -- Zoom indicator overlay (top-right of canvas) ----------------------
    if (app.view.zoom - 1.0).abs() > 0.01 {
        let zoom_text = format!("{:.1}×", app.view.zoom);
        let pos = egui::pos2(available.right() - 8.0, available.top() + 8.0);
        ui.painter().text(
            pos,
            egui::Align2::RIGHT_TOP,
            &zoom_text,
            egui::FontId::monospace(13.0),
            egui::Color32::from_rgba_premultiplied(200, 200, 100, 200),
        );
    }

    // -- Frame counter overlay (bottom-left of canvas, unobtrusive) --------
    // Shows permanently, including during screenshots.
    {
        let fps_a = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
        let current_pts = app.playback().current_pts;
        let frame_num = (current_pts * fps_a).round() as u64;
        let is_es = app.view().lang == Language::Es;

        let frame_text = format!("{} {}", if is_es { "Fr." } else { "Frame" }, frame_num);
        let pos = egui::pos2(available.left() + 8.0, available.bottom() - 8.0);
        ui.painter().text(
            pos,
            egui::Align2::LEFT_BOTTOM,
            &frame_text,
            egui::FontId::monospace(14.0),
            egui::Color32::from_black_alpha(150), // Subtle shadow
        );
        ui.painter().text(
            pos - egui::Vec2::new(1.0, 1.0),
            egui::Align2::LEFT_BOTTOM,
            &frame_text,
            egui::FontId::monospace(14.0),
            egui::Color32::from_white_alpha(150), // Unobtrusive text
        );
    }
}

fn default_rect() -> egui::Rect {
    egui::Rect::NOTHING
}

// ---------------------------------------------------------------------------
//  Font setup
// ---------------------------------------------------------------------------

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // List of common font paths for different OSes
    let font_paths = [
        "C:/Windows/Fonts/arial.ttf",                   // Windows
        "/Library/Fonts/Arial.ttf",                     // macOS
        "/System/Library/Fonts/Supplemental/Arial.ttf", // macOS Supplemental
        "/Library/Fonts/Helvetica.ttc",                 // macOS Helvetica fallback
    ];

    for path in font_paths {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert("DefaultFont".to_owned(), egui::FontData::from_owned(bytes));
            // Insert at the front of the proportional list
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "DefaultFont".to_owned());
            // Also use as monospace fallback
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("DefaultFont".to_owned());
            log::info!("Loaded font from: {:?}", path);
            break;
        }
    }

    ctx.set_fonts(fonts);

    // Apply overall style tweaks
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.slider_width = 120.0;
    ctx.set_style(style);
}

// Expose app fields the UI modules need
impl DiffPlayerApp {
    pub fn view_mut(&mut self) -> &mut ViewState {
        &mut self.view
    }
    pub fn view(&self) -> &ViewState {
        &self.view
    }
    pub fn playback(&self) -> &PlaybackState {
        &self.playback
    }
    pub fn playback_mut(&mut self) -> &mut PlaybackState {
        &mut self.playback
    }
    pub fn decoder_a_meta(&self) -> Option<&ColorMetadata> {
        self.decoder_a.as_ref().map(|d| &d.meta)
    }
    pub fn decoder_b_meta(&self) -> Option<&ColorMetadata> {
        self.decoder_b.as_ref().map(|d| &d.meta)
    }
    pub fn decoder_a_path(&self) -> Option<&str> {
        self.decoder_a.as_ref().map(|d| d.path.as_str())
    }
    pub fn decoder_b_path(&self) -> Option<&str> {
        self.decoder_b.as_ref().map(|d| d.path.as_str())
    }
    pub fn open_video_a(&mut self, ctx: &egui::Context) {
        self.open_video(Channel::A, ctx);
    }
    pub fn open_video_b(&mut self, ctx: &egui::Context) {
        self.open_video(Channel::B, ctx);
    }
    pub fn open_video_a_from_path(&mut self, path: String, ctx: &egui::Context) {
        self.open_video_from_path(path, Channel::A, ctx);
    }
    pub fn open_video_b_from_path(&mut self, path: String, ctx: &egui::Context) {
        self.open_video_from_path(path, Channel::B, ctx);
    }
    pub fn do_play(&mut self, _ctx: &egui::Context) {
        self.pending_play_pause_toggle = true;
    }
    pub fn do_pause(&mut self, _ctx: &egui::Context) {
        self.pending_play_pause_toggle = true;
    }
    /// Enqueue step forward; processed at start of next update (avoids re-entrancy from keyboard/UI).
    pub fn do_step_fwd(&mut self, _ctx: &egui::Context) {
        self.pending_key_action = PendingKeyAction::StepFwd;
    }
    /// Enqueue step back; processed at start of next update (avoids re-entrancy from keyboard/UI).
    pub fn do_step_bck(&mut self, _ctx: &egui::Context) {
        self.pending_key_action = PendingKeyAction::StepBck;
    }

    pub fn calculate_psnr(&mut self) {
        self.view.last_psnr = None;
        if let (Some(dec_a), Some(dec_b)) = (&self.decoder_a, &self.decoder_b) {
            if let (Some(frame_a), Some(frame_b)) = (&dec_a.last_frame, &dec_b.last_frame) {
                self.view.last_psnr =
                    crate::metrics::compute_psnr(&frame_a.rgba_data, &frame_b.rgba_data);
            }
        }
    }

    /// Called from start of update() when pending_key_action was StepFwd.
    fn do_step_fwd_inner(&mut self, ctx: &egui::Context) {
        if self.playback.is_playing {
            self.pause_both(ctx);
        }
        self.step_forward(ctx);
    }
    /// Called from start of update() when pending_key_action was StepBck.
    fn do_step_bck_inner(&mut self, ctx: &egui::Context) {
        if self.playback.is_playing {
            self.pause_both(ctx);
        }
        let fps = match (self.decoder_a_meta(), self.decoder_b_meta()) {
            (Some(a), _) if a.fps > 0.0 => a.fps,
            (_, Some(b)) if b.fps > 0.0 => b.fps,
            _ => 25.0,
        };
        let t = (self.playback.current_pts - 1.0 / fps).max(0.0);
        self.do_seek_inner(t, ctx);
    }
    pub fn do_seek(&mut self, t: f64, ctx: &egui::Context) {
        self.do_seek_inner(t, ctx);
    }
    fn do_seek_inner(&mut self, t: f64, ctx: &egui::Context) {
        crate::trace_log::log(&format!("Seek to {:.3}s", t));
        self.seek_both(t, ctx);
        self.playback.current_pts = t;

        // Clear audio sink buffers since we are jumping in time
        if let Some(s) = &self.sink_a {
            s.clear();
            s.play();
        }
        if let Some(s) = &self.sink_b {
            s.clear();
            s.play();
        }
        if !self.playback.is_playing {
            if let Some(s) = &self.sink_a {
                s.pause();
            }
            if let Some(s) = &self.sink_b {
                s.pause();
            }
        }

        // Discard any trailing frames in the pipeline so the next frame is exactly the requested one
        if let Some(dec) = &mut self.decoder_a {
            dec.next_frame = None;
            while dec.frame_rx.try_recv().is_ok() {}
            while dec.audio_rx.try_recv().is_ok() {}
        }
        if let Some(dec) = &mut self.decoder_b {
            dec.next_frame = None;
            while dec.frame_rx.try_recv().is_ok() {}
            while dec.audio_rx.try_recv().is_ok() {}
        }

        // Restore decoder playback state if we were playing. Decoder threads pause automatically on seek.
        if self.playback.is_playing {
            self.play_both(ctx);
        }
        ctx.request_repaint();
    }
}

fn default_true() -> bool {
    true
}
</file>

<file path="src/ui/theme.rs">
//! egui visual presets. Selection/hover accents are defined per palette below; the timeline and
//! several controls use [`crate::ui::design::ACCENT_PRIMARY`] so the default blue accent stays
//! consistent across widgets.

use egui::{Color32, Context, Stroke, Visuals};

pub fn apply_theme(ctx: &Context, theme: crate::types::Theme) {
    match theme {
        crate::types::Theme::Dark => ctx.set_visuals(Visuals::dark()),
        crate::types::Theme::Light => ctx.set_visuals(Visuals::light()),
        _ => {
            let (is_dark, bg, panel, accent, text) = match theme {
                crate::types::Theme::Rust => (
                    true,
                    Color32::from_rgb(43, 43, 43),
                    Color32::from_rgb(32, 32, 32),
                    Color32::from_rgb(252, 60, 20),
                    Color32::from_rgb(240, 230, 220),
                ),
                crate::types::Theme::SolarizedDark => (
                    true,
                    Color32::from_rgb(0, 43, 54),
                    Color32::from_rgb(7, 54, 66),
                    Color32::from_rgb(181, 137, 0),
                    Color32::from_rgb(131, 148, 150),
                ),
                crate::types::Theme::SolarizedLight => (
                    false,
                    Color32::from_rgb(253, 246, 227),
                    Color32::from_rgb(238, 232, 213),
                    Color32::from_rgb(38, 139, 210),
                    Color32::from_rgb(101, 123, 131),
                ),
                crate::types::Theme::Dracula => (
                    true,
                    Color32::from_rgb(40, 42, 54),
                    Color32::from_rgb(68, 71, 90),
                    Color32::from_rgb(189, 147, 249),
                    Color32::from_rgb(248, 248, 242),
                ),
                crate::types::Theme::Gruvbox => (
                    true,
                    Color32::from_rgb(40, 40, 40),
                    Color32::from_rgb(60, 56, 54),
                    Color32::from_rgb(250, 189, 47),
                    Color32::from_rgb(235, 219, 178),
                ),
                crate::types::Theme::Nord => (
                    true,
                    Color32::from_rgb(46, 52, 64),
                    Color32::from_rgb(59, 66, 82),
                    Color32::from_rgb(136, 192, 208),
                    Color32::from_rgb(236, 239, 244),
                ),
                crate::types::Theme::Monokai => (
                    true,
                    Color32::from_rgb(39, 40, 34),
                    Color32::from_rgb(62, 61, 50),
                    Color32::from_rgb(249, 38, 114),
                    Color32::from_rgb(248, 248, 242),
                ),
                crate::types::Theme::OneDark => (
                    true,
                    Color32::from_rgb(40, 44, 52),
                    Color32::from_rgb(44, 49, 58),
                    Color32::from_rgb(97, 175, 239),
                    Color32::from_rgb(171, 178, 191),
                ),
                crate::types::Theme::OneLight => (
                    false,
                    Color32::from_rgb(250, 250, 250),
                    Color32::from_rgb(240, 240, 240),
                    Color32::from_rgb(82, 111, 255),
                    Color32::from_rgb(56, 58, 66),
                ),
                crate::types::Theme::Catppuccin => (
                    true,
                    Color32::from_rgb(30, 30, 46),
                    Color32::from_rgb(24, 24, 37),
                    Color32::from_rgb(203, 166, 247),
                    Color32::from_rgb(205, 214, 244),
                ),
                crate::types::Theme::TokyoNight => (
                    true,
                    Color32::from_rgb(26, 27, 38),
                    Color32::from_rgb(22, 22, 30),
                    Color32::from_rgb(122, 162, 247),
                    Color32::from_rgb(192, 202, 245),
                ),
                crate::types::Theme::NightOwl => (
                    true,
                    Color32::from_rgb(1, 22, 39),
                    Color32::from_rgb(11, 41, 66),
                    Color32::from_rgb(130, 170, 255),
                    Color32::from_rgb(214, 222, 235),
                ),
                crate::types::Theme::Ayc => (
                    true,
                    Color32::from_rgb(15, 20, 25),
                    Color32::from_rgb(20, 25, 31),
                    Color32::from_rgb(230, 180, 80),
                    Color32::from_rgb(191, 186, 176),
                ),
                crate::types::Theme::MaterialDesign => (
                    true,
                    Color32::from_rgb(38, 50, 56),
                    Color32::from_rgb(55, 71, 79),
                    Color32::from_rgb(128, 203, 196),
                    Color32::from_rgb(236, 239, 241),
                ),
                crate::types::Theme::Everforest => (
                    true,
                    Color32::from_rgb(43, 51, 57),
                    Color32::from_rgb(50, 60, 65),
                    Color32::from_rgb(167, 192, 128),
                    Color32::from_rgb(211, 198, 170),
                ),
                crate::types::Theme::TomorrowNight => (
                    true,
                    Color32::from_rgb(29, 31, 33),
                    Color32::from_rgb(40, 42, 46),
                    Color32::from_rgb(129, 162, 190),
                    Color32::from_rgb(197, 200, 198),
                ),
                crate::types::Theme::RosePine => (
                    true,
                    Color32::from_rgb(25, 23, 36),
                    Color32::from_rgb(31, 29, 46),
                    Color32::from_rgb(196, 167, 231),
                    Color32::from_rgb(224, 222, 244),
                ),
                crate::types::Theme::SynthWave84 => (
                    true,
                    Color32::from_rgb(38, 35, 58),
                    Color32::from_rgb(43, 33, 58),
                    Color32::from_rgb(255, 126, 219),
                    Color32::from_rgb(249, 42, 173),
                ),
                crate::types::Theme::Nordic => (
                    true,
                    Color32::from_rgb(36, 41, 51),
                    Color32::from_rgb(46, 52, 64),
                    Color32::from_rgb(143, 188, 187),
                    Color32::from_rgb(216, 222, 233),
                ),
                crate::types::Theme::OceanicNext => (
                    true,
                    Color32::from_rgb(27, 43, 52),
                    Color32::from_rgb(52, 61, 70),
                    Color32::from_rgb(102, 153, 204),
                    Color32::from_rgb(192, 197, 206),
                ),
                crate::types::Theme::Palenight => (
                    true,
                    Color32::from_rgb(41, 45, 62),
                    Color32::from_rgb(50, 55, 77),
                    Color32::from_rgb(199, 146, 234),
                    Color32::from_rgb(191, 199, 213),
                ),
                crate::types::Theme::Powerlevel10k => (
                    true,
                    Color32::from_rgb(0, 0, 0),
                    Color32::from_rgb(28, 28, 28),
                    Color32::from_rgb(0, 135, 255),
                    Color32::from_rgb(255, 255, 255),
                ),
                crate::types::Theme::Snazzy => (
                    true,
                    Color32::from_rgb(40, 42, 54),
                    Color32::from_rgb(52, 53, 65),
                    Color32::from_rgb(255, 92, 87),
                    Color32::from_rgb(239, 240, 235),
                ),
                _ => (
                    true,
                    Color32::from_rgb(43, 43, 43),
                    Color32::from_rgb(32, 32, 32),
                    Color32::from_rgb(252, 60, 20),
                    Color32::from_rgb(240, 230, 220),
                ),
            };

            let mut visuals = if is_dark {
                Visuals::dark()
            } else {
                Visuals::light()
            };
            visuals.widgets.noninteractive.bg_fill = if is_dark {
                bg.linear_multiply(1.5)
            } else {
                bg.linear_multiply(0.9)
            };
            visuals.widgets.noninteractive.bg_stroke = Stroke::new(
                1.0,
                if is_dark {
                    panel
                } else {
                    panel.linear_multiply(0.8)
                },
            );
            visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text);

            visuals.widgets.inactive.bg_fill = panel;
            visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text);

            visuals.widgets.hovered.bg_fill = accent.gamma_multiply(0.2);
            visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, accent);
            visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, accent);

            visuals.widgets.active.bg_fill = accent.gamma_multiply(0.4);
            visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            visuals.widgets.active.bg_stroke = Stroke::new(1.0, accent);

            visuals.selection.bg_fill = accent;
            visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);

            visuals.panel_fill = panel;
            visuals.window_fill = bg;
            ctx.set_visuals(visuals);
        }
    }
}
</file>

<file path="build.ps1">
# build.ps1 - Setup dependencies and build DiffPlayerQC on Windows
# Run with: powershell -ExecutionPolicy Bypass -File build.ps1
$ErrorActionPreference = "Stop"

$IsWin = $IsWindows -or ($env:OS -eq "Windows_NT") -or ($PSVersionTable.PSVersion.Major -lt 6)
if (-not $IsWin) {
    Write-Host "This script is for Windows. On macOS/Linux use build.sh or cargo build." -ForegroundColor Yellow
    exit 1
}

Write-Host "=== DiffPlayerQC - Setup and Build (Windows) ===" -ForegroundColor Cyan

# ---------- 1. Rust (rustup + cargo) ----------
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) {
    Write-Host "`n[1/4] Rust not found. Installing rustup..." -ForegroundColor Yellow
    $rustupUrl = "https://win.rustup.org/x86_64"
    $rustupExe = "$env:TEMP\rustup-init.exe"
    try {
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupExe -UseBasicParsing
        & $rustupExe -y
        $env:PATH = "$env:USERPROFILE\.cargo\bin;" + $env:PATH
        $cargo = Get-Command cargo -ErrorAction SilentlyContinue
        if (-not $cargo) {
            Write-Host "Rust installed. Please close and reopen PowerShell, then run this script again." -ForegroundColor Green
            exit 0
        }
    } catch {
        Write-Host "Failed to download rustup: $_" -ForegroundColor Red
        Write-Host "Install manually from https://rustup.rs and run this script again." -ForegroundColor Yellow
        exit 1
    }
} else {
    Write-Host "`n[1/4] Rust found: $($cargo.Source)" -ForegroundColor Green
}

# Ensure GNU target for Windows (needed when using MSYS2 gcc)
rustup target add x86_64-pc-windows-gnu 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Adding target x86_64-pc-windows-gnu via rustup..." -ForegroundColor DarkGray
}

# ---------- 2. MSYS2 ----------
$msys64 = "C:\msys64"
$ucrt64 = "$msys64\ucrt64"
$ucrt64Bin = "$ucrt64\bin"

if (-not (Test-Path $ucrt64Bin)) {
    Write-Host "`n[2/4] MSYS2 UCRT64 not found at $msys64" -ForegroundColor Yellow
    # Try winget first
    $winget = Get-Command winget -ErrorAction SilentlyContinue
    if ($winget) {
        Write-Host "Installing MSYS2 via winget (this may take a few minutes)..." -ForegroundColor Cyan
        winget install --id MSYS2.MSYS2 --accept-package-agreements --accept-source-agreements
        if (-not (Test-Path $msys64)) {
            $msys64 = "${env:ProgramFiles}\msys64"
            $ucrt64 = "$msys64\ucrt64"
            $ucrt64Bin = "$ucrt64\bin"
        }
    }
    if (-not (Test-Path $ucrt64Bin)) {
        Write-Host "MSYS2 not found. Please install it manually:" -ForegroundColor Red
        Write-Host "  1. Download from https://www.msys2.org/" -ForegroundColor White
        Write-Host "  2. Run the installer (default: C:\msys64)" -ForegroundColor White
        Write-Host "  3. Open 'MSYS2 UCRT64' from Start Menu and run:" -ForegroundColor White
        Write-Host "     pacman -Syu" -ForegroundColor White
        Write-Host "     pacman -S mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf" -ForegroundColor White
        Write-Host "  4. Run this script again." -ForegroundColor White
        exit 1
    }
}
Write-Host "[2/4] MSYS2 UCRT64 found: $ucrt64" -ForegroundColor Green

# ---------- 3. MSYS2 packages (FFmpeg, GCC, pkg-config) ----------
Write-Host "`n[3/4] Checking MSYS2 build dependencies..." -ForegroundColor Cyan
$gccExe = "$ucrt64Bin\gcc.exe"
$pkgConfig = "$ucrt64Bin\pkg-config.exe"
$ffmpegPc = "$ucrt64\lib\pkgconfig\libavcodec.pc"

if (-not (Test-Path $gccExe) -or -not (Test-Path $ffmpegPc)) {
    Write-Host "Installing build tools and FFmpeg in MSYS2 UCRT64..." -ForegroundColor Yellow
    $bash = "$msys64\usr\bin\bash.exe"
    if (-not (Test-Path $bash)) {
        Write-Host "bash.exe not found. Open 'MSYS2 UCRT64' and run:" -ForegroundColor Red
        Write-Host "  pacman -Syu" -ForegroundColor White
        Write-Host "  pacman -S mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf" -ForegroundColor White
        exit 1
    }
    & $bash -lc "pacman -Syu --noconfirm"
    & $bash -lc "pacman -S --noconfirm mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf"
    if ($LASTEXITCODE -ne 0) {
        Write-Host "pacman install failed. Run manually in MSYS2 UCRT64:" -ForegroundColor Red
        Write-Host "  pacman -S mingw-w64-ucrt-x86_64-ffmpeg mingw-w64-ucrt-x86_64-gcc mingw-w64-ucrt-x86_64-pkgconf" -ForegroundColor White
        exit 1
    }
    Write-Host "MSYS2 packages installed." -ForegroundColor Green
} else {
    Write-Host "[3/4] Build dependencies (gcc, ffmpeg, pkg-config) OK." -ForegroundColor Green
}

# ---------- 4. Build ----------
Write-Host "`n[4/4] Building DiffPlayerQC (Release)..." -ForegroundColor Cyan

# Unset FFMPEG_DIR so ffmpeg-sys-next uses pkg-config
$env:FFMPEG_DIR = $null
$env:PATH = "$ucrt64Bin;" + $env:PATH
$env:PKG_CONFIG_PATH = "$ucrt64\lib\pkgconfig"
$env:PKG_CONFIG_ALLOW_CROSS = "1"
$env:LIBCLANG_PATH = $ucrt64Bin
$env:CC = "gcc"

Push-Location $PSScriptRoot
try {
    cargo build --release --target x86_64-pc-windows-gnu
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Build failed." -ForegroundColor Red
        exit $LASTEXITCODE
    }

    $VERSION = (Select-String -Path "Cargo.toml" -Pattern '^version = "(.*)"').Matches.Groups[1].Value
    Write-Host "`nBuild successful (version $VERSION)." -ForegroundColor Green

    $RELEASE_DIR = "target\x86_64-pc-windows-gnu\release"
    $DIST_DIR = "dist\Windows_v$VERSION"
    New-Item -ItemType Directory -Force -Path $DIST_DIR | Out-Null

    $appName = "diffplayerqc.exe"
    $runningProcesses = Get-Process -Name ($appName -replace "\.exe$", "") -ErrorAction SilentlyContinue
    if ($runningProcesses) {
        Write-Host "Closing running instances of $appName..." -ForegroundColor Cyan
        $runningProcesses | Stop-Process -Force -ErrorAction SilentlyContinue
        Start-Sleep -Milliseconds 500
    }

    if (Test-Path $DIST_DIR) { Remove-Item -Recurse -Force $DIST_DIR -ErrorAction SilentlyContinue }
    New-Item -ItemType Directory -Force -Path $DIST_DIR | Out-Null

    Copy-Item "$RELEASE_DIR\diffplayerqc.exe" -Destination "$DIST_DIR\diffplayerqc-v$VERSION.exe"

    # Copy required DLLs from MSYS2
    $LDD_EXE = "$msys64\usr\bin\ldd.exe"
    if (Test-Path $LDD_EXE) {
        $lddOutput = & $LDD_EXE "$DIST_DIR\diffplayerqc-v$VERSION.exe" 2>$null
        $count = 0
        foreach ($line in $lddOutput) {
            if ($line -match "=>\s+(/ucrt64/bin/.*?\.dll)") {
                $winPath = ($matches[1] -replace "^/ucrt64/", "$ucrt64\") -replace "/", "\"
                if (Test-Path $winPath) {
                    Copy-Item $winPath -Destination "$DIST_DIR\" -ErrorAction SilentlyContinue
                    $count++
                }
            }
        }
        Write-Host "Copied $count DLLs to $DIST_DIR" -ForegroundColor Green
    }

    Write-Host "`nOutput: $DIST_DIR\diffplayerqc-v$VERSION.exe" -ForegroundColor Cyan
    Write-Host "Dist folder ready: $DIST_DIR" -ForegroundColor White
} finally {
    Pop-Location
}
</file>

<file path="src/ui/mod.rs">
//! Paneles egui: controles, información, timeline y temas visuales.

pub mod controls;
pub mod design;
pub mod i18n;
pub mod info_panel;
pub mod markers;
pub mod theme;
pub mod timeline;
pub mod vu_meter;
</file>

<file path="src/ui/timeline.rs">
// ui/timeline.rs — Scrubber / playhead widget

use egui::{Pos2, Rect, Sense, Ui, Vec2};

use crate::app::DiffPlayerApp;
use crate::types::Language;
use crate::ui::design::{tr, ACCENT_PRIMARY, FONT_MONO, FONT_MONO_SMALL, TIMELINE_HEIGHT};

fn timecode_to_secs(tc: &str, fps: f64) -> f64 {
    let parts: Vec<&str> = tc.split(|c| c == ':' || c == ';').collect();
    if parts.len() == 4 {
        let h: f64 = parts[0].parse().unwrap_or(0.0);
        let m: f64 = parts[1].parse().unwrap_or(0.0);
        let s: f64 = parts[2].parse().unwrap_or(0.0);
        let f: f64 = parts[3].parse().unwrap_or(0.0);
        h * 3600.0 + m * 60.0 + s + f / fps.max(1.0)
    } else {
        0.0
    }
}

/// Draw the timeline scrubber at the bottom of the window.
pub fn show(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    let duration = app.playback().duration_a.max(app.playback().duration_b);
    let fps = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
    let start_tc = app.decoder_a_meta().and_then(|m| m.start_timecode.clone());
    let start_tc_secs = if let Some(tc) = start_tc {
        timecode_to_secs(&tc, fps)
    } else {
        0.0
    };

    // Always reserve the full width
    let available_width = ui.available_width();
    let desired_size = Vec2::new(available_width, TIMELINE_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    let painter = ui.painter().clone();

    // ── Background track ──────────────────────────────────────────────────
    let track_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 4.0, rect.center().y - 4.0),
        Pos2::new(rect.right() - 4.0, rect.center().y + 4.0),
    );
    painter.rect_filled(track_rect, 4.0, ui.visuals().faint_bg_color);

    let mut dragging_handle = false;
    // ── Loop Range ────────────────────────────────────────────────────────
    let loop_in = app.playback().loop_in;
    let loop_out = app.playback().loop_out;

    if let (Some(in_pts), Some(out_pts)) = (loop_in, loop_out) {
        let fraction_in = if duration > 0.0 { (in_pts / duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let fraction_out = if duration > 0.0 { (out_pts / duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let x_in = track_rect.left() + track_rect.width() * fraction_in;
        let x_out = track_rect.left() + track_rect.width() * fraction_out;

        let range_rect = Rect::from_min_max(
            Pos2::new(x_in, track_rect.top()),
            Pos2::new(x_out, track_rect.bottom()),
        );
        painter.rect_filled(
            range_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(255, 200, 0, 60),
        );

        // Handle In
        let in_rect = Rect::from_center_size(
            Pos2::new(x_in, track_rect.center().y),
            Vec2::new(10.0, track_rect.height() + 8.0),
        );
        let in_resp = ui.allocate_rect(in_rect, Sense::drag());
        if in_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }
        if in_resp.dragged() {
            dragging_handle = true;
            if let Some(pos) = in_resp.interact_pointer_pos() {
                let fraction = ((pos.x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
                app.playback_mut().loop_in = Some((fraction as f64 * duration).min(out_pts));
            }
        }
        painter.rect_filled(in_rect, 2.0, egui::Color32::from_rgb(255, 200, 0));

        // Handle Out
        let out_rect = Rect::from_center_size(
            Pos2::new(x_out, track_rect.center().y),
            Vec2::new(10.0, track_rect.height() + 8.0),
        );
        let out_resp = ui.allocate_rect(out_rect, Sense::drag());
        if out_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }
        if out_resp.dragged() {
            dragging_handle = true;
            if let Some(pos) = out_resp.interact_pointer_pos() {
                let fraction = ((pos.x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
                app.playback_mut().loop_out = Some((fraction as f64 * duration).max(in_pts));
            }
        }
        painter.rect_filled(out_rect, 2.0, egui::Color32::from_rgb(255, 200, 0));
    }

    // ── Markers ───────────────────────────────────────────────────────────
    for marker in &app.session.markers {
        let fraction = if duration > 0.0 { (marker.pts / duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let x = track_rect.left() + track_rect.width() * fraction;
        let p0 = Pos2::new(x, track_rect.top() - 6.0);
        let p1 = Pos2::new(x + 4.0, track_rect.center().y);
        let p2 = Pos2::new(x, track_rect.bottom() + 6.0);
        let p3 = Pos2::new(x - 4.0, track_rect.center().y);

        let color = egui::Color32::from_rgb(
            (marker.color[0] * 255.0) as u8,
            (marker.color[1] * 255.0) as u8,
            (marker.color[2] * 255.0) as u8,
        );

        painter.add(egui::Shape::convex_polygon(
            vec![p0, p1, p2, p3],
            color,
            egui::Stroke::new(1.0, egui::Color32::BLACK),
        ));
    }

    // ── Interaction ───────────────────────────────────────────────────────
    if (response.dragged() || response.clicked()) && !dragging_handle {
        if let Some(pos) = response.interact_pointer_pos() {
            let x = pos.x.clamp(track_rect.left(), track_rect.right());
            let fraction = (x - track_rect.left()) / track_rect.width();
            let new_pts = (fraction as f64 * duration).max(0.0);
            app.do_seek(new_pts, ui.ctx());
        }
    }

    // ── Played portion ────────────────────────────────────────────────────
    let current_pts = app.playback().current_pts;
    let fraction = if duration > 0.0 {
        (current_pts / duration).clamp(0.0, 1.0) as f32
    } else {
        0.0
    };
    let played_right = track_rect.left() + track_rect.width() * fraction;

    painter.rect_filled(
        Rect::from_min_max(track_rect.min, Pos2::new(played_right, track_rect.max.y)),
        4.0,
        ACCENT_PRIMARY,
    );

    // ── Playhead ──────────────────────────────────────────────────────────

    let handle_x = track_rect.left() + fraction * track_rect.width();
    let handle_center = Pos2::new(handle_x, track_rect.center().y);

    let is_hovered = response.hovered();
    let handle_radius = if is_hovered { 9.0 } else { 7.0 };
    painter.circle_filled(
        handle_center,
        handle_radius + 1.5,
        ui.visuals().window_fill(),
    );
    painter.circle_filled(handle_center, handle_radius, ACCENT_PRIMARY);

    // ── Timecode labels ───────────────────────────────────────────────────
    let current_label = format_timecode(current_pts, fps, start_tc_secs);
    let duration_label = format_timecode(duration, fps, start_tc_secs);
    let font = egui::FontId::monospace(FONT_MONO);
    let dim = ui.visuals().text_color().gamma_multiply(0.7);

    painter.text(
        Pos2::new(rect.left() + 6.0, rect.top() + 4.0),
        egui::Align2::LEFT_TOP,
        &current_label,
        font.clone(),
        dim,
    );
    painter.text(
        Pos2::new(rect.right() - 6.0, rect.top() + 4.0),
        egui::Align2::RIGHT_TOP,
        &duration_label,
        font,
        dim,
    );

    // ── Frame number ──────────────────────────────────────────────────────
    let fps_a = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
    let frame_num = (current_pts * fps_a).round() as u64;
    let frame_prefix = match lang {
        Language::Es => "Cuad.",
        Language::En => "Frm.",
        Language::Quenya => "Fr.",
    };
    painter.text(
        handle_center - Vec2::new(0.0, 18.0),
        egui::Align2::CENTER_CENTER,
        format!("{frame_prefix}{frame_num}"),
        egui::FontId::monospace(FONT_MONO_SMALL),
        ACCENT_PRIMARY,
    );

    // ── Seek on click / drag ──────────────────────────────────────────────
    let interact = response.interact_pointer_pos();
    if response.clicked() || response.dragged() {
        if let Some(pos) = interact {
            let t = ((pos.x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
            let seek_secs = t as f64 * duration;
            app.do_seek(seek_secs, ui.ctx());
        }
    }
}

/// Format `secs` as HH:MM:SS:FF using real fps and optional start offset
fn format_timecode(secs: f64, fps: f64, start_tc_secs: f64) -> String {
    let total_secs = secs + start_tc_secs;
    let total = total_secs.max(0.0) as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    let fps_val = fps.max(1.0);
    let f = ((total_secs.fract()) * fps_val).round() as u64 % (fps_val.round() as u64).max(1);
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
</file>

<file path="src/renderer.rs">
//! Pipeline wgpu integrado con egui: texturas por canal, uniforms y shader `compare.wgsl`.
//!
//! Sube buffers RGBA desde el decoder a la GPU; la conversión YUV→RGB ocurre aún en CPU
//! (`decoder`). Ver `docs/GPU_YUV_PIPELINE.md` para una posible ruta futura en shader.

use bytemuck::{Pod, Zeroable};
use egui_wgpu::wgpu;
use wgpu::util::DeviceExt;
// No unnecessary crate imports here

// ---------------------------------------------------------------------------
// Uniform buffer layout (must match compare.wgsl)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ShaderUniforms {
    /// 0.0–1.0 curtain split position (Split-Screen mode)
    pub split_pos: f32,
    /// Compare mode: 0=SplitScreen, 1=AbsDiff, 2=Heatmap, 3=SideBySide
    pub mode: u32,
    /// Subtraction mode inside AbsDiff: 0=LegacyAbs, 1=AbsLinear, 2=AbsSqrt, 3=SignedDiverging
    pub diff_mode: u32,
    /// Error amplifier for heatmap mode (1.0–50.0)
    pub amplifier: f32,

    /// Current zoom level (>1.0 = zoomed in)
    pub zoom: f32,
    /// UV pan offsets
    pub pan_u: f32,
    pub pan_v: f32,
    /// Aspect ratio letterbox scales
    pub scale_u: f32,

    pub scale_v: f32,
    pub bg_color: [f32; 3],
    /// 0 = vertical curtain (split on X), 1 = horizontal curtain (split on Y)
    pub split_horizontal: u32,
}

impl Default for ShaderUniforms {
    fn default() -> Self {
        Self {
            split_pos: 0.5,
            mode: 0,
            diff_mode: 1,
            amplifier: 5.0,
            zoom: 1.0,
            pan_u: 0.0,
            pan_v: 0.0,
            scale_u: 1.0,
            scale_v: 1.0,
            bg_color: [0.0, 0.0, 0.0],
            split_horizontal: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// VideoTexture — owns a wgpu texture and the view/sampler for one video
// ---------------------------------------------------------------------------

pub struct VideoTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub width: u32,
    pub height: u32,
}

impl VideoTexture {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("video_texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            texture,
            view,
            width,
            height,
        }
    }

    /// Upload new RGBA pixel data. Recreates the texture if dimensions changed.
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba_data: &[u8],
        width: u32,
        height: u32,
    ) {
        // Recreate texture if size changed
        if self.width != width || self.height != height {
            *self = Self::new(device, width, height);
        }

        let bytes_per_row = width * 4;
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }
}

// ---------------------------------------------------------------------------
// VideoRenderer — the egui_wgpu::CallbackTrait implementation
// ---------------------------------------------------------------------------

pub struct VideoRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
    pub sampler: wgpu::Sampler,
    pub tex_a: VideoTexture,
    pub tex_b: VideoTexture,
    pub bind_group: wgpu::BindGroup,
    pub uniforms: ShaderUniforms,
}

impl VideoRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        // Load WGSL shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compare_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/compare.wgsl").into()),
        });

        // Uniform buffer
        let uniforms = ShaderUniforms::default();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Sampler — linear filtering for sub-pixel zoom
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("video_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Placeholder 1×1 textures
        let tex_a = VideoTexture::new(device, 1, 1);
        let tex_b = VideoTexture::new(device, 1, 1);

        // Bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compare_bind_group_layout"),
            entries: &[
                // binding 0: texture A
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // binding 1: texture B
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // binding 2: sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // binding 3: uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = make_bind_group(
            device,
            &bind_group_layout,
            &tex_a,
            &tex_b,
            &sampler,
            &uniform_buffer,
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compare_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("compare_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[], // fullscreen triangle, no vertex buffer
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            pipeline,
            bind_group_layout,
            uniform_buffer,
            sampler,
            tex_a,
            tex_b,
            bind_group,
            uniforms,
        }
    }

    /// Upload new RGBA data for channel A.
    pub fn update_texture_a(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) {
        let size_changed = self.tex_a.width != width || self.tex_a.height != height;
        self.tex_a.update(device, queue, rgba, width, height);
        if size_changed {
            self.bind_group = make_bind_group(
                device,
                &self.bind_group_layout,
                &self.tex_a,
                &self.tex_b,
                &self.sampler,
                &self.uniform_buffer,
            );
        }
    }

    /// Upload new RGBA data for channel B.
    pub fn update_texture_b(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) {
        let size_changed = self.tex_b.width != width || self.tex_b.height != height;
        self.tex_b.update(device, queue, rgba, width, height);
        if size_changed {
            self.bind_group = make_bind_group(
                device,
                &self.bind_group_layout,
                &self.tex_a,
                &self.tex_b,
                &self.sampler,
                &self.uniform_buffer,
            );
        }
    }

    /// Write uniforms to GPU buffer.
    pub fn upload_uniforms(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&self.uniforms));
    }
}

// egui_wgpu Callback trait integration
pub struct RenderCallback {
    pub renderer: std::sync::Arc<parking_lot::Mutex<VideoRenderer>>,
}

impl egui_wgpu::CallbackTrait for RenderCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let rend = self.renderer.lock();
        rend.upload_uniforms(queue);
        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        _callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        let rend = self.renderer.lock();

        // SAFETY: We are recording commands into the RenderPass which will be submitted immediately.
        // The VideoRenderer (and its pipeline/bind_group) is kept alive by the Arc in RenderCallback.
        unsafe {
            // Helper to bypass restrictive lifetime bounds on RenderPass.
            unsafe fn extend<'a, T>(t: &T) -> &'a T {
                std::mem::transmute(t)
            }
            let rp: &mut wgpu::RenderPass<'a> = std::mem::transmute(render_pass);
            rp.set_pipeline(extend(&rend.pipeline));
            rp.set_bind_group(0, extend(&rend.bind_group), &[]);
            rp.draw(0..3, 0..1);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    tex_a: &VideoTexture,
    tex_b: &VideoTexture,
    sampler: &wgpu::Sampler,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("compare_bind_group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&tex_a.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&tex_b.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
    })
}
</file>

<file path="features.md">
# DiffPlayerQC - Características del Programa (Features)

DiffPlayerQC es un reproductor avanzado multiplataforma (Windows, macOS, Linux) desarrollado en Rust para la comparación diferencial de video y control de calidad (QC) con precisión por cuadro. 

## 🎬 Modos de Comparación
- **Pantalla Dividida (Split Screen):** Visualiza los videos A y B con un separador móvil (deslizable con el ratón).
- **Diferencia Absoluta (AbsDiff):** Muestra visualmente la diferencia matemática entre los píxeles de ambos videos para detectar artefactos de compresión o fallos.
- **Mapa de Calor (Heatmap):** Resalta las diferencias entre videos usando una escala de colores térmicos.
- **Lado a Lado (Side-by-Side):** Muestra el video A y el video B adyacentes al mismo tiempo.

## ⚙️ Capacidades de Reproducción y Decodificación
- **Soporte Multiformato:** Compatible con una amplia gama de formatos profesionales y de consumo (mp4, mov, mxf, mkv, avi, prores, mts, mpg, mpeg, ts) gracias a su integración con FFmpeg.
- **Precisión por Frame:** Reproducción controlada cuadro a cuadro para un análisis minucioso.
- **Reproducción de Audio Sincronizado:** Permite escuchar y comparar el audio de cada canal gracias al motor `rodio`, con controles independientes de volumen y mute por canal.
- **Aceleración por Hardware:** Utiliza la API WGPU (Vulkan, Metal, GL) para garantizar un renderizado eficiente en la GPU y una reproducción fluida con bajos tiempos de respuesta.

## 🔎 Herramientas de Inspección Visual
- **Zoom y Paneo:** Haz zoom en áreas específicas del video girando la rueda del ratón (hasta 32x) y arrastra el lienzo para inspeccionar detalles concretos. El zoom se puede restablecer con doble clic o con la tecla 'R'.
- **Amplificador de Diferencias:** Aumenta la intensidad visual en los modos de Diferencia Absoluta o Mapa de Calor para visibilizar discrepancias casi imperceptibles.
- **Lupa de Información (HUD):** Paneles superpuestos para metadatos del video activo, controles de reproducción y la línea de tiempo.
- **Ventana Secundaria "Clean Feed":** Permite desacoplar una vista limpia sin interfaz de usuario (ideal para ser capturada con OBS u otro software de transmisión/grabación).
- **Capturas de Pantalla Nativas:** Usa la tecla 'F' para tomar capturas automáticas del visor (vía `xcap`) y guardarlas directamente al escritorio con una marca de tiempo.

## 🖥️ Interfaz y Usabilidad (UI/UX)
- **Tema Automático:** Detecta el modo claro/oscuro del sistema operativo automáticamente y se ajusta mediante el framework `egui`.
- **Soporte Multilingüe:** Interfaz disponible en Inglés y Español.
- **Atajos de Teclado Extendidos:**
  - `Espacio`: Reproducir / Pausar
  - `Flechas Izq/Der`: Avanzar o Retroceder un fotograma
  - `Inicio`: Volver al inicio del video
  - `Y`: Recorrer los modos de comparación
  - `L`: Acceso rápido a Lado a Lado
  - `1`, `2`: Ajuste rápido del separador de pantalla dividida al 50%, inicio o fin
  - `3`: Alternar la visualización de la interfaz HUD
  - `4` al `9`: Niveles predeterminados de Zoom
  - `S`: Intercambiar video A con video B
  
## 💾 Persistencia y Configuración
- **Guardado Automático:** La aplicación recuerda tus preferencias (tema, idioma, carpeta de capturas, filtros y color de fondo) automáticamente al cerrar.
- **Robustez Industrial:** Utiliza un sistema de guardado atómico para prevenir la pérdida de datos y fallos en el archivo de configuración.
- **Gestión de Capturas:** Permite definir una carpeta personalizada para las capturas de pantalla, que se mantiene entre sesiones.

## 📦 Despliegue y Portabilidad
- **Portabilidad Total:** El sistema de auto-empaquetado distribuye el programa sin requerir dependencias externas del sistema (los binarios de FFmpeg se integran con la aplicación).
- **Instaladores Nativos:** Empaquetado en un archivo portátil limpio para Windows y distribuido en formato `.pkg` fácil de instalar para macOS.

## 🆕 Novedades en la Versión 1.3.0
- **Arquitectura más mantenible:** Refactor del bucle principal (`update`) con extracción de responsabilidades para HUD, ventanas modales, teclado y audio.
- **Sistema de diseño unificado:** Tokens visuales compartidos para tipografía/acentos en paneles y timeline.
- **i18n ampliado:** Cobertura consistente ES/EN/Quenya en menús, overlays, panel de audio y etiquetas de modos de diferencia.
- **Calidad reforzada:** Nuevas pruebas unitarias para utilidades de traducción y consistencia de menús de tema.

## 🆕 Novedades en la Versión 1.2.14
- **Estabilidad de Reproducción:** Solución garantizada para la reproducción fluida mientras se mantiene el sistema "Turbo Stepping" para búsquedas manuales rápidas.

## 🆕 Novedades en la Versión 1.2.13
- **Fluidez Máxima (Turbo Draining):** Sistema de visualización ultra-rápido que permite avanzar o retroceder cuadros instantáneamente, incluso manteniendo las teclas pulsadas, sin retrasos ni bloqueos.
- **Eficiencia Mecánica:** Procesamiento de fotogramas optimizado para minimizar el uso de CPU/GPU durante búsquedas rápidas.

## 🆕 Novedades en la Versión 1.2.12
- **Línea de Tiempo Fluida:** Corrección crítica en el sistema de avance cuadro a cuadro que evitaba la congelación de la imagen al realizar búsquedas rápidas pulsando repetidamente los controles.
- **Sincronización de Reloj Robusta:** Mejorada la lógica de visualización de frames en modo pausado.

## 🆕 Novedades en la Versión 1.2.11
- **Interfaz Adaptativa:** Controles del menú superior que se contraen automáticamente en resoluciones bajas para garantizar que todas las opciones de filtrado (Signed, Linear, Sqrt) sigan siendo accesibles.
- **Optimización de Espacio:** Uso de menús desplegables contextuales basados en el ancho disponible de la ventana.

## 🆕 Novedades en la Versión 1.2.10
- **Compatibilidad Total de Símbolos:** Corrección de la visibilidad de iconos en macOS mediante carga dinámica de fuentes del sistema.
- **Drag & Drop Inteligente:** Validación de archivos, alertas de formato y auto-asignación alfabética de canales A y B al soltar dos vídeos.
- **Persistencia Mejorada:** Guardado robusto y soporte nativo para recordar todas las preferencias del usuario.
- **Alertas Premium:** Nuevo sistema visual de mensajes de error y avisos.
</file>

<file path="README.md">
# WPP Production Media Diferencial Player

Frame-accurate differential video QC player

WPP Production Media Diferencial Player es un reproductor avanzado multiplataforma (Windows, macOS, Linux) desarrollado en Rust para la comparación diferencial de video y control de calidad (QC) con precisión por cuadro. Utiliza FFmpeg para decodificación, WGPU para renderizado acelerado por hardware y eframe/egui para una interfaz de usuario moderna y responsiva.

## Características

### Modos de Comparación
- **Pantalla Dividida (Split Screen):** Visualiza los videos A y B con un separador móvil (deslizable con el ratón).
- **Diferencia Absoluta (AbsDiff):** Muestra visualmente la diferencia matemática entre los píxeles de ambos videos para detectar artefactos de compresión o fallos.
- **Mapa de Calor (Heatmap):** Resalta las diferencias entre videos usando una escala de colores térmicos.
- **Lado a Lado (Side-by-Side):** Muestra el video A y el video B adyacentes al mismo tiempo.

### Capacidades de Reproducción y Decodificación
- **Soporte Multiformato:** Compatible con una amplia gama de formatos profesionales y de consumo (mp4, mov, mxf, mkv, avi, prores, mts, mpg, mpeg, ts) gracias a su integración con FFmpeg.
- **Precisión por Frame:** Reproducción controlada cuadro a cuadro para un análisis minucioso.
- **Reproducción de Audio Sincronizado:** Permite escuchar y comparar el audio de cada canal gracias al motor `rodio`, con controles independientes de volumen y mute por canal.
- **Aceleración por Hardware:** Utiliza la API WGPU (Vulkan, Metal, GL) para garantizar un renderizado eficiente en la GPU y una reproducción fluida con bajos tiempos de respuesta.

### Herramientas de Inspección Visual
- **Zoom y Paneo:** Haz zoom en áreas específicas del video girando la rueda del ratón (hasta 32x) y arrastra el lienzo para inspeccionar detalles concretos. El zoom se puede restablecer con doble clic o con la tecla 'R'.
- **Amplificador de Diferencias:** Aumenta la intensidad visual en los modos de Diferencia Absoluta o Mapa de Calor para visibilizar discrepancias casi imperceptibles.
- **Lupa de Información (HUD):** Paneles superpuestos para metadatos del video activo, controles de reproducción y la línea de tiempo.
- **Ventana Secundaria "Clean Feed":** Permite desacoplar una vista limpia sin interfaz de usuario (ideal para ser capturada con OBS u otro software de transmisión/grabación).
- **Capturas de Pantalla Nativas:** Usa la tecla 'F' para tomar capturas automáticas del visor (vía `xcap`) y guardarlas directamente al escritorio con una marca de tiempo.

### Interfaz y Usabilidad (UI/UX)
- **Persistencia de Configuración:** Recuerda automáticamente el tema, idioma, carpeta de capturas, filtros (amplificador, modo diff) y el color del lienzo entre sesiones.
- **Guardado Robusto:** Implementa escritura atómica para proteger los archivos de configuración contra cierres inesperados.
- **Interfaz Adaptativa:** La barra de menú superior contrae sus controles dinámicamente en resoluciones bajas para mantener la visibilidad total de las opciones.
- **Tema Automático:** Detecta el modo claro/oscuro del sistema operativo automáticamente y se ajusta mediante el framework `egui`.
- **Soporte Multilingüe:** Interfaz disponible en Inglés, Español y Quenya.
- **Compatibilidad de Símbolos:** Iconería optimizada para macOS/Darwin mediante carga dinámica de fuentes del sistema y símbolos robustos.
- **Atajos de Teclado Extendidos:**
  - `Espacio`: Reproducir / Pausar
  - `Flechas Izq/Der`: Avanzar o Retroceder un fotograma
  - `Inicio`: Volver al inicio del video
  - `Y`: Recorrer los modos de comparación
  - `L`: Acceso rápido a Lado a Lado
  - `1`, `2`: Ajuste rápido del separador de pantalla dividida al 50%, inicio o fin
  - `3`: Alternar la visualización de la interfaz HUD
  - `4` al `9`: Niveles predeterminados de Zoom
  - `S`: Intercambiar video A con video B
  - `Arriba/Abajo (Scroll)`: Zoom en la posición del ratón
- **Arrastrar y Soltar (Drag & Drop) Inteligente:**
  - Suelta archivos en la mitad izquierda para el Canal A o derecha para el Canal B.
  - Al soltar **dos vídeos** simultáneamente, se asignan automáticamente a los canales A y B por orden alfabético.
  - Validación instantánea: aviso visual si se arrastran más de dos archivos o formatos no soportados.

### Despliegue y Portabilidad
- **Portabilidad Total:** El sistema de auto-empaquetado distribuye el programa sin requerir dependencias externas del sistema (los binarios de FFmpeg se integran con la aplicación).
- **Instaladores Nativos:** Empaquetado en un archivo portátil limpio para Windows y distribuido en formato `.pkg` fácil de instalar para macOS.

## Instalación

### Binarios Precompilados
Descarga la última versión desde la [página de releases](https://github.com/tu-usuario/diffplayerqc/releases).

- **Windows:** Archivo `.zip` portátil.
- **macOS:** Instalador `.pkg`.
- **Linux:** Archivo `.tar.gz` con binario.

### Construcción desde Fuente
Requiere Rust 1.70+ y FFmpeg instalado en el sistema.

```bash
git clone https://github.com/tu-usuario/diffplayerqc.git
cd diffplayerqc
cargo build --release
# Binario más pequeño (menos rápido en CPU): ver docs/BUILD_PROFILES.md
# cargo build --profile release-small
```

Para Windows, usa `build.ps1` o `build.sh` para scripts de construcción automatizados.

## Uso

1. Ejecuta `diffplayerqc` o el binario correspondiente.
2. Carga los videos A y B usando los botones de carga.
3. Selecciona el modo de comparación.
4. Usa los controles de reproducción para navegar por los videos.
5. Ajusta zoom, pan y otros parámetros según sea necesario.

## Contribución

Contribuciones son bienvenidas. Por favor, abre un issue o pull request en [GitHub](https://github.com/tu-usuario/diffplayerqc).

## Licencia

Este proyecto está bajo la licencia MIT. Ver [LICENSE](LICENSE) para más detalles.

## Créditos

Desarrollado por [WPP Production](https://github.com/hansnone/diffplayerqc). Utiliza FFmpeg para decodificación de video.
</file>

<file path="src/ui/info_panel.rs">
// ui/info_panel.rs — Left side panel: metadata, color info, status

use egui::{Color32, RichText, Ui};

use crate::app::DiffPlayerApp;
use crate::types::{ColorMetadata, Language};
use crate::ui::design::{tr, FONT_LABEL, FONT_SUBTITLE, FONT_TITLE, FONT_VALUE};

pub fn show(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    let playback = app.playback().clone();
    let meta_a = app.decoder_a_meta().cloned();
    let meta_b = app.decoder_b_meta().cloned();
    let path_a = app.decoder_a_path().map(|s| s.to_string());
    let path_b = app.decoder_b_path().map(|s| s.to_string());
    let zoom = app.view().zoom;
    let pan_u = app.view().pan_u;
    let pan_v = app.view().pan_v;

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(6.0);

        // ── App title ─────────────────────────────────────────────────────
        ui.label(
            RichText::new("WPP Production Media Differential Player")
                .size(FONT_TITLE)
                .strong()
                .color(Color32::from_rgb(80, 160, 230)),
        );
        ui.label(
            RichText::new(tr(
                lang,
                "Control de Calidad Frame a Frame",
                "Frame-Accurate Video QC",
                "QC vídeo nu per ranga",
            ))
            .size(FONT_SUBTITLE)
            .weak(),
        );

        if let (Some(a), Some(b)) = (&meta_a, &meta_b) {
            let fps_diff = (a.fps - b.fps).abs();
            if fps_diff > 0.001 {
                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.label(
                        RichText::new("⚠️ AVISO: Discrepancia de Framerate")
                            .color(Color32::from_rgb(255, 100, 100))
                            .strong()
                    );
                    ui.label(format!("Canal A: {:.3} fps\nCanal B: {:.3} fps", a.fps, b.fps));
                    ui.label(
                        RichText::new("La sincronización de frames fallará o presentará tirones a lo largo de la reproducción.")
                            .size(11.0)
                            .weak()
                    );
                });
            }
        }

        ui.separator();

        // ── Current playback info ─────────────────────────────────────────
        let pts = playback.current_pts;
        let fps_a = meta_a.as_ref().map(|m| m.fps).unwrap_or(0.0);
        let frame_n = (pts * fps_a).round() as u64;

        egui::Grid::new("playback_grid")
            .num_columns(2)
            .spacing([8.0, 3.0])
            .show(ui, |ui| {
                kv(ui, tr(lang, "PTS", "PTS", "PTS"), &format!("{pts:.4} s"));
                kv(
                    ui,
                    tr(lang, "Cuadro", "Frame", "Quanta"),
                    &frame_n.to_string(),
                );
                kv(
                    ui,
                    tr(lang, "Zoom", "Zoom", "Hyanda"),
                    &format!("{zoom:.2}×"),
                );
                kv(
                    ui,
                    tr(lang, "Panorámica", "Pan", "Pano"),
                    &format!("{pan_u:.2}, {pan_v:.2}"),
                );
            });

        ui.add_space(8.0);
        ui.group(|ui| {
            if ui.button(tr(lang, "Calcular PSNR (Frame actual)", "Calculate PSNR (Current Frame)", "PSNR")).clicked() {
                app.calculate_psnr();
            }
            if let Some(psnr) = app.view().last_psnr {
                ui.label(RichText::new(format!("PSNR: {:.2} dB", psnr)).strong().color(Color32::from_rgb(100, 255, 100)));
            } else {
                ui.label(RichText::new("PSNR: --").weak());
            }
        });

        ui.add_space(6.0);
        ui.separator();

        let dark_mode = ui.visuals().dark_mode;
        let color_a = if dark_mode {
            Color32::from_rgb(100, 200, 120)
        } else {
            Color32::from_rgb(20, 110, 50)
        };
        let color_b = if dark_mode {
            Color32::from_rgb(100, 160, 240)
        } else {
            Color32::from_rgb(30, 70, 180)
        };

        // ── Video A info ──────────────────────────────────────────────────
        channel_section(
            ui,
            tr(lang, "VÍDEO A", "VIDEO A", "VÍDEO A"),
            color_a,
            path_a.as_deref(),
            meta_a.as_ref(),
            lang,
        );

        ui.add_space(8.0);
        ui.separator();

        // ── Video B info ──────────────────────────────────────────────────
        channel_section(
            ui,
            tr(lang, "VÍDEO B", "VIDEO B", "VÍDEO B"),
            color_b,
            path_b.as_deref(),
            meta_b.as_ref(),
            lang,
        );

        ui.add_space(8.0);
        ui.separator();

        // ── Color mismatch warning ────────────────────────────────────────
        if let (Some(ma), Some(mb)) = (meta_a.as_ref(), meta_b.as_ref()) {
            if ma.colorspace != mb.colorspace
                || ma.color_transfer != mb.color_transfer
                || ma.color_primaries != mb.color_primaries
            {
                ui.add_space(6.0);
                egui::Frame::none()
                    .fill(Color32::from_rgba_premultiplied(180, 80, 0, 60))
                    .inner_margin(egui::Margin::symmetric(8.0, 5.0))
                    .rounding(4.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(tr(
                                lang,
                                "⚠ ¡Discrepancia de metadatos de color!",
                                "⚠ Color metadata mismatch detected!",
                                "⚠ Cala meta winya!",
                            ))
                            .color(Color32::from_rgb(255, 180, 60))
                            .size(11.5)
                            .strong(),
                        );
                        if ma.colorspace != mb.colorspace {
                            ui.label(
                                RichText::new(format!(
                                    "  {}: {} ≠ {}",
                                    tr(lang, "Espacio", "Colorspace", "Cala"),
                                    ma.colorspace,
                                    mb.colorspace
                                ))
                                .size(FONT_LABEL)
                                .color(Color32::LIGHT_GRAY),
                            );
                        }
                        if ma.color_transfer != mb.color_transfer {
                            ui.label(
                                RichText::new(format!(
                                    "  {}: {} ≠ {}",
                                    tr(lang, "Transferencia", "Transfer", "Tíra"),
                                    ma.color_transfer,
                                    mb.color_transfer
                                ))
                                .size(FONT_LABEL)
                                .color(Color32::LIGHT_GRAY),
                            );
                        }
                        if ma.color_primaries != mb.color_primaries {
                            ui.label(
                                RichText::new(format!(
                                    "  {}: {} ≠ {}",
                                    tr(lang, "Primarios", "Primaries", "Hairë"),
                                    ma.color_primaries,
                                    mb.color_primaries
                                ))
                                .size(FONT_LABEL)
                                .color(Color32::LIGHT_GRAY),
                            );
                        }
                    });
            }
        }

        ui.add_space(6.0);

        // ── Usage hints ───────────────────────────────────────────────────
        ui.separator();
        ui.label(
            RichText::new(tr(lang, "Atajos", "Shortcuts", "Quanta ranga"))
                .size(FONT_SUBTITLE)
                .strong()
                .weak(),
        );
        let hints: Vec<(&str, &str)> = match lang {
            Language::Es => vec![
                ("Espacio", "Reproducir / Pausa"),
                ("← →", "Avanzar frame"),
                ("Rueda", "Acercar / Alejar"),
                ("Arrastrar", "Desplazar"),
                ("Doble clk / R", "Restaurar zoom"),
                ("Inicio", "Ir al principio"),
                ("S", "Intercambiar A y B"),
                ("F", "Capturar pantalla (PNG)"),
                ("3", "Ocultar / Mostrar Interfaz"),
                ("4..9", "Ajustes rápidos de zoom"),
            ],
            Language::En => vec![
                ("Space", "Play / Pause"),
                ("← →", "Step frame"),
                ("Scroll", "Zoom in / out"),
                ("Drag", "Pan"),
                ("Dbl-clk / R", "Reset zoom"),
                ("Home", "Go to start"),
                ("S", "Swap A and B"),
                ("F", "Take screenshot (PNG)"),
                ("3", "Toggle UI / HUD"),
                ("4..9", "Quick zoom presets"),
            ],
            Language::Quenya => vec![
                ("Space", "Lir / Talta"),
                ("← →", "Quanta ranga"),
                ("Scroll", "Hyanda"),
                ("Drag", "Pano"),
                ("Dbl-clk / R", "En-panya zoom"),
                ("Home", "Yessë"),
                ("S", "Quista A ar B"),
                ("F", "Harya PNG"),
                ("3", "HUD"),
                ("4..9", "Zoom ve"),
            ],
        };
        egui::Grid::new("hints_grid")
            .num_columns(2)
            .spacing([6.0, 2.0])
            .show(ui, |ui| {
                for (key, desc) in hints {
                    ui.label(
                        RichText::new(key)
                            .monospace()
                            .size(FONT_LABEL)
                            .color(Color32::from_rgb(150, 200, 255)),
                    );
                    ui.label(RichText::new(desc).size(FONT_LABEL).weak());
                    ui.end_row();
                }
            });
    });
}

// ---------------------------------------------------------------------------

fn channel_section(
    ui: &mut Ui,
    label: &str,
    accent: Color32,
    path: Option<&str>,
    meta: Option<&ColorMetadata>,
    lang: Language,
) {
    ui.label(RichText::new(label).size(12.0).strong().color(accent));

    if let Some(path) = path {
        let filename = std::path::Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        ui.label(RichText::new(&filename).size(11.5).strong());
        ui.label(RichText::new(path).size(FONT_VALUE).weak().italics());
    } else {
        ui.label(
            RichText::new(tr(
                lang,
                "Ningún archivo cargado",
                "No file loaded",
                "La parma",
            ))
            .weak()
            .italics()
            .size(FONT_VALUE),
        );
        return;
    }

    if let Some(m) = meta {
        ui.add_space(3.0);
        egui::Grid::new(format!("meta_{label}"))
            .num_columns(2)
            .spacing([8.0, 2.0])
            .show(ui, |ui| {
                kv(
                    ui,
                    tr(lang, "Resolución", "Resolution", "Palúrë"),
                    &format!("{}×{}", m.width, m.height),
                );
                kv(ui, tr(lang, "FPS", "FPS", "FPS"), &format!("{:.4}", m.fps));
                kv(
                    ui,
                    tr(lang, "Duración", "Duration", "Lúmë"),
                    &format_dur(m.duration_secs),
                );
                kv(
                    ui,
                    tr(lang, "Tasa bits", "Bitrate", "Tix"),
                    &format!("{} kbps", m.bitrate_kbps),
                );
                kv(
                    ui,
                    tr(lang, "Fmt Píxel", "Pixel Fmt", "Píxel"),
                    &m.pixel_format,
                );
                kv(ui, tr(lang, "Espacio", "Colorspace", "Cala"), &m.colorspace);
                kv(
                    ui,
                    tr(lang, "Transfer", "Transfer", "Tíra"),
                    &m.color_transfer,
                );
                kv(
                    ui,
                    tr(lang, "Primarios", "Primaries", "Hairë"),
                    &m.color_primaries,
                );
                kv(
                    ui,
                    tr(lang, "Códec video", "Video codec", "Códec vídeo"),
                    if m.video_codec.is_empty() {
                        "—"
                    } else {
                        m.video_codec.as_str()
                    },
                );
                kv(
                    ui,
                    tr(lang, "Códec audio", "Audio codec", "Códec audio"),
                    if m.audio_codec.is_empty() {
                        "—"
                    } else {
                        m.audio_codec.as_str()
                    },
                );
                kv(
                    ui,
                    tr(lang, "Marca contenedor", "Major brand", "Marca"),
                    if m.major_brand.is_empty() || m.major_brand == "—" {
                        "—"
                    } else {
                        m.major_brand.as_str()
                    },
                );
                {
                    let v = if m.video_stream_metadata.is_empty() {
                        "—".to_string()
                    } else {
                        truncate_meta(&m.video_stream_metadata, 50)
                    };
                    kv(
                        ui,
                        tr(
                            lang,
                            "Stream vídeo (meta)",
                            "Stream video (meta)",
                            "Stream vídeo",
                        ),
                        &v,
                    );
                }
                {
                    let v = if m.audio_stream_metadata.is_empty() || m.audio_stream_metadata == "—"
                    {
                        "—".to_string()
                    } else {
                        truncate_meta(&m.audio_stream_metadata, 50)
                    };
                    kv(
                        ui,
                        tr(
                            lang,
                            "Stream audio (meta)",
                            "Stream audio (meta)",
                            "Stream audio",
                        ),
                        &v,
                    );
                }
            });
    }
}

fn truncate_meta(s: &str, max_len: usize) -> String {
    let one_line: String = s.replace('\n', " ");
    if one_line.len() <= max_len {
        one_line
    } else {
        format!("{}…", one_line.chars().take(max_len).collect::<String>())
    }
}

fn kv(ui: &mut Ui, key: &str, value: &str) {
    ui.label(RichText::new(key).size(FONT_LABEL).weak());
    ui.add(egui::Label::new(RichText::new(value).size(FONT_VALUE).monospace()).wrap(true));
    ui.end_row();
}

fn format_dur(secs: f64) -> String {
    let total = secs.trunc() as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    let ms = (secs.fract() * 1000.0).trunc() as u64;
    format!("{h:02}:{m:02}:{s:02}.{ms:03}")
}
</file>

<file path=".gitignore">
/target
/dist
.DS_Store
.aider*
*_Diff_start.log
</file>

<file path="src/types.rs">
//! Tipos compartidos entre UI, decoders y renderer: frames, comandos y estado de reproducción.

use serde::{Deserialize, Serialize};

/// A decoded video frame ready for GPU upload.
#[derive(Clone)]
pub struct VideoFrame {
    /// Presentation timestamp in seconds.
    pub pts: f64,
    /// Raw RGBA bytes, row-major, no padding.
    pub rgba_data: std::sync::Arc<[u8]>,
    pub width: u32,
    pub height: u32,
}

/// A decoded audio frame ready for playback.
#[derive(Clone)]
pub struct AudioFrame {
    /// Raw f32 interleaved PCM samples.
    pub samples: Vec<f32>,
    /// Number of channels.
    pub channels: u16,
    /// Sample rate in Hz.
    pub sample_rate: u32,
}

/// Commands sent from the UI thread to a decoder thread.
#[derive(Debug)]
pub enum DecoderCommand {
    /// Start continuous playback from the current position.
    Play,
    /// Pause decoding (decoder stays alive, waiting for next command).
    Pause,
    /// Seek to the given PTS (seconds). The decoder will find the nearest
    /// keyframe and then step forward to the exact target.
    Seek(f64),
    /// Decode exactly one frame forward from the current position.
    StepForward,
    /// Terminate the decoder thread.
    Stop,
    /// Reservado: el volumen se aplica en `rodio` desde la UI, no en el hilo decoder.
    #[allow(dead_code)]
    SetVolume(f32),
}

/// Color metadata extracted from the video stream header.
#[derive(Debug, Clone, Default)]
pub struct ColorMetadata {
    pub colorspace: String,
    pub color_transfer: String,
    pub color_primaries: String,
    pub pixel_format: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub duration_secs: f64,
    pub bitrate_kbps: i64,
    pub video_codec: String,
    pub audio_codec: String,
    pub major_brand: String,
    pub start_timecode: Option<String>,
    pub video_stream_metadata: String,
    pub audio_stream_metadata: String,
}

/// Current display mode for the comparison shader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum CompareMode {
    SplitScreen = 0,
    AbsDiff = 1,
    Heatmap = 2,
    SideBySide = 3,
}

impl Default for CompareMode {
    fn default() -> Self {
        Self::SplitScreen
    }
}

/// Safe zone overlay mode: none, TV (EBU R95), or social/mobile (9:16).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SafeZoneMode {
    #[default]
    None,
    /// TV 16:9 — Action Safe 93%, Title Safe (5% top/bottom, 10% sides), centre cross.
    TvEbu,
    /// Social 9:16 — Safe zone + danger zones (top 15%, bottom 22%, right 15%, left 5%) shaded.
    Social,
}

/// The specific algorithm used when evaluating `CompareMode::AbsDiff`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum DiffMode {
    LegacyAbs = 0,
    AbsLinear = 1,
    AbsSqrt = 2,
    SignedDiverging = 3,
    None = 4,
}

impl Default for DiffMode {
    fn default() -> Self {
        Self::AbsLinear
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    En,
    Es,
    Quenya,
}

impl Default for Language {
    fn default() -> Self {
        Self::Es
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    Rust,
    SolarizedDark,
    SolarizedLight,
    Dracula,
    Gruvbox,
    Nord,
    Monokai,
    OneDark,
    OneLight,
    Catppuccin,
    TokyoNight,
    NightOwl,
    Ayc,
    MaterialDesign,
    Everforest,
    TomorrowNight,
    RosePine,
    SynthWave84,
    Nordic,
    OceanicNext,
    Palenight,
    Powerlevel10k,
    Snazzy,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

/// Estado de reproducción compartido entre la UI y la coordinación con los decoders.
///
/// Reloj maestro: al reproducir, `current_pts = playback_start_pts + elapsed` desde `playback_start_instant`.
#[derive(Debug, Clone)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_pts: f64,
    pub duration_a: f64,
    pub duration_b: f64,
    pub loop_in: Option<f64>,
    pub loop_out: Option<f64>,
    pub loop_range_active: bool,
    /// When set, current_pts is derived from this instant + playback_start_pts (system-time master clock).
    pub playback_start_instant: Option<std::time::Instant>,
    /// PTS at the moment we started (or seeked during) playback.
    pub playback_start_pts: f64,
    /// Audio sample rate preferred by the host output device.
    pub target_sample_rate: u32,
    /// Audio channel count preferred by the host output device.
    pub target_channels: u16,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_pts: 0.0,
            duration_a: 0.0,
            duration_b: 0.0,
            loop_in: None,
            loop_out: None,
            loop_range_active: false,
            playback_start_instant: None,
            playback_start_pts: 0.0,
            target_sample_rate: 44100,
            target_channels: 2,
        }
    }
}

/// Which video channel (A or B).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Channel {
    A,
    B,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Marker {
    pub pts: f64,
    pub note: String,
    pub color: [f32; 3],
    pub channel_hint: Option<Channel>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SessionState {
    pub markers: Vec<Marker>,
    pub video_a_path: Option<String>,
    pub video_b_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Los discriminantes deben coincidir con `compare.wgsl` / uniforms.
    #[test]
    fn compare_mode_shader_indices() {
        assert_eq!(CompareMode::SplitScreen as u32, 0);
        assert_eq!(CompareMode::AbsDiff as u32, 1);
        assert_eq!(CompareMode::Heatmap as u32, 2);
        assert_eq!(CompareMode::SideBySide as u32, 3);
    }
}
</file>

<file path="CHANGELOG.md">
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.0] - 2026-03-25

### Changed
- Refactor parcial de la arquitectura de UI y del bucle principal: extracción de responsabilidades en `app/mod.rs` para reducir acoplamiento y mejorar mantenibilidad.
- Unificación de i18n (ES/EN/Quenya) en paneles, menús y overlays clave, con funciones reutilizables de traducción.
- Mejora de consistencia visual con tokens de diseño compartidos (`ui/design.rs`) aplicados en paneles y timeline.
- Internacionalización de etiquetas de `DiffMode` y centralización de nombres de tema en módulos dedicados de i18n.

### Fixed
- Reducción de ruido de logging por frame (`info` a `trace`) para mejorar rendimiento y legibilidad de logs.
- Eliminado `expect` frágil en el flujo de proxy FFmpeg con manejo de error seguro y recuperable.

## [1.2.14] - 2026-03-10

### Fixed
- Restaurada la reproducción de vídeo: corregido un error en la lógica de procesamiento de fotogramas que impedía la reproducción normal tras las optimizaciones de fluidez de la versión anterior.
- Consistencia del motor Sincronizado: el reproductor ahora gestiona correctamente la transición entre los modos de pausa/paso manual y reproducción continua sin perder la alineación temporal.

## [1.2.13] - 2026-03-10

### Fixed
- Super Fluid Stepping: se ha optimizado radicalmente el sistema de drenaje de fotogramas. Ahora la aplicación consume todos los fotogramas pendientes en el canal en un solo ciclo de actualización, eliminando cualquier posible "congelación" al mantener pulsados los botones de avance/retroceso.
- Optimización de comandos: reducción de la carga en los decodificadores al evitar el envío redundante de comandos de pausa durante el paso manual de cuadros.

## [1.2.12] - 2026-03-10

### Fixed
- Congelación en el avance de fotogramas: se ha corregido un problema por el cual la imagen se quedaba bloqueada al mantener pulsado el botón de avance rápido cuadro a cuadro.
- Sincronización de reloj mejorada: la aplicación ahora utiliza el tiempo real de los fotogramas decodificados para actualizar su línea de tiempo, evitando el "reloj desbocado" durante el paso manual.
- Umbral de aceptación de frames relajado: se ha incrementado el margen de tolerancia en el modo pausado (de 0.04s a 0.1s) para garantizar la visualización fluida en videos de 24/25 fps.

## [1.2.11] - 2026-03-10

### Added
- Interfaz Responsiva: la barra de menú superior ahora adapta sus controles dinámicamente según el ancho de la ventana.
- Soporte para resoluciones bajas: las opciones de modo de diferencia (Signed, Linear, Sqrt, etc.) se agrupan en un menú desplegable (ComboBox) cuando no hay espacio suficiente para mostrarlas todas inline.

### Fixed
- Visibilidad de controles: arreglado el problema por el cual algunas opciones de filtrado desaparecían en resoluciones de pantalla más pequeñas.

## [1.2.10] - 2026-03-10

### Fixed
- Renderizado de símbolos en macOS: se ha corregido el problema por el cual los atajos de teclado y botones de control mostraban cuadrados en lugar de iconos.
- Carga de fuentes: la aplicación ahora busca fuentes locales (`Arial`, `Helvetica`) en rutas estándar de macOS.

### Changed
- Símbolos de interfaz robustos: se han reemplazado los símbolos Unicode complejos por alternativas ASCII seguras (`|<`, `||`, `>>`, etc.) para garantizar la visibilidad en todos los sistemas.
- Etiquetas de atajos simplificadas: `(←)` y `(→)` ahora se muestran como `(Left)` y `(Right)`.

## [1.2.9] - 2026-03-10

### Added
- Mejoras en Arrastrar y Soltar (Drag & Drop):
  - Validación de formatos: ahora se muestra un aviso si se intentan cargar archivos no soportados.
  - Límite de archivos: aviso visual si se arrastran más de 2 vídeos.
  - Auto-asignación inteligente: si se arrastran exactamente 2 vídeos, se asignan automáticamente a los canales A y B por orden alfabético.
- Interfaz de Alertas Premium: nuevo diseño de ventanas modales para errores con encabezados en color y mejor legibilidad.

### Fixed
- Persistencia Robusta: implementación de escritura atómica para los archivos de configuración (evita corrupción si el programa se cierra inesperadamente).
- Guardado garantizado: se ha habilitado la característica de persistencia de `eframe` y se ha forzado el guardado al salir (`on_exit`).
- Registro de logs mejorado para la carga y guardado de preferencias.

## [1.2.8] - 2026-03-09

### Added
- Persistencia de configuración: ahora la aplicación recuerda el filtro (`diff_mode`), el idioma (`lang`), el tema (`theme`), la carpeta de capturas (`screenshot_dir`) y el color del fondo entre sesiones.
- Tooltips con la ruta completa en los botones de "Vídeo A" y "Vídeo B" de la barra superior.

## [1.2.7] - 2026-03-09

### Fixed
- Corrección del bug de arrastrar y soltar: el archivo ahora se carga en el canal correcto (A izquierda / B derecha) en lugar de siempre en B.

### Changed
- Interfaz limpiada: barra de herramientas eliminada. Todos los controles (apertura de ficheros, reproducción, modos, sliders, zoom, color de fondo) se han integrado inline en la barra de menú superior en una sola fila compacta.

## [1.2.6] - 2026-03-09

### Added
- Soporte para arrastrar y soltar archivos de vídeo directamente sobre la ventana. Soltar en la mitad izquierda carga como Vídeo A; en la mitad derecha como Vídeo B. Si ya había un vídeo cargado se reemplaza.
- Indicador visual (overlay) durante el arrastre que muestra las zonas A y B con etiquetas.

## [1.2.5] - 2026-03-09

### Fixed
- Paneo desactivado cuando la imagen está ajustada al cuadro (zoom 1.0 / fit-to-frame).
- Icono personalizado de la aplicación ahora se muestra correctamente en el Dock y Finder al instalar la app.
- Interfaz responsiva mejorada: la barra de herramientas ya no tiene altura fija, los elementos se adaptan a pantallas grandes o pequeñas sin desaparecer ni superponerse.
- Panel de información lateral con anchura máxima para evitar expansión excesiva en pantallas ultrawide.

## [1.2.0] - 2026-03-08

### Added
- Nueva funcionalidad de ejemplo agregada en esta versión.
- Mejoras en la interfaz de usuario.

### Changed
- Actualización de dependencias.
- Mejoras en el rendimiento de renderizado.

### Fixed
- Corrección de bugs menores en la reproducción de audio.

## [1.1.0] - 2025-12-01

### Added
- Soporte inicial para comparación de videos.
- Modos de comparación: Split Screen, AbsDiff, Heatmap, Side-by-Side.
- Reproducción de audio sincronizada.
- Aceleración por hardware con WGPU.

### Changed
- Interfaz de usuario mejorada con egui.

### Fixed
- Problemas iniciales de decodificación.
</file>

<file path="build.sh">
#!/usr/bin/env bash
# build.sh - Autonomously-bundled MacOS / Linux Build Script for DiffPlayerQC

set -e

OS="$(uname -s)"
echo -e "\033[1;36m========================================================\033[0m"
echo -e "\033[1;36m Construyendo WPP DiffPlayerQC Portable para $OS        \033[0m"
echo -e "\033[1;36m========================================================\033[0m"

echo -e "\n\033[1;33m[1/3] Compilando codigo Rust (Modo Release)...\033[0m"

# Extraer versión de Cargo.toml
VERSION=$(grep "^version =" Cargo.toml | head -n 1 | cut -d '"' -f 2)
echo -e "\033[1;32mVersion detectada: $VERSION\033[0m"

# Configuración para macOS con Homebrew y FFmpeg@7
if [ "$OS" = "Darwin" ]; then
    if brew --prefix ffmpeg@7 >/dev/null 2>&1; then
        export FFMPEG_DIR=$(brew --prefix ffmpeg@7)
        export PKG_CONFIG_PATH="/opt/homebrew/opt/ffmpeg@7/lib/pkgconfig"
        export BINDGEN_EXTRA_CLANG_ARGS="-I${FFMPEG_DIR}/include"
        echo -e "\033[1;32mEntorno configurado para FFmpeg@7 detectado en $FFMPEG_DIR\033[0m"
    fi
fi

cargo build --release

if [ "$OS" = "Darwin" ]; then
    echo -e "\n\033[1;33m[2/3] Plataforma macOS. Empacando como .app standalone...\033[0m"
    
    APP_NAME="WPP DiffPlayerQC v$VERSION.app"
    DIST_DIR="dist/macOS"
    CONTENTS="$DIST_DIR/$APP_NAME/Contents"
    BINS="$CONTENTS/MacOS"
    RES="$CONTENTS/Resources"
    LIBS="$CONTENTS/Frameworks"
    
    rm -rf "$DIST_DIR"
    mkdir -p "$BINS" "$RES" "$LIBS"
    
    cat <<EOF > "$CONTENTS/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>diffplayerqc</string>
    <key>CFBundleIdentifier</key>
    <string>com.wpp.diffplayerqc</string>
    <key>CFBundleName</key>
    <string>WPP DiffPlayerQC</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
EOF
    
    # 1. Copiamos el ejecutable y los iconos dentro del contenedor final
    cp target/release/diffplayerqc "$BINS/"
    if [ -f "assets/AppIcon.icns" ]; then
        cp "assets/AppIcon.icns" "$RES/"
    fi
    
    echo -e "\n\033[1;33m[3/3] Incorporando dependencias (.dylib) internamente a la App...\033[0m"
    # Este paso rastrea (otool) todas las dylib del sistema (ffmpeg) de las que tira el binario,
    # y las mete dentro de Frameworks alterando las rutas absolutas para que sea portable.
    if ! command -v dylibbundler &> /dev/null; then
        echo -e "\033[1;35mAviso: dylibbundler no instalado.\033[0m"
        echo -e "Como solucion rapida incrustando con Otool manual..."
        
        # Copiar librerias de FFmpeg al interior de la aplicacion de forma manual si es posible
        for lib in $(otool -L "$BINS/diffplayerqc" | grep -E "libav|libsw|libpostproc" | awk '{print $1}'); do
            if [ -f "$lib" ]; then
                cp "$lib" "$LIBS/"
                install_name_tool -change "$lib" "@executable_path/../Frameworks/$(basename "$lib")" "$BINS/diffplayerqc"
            fi
        done
        echo "Librerías principales empaquetadas."
    else
        dylibbundler -b -x "$BINS/diffplayerqc" -d "$LIBS/" -p "@executable_path/../Frameworks/"
    fi
    
    # [HOTFIX] Homebrew's sdl2-compat requires SDL3 at runtime via dlopen.
    # dylibbundler misses it porque no analiza llamadas dinámicas (dlopen).
    if [ -f "/opt/homebrew/lib/libSDL3.dylib" ]; then
        cp -L "/opt/homebrew/lib/libSDL3.dylib" "$LIBS/libSDL3.dylib"
        chmod +w "$LIBS/libSDL3.dylib"
        install_name_tool -id "@executable_path/../Frameworks/libSDL3.dylib" "$LIBS/libSDL3.dylib"
        codesign --force --sign - "$LIBS/libSDL3.dylib"
    fi
    
    echo -e "\n\033[1;33m[4/3] Preparando instalador con integración Youlean...\033[0m"
    SCRIPTS_DIR="$DIST_DIR/scripts"
    rm -rf "$SCRIPTS_DIR"
    mkdir -p "$SCRIPTS_DIR/Settings"
    
    cp "assets/Youlean-Loudness-Meter-2-V2.5.14-macOS-1.dmg" "$SCRIPTS_DIR/Youlean.dmg"
    cp -R "assets/youlean_settings/"* "$SCRIPTS_DIR/Settings/"
    
    cat <<'EOF' > "$SCRIPTS_DIR/postinstall"
#!/bin/bash
DIR=$(dirname "$0")

hdiutil attach "$DIR/Youlean.dmg" -nobrowse -mountpoint /tmp/youlean_mount
installer -pkg "/tmp/youlean_mount/Youlean Loudness Meter 2 - Installer.pkg" -target "$3"
hdiutil detach /tmp/youlean_mount -force

CONSOLE_USER=$(stat -f "%Su" /dev/console)
if [ "$CONSOLE_USER" != "root" ]; then
    USER_HOME=$(dscl . -read /Users/$CONSOLE_USER NFSHomeDirectory | awk '{print $2}')
    YOULEAN_DIR="$USER_HOME/Library/Application Support/Youlean/Youlean Loudness Meter 2"
    mkdir -p "$YOULEAN_DIR"
    cp -R "$DIR/Settings/"* "$YOULEAN_DIR/"
    chown -R $CONSOLE_USER "$YOULEAN_DIR"
fi
exit 0
EOF
    chmod +x "$SCRIPTS_DIR/postinstall"
    
    pkgbuild --component "$DIST_DIR/$APP_NAME" \
             --install-location /Applications \
             --scripts "$SCRIPTS_DIR" \
             "$DIST_DIR/WPP_DiffPlayerQC_Installer_v$VERSION.pkg" || true
             
    echo -e "\n\033[1;32m========================================================\033[0m"
    echo -e "\033[1;32m¡Empaquetado macOS sin dependencias externas completado!\033[0m"
    echo -e "- Aplicacion Lista para entregar: \033[1;37m$DIST_DIR/$APP_NAME\033[0m"
    echo -e "\033[1;32m========================================================\033[0m"

elif [ "$OS" = "Linux" ]; then
    echo -e "\n\033[1;33m[2/3] Plataforma Linux. Preparando ejecutable...\033[0m"
    DIST_DIR="dist/Linux"
    rm -rf "$DIST_DIR"
    mkdir -p "$DIST_DIR/libs"
    
    cp target/release/diffplayerqc "$DIST_DIR/wpp-diffplayerqc-v$VERSION"
    chmod +x "$DIST_DIR/wpp-diffplayerqc-v$VERSION"
    
    echo -e "\n\033[1;33m[3/3] Empaquetando dependencias SO (.so)...\033[0m"
    # LDD en unix rastrea librerias. Copiamos las que no son del kernel.
    ldd "$DIST_DIR/wpp-diffplayerqc-v$VERSION" | grep -E "libav|libsw" | awk '{print $3}' | while read -r lib; do
        if [ -f "$lib" ]; then
            cp "$lib" "$DIST_DIR/libs/"
        fi
    done
    
    echo -e "\n\033[1;32m========================================================\033[0m"
    echo -e "\033[1;32m¡Linux Build Completado!\033[0m"
    echo -e "Binaros y Librerias en: \033[1;37m$DIST_DIR/\033[0m"
    echo -e "Asegurate de lanzar el programa usando LD_LIBRARY_PATH=./libs ./wpp-diffplayerqc"
    echo -e "\033[1;32m========================================================\033[0m"
fi
</file>

<file path="src/decoder.rs">
//! Decodificación de vídeo y audio en un hilo dedicado (API C de FFmpeg vía `ffmpeg-sys-next`).
//!
//! El bucle recibe [`DecoderCommand`](crate::types::DecoderCommand), emite [`VideoFrame`](crate::types::VideoFrame)
//! (YUV→RGBA con libswscale, ver `convert_frame`) y [`AudioFrame`](crate::types::AudioFrame) vía `swr`.
//! El hilo de UI **no** debe bloquearse en estas operaciones.

use anyhow::{anyhow, Context, Result};
use crossbeam_channel::{Receiver, Sender};
use std::ffi::{CStr, CString};
use std::ptr;

use ffmpeg_sys_next as ffi;

/// `SWS_FAST_BILINEAR` (libswscale): más barato que `SWS_BILINEAR` para QC en tiempo real.
const SWS_FAST_BILINEAR: i32 = 1 << 2;

use crate::trace_log;
use crate::types::{AudioFrame, ColorMetadata, DecoderCommand, VideoFrame};

/// Spawn a decoder thread for the given file path.
pub fn spawn_decoder(
    path: &str,
    target_sample_rate: i32,
    target_channels: i32,
) -> Result<(
    Sender<DecoderCommand>,
    Receiver<VideoFrame>,
    Receiver<AudioFrame>,
    ColorMetadata,
)> {
    // Initialise FFmpeg (safe to call multiple times)
    unsafe {
        ffi::av_log_set_level(ffi::AV_LOG_ERROR);
    }

    let path_owned = path.to_owned();

    // Extract metadata synchronously before spawning thread
    let meta = extract_metadata(&path_owned)?;

    let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded::<DecoderCommand>();
    let (frame_tx, frame_rx) = crossbeam_channel::bounded::<VideoFrame>(8);
    let (audio_tx, audio_rx) = crossbeam_channel::bounded::<AudioFrame>(256);

    std::thread::Builder::new()
        .name(format!("decoder:{}", &path_owned))
        .spawn(move || {
            if let Err(e) = decoder_loop(
                &path_owned,
                target_sample_rate,
                target_channels,
                cmd_rx,
                frame_tx,
                audio_tx,
            ) {
                log::error!("Decoder thread error: {e:#}");
            }
        })?;

    Ok((cmd_tx, frame_rx, audio_rx, meta))
}

// ---------------------------------------------------------------------------
// Metadata extraction helpers (AVDictionary)
// ---------------------------------------------------------------------------

/// Read a single metadata value by key from an AVDictionary. Returns empty string if not found or null.
unsafe fn dict_get(m: *const ffi::AVDictionary, key: &str) -> String {
    if m.is_null() {
        return String::new();
    }
    let c_key = match CString::new(key) {
        Ok(k) => k,
        Err(_) => return String::new(),
    };
    let entry = ffi::av_dict_get(m, c_key.as_ptr(), ptr::null_mut(), 0);
    if entry.is_null() {
        return String::new();
    }
    let val = (*entry).value;
    if val.is_null() {
        String::new()
    } else {
        CStr::from_ptr(val).to_string_lossy().into_owned()
    }
}

/// Iterate all AVDictionary entries and format as "key: value\n" lines.
unsafe fn dict_to_string(m: *const ffi::AVDictionary) -> String {
    if m.is_null() {
        return String::new();
    }
    let mut out = String::new();
    let mut prev = ptr::null_mut::<ffi::AVDictionaryEntry>();
    loop {
        let entry = ffi::av_dict_get(m, ptr::null(), prev, 0);
        if entry.is_null() {
            break;
        }
        let key: String = if (*entry).key.is_null() {
            String::new()
        } else {
            CStr::from_ptr((*entry).key).to_string_lossy().into_owned()
        };
        let val: String = if (*entry).value.is_null() {
            String::new()
        } else {
            CStr::from_ptr((*entry).value)
                .to_string_lossy()
                .into_owned()
        };
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&key);
        out.push_str(": ");
        out.push_str(&val);
        prev = entry;
    }
    out
}

// ---------------------------------------------------------------------------
// Metadata extraction (open file, read stream headers, close)
// ---------------------------------------------------------------------------

fn extract_metadata(path: &str) -> Result<ColorMetadata> {
    let c_path = CString::new(path).context("invalid path")?;
    unsafe {
        let mut fmt_ctx: *mut ffi::AVFormatContext = ptr::null_mut();

        let ret =
            ffi::avformat_open_input(&mut fmt_ctx, c_path.as_ptr(), ptr::null(), ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("avformat_open_input: {}", av_err(ret)));
        }

        let ret = ffi::avformat_find_stream_info(fmt_ctx, ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("avformat_find_stream_info: {}", av_err(ret)));
        }

        let nb = (*fmt_ctx).nb_streams as usize;
        let streams = std::slice::from_raw_parts((*fmt_ctx).streams, nb);

        let video_idx = find_video_stream(streams);
        if video_idx < 0 {
            return Err(anyhow!("no video stream in '{path}'"));
        }

        let stream = *streams[video_idx as usize];
        let par = *stream.codecpar;

        let fps = {
            let r = stream.avg_frame_rate;
            if r.den == 0 {
                0.0
            } else {
                r.num as f64 / r.den as f64
            }
        };

        let duration_secs = if stream.duration > 0 {
            let tb = stream.time_base;
            stream.duration as f64 * tb.num as f64 / tb.den as f64
        } else {
            (*fmt_ctx).duration as f64 / ffi::AV_TIME_BASE as f64
        };

        let colorspace = color_space_str(par.color_space);
        let color_transfer = color_trc_str(par.color_trc);
        let color_primaries = color_primaries_str(par.color_primaries);

        // Pixel format name
        let pix_name = ffi::av_get_pix_fmt_name(std::mem::transmute(par.format));
        let pixel_format = if pix_name.is_null() {
            "unknown".to_owned()
        } else {
            CStr::from_ptr(pix_name).to_string_lossy().into_owned()
        };

        // Video codec name
        let video_codec = {
            let name_ptr = ffi::avcodec_get_name(par.codec_id);
            if name_ptr.is_null() {
                "unknown".to_owned()
            } else {
                CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
            }
        };

        // Audio codec name (if present)
        let audio_codec = {
            let a_idx = find_audio_stream(streams);
            if a_idx >= 0 {
                let a_stream = *streams[a_idx as usize];
                let a_par = *a_stream.codecpar;
                let name_ptr = ffi::avcodec_get_name(a_par.codec_id);
                if name_ptr.is_null() {
                    "—".to_owned()
                } else {
                    CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
                }
            } else {
                "—".to_owned()
            }
        };

        // Format-level metadata: major_brand
        let major_brand = {
            let s = dict_get((*fmt_ctx).metadata, "major_brand");
            if s.is_empty() {
                "—".to_owned()
            } else {
                s
            }
        };

        // Video stream metadata (Stream #0:0)
        let video_stream_metadata = dict_to_string(stream.metadata);

        // Timecode extraction
        let mut start_timecode = dict_get(stream.metadata, "timecode");
        if start_timecode.is_empty() {
            start_timecode = dict_get((*fmt_ctx).metadata, "timecode");
        }
        let start_timecode_opt = if start_timecode.is_empty() {
            None
        } else {
            Some(start_timecode)
        };

        // Audio stream metadata (Stream #0:1) if present
        let audio_stream_metadata = {
            let a_idx = find_audio_stream(streams);
            if a_idx >= 0 {
                let a_stream = *streams[a_idx as usize];
                let s = dict_to_string(a_stream.metadata);
                if s.is_empty() {
                    "—".to_owned()
                } else {
                    s
                }
            } else {
                "—".to_owned()
            }
        };

        let meta = ColorMetadata {
            colorspace,
            color_transfer,
            color_primaries,
            pixel_format,
            width: par.width as u32,
            height: par.height as u32,
            fps,
            duration_secs,
            bitrate_kbps: (*fmt_ctx).bit_rate / 1000,
            video_codec,
            audio_codec,
            major_brand,
            start_timecode: start_timecode_opt,
            video_stream_metadata,
            audio_stream_metadata,
        };

        ffi::avformat_close_input(&mut fmt_ctx);
        Ok(meta)
    }
}

// ---------------------------------------------------------------------------
// Decoder loop — runs on its own thread
// ---------------------------------------------------------------------------

struct DecoderCtx {
    fmt_ctx: *mut ffi::AVFormatContext,
    codec_ctx: *mut ffi::AVCodecContext,
    sws_ctx: *mut ffi::SwsContext,
    /// Búfer RGBA reutilizable para `sws_scale` (evita `av_frame_alloc` por fotograma).
    rgba_scratch: *mut ffi::AVFrame,
    stream_idx: i32,
    time_base: ffi::AVRational,
    width: u32,
    height: u32,
    fps: f64,

    audio_stream_idx: i32,
    audio_codec_ctx: *mut ffi::AVCodecContext,
    swr_ctx: *mut ffi::SwrContext,
    /// Reservado para PTS de audio alineado al stream; hoy el PCM va a rodio sin marca temporal aquí.
    _audio_time_base: ffi::AVRational,
    target_sample_rate: i32,
    target_channels: i32,
    audio_scratch: *mut u8,
    audio_scratch_cap: i32,
}

impl Drop for DecoderCtx {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                ffi::avcodec_free_context(&mut self.codec_ctx);
            }
            if !self.audio_codec_ctx.is_null() {
                ffi::avcodec_free_context(&mut self.audio_codec_ctx);
            }
            if !self.fmt_ctx.is_null() {
                ffi::avformat_close_input(&mut self.fmt_ctx);
            }
            if !self.sws_ctx.is_null() {
                ffi::sws_freeContext(self.sws_ctx);
            }
            if !self.rgba_scratch.is_null() {
                ffi::av_frame_free(&mut self.rgba_scratch);
            }
            if !self.swr_ctx.is_null() {
                ffi::swr_free(&mut self.swr_ctx);
            }
            if !self.audio_scratch.is_null() {
                ffi::av_freep(&mut self.audio_scratch as *mut _ as *mut _);
            }
        }
    }
}

fn open_decoder(path: &str, target_sample_rate: i32, target_channels: i32) -> Result<DecoderCtx> {
    let c_path = CString::new(path)?;
    unsafe {
        let mut fmt_ctx: *mut ffi::AVFormatContext = ptr::null_mut();
        let ret =
            ffi::avformat_open_input(&mut fmt_ctx, c_path.as_ptr(), ptr::null(), ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("open: {}", av_err(ret)));
        }

        let ret = ffi::avformat_find_stream_info(fmt_ctx, ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("stream info: {}", av_err(ret)));
        }

        let nb = (*fmt_ctx).nb_streams as usize;
        let streams = std::slice::from_raw_parts((*fmt_ctx).streams, nb);
        let video_idx = find_video_stream(streams);
        if video_idx < 0 {
            return Err(anyhow!("no video stream"));
        }

        let stream = *streams[video_idx as usize];
        let par = stream.codecpar;
        let time_base = stream.time_base;

        let codec = ffi::avcodec_find_decoder((*par).codec_id);
        if codec.is_null() {
            return Err(anyhow!("codec not found"));
        }

        let mut codec_ctx = ffi::avcodec_alloc_context3(codec);
        if codec_ctx.is_null() {
            return Err(anyhow!("avcodec_alloc_context3"));
        }

        let ret = ffi::avcodec_parameters_to_context(codec_ctx, par);
        if ret < 0 {
            return Err(anyhow!("params_to_ctx: {}", av_err(ret)));
        }

        // Enable multithreaded decoding
        (*codec_ctx).thread_count = 0; // auto
        (*codec_ctx).thread_type = ffi::FF_THREAD_FRAME as i32;

        let ret = ffi::avcodec_open2(codec_ctx, codec, ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("avcodec_open2: {}", av_err(ret)));
        }

        let width = (*codec_ctx).width as u32;
        let height = (*codec_ctx).height as u32;
        let src_fmt = (*codec_ctx).pix_fmt;

        let fps = if stream.avg_frame_rate.den != 0 {
            stream.avg_frame_rate.num as f64 / stream.avg_frame_rate.den as f64
        } else {
            25.0
        };

        let sws_ctx = ffi::sws_getContext(
            width as i32,
            height as i32,
            src_fmt,
            width as i32,
            height as i32,
            ffi::AVPixelFormat::AV_PIX_FMT_RGBA,
            SWS_FAST_BILINEAR,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null(),
        );
        if sws_ctx.is_null() {
            return Err(anyhow!("sws_getContext failed"));
        }

        let mut rgba_scratch = ffi::av_frame_alloc();
        if rgba_scratch.is_null() {
            ffi::sws_freeContext(sws_ctx);
            ffi::avcodec_free_context(&mut codec_ctx);
            ffi::avformat_close_input(&mut fmt_ctx);
            return Err(anyhow!("av_frame_alloc (rgba scratch)"));
        }
        (*rgba_scratch).width = width as i32;
        (*rgba_scratch).height = height as i32;
        (*rgba_scratch).format = ffi::AVPixelFormat::AV_PIX_FMT_RGBA as i32;
        let buf_ret = ffi::av_frame_get_buffer(rgba_scratch, 0);
        if buf_ret < 0 {
            ffi::av_frame_free(&mut rgba_scratch);
            ffi::sws_freeContext(sws_ctx);
            ffi::avcodec_free_context(&mut codec_ctx);
            ffi::avformat_close_input(&mut fmt_ctx);
            return Err(anyhow!("av_frame_get_buffer (rgba): {}", av_err(buf_ret)));
        }

        let mut audio_stream_idx = -1;
        let mut audio_codec_ctx = ptr::null_mut();
        let mut swr_ctx = ptr::null_mut();
        let mut audio_time_base = ffi::AVRational { num: 0, den: 1 };

        let a_idx = find_audio_stream(streams);
        if a_idx >= 0 {
            audio_stream_idx = a_idx;
            let a_stream = *streams[a_idx as usize];
            let a_par = a_stream.codecpar;
            audio_time_base = a_stream.time_base;
            let a_codec = ffi::avcodec_find_decoder((*a_par).codec_id);
            if !a_codec.is_null() {
                audio_codec_ctx = ffi::avcodec_alloc_context3(a_codec);
                if ffi::avcodec_parameters_to_context(audio_codec_ctx, a_par) >= 0 {
                    if ffi::avcodec_open2(audio_codec_ctx, a_codec, ptr::null_mut()) >= 0 {
                        swr_ctx = ffi::swr_alloc();

                        let mut out_ch_layout: ffi::AVChannelLayout = std::mem::zeroed();
                        ffi::av_channel_layout_default(&mut out_ch_layout, target_channels);

                        let ret = ffi::swr_alloc_set_opts2(
                            &mut swr_ctx,
                            &out_ch_layout,
                            ffi::AVSampleFormat::AV_SAMPLE_FMT_FLT,
                            target_sample_rate,
                            &(*a_par).ch_layout,
                            std::mem::transmute((*a_par).format),
                            (*a_par).sample_rate,
                            0,
                            ptr::null_mut(),
                        );

                        if ret >= 0 {
                            ffi::swr_init(swr_ctx);
                            log::info!(
                                "SWR ctx initialized: out {} Hz, {} ch",
                                target_sample_rate,
                                target_channels
                            );
                        } else {
                            log::warn!("Failed to init SwrContext, audio disabled.");
                            ffi::swr_free(&mut swr_ctx);
                            swr_ctx = ptr::null_mut();
                            audio_stream_idx = -1;
                        }
                    } else {
                        audio_stream_idx = -1;
                    }
                } else {
                    audio_stream_idx = -1;
                }
            } else {
                audio_stream_idx = -1;
            }
        }

        Ok(DecoderCtx {
            fmt_ctx,
            codec_ctx,
            sws_ctx,
            rgba_scratch,
            stream_idx: video_idx,
            time_base,
            width,
            height,
            fps,
            audio_stream_idx,
            audio_codec_ctx,
            swr_ctx,
            _audio_time_base: audio_time_base,
            target_sample_rate,
            target_channels,
            audio_scratch: ptr::null_mut(),
            audio_scratch_cap: 0,
        })
    }
}

fn decoder_loop(
    path: &str,
    target_sample_rate: i32,
    target_channels: i32,
    cmd_rx: Receiver<DecoderCommand>,
    frame_tx: Sender<VideoFrame>,
    audio_tx: Sender<AudioFrame>,
) -> Result<()> {
    let mut ctx = open_decoder(path, target_sample_rate, target_channels)?;
    log::info!(
        "Decoder open: '{path}' {}×{} @ {:.2}fps",
        ctx.width,
        ctx.height,
        ctx.fps
    );

    let mut is_playing = false;
    let mut current_pts: i64 = 0;
    let _frame_dur = if ctx.fps > 0.0 {
        secs_to_pts(1.0 / ctx.fps, ctx.time_base)
    } else {
        1
    };

    unsafe {
        let packet = ffi::av_packet_alloc();
        let frame = ffi::av_frame_alloc();
        let mut pending_frame: Option<VideoFrame> = None;

        loop {
            // Priority 1: drain existing commands without blocking
            let mut pending_seek = None;
            let mut pending_play_state: Option<bool> = None;

            while let Ok(cmd) = cmd_rx.try_recv() {
                match cmd {
                    DecoderCommand::Seek(secs) => {
                        pending_seek = Some(secs);
                    }
                    DecoderCommand::Play => {
                        pending_play_state = Some(true);
                    }
                    DecoderCommand::Pause => {
                        pending_play_state = Some(false);
                    }
                    DecoderCommand::StepForward => {
                        // Process pending Seek/Play/Pause before this step
                        if let Some(secs) = pending_seek.take() {
                            handle_cmd(
                                DecoderCommand::Seek(secs),
                                &mut ctx,
                                &frame_tx,
                                &audio_tx,
                                &mut is_playing,
                                &mut current_pts,
                            )?;
                        }
                        if let Some(play) = pending_play_state.take() {
                            handle_cmd(
                                if play {
                                    DecoderCommand::Play
                                } else {
                                    DecoderCommand::Pause
                                },
                                &mut ctx,
                                &frame_tx,
                                &audio_tx,
                                &mut is_playing,
                                &mut current_pts,
                            )?;
                        }
                        handle_cmd(
                            cmd,
                            &mut ctx,
                            &frame_tx,
                            &audio_tx,
                            &mut is_playing,
                            &mut current_pts,
                        )?;
                        if !is_playing {
                            pending_frame = None;
                        }
                        // Stop draining more commands this cycle to prevent thread-hogging if holding keys
                        break;
                    }
                    _ => {
                        handle_cmd(
                            cmd,
                            &mut ctx,
                            &frame_tx,
                            &audio_tx,
                            &mut is_playing,
                            &mut current_pts,
                        )?;
                        if !is_playing {
                            pending_frame = None;
                        }
                    }
                }
            }
            if let Some(secs) = pending_seek.take() {
                handle_cmd(
                    DecoderCommand::Seek(secs),
                    &mut ctx,
                    &frame_tx,
                    &audio_tx,
                    &mut is_playing,
                    &mut current_pts,
                )?;
                if !is_playing {
                    pending_frame = None;
                }
            }
            if let Some(play) = pending_play_state.take() {
                let state_cmd = if play {
                    DecoderCommand::Play
                } else {
                    DecoderCommand::Pause
                };
                handle_cmd(
                    state_cmd,
                    &mut ctx,
                    &frame_tx,
                    &audio_tx,
                    &mut is_playing,
                    &mut current_pts,
                )?;
            }

            // If we need a frame, decode one
            if is_playing && pending_frame.is_none() {
                if let Some(f) = decode_one_frame(&mut ctx, packet, frame, &audio_tx)? {
                    current_pts = secs_to_pts(f.pts, ctx.time_base);
                    pending_frame = Some(f);
                } else {
                    log::info!("Decoder EOF or stopped at end of file: '{path}'");
                    is_playing = false; // EOF
                }
            }

            if let Some(f) = pending_frame.take() {
                // If paused, we don't try to send to avoid filling channel and blocking,
                // wait, if paused we DO want to send one frame to show the current frame!
                // But if it's paused we only send it once.
                crossbeam_channel::select! {
                    send(frame_tx, f.clone()) -> res => {
                        if res.is_err() {
                            log::warn!("Decoder thread exiting: UI frame channel disconnected");
                            return Ok(());
                        }
                        // Throttle removed: we rely on channel bounds (frame_tx = 8) to pace the decoder naturally.
                        // This allows audio_tx to build a healthy buffer so rodio never starves.
                    }
                    recv(cmd_rx) -> msg => {
                        pending_frame = Some(f); // Put it back
                        if let Ok(cmd) = msg {
                            log::trace!("Decoder received command: {:?}", cmd);
                            handle_cmd(cmd, &mut ctx, &frame_tx, &audio_tx, &mut is_playing, &mut current_pts)?;
                            if !is_playing { pending_frame = None; }
                        } else {
                            log::warn!("Decoder thread exiting: Command channel disconnected");
                            return Ok(());
                        }
                    }
                }
            } else {
                // No pending frame (either EOF or paused without a frame). Block on commands.
                let msg = cmd_rx.recv();
                if let Ok(cmd) = msg {
                    log::trace!("Decoder received command (idle): {:?}", cmd);
                    handle_cmd(
                        cmd,
                        &mut ctx,
                        &frame_tx,
                        &audio_tx,
                        &mut is_playing,
                        &mut current_pts,
                    )?;
                } else {
                    log::warn!("Decoder thread exiting (idle): Command channel disconnected");
                    return Ok(());
                }
            }
        } // loop
    } // unsafe
}

unsafe fn handle_cmd(
    cmd: DecoderCommand,
    ctx: &mut DecoderCtx,
    frame_tx: &Sender<VideoFrame>,
    audio_tx: &Sender<AudioFrame>,
    is_playing: &mut bool,
    current_pts: &mut i64,
) -> Result<()> {
    match &cmd {
        DecoderCommand::Play => {
            trace_log::log("Decoder: Play");
            *is_playing = true;
        }
        DecoderCommand::Pause => {
            trace_log::log("Decoder: Pause");
            *is_playing = false;
        }
        DecoderCommand::Stop => {
            trace_log::log("Decoder: Stop");
            // Thread will exit because we don't have a way to gracefully return from here directly,
            // but we can close the channel or trigger disconnect.
        }
        DecoderCommand::Seek(secs) => {
            trace_log::log(&format!("Decoder: Seek {:.3}s", secs));
            *is_playing = false;
            let target = secs_to_pts(*secs, ctx.time_base);
            seek_exact(ctx, target, frame_tx, audio_tx)?;
            *current_pts = target;
        }
        DecoderCommand::StepForward => {
            trace_log::log("Decoder: StepForward");
            // For step forward, we decode one immediately and send it.
            // Using blocking send() to ensure the decoder and the UI clock stay perfectly in sync.
            let packet = ffi::av_packet_alloc();
            let frame = ffi::av_frame_alloc();
            if let Some(f) = decode_one_frame(ctx, packet, frame, audio_tx)? {
                let frame_pts_raw = secs_to_pts(f.pts, ctx.time_base);
                if frame_tx.try_send(f).is_ok() {
                    *current_pts = frame_pts_raw;
                }
            }
            ffi::av_packet_free(&mut (packet as *mut _));
            ffi::av_frame_free(&mut (frame as *mut _));
        }
        DecoderCommand::SetVolume(_) => {}
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Decode one frame and send it
// ---------------------------------------------------------------------------

unsafe fn decode_one_frame(
    ctx: &mut DecoderCtx,
    packet: *mut ffi::AVPacket,
    frame: *mut ffi::AVFrame,
    audio_tx: &Sender<AudioFrame>,
) -> Result<Option<VideoFrame>> {
    // Try to receive a buffered frame first
    let ret = ffi::avcodec_receive_frame(ctx.codec_ctx, frame);
    if ret == 0 {
        let pts = (*frame).best_effort_timestamp;
        let vf = convert_frame(ctx, frame, pts)?;
        ffi::av_frame_unref(frame);
        return Ok(Some(vf));
    } else if ret != ffi::AVERROR(ffi::EAGAIN) && ret != ffi::AVERROR_EOF {
        log::warn!("avcodec_receive_frame error code: {}", ret);
    }

    // Read packets until we get a frame
    loop {
        let ret = ffi::av_read_frame(ctx.fmt_ctx, packet);
        if ret < 0 {
            // EOF — flush decoder
            ffi::avcodec_send_packet(ctx.codec_ctx, ptr::null());
            let r2 = ffi::avcodec_receive_frame(ctx.codec_ctx, frame);
            if r2 == 0 {
                let pts = (*frame).best_effort_timestamp;
                let vf = convert_frame(ctx, frame, pts)?;
                ffi::av_frame_unref(frame);
                return Ok(Some(vf));
            }
            return Ok(None);
        }

        if (*packet).stream_index == ctx.stream_idx {
            ffi::avcodec_send_packet(ctx.codec_ctx, packet);
            ffi::av_packet_unref(packet);

            let r2 = ffi::avcodec_receive_frame(ctx.codec_ctx, frame);
            if r2 == 0 {
                let pts = (*frame).best_effort_timestamp;
                let vf = convert_frame(ctx, frame, pts)?;
                ffi::av_frame_unref(frame);
                return Ok(Some(vf));
            }
        } else if (*packet).stream_index == ctx.audio_stream_idx {
            ffi::avcodec_send_packet(ctx.audio_codec_ctx, packet);
            ffi::av_packet_unref(packet);

            while ffi::avcodec_receive_frame(ctx.audio_codec_ctx, frame) == 0 {
                if let Some(audio) = convert_audio_frame(ctx, frame)? {
                    // Send blocking (with bounds) so we don't drop audio packets and cause glitches
                    let _ = audio_tx.send(audio);
                }
                ffi::av_frame_unref(frame);
            }
        } else {
            ffi::av_packet_unref(packet);
        }
    }
}

unsafe fn seek_exact(
    ctx: &mut DecoderCtx,
    target_pts: i64,
    frame_tx: &Sender<VideoFrame>,
    audio_tx: &Sender<AudioFrame>,
) -> Result<()> {
    // Seek to slightly before target to get the keyframe
    ffi::av_seek_frame(
        ctx.fmt_ctx,
        ctx.stream_idx,
        target_pts,
        ffi::AVSEEK_FLAG_BACKWARD as i32,
    );
    ffi::avcodec_flush_buffers(ctx.codec_ctx);

    let packet = ffi::av_packet_alloc();
    let frame = ffi::av_frame_alloc();

    loop {
        let ret = ffi::av_read_frame(ctx.fmt_ctx, packet);
        if ret < 0 {
            break;
        }

        if (*packet).stream_index == ctx.stream_idx {
            ffi::avcodec_send_packet(ctx.codec_ctx, packet);
            ffi::av_packet_unref(packet);

            let r2 = ffi::avcodec_receive_frame(ctx.codec_ctx, frame);
            if r2 == 0 {
                let pts = (*frame).best_effort_timestamp;
                if pts >= target_pts {
                    let vf = convert_frame(ctx, frame, pts)?;
                    let _ = frame_tx.send(vf);
                    ffi::av_frame_unref(frame);
                    break;
                }
                ffi::av_frame_unref(frame);
            }
        } else if (*packet).stream_index == ctx.audio_stream_idx {
            ffi::avcodec_send_packet(ctx.audio_codec_ctx, packet);
            ffi::av_packet_unref(packet);

            while ffi::avcodec_receive_frame(ctx.audio_codec_ctx, frame) == 0 {
                let pts = (*frame).best_effort_timestamp;
                if pts >= target_pts {
                    if let Some(audio) = convert_audio_frame(ctx, frame)? {
                        let _ = audio_tx.send(audio);
                    }
                }
                ffi::av_frame_unref(frame);
            }
        } else {
            ffi::av_packet_unref(packet);
        }
    }

    ffi::av_packet_free(&mut (packet as *mut _));
    ffi::av_frame_free(&mut (frame as *mut _));
    Ok(())
}

unsafe fn convert_frame(
    ctx: &DecoderCtx,
    frame: *mut ffi::AVFrame,
    pts_raw: i64,
) -> Result<VideoFrame> {
    let w = ctx.width;
    let h = ctx.height;
    let pts = pts_to_secs(pts_raw, ctx.time_base);

    let dst_frame = ctx.rgba_scratch;
    if dst_frame.is_null() {
        return Err(anyhow!("rgba scratch frame null"));
    }

    let src_data: [*const u8; 4] = [
        (*frame).data[0],
        (*frame).data[1],
        (*frame).data[2],
        (*frame).data[3],
    ];

    ffi::sws_scale(
        ctx.sws_ctx,
        src_data.as_ptr(),
        (*frame).linesize.as_ptr(),
        0,
        h as i32,
        (*dst_frame).data.as_mut_ptr(),
        (*dst_frame).linesize.as_mut_ptr(),
    );

    let stride = (*dst_frame).linesize[0] as usize;
    let pixel_bytes = 4;
    let mut rgba_data = Vec::with_capacity((w * h) as usize * pixel_bytes);
    let src_ptr = (*dst_frame).data[0];
    for row in 0..h as usize {
        let row_start = src_ptr.add(row * stride);
        let row_slice = std::slice::from_raw_parts(row_start, w as usize * pixel_bytes);
        rgba_data.extend_from_slice(row_slice);
    }

    Ok(VideoFrame {
        pts,
        rgba_data: rgba_data.into(),
        width: w,
        height: h,
    })
}

unsafe fn convert_audio_frame(
    ctx: &mut DecoderCtx,
    frame: *mut ffi::AVFrame,
) -> Result<Option<AudioFrame>> {
    if ctx.swr_ctx.is_null() {
        return Ok(None);
    }

    let nb_samples = (*frame).nb_samples;
    // Calculate out samples (allowing up to 10% more for resampling drift)
    let out_samples_cap = ffi::swr_get_out_samples(ctx.swr_ctx, nb_samples);

    if ctx.audio_scratch_cap < out_samples_cap {
        if !ctx.audio_scratch.is_null() {
            ffi::av_freep(&mut ctx.audio_scratch as *mut _ as *mut _);
        }
        ffi::av_samples_alloc(
            &mut ctx.audio_scratch,
            ptr::null_mut(),
            ctx.target_channels,
            out_samples_cap,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_FLT,
            0,
        );
        ctx.audio_scratch_cap = out_samples_cap;
    }

    let out_samples_count = ffi::swr_convert(
        ctx.swr_ctx,
        &mut ctx.audio_scratch,
        out_samples_cap,
        (*frame).data.as_ptr() as *mut *const u8,
        nb_samples,
    );

    if out_samples_count < 0 {
        return Err(anyhow!("swr_convert failed"));
    }

    let byte_size = out_samples_count as usize * ctx.target_channels as usize * 4; // count * channels * sizeof(f32)
    let slice = std::slice::from_raw_parts(ctx.audio_scratch as *const f32, byte_size / 4);
    let mut samples = Vec::with_capacity(slice.len());
    samples.extend_from_slice(slice);

    Ok(Some(AudioFrame {
        samples,
        channels: ctx.target_channels as u16,
        sample_rate: ctx.target_sample_rate as u32,
    }))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pts_to_secs(pts: i64, tb: ffi::AVRational) -> f64 {
    if tb.den == 0 {
        return 0.0;
    }
    pts as f64 * tb.num as f64 / tb.den as f64
}

fn secs_to_pts(secs: f64, tb: ffi::AVRational) -> i64 {
    if tb.num == 0 {
        return 0;
    }
    (secs * tb.den as f64 / tb.num as f64) as i64
}

unsafe fn find_video_stream(streams: &[*mut ffi::AVStream]) -> i32 {
    for (idx, &stream) in streams.iter().enumerate() {
        if (*(*stream).codecpar).codec_type == ffi::AVMediaType::AVMEDIA_TYPE_VIDEO {
            return idx as i32;
        }
    }
    -1
}

unsafe fn find_audio_stream(streams: &[*mut ffi::AVStream]) -> i32 {
    for (idx, &stream) in streams.iter().enumerate() {
        if (*(*stream).codecpar).codec_type == ffi::AVMediaType::AVMEDIA_TYPE_AUDIO {
            return idx as i32;
        }
    }
    -1
}

fn color_space_str(cs: ffi::AVColorSpace) -> String {
    unsafe {
        let ptr = ffi::av_color_space_name(cs);
        if ptr.is_null() {
            return "unknown".into();
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

fn color_trc_str(trc: ffi::AVColorTransferCharacteristic) -> String {
    unsafe {
        let ptr = ffi::av_color_transfer_name(trc);
        if ptr.is_null() {
            return "unknown".into();
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

fn color_primaries_str(prim: ffi::AVColorPrimaries) -> String {
    unsafe {
        let ptr = ffi::av_color_primaries_name(prim);
        if ptr.is_null() {
            return "unknown".into();
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

fn av_err(code: i32) -> String {
    let mut buf = [0i8; 256];
    unsafe {
        ffi::av_strerror(code, buf.as_mut_ptr(), buf.len());
    }
    let s = unsafe { CStr::from_ptr(buf.as_ptr()) };
    s.to_string_lossy().into_owned()
}

// SAFETY: AVFormatContext, AVCodecContext etc. pointers are only used on the decoder thread.
unsafe impl Send for DecoderCtx {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pts_to_secs() {
        let tb = ffi::AVRational { num: 1, den: 1000 };
        assert_eq!(pts_to_secs(500, tb), 0.5);
        assert_eq!(pts_to_secs(0, tb), 0.0);

        let tb_zero = ffi::AVRational { num: 1, den: 0 };
        assert_eq!(pts_to_secs(100, tb_zero), 0.0);
    }

    #[test]
    fn test_secs_to_pts() {
        let tb = ffi::AVRational { num: 1, den: 1000 };
        assert_eq!(secs_to_pts(0.5, tb), 500);
        assert_eq!(secs_to_pts(0.0, tb), 0);

        let tb_zero = ffi::AVRational { num: 1, den: 0 };
        assert_eq!(secs_to_pts(1.0, tb_zero), 0);
    }
}
</file>

<file path="src/main.rs">
//! Punto de entrada del binario: logging, icono de ventana y arranque de `eframe`.
//!
//! Los módulos de la app viven en `app/`, `decoder`, `renderer`, etc. No contiene lógica de QC;
//! solo configura el entorno nativo antes de delegar en [`DiffPlayerApp`](crate::app::DiffPlayerApp).

mod app;
mod decoder;
mod error;
pub use error::AppError;
pub mod metrics;
mod proxy;
mod renderer;
mod trace_log;
mod types;
mod ui;

use eframe::{egui, App, CreationContext};
use image::imageops::FilterType;

fn main() -> anyhow::Result<()> {
    // Escupir logs a un fichero temporal incondicionalmente para poder leer por qué no arranca la vista
    let log_file = std::fs::File::create("/tmp/diffplayerqc_app.log")?;
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Pipe(Box::new(log_file)));
    builder.filter_level(log::LevelFilter::Info);
    builder.init();

    log::info!("=== DiffPlayerQC Startup (LOG REDIRECTED) ===");

    // Human-readable trace log (one file per run: yyyy_mm_dd_hh_mm_ss_Diff_start.log)
    let log_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    if let Err(e) = trace_log::init(log_dir) {
        log::warn!("Trace log init failed: {e}");
    } else {
        trace_log::log("DiffPlayerQC started");
    }

    // Load icon and resize to 64x64 so macOS window creation doesn't block (large icons can hang).
    let icon_data: Option<egui::IconData> = {
        let icon_bytes = include_bytes!("../assets/Icon-iOS-Default-1024x1024@1x.png");
        image::load_from_memory(icon_bytes).ok().map(|img| {
            let rgba = img.into_rgba8();
            let small = image::imageops::resize(&rgba, 64, 64, FilterType::Triangle);
            let (w, h) = small.dimensions();
            let pixels = small.into_raw();
            egui::IconData {
                rgba: pixels,
                width: w,
                height: h,
            }
        })
    };

    let mut viewport_builder = egui::ViewportBuilder::default()
        .with_title("WPP Production Media Diferencial Player")
        .with_inner_size([1600.0, 900.0])
        .with_min_inner_size([900.0, 560.0]);

    if let Some(icon) = icon_data {
        viewport_builder = viewport_builder.with_icon(std::sync::Arc::new(icon));
    }

    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: viewport_builder,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        centered: false,
        ..Default::default()
    };

    log::info!("Starting eframe application loop...");

    eframe::run_native(
        "WPP Production Media Diferencial Player",
        native_options,
        Box::new(|cc: &CreationContext<'_>| {
            log::info!("CreationContext initialized, building app...");
            trace_log::log("CreationContext ready, building app");
            let app = app::DiffPlayerApp::new(cc);
            Box::new(app) as Box<dyn App>
        }),
    )
    .map_err(|e| {
        log::error!("Eframe execution error: {e}");
        anyhow::anyhow!("{e}")
    })?;

    log::info!("Application exited cleanly.");
    Ok(())
}
</file>

<file path="src/ui/controls.rs">
// ui/controls.rs — Menu bar (drop-down menus + inline toolbar controls)

use egui::{Color32, RichText, Ui};

use crate::app::DiffPlayerApp;
use crate::types::{Channel, CompareMode, DiffMode, Language, SafeZoneMode};
use crate::ui::design::{tr, ACCENT_PRIMARY, FONT_LABEL};
use crate::ui::i18n::{diff_mode_label, THEME_MENU_CHOICES};
use crate::ui::theme::apply_theme;

/// Title for the OBS clean-feed secondary viewport.
pub fn clean_feed_window_title(lang: Language) -> String {
    tr(
        lang,
        "DiffPlayerQC — Salida limpia",
        "DiffPlayerQC — Clean Feed",
        "DiffPlayerQC — Cén sirima",
    )
    .to_string()
}

/// Single-line overlay (mode, channel, PTS, frame) for the clean-feed window.
pub fn clean_feed_overlay_text(
    lang: Language,
    mode: CompareMode,
    split_pos: f32,
    pts: f64,
    fps: f64,
) -> String {
    let mode_str = match mode {
        CompareMode::SplitScreen => {
            if split_pos <= 0.01 {
                tr(lang, "Solo B", "B Only", "Erya B")
            } else if split_pos >= 0.99 {
                tr(lang, "Solo A", "A Only", "Erya A")
            } else {
                tr(lang, "Cortina", "Split", "Hyanda")
            }
        }
        CompareMode::AbsDiff => tr(lang, "Diferencia", "Diff", "Winya"),
        CompareMode::Heatmap => tr(lang, "Mapa de calor", "Heatmap", "Úrë"),
        CompareMode::SideBySide => tr(lang, "Lado a lado", "Side by side", "Ara"),
    };

    let video_str = match mode {
        CompareMode::SplitScreen => {
            if split_pos <= 0.01 {
                tr(lang, "VÍDEO B", "VIDEO B", "VÍDEO B")
            } else if split_pos >= 0.99 {
                tr(lang, "VÍDEO A", "VIDEO A", "VÍDEO A")
            } else {
                tr(lang, "VÍDEO A + B", "VIDEO A + B", "A + B")
            }
        }
        _ => tr(lang, "VÍDEO A + B", "VIDEO A + B", "A + B"),
    };

    let rough_frame = (pts * fps).round() as u64;
    format!(
        "{} | {} | {}: {:.3}s | {} {}",
        video_str,
        mode_str,
        tr(lang, "PTS", "PTS", "PTS"),
        pts,
        tr(lang, "Cuad.", "Frm.", "Fr."),
        rough_frame
    )
}

pub fn proxy_loading_caption(lang: Language) -> &'static str {
    tr(
        lang,
        "Cargando imágenes…",
        "Loading images…",
        "Cárala yando…",
    )
}

/// Renders the full menu bar: classic dropdown menus followed by an inline
/// compact toolbar row, all in a single top panel.
pub fn show_menu_bar(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    egui::menu::bar(ui, |ui| {
        // ── Dropdown menus ──────────────────────────────────────────────────

        ui.menu_button(tr(lang, "Archivo", "File", "Parma"), |ui| {
            if ui
                .button(tr(
                    lang,
                    "Abrir VÍDEO A…",
                    "Open VIDEO A…",
                    "Panya VÍDEO A…",
                ))
                .clicked()
            {
                app.open_video_a(ui.ctx());
                ui.close_menu();
            }

            if ui
                .button(tr(
                    lang,
                    "Abrir VÍDEO B…",
                    "Open VIDEO B…",
                    "Panya VÍDEO B…",
                ))
                .clicked()
            {
                app.open_video_b(ui.ctx());
                ui.close_menu();
            }

            ui.separator();

            if ui
                .button(tr(
                    lang,
                    "Cargar sesión (.dpqc)…",
                    "Load Session (.dpqc)…",
                    "Load Session (.dpqc)…",
                ))
                .clicked()
            {
                app.load_session(ui.ctx());
                ui.close_menu();
            }

            if ui
                .button(tr(
                    lang,
                    "Guardar sesión (.dpqc)…",
                    "Save Session (.dpqc)…",
                    "Save Session (.dpqc)…",
                ))
                .clicked()
            {
                app.save_session();
                ui.close_menu();
            }

            if ui
                .button(tr(
                    lang,
                    "Exportar marcadores a CSV…",
                    "Export markers to CSV…",
                    "Export markers to CSV…",
                ))
                .clicked()
            {
                app.export_csv();
                ui.close_menu();
            }

            ui.separator();

            if ui
                .button(tr(
                    lang,
                    "Abrir secuencia EXR (A)…",
                    "Open EXR sequence (A)…",
                    "Panya EXR sequence (A)…",
                ))
                .clicked()
            {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    app.start_proxy_from_exr_input_dir(folder, Channel::A, ui.ctx());
                }
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Abrir secuencia EXR (B)…",
                    "Open EXR sequence (B)…",
                    "Panya EXR sequence (B)…",
                ))
                .clicked()
            {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    app.start_proxy_from_exr_input_dir(folder, Channel::B, ui.ctx());
                }
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Abrir archivos EXR (A)…",
                    "Open EXR files (A)…",
                    "Panya EXR files (A)…",
                ))
                .clicked()
            {
                if let Some(files) = rfd::FileDialog::new()
                    .add_filter("EXR", &["exr"])
                    .pick_files()
                {
                    if !files.is_empty() {
                        app.start_proxy_from_exr_input_files(files, Channel::A, ui.ctx());
                    }
                }
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Abrir archivos EXR (B)…",
                    "Open EXR files (B)…",
                    "Panya EXR files (B)…",
                ))
                .clicked()
            {
                if let Some(files) = rfd::FileDialog::new()
                    .add_filter("EXR", &["exr"])
                    .pick_files()
                {
                    if !files.is_empty() {
                        app.start_proxy_from_exr_input_files(files, Channel::B, ui.ctx());
                    }
                }
                ui.close_menu();
            }

            ui.separator();
            if ui
                .button(tr(
                    lang,
                    "Guardar Frame como PNG  (F)",
                    "Save Frame as PNG  (F)",
                    "Marta Frame ve PNG  (F)",
                ))
                .clicked()
            {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Screenshot);
                ui.close_menu();
            }

            if ui
                .button(tr(
                    lang,
                    "Elegir carpeta de capturas…",
                    "Set Screenshot Folder…",
                    "Cilta Screenshot Nómë…",
                ))
                .clicked()
            {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    app.view_mut().screenshot_dir = Some(folder);
                }
                ui.close_menu();
            }
            ui.separator();
            if ui
                .button(tr(lang, "Salir  (Esc)", "Quit  (Esc)", "Vanya  (Esc)"))
                .clicked()
            {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        ui.menu_button(tr(lang, "Vista", "View", "Cén"), |ui| {
            if ui
                .button(tr(
                    lang,
                    "Ocultar/Mostrar Interfaz  (3)",
                    "Toggle HUD  (3)",
                    "Halya/Tanë HUD  (3)",
                ))
                .clicked()
            {
                let v = app.view().show_hud;
                app.view_mut().show_hud = !v;
                ui.close_menu();
            }
            ui.separator();
            let mut left = app.view().show_left_panel;
            if ui
                .checkbox(
                    &mut left,
                    tr(
                        lang,
                        "Barra izquierda (datos del vídeo)",
                        "Left panel (video data)",
                        "Parma left (video data)",
                    ),
                )
                .changed()
            {
                app.view_mut().show_left_panel = left;
                ui.close_menu();
            }
            let mut right = app.view().show_right_panel;
            if ui
                .checkbox(
                    &mut right,
                    tr(
                        lang,
                        "Barra derecha (controles y audio)",
                        "Right panel (controls & audio)",
                        "Parma right (controls & audio)",
                    ),
                )
                .changed()
            {
                app.view_mut().show_right_panel = right;
                ui.close_menu();
            }
            ui.separator();
            if ui
                .button(tr(
                    lang,
                    "Restaurar Zoom  (R)",
                    "Reset Zoom  (R)",
                    "En-panya Zoom  (R)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 1.0;
                app.view_mut().pan_u = 0.0;
                app.view_mut().pan_v = 0.0;
                ui.close_menu();
            }
            if ui
                .button(tr(lang, "Zoom 50%  (5)", "Zoom 50%  (5)", "Zoom 50%  (5)"))
                .clicked()
            {
                app.view_mut().zoom = 0.5;
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Zoom 100%  (6)",
                    "Zoom 100%  (6)",
                    "Zoom 100%  (6)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 1.0;
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Zoom 200%  (7)",
                    "Zoom 200%  (7)",
                    "Zoom 200%  (7)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 2.0;
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Zoom 400%  (8)",
                    "Zoom 400%  (8)",
                    "Zoom 400%  (8)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 4.0;
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Zoom 800%  (9)",
                    "Zoom 800%  (9)",
                    "Zoom 800%  (9)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 8.0;
                ui.close_menu();
            }
        });

        ui.menu_button(tr(lang, "Reproducción", "Playback", "Lirë"), |ui| {
            let is_p = app.playback().is_playing;
            if ui
                .button(if is_p {
                    tr(
                        lang,
                        "Pausar  (Espacio)",
                        "Pause  (Space)",
                        "Talta  (Espacio)",
                    )
                } else {
                    tr(
                        lang,
                        "Reproducir  (Espacio)",
                        "Play  (Space)",
                        "Lir  (Espacio)",
                    )
                })
                .clicked()
            {
                if is_p {
                    app.do_pause(ui.ctx());
                } else {
                    app.do_play(ui.ctx());
                }
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Retroceder Frame (Izquierda / Left)",
                    "Step Backward (Left)",
                    "Nánë Frame (Left)",
                ))
                .clicked()
            {
                app.do_step_bck(ui.ctx());
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Avanzar Frame (Derecha / Right)",
                    "Step Forward (Right)",
                    "Pónë Frame (Right)",
                ))
                .clicked()
            {
                app.do_step_fwd(ui.ctx());
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Ir al inicio  (Home)",
                    "Go to Start  (Home)",
                    "Mena Yessë  (Home)",
                ))
                .clicked()
            {
                app.do_seek(0.0, ui.ctx());
                ui.close_menu();
            }
        });

        ui.menu_button(tr(lang, "Opciones", "Options", "Cilmë"), |ui| {
            if ui
                .button(tr(
                    lang,
                    "Intercambiar A y B  (S)",
                    "Swap A and B  (S)",
                    "Quista A ar B  (S)",
                ))
                .clicked()
            {
                app.swap_videos(ui.ctx());
                ui.close_menu();
            }

            ui.separator();

            // Canvas background colour
            ui.horizontal(|ui| {
                ui.label(tr(lang, "Color fondo:", "Canvas color:", "Talan cala:"));
                let mut bg = app.view().canvas_bg_color;
                if ui.color_edit_button_rgb(&mut bg).changed() {
                    app.view_mut().canvas_bg_color = bg;
                }
            });

            ui.separator();
            ui.menu_button(
                tr(lang, "Idioma / Language", "Language / Idioma", "Lambë"),
                |ui| {
                    if ui
                        .radio_value(&mut app.view_mut().lang, Language::En, "English")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(&mut app.view_mut().lang, Language::Es, "Español")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut app.view_mut().lang,
                            Language::Quenya,
                            "Quenya (Elvish)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                },
            );
            ui.menu_button(tr(lang, "Tema / Theme", "Theme / Tema", "Cala"), |ui| {
                let mut current_theme = app.view().theme;
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        for &(theme_val, name) in THEME_MENU_CHOICES {
                            if ui
                                .radio_value(&mut current_theme, theme_val, name)
                                .clicked()
                            {
                                app.view_mut().theme = theme_val;
                                apply_theme(ui.ctx(), theme_val);
                                ui.close_menu();
                            }
                        }
                    });
            });
        });

        ui.menu_button(tr(lang, "Emisión", "Broadcast", "Sirë"), |ui| {
            let mut enabled = app.view().show_clean_feed_window;
            if ui
                .checkbox(
                    &mut enabled,
                    tr(
                        lang,
                        "Ventana de Salida  (OBS)",
                        "Clean Feed Window  (OBS)",
                        "Vëa Cén  (OBS)",
                    ),
                )
                .clicked()
            {
                app.view_mut().show_clean_feed_window = enabled;
                ui.close_menu();
            }
            ui.label(
                RichText::new(tr(
                    lang,
                    "Capturar ventana en OBS",
                    "Capture window in OBS",
                    "Mapa vëa mi OBS",
                ))
                .weak()
                .size(FONT_LABEL),
            );
            ui.separator();
            ui.label(
                RichText::new(tr(lang, "Zonas seguras", "Safe Zones", "Safe zones"))
                    .weak()
                    .size(FONT_LABEL),
            );
            let mut safe_zone = app.view().safe_zone;
            if ui
                .radio_value(
                    &mut safe_zone,
                    SafeZoneMode::None,
                    tr(lang, "Desactivado", "Off", "Off"),
                )
                .clicked()
            {
                ui.close_menu();
            }
            if ui
                .radio_value(&mut safe_zone, SafeZoneMode::TvEbu, "TV: EBU R95 (16:9)")
                .clicked()
            {
                ui.close_menu();
            }
            if ui
                .radio_value(
                    &mut safe_zone,
                    SafeZoneMode::Social,
                    tr(
                        lang,
                        "Móvil: Redes Sociales (9:16)",
                        "Mobile: Social (9:16)",
                        "Social (9:16)",
                    ),
                )
                .clicked()
            {
                ui.close_menu();
            }
            app.view_mut().safe_zone = safe_zone;
        });

        // ── Separator before inline controls ───────────────────────────────
        ui.separator();

        // ── Inline compact controls ─────────────────────────────────────────

        // File open buttons
        let has_a = app.decoder_a_path().is_some();
        let has_b = app.decoder_b_path().is_some();

        let a_label = app
            .decoder_a_path()
            .map(short_name)
            .unwrap_or_else(|| "A…".to_owned());
        let b_label = app
            .decoder_b_path()
            .map(short_name)
            .unwrap_or_else(|| "B…".to_owned());

        let a_tooltip = app
            .decoder_a_path()
            .map(|p| p.to_owned())
            .unwrap_or_else(|| tr(lang, "Abrir Vídeo A", "Open Video A", "Panya A").to_owned());
        if ui
            .add(egui::Button::new(
                RichText::new(format!("▶A {a_label}")).color(if has_a {
                    Color32::from_rgb(100, 200, 120)
                } else {
                    Color32::LIGHT_GRAY
                }),
            ))
            .on_hover_text(a_tooltip)
            .clicked()
        {
            app.open_video_a(ui.ctx());
        }

        let b_tooltip = app
            .decoder_b_path()
            .map(|p| p.to_owned())
            .unwrap_or_else(|| tr(lang, "Abrir Vídeo B", "Open Video B", "Panya B").to_owned());
        if ui
            .add(egui::Button::new(
                RichText::new(format!("▶B {b_label}")).color(if has_b {
                    Color32::from_rgb(100, 160, 240)
                } else {
                    Color32::LIGHT_GRAY
                }),
            ))
            .on_hover_text(b_tooltip)
            .clicked()
        {
            app.open_video_b(ui.ctx());
        }

        ui.separator();

        // Playback controls
        let mut loop_playback = app.view().loop_playback;
        if ui
            .checkbox(&mut loop_playback, tr(lang, "Bucle", "Loop", "Loop"))
            .changed()
        {
            app.view_mut().loop_playback = loop_playback;
            if loop_playback {
                app.playback_mut().loop_range_active = false;
            }
        }

        let mut loop_range = app.playback().loop_range_active;
        if ui
            .checkbox(
                &mut loop_range,
                tr(lang, "Bucle Rango", "Loop Range", "Loop Range"),
            )
            .changed()
        {
            if loop_range {
                app.toggle_loop_range(); // This handles turning it on and disabling `loop_playback`
            } else {
                app.playback_mut().loop_range_active = false;
            }
        }

        ui.add_space(4.0);
        if ui
            .button("[ I ]")
            .on_hover_text(tr(
                lang,
                "Marcar inicio de bucle",
                "Set Loop In",
                "Set Loop In",
            ))
            .clicked()
        {
            app.set_loop_in();
        }
        if ui
            .button("[ O ]")
            .on_hover_text(tr(
                lang,
                "Marcar fin de bucle",
                "Set Loop Out",
                "Set Loop Out",
            ))
            .clicked()
        {
            app.set_loop_out();
        }
        ui.add_space(4.0);

        let is_playing = app.playback().is_playing;
        if ui
            .button(RichText::new("|<").size(16.0))
            .on_hover_text(tr(lang, "Inicio", "Start", "Yessë"))
            .clicked()
        {
            app.do_seek(0.0, ui.ctx());
        }
        if ui
            .button(RichText::new("<<").size(16.0))
            .on_hover_text(tr(
                lang,
                "Retroceder (Izquierda)",
                "Step back (Left)",
                "Nánë (Left)",
            ))
            .clicked()
        {
            app.do_step_bck(ui.ctx());
        }
        if ui
            .button(RichText::new(if is_playing { "||" } else { ">" }).size(16.0))
            .on_hover_text(tr(
                lang,
                "Reproducir/Pausar (Espacio)",
                "Play/Pause (Space)",
                "Lir/Talta",
            ))
            .clicked()
        {
            if is_playing {
                app.do_pause(ui.ctx());
            } else {
                app.do_play(ui.ctx());
            }
        }
        if ui
            .button(RichText::new(">>").size(16.0))
            .on_hover_text(tr(
                lang,
                "Avanzar (Derecha)",
                "Step fwd (Right)",
                "Pónë (Right)",
            ))
            .clicked()
        {
            app.do_step_fwd(ui.ctx());
        }
        // Mode selector and contextual options moved to right sidebar (show_audio_panel) for low-res visibility.
    });
}

fn short_name(path: &str) -> String {
    // Show only the file stem (no extension) truncated to 18 chars for compactness
    let name = std::path::Path::new(path)
        .file_stem()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_owned());
    if name.len() > 18 {
        format!("{}…", &name[..18])
    } else {
        name
    }
}

/// Mode selector and contextual options (Cortina, Amp, Diff mode, Zoom). Used in the right sidebar.
pub fn show_mode_toolbar(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    let c_mode = app.view().mode;
    let split = app.view().split_pos;
    let is_a = c_mode == CompareMode::SplitScreen && split > 0.95;
    let is_b = c_mode == CompareMode::SplitScreen && split < 0.05;
    let is_split = c_mode == CompareMode::SplitScreen && !is_a && !is_b;
    let active = ACCENT_PRIMARY;

    ui.vertical(|ui| {
        ui.set_min_width(90.0);
        // Display mode buttons (stacked for narrow sidebar)
        if ui
            .add(
                egui::Button::new(tr(lang, "Solo A", "A Only", "Erya A")).fill(if is_a {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SplitScreen;
            app.view_mut().split_pos = 1.0;
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Solo B", "B Only", "Erya B")).fill(if is_b {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SplitScreen;
            app.view_mut().split_pos = 0.0;
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Cortina", "Split", "Hyanda")).fill(if is_split {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SplitScreen;
            if is_a || is_b {
                app.view_mut().split_pos = 0.5;
            }
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Diferencia", "Diff", "Winya")).fill(
                    if c_mode == CompareMode::AbsDiff {
                        active
                    } else {
                        Color32::TRANSPARENT
                    },
                ),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::AbsDiff;
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Mapa Calor", "Heatmap", "Úrë")).fill(
                    if c_mode == CompareMode::Heatmap {
                        active
                    } else {
                        Color32::TRANSPARENT
                    },
                ),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::Heatmap;
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Lado a Lado", "Side×Side", "Ara")).fill(
                    if c_mode == CompareMode::SideBySide {
                        active
                    } else {
                        Color32::TRANSPARENT
                    },
                ),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SideBySide;
        }

        ui.separator();

        match app.view().mode {
            CompareMode::SplitScreen => {
                let is_h = app.view().split_horizontal;
                if ui
                    .button(if is_h {
                        tr(lang, "Cortina H", "Split H", "Hya H")
                    } else {
                        tr(lang, "Cortina V", "Split V", "Hya V")
                    })
                    .clicked()
                {
                    app.view_mut().split_horizontal = !app.view().split_horizontal;
                }
                ui.label(if is_h {
                    tr(lang, "Cortina (Y):", "Split (Y):", "Hyanda (Y):")
                } else {
                    tr(lang, "Cortina (X):", "Split (X):", "Hyanda (X):")
                });
                let mut sp = app.view().split_pos;
                if ui
                    .add(egui::Slider::new(&mut sp, 0.0..=1.0).fixed_decimals(2))
                    .changed()
                {
                    app.view_mut().split_pos = sp;
                }
            }
            CompareMode::Heatmap | CompareMode::AbsDiff => {
                ui.label(tr(lang, "Amplificación:", "Amplification:", "Amp:"));
                let mut amp = app.view().amplifier;
                if ui
                    .add(
                        egui::Slider::new(&mut amp, 1.0..=50.0)
                            .step_by(0.5)
                            .suffix("×"),
                    )
                    .changed()
                {
                    app.view_mut().amplifier = amp;
                }
                if app.view().mode == CompareMode::AbsDiff {
                    ui.separator();
                    let mut d_mode = app.view().diff_mode;
                    egui::ComboBox::from_id_source("diff_mode_side")
                        .selected_text(diff_mode_label(lang, d_mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut d_mode,
                                DiffMode::LegacyAbs,
                                diff_mode_label(lang, DiffMode::LegacyAbs),
                            );
                            ui.selectable_value(
                                &mut d_mode,
                                DiffMode::AbsLinear,
                                diff_mode_label(lang, DiffMode::AbsLinear),
                            );
                            ui.selectable_value(
                                &mut d_mode,
                                DiffMode::AbsSqrt,
                                diff_mode_label(lang, DiffMode::AbsSqrt),
                            );
                            ui.selectable_value(
                                &mut d_mode,
                                DiffMode::SignedDiverging,
                                diff_mode_label(lang, DiffMode::SignedDiverging),
                            );
                        });
                    if d_mode != app.view().diff_mode {
                        app.view_mut().diff_mode = d_mode;
                    }
                }
            }
            CompareMode::SideBySide => {
                ui.label(tr(lang, "Amplificación:", "Amplification:", "Amp:"));
                let mut amp = app.view().amplifier;
                if ui
                    .add(
                        egui::Slider::new(&mut amp, 1.0..=50.0)
                            .step_by(0.5)
                            .suffix("×"),
                    )
                    .changed()
                {
                    app.view_mut().amplifier = amp;
                }
                ui.separator();
                let mut d_mode = app.view().diff_mode;
                egui::ComboBox::from_id_source("diff_mode_sbs_side")
                    .selected_text(match d_mode {
                        DiffMode::None => tr(lang, "Sin Filtro", "No Filter", "Munca").to_string(),
                        _ => diff_mode_label(lang, d_mode).to_string(),
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::LegacyAbs,
                            diff_mode_label(lang, DiffMode::LegacyAbs),
                        );
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::AbsLinear,
                            diff_mode_label(lang, DiffMode::AbsLinear),
                        );
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::AbsSqrt,
                            diff_mode_label(lang, DiffMode::AbsSqrt),
                        );
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::SignedDiverging,
                            diff_mode_label(lang, DiffMode::SignedDiverging),
                        );
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::None,
                            tr(lang, "Sin Filtro", "No Filter", "Munca"),
                        );
                    });
                if d_mode != app.view().diff_mode {
                    app.view_mut().diff_mode = d_mode;
                }
            }
        }

        let zoom = app.view().zoom;
        if (zoom - 1.0).abs() > 0.01 {
            ui.separator();
            if ui
                .button(format!("Zoom {:.1}×", zoom))
                .on_hover_text(tr(
                    lang,
                    "Restaurar zoom y paneo",
                    "Reset zoom and pan",
                    "En-panya hyanda ar pan",
                ))
                .clicked()
            {
                app.view_mut().zoom = 1.0;
                app.view_mut().pan_u = 0.0;
                app.view_mut().pan_v = 0.0;
            }
        }
    });
}

pub fn show_audio_panel(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    ui.vertical_centered(|ui| {
        show_mode_toolbar(ui, app);
        ui.separator();
        ui.heading(tr(lang, "Audio", "Audio", "Lind"));
        ui.add_space(6.0);

        ui.label(
            RichText::new("A")
                .color(Color32::from_rgb(100, 200, 120))
                .strong(),
        );
        let mut mute_a = app.view().mute_a;
        let resp_a = ui.button(if mute_a {
            tr(lang, "Activar", "Unmute", "Nanquet")
        } else {
            tr(lang, "Silenciar", "Mute", "Tamya")
        });
        if resp_a.clicked() {
            mute_a = !mute_a;
            app.view_mut().mute_a = mute_a;
            if !mute_a {
                app.view_mut().mute_b = true; // Mutuamente excluyentes
            }
            ui.ctx().request_repaint();
        }
        resp_a.on_hover_text(if mute_a {
            tr(
                lang,
                "Canal A silenciado (clic para activar el sonido)",
                "Channel A muted (click to unmute)",
                "A tamya (nanquet)",
            )
        } else {
            tr(
                lang,
                "Canal A con sonido (clic para silenciar)",
                "Channel A audible (click to mute)",
                "A lind (tamya)",
            )
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        ui.label(
            RichText::new("B")
                .color(Color32::from_rgb(100, 160, 240))
                .strong(),
        );
        let mut mute_b = app.view().mute_b;
        let resp_b = ui.button(if mute_b {
            tr(lang, "Activar", "Unmute", "Nanquet")
        } else {
            tr(lang, "Silenciar", "Mute", "Tamya")
        });
        if resp_b.clicked() {
            mute_b = !mute_b;
            app.view_mut().mute_b = mute_b;
            if !mute_b {
                app.view_mut().mute_a = true; // Mutuamente excluyentes
            }
            ui.ctx().request_repaint();
        }
        resp_b.on_hover_text(if mute_b {
            tr(
                lang,
                "Canal B silenciado (clic para activar el sonido)",
                "Channel B muted (click to unmute)",
                "B tamya (nanquet)",
            )
        } else {
            tr(
                lang,
                "Canal B con sonido (clic para silenciar)",
                "Channel B audible (click to mute)",
                "B lind (tamya)",
            )
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        #[cfg(target_os = "macos")]
        {
            if ui
                .button(tr(
                    lang,
                    "Audiometer (Abrir/Cerrar)",
                    "Audiometer (Toggle)",
                    "Audiometer",
                ))
                .on_hover_text(tr(
                    lang,
                    "Abre o cierra el medidor Audiometer (Youlean)",
                    "Opens or closes the Audiometer",
                    "Audiometer",
                ))
                .clicked()
            {
                let is_running = std::process::Command::new("osascript")
                    .arg("-e")
                    .arg("application \"Youlean Loudness Meter 2\" is running")
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "true")
                    .unwrap_or(false);

                if is_running {
                    if let Err(e) = std::process::Command::new("osascript")
                        .arg("-e")
                        .arg("tell application \"Youlean Loudness Meter 2\" to quit")
                        .spawn()
                    {
                        log::warn!("Failed to quit Youlean Loudness Meter 2: {}", e);
                    }
                    if let Some(saved) = app.view_mut().saved_loop_playback.take() {
                        app.view_mut().loop_playback = saved;
                    }
                } else {
                    if let Err(e) = std::process::Command::new("open")
                        .arg("-a")
                        .arg("Youlean Loudness Meter 2")
                        .spawn()
                    {
                        log::warn!("Failed to open Youlean Loudness Meter 2: {}", e);
                    }
                    app.view_mut().saved_loop_playback = Some(app.view().loop_playback);
                    app.view_mut().loop_playback = false;
                    app.do_seek(0.0, ui.ctx());

                    // Wait 2.5 seconds for Youlean GUI to open before playing
                    if app.playback().is_playing {
                        app.do_pause(ui.ctx());
                    }
                    app.view_mut().pending_play_after_delay =
                        Some(std::time::Instant::now() + std::time::Duration::from_millis(2500));
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            ui.add_enabled(
                false,
                egui::Button::new(tr(
                    lang,
                    "Audiometer (No soportado)",
                    "Audiometer (Unsupported)",
                    "Audiometer",
                )),
            )
            .on_hover_text(tr(
                lang,
                "Integración Youlean no soportada en esta plataforma.",
                "Youlean integration not supported on this platform.",
                "Youlean integration not supported on this platform.",
            ));
        }

        ui.add_space(10.0);

        if ui
            .button(tr(
                lang,
                "VU Meter (Abrir/Cerrar)",
                "VU Meter (Toggle)",
                "VU Meter",
            ))
            .on_hover_text(tr(
                lang,
                "Abre o cierra el vúmetro digital LED",
                "Opens or closes the digital LED VU Meter",
                "VU Meter",
            ))
            .clicked()
        {
            app.view_mut().show_vu_meter = !app.view().show_vu_meter;
        }
    });
}
</file>

<file path="Cargo.toml">
[package]
name = "diffplayerqc"
version = "1.3.4"
edition = "2021"
description = "Frame-accurate differential video QC player"
authors = ["DiffPlayerQC"]
license = "MIT"

[[bin]]
name = "diffplayerqc"
path = "src/main.rs"

[dependencies]
eframe        = { version = "0.27", features = ["wgpu", "persistence"] }
egui          = "0.27"
egui-wgpu     = "0.27"
wgpu          = { version = "0.19", features = ["wgsl"] }
ffmpeg-sys-next = "7.1"
crossbeam-channel = "0.5"
anyhow        = "1"
bytemuck      = { version = "1", features = ["derive"] }
dark-light    = "1"
rfd           = "0.14"
log           = "0.4"
env_logger    = "0.11"
parking_lot   = "0.12"
image = "0.25.9"
directories = "6.0.0"
chrono = "0.4.44"
xcap = "0.8.3"
rodio = "0.19"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
ebur128 = "0.1.10"
thiserror = "2.0.18"
csv = "1.4.0"

# Por defecto: velocidad para uso interactivo QC. Ver docs/BUILD_PROFILES.md.
[profile.release]
opt-level     = 3
lto           = true
codegen-units = 1
strip         = true
panic         = "abort"

# Binario más pequeño para distribución (más lento en CPU).
[profile.release-small]
inherits      = "release"
opt-level     = "s"

[profile.dev]
opt-level     = 1

# Desactivar las aserciones de debug estrictas en icrate y objc2 para evitar
# el crasheo en macOS 15+ (Sequoia) debido a countByEnumeratingWithState

[profile.dev.package.icrate]
debug-assertions = false

[profile.dev.package.objc2]
debug-assertions = false
</file>

</files>
