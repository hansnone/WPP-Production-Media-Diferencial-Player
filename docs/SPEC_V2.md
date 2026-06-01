# DiffPlayerQC v2 — Especificación técnica de migración a Tauri + Svelte

**Versión objetivo:** 2.0.0  
**Base actual:** 1.3.0 (Rust + egui + wgpu + FFmpeg)  
**Stack destino:** Tauri 2 + Svelte 5 + TypeScript + Rust (sin egui)

Documento de implementación para agente de programación. Los mockups de referencia están en `Binder1.pdf` (concept board Compare / Audio).

## 0. Cómo leer este documento

- Las secciones marcadas **[MUST]** son requisitos no negociables.
- Las marcadas **[SHOULD]** son fuertemente recomendadas.
- Las marcadas **[MAY]** son opcionales.
- Cada hito (**M0–M7**, completados en v2.0.0) tiene criterios de aceptación obligatorios en su fase.
- Los hitos **M8–M16** (post-release) están en [`docs/ROADMAP_M8_M16.md`](ROADMAP_M8_M16.md).
- Los mockups visuales tienen precedencia sobre descripciones textuales cuando haya conflicto.

## 1. Visión y objetivos

### 1.1 Producto

Reproductor diferencial de vídeo y audio para QC profesional (broadcast / VFX / post). Compara dos fuentes A y B con precisión de fotograma y de muestra de audio.

### 1.2 Por qué v2

- UI densa tipo DaVinci Resolve (paleta industrial, tipografía condensada).
- Workspaces intercambiables (Compare / Inspect / Audio / Report / Export).
- Paneles plegables con persistencia de layout.
- Command Palette (Cmd/Ctrl+K).
- Soporte futuro: waveforms, scopes, export PDF.

### 1.3 No-objetivos

- No es un editor NLE.
- No reemplaza a Resolve.
- Sin colaboración en tiempo real ni nube.

## 2. Arquitectura [MUST]

```
Frontend (Tauri WebView)     IPC (Tauri commands/events)     Backend Rust
Svelte 5 + TS + CSS vars  <--------------------------->  core/ decoder/ render/ audio/
```

- `**crates/core/**`: dominio puro (`PlayerState`, modos, reloj, workspaces, marcadores, layout).
- `**src-tauri/**`: shell nativa, permisos, empaquetado.
- `**frontend/**`: UI Svelte 5 (sin acceso directo a FFmpeg).
- **v1 legacy**: binario `diffplayerqc` (egui); rama `egui-legacy`.

## 3. Migración desde v1 (fases)


| Fase | Contenido                                       |
| ---- | ----------------------------------------------- |
| A    | Extraer `core/` de v1; v1 compila contra `core` |
| B    | Shell Tauri 2 + Svelte 5                        |
| C    | UI Svelte (shell, menús, workspaces)            |
| D    | Render wgpu embebido en WebView                 |
| E    | Audio (rodio / análisis)                        |
| F    | Acabado, firmado, release                       |


## 15. Hitos y criterios de aceptación [MUST]

### M0 — Bootstrap (1 sem)

**Entregables:** repo Tauri 2 + Svelte 5 compila; CI verde; `core/` extraído de v1.

**Aceptación:**

- `cargo build` && `pnpm build` funcionan en macOS, Windows, Linux.
- `cargo test -p diffplayerqc-core` con cobertura **> 70 %** sobre `crates/core/`.
- Workflow `.github/workflows/ci.yml` corre `fmt` + `clippy` + tests + `pnpm test`.

**Rama:** `m0/bootstrap`

### M1 — Apertura y reproducción (1–2 sem)

**Entregables:** abrir A y B, reproducir, seek, step; canvas placeholder negro (`#canvas-slot`).

**Aceptación:**

- Cargar 4 archivos de test (mp4, mov ProRes, mkv h264, mxf) sin error.
- Eventos `playback-tick` ~60 Hz en reproducción (ver `docs/IPC_PROTOCOL.md`).
- Audio (rodio) sin underruns perceptibles 60 s seguidos.

**Rama:** `m1/playback`

### M2 — UI shell completa (2 sem)

**Entregables:** menubar, toolbar, paneles colapsables, workspace tabs, command palette, atajos.

**Aceptación:**

- Layout Compare / Audio / Inspect alineado con mockups (desviación pixel ≤ 2 % en M2+).
- Cambio de workspace < 100 ms (sin animación de transform en tabs).
- Command palette `Ctrl/Cmd+K` con navegación teclado.
- Tests Playwright: cada workspace visible (`frontend/e2e/workspaces.spec.ts`).

**Rama:** `m2/ui-shell`

### M3 — Render wgpu (1–2 sem)

**Entregables:** crate `diffplayerqc-render` (`compare.wgsl`); ventana overlay Tauri alineada con `#canvas-slot`; panel de modos en workspace Compare.

**Aceptación:**

- Con A y B cargados, el overlay muestra vídeo sincronizado al slot (ResizeObserver → `sincronizar_viewport`) — QA manual.
- Modos `SplitScreen`, `AbsDiff`, `Heatmap`, `SideBySide` y `DiffMode` vía `establecer_vista_compare`.
- `cargo build --workspace` y `pnpm build` sin error.
- E2E: panel modos compare visible (`compare-mode-panel`).

**Rama:** `m3/render`

### M4 — Audio nuevo (2 sem)

Workspace Audio: waveforms A/B + diff + LUFS.

**Entregables:** escaneo offline FFmpeg (`forma_onda.rs`); canvas A/B/diff con playhead; panel LUFS integrado.

**Aceptación:**

- Al abrir A/B, waveforms visibles en workspace Audio (evento `forma-onda-lista`).
- Franja diff |A−B| con A y B cargados.
- LUFS integrado estimado por canal + delta B−A.
- Click en waveform hace seek.
- E2E: panel loudness y canvas visibles (`audio-loudness`, `waveform-canvas-a`).

**Rama:** `m4/audio`

### M5 — Inspect + scopes (2 sem)

Histograma, vectorscope, waveform monitor.

**Entregables:** `analisis_scopes.rs`; evento `scopes-actualizados`; panel derecho en Inspect con tres scopes canvas.

**Aceptación:**

- [x] Histograma RGB del fotograma actual (canal A preferido, si no B).
- [x] Vectoscopio 128×128 (Cb/Cr).
- [x] Monitor de luma (máximo Y por columna).
- [x] Scopes se actualizan al cambiar PTS (seek / play / step).
- [x] Workspace Inspect con viewport GPU alineado (`#canvas-slot`).
- [x] E2E: `inspect-scopes-panel` y `scope-histograma` visibles.

**Rama:** `m5/inspect`

### M6 — Persistencia y polish (1 sem)

Layouts por workspace, Es/En, recent files.

**Entregables:** `layout.svelte.ts` (localStorage por workspace); `idioma.svelte.ts` + `traducciones.ts`; `recientes.svelte.ts`; menú Archivo/Ver ampliado.

**Aceptación:**

- [x] Al cambiar workspace o plegar paneles, el layout se guarda y restaura al recargar.
- [x] Menú Ver → Idioma Español / English; UI reactiva sin recargar.
- [x] Archivos abiertos aparecen en Archivo → Recientes (canal A/B recordado).
- [x] Restablecer layout del workspace actual (menú Ver o paleta).
- [x] E2E: persistencia layout/idioma + menú recientes.

**Rama:** `m6/persistencia`

### M7 — Release (1 sem)

Builds firmados; bundle macOS < 25 MB; CI release en GitHub Releases.

**Entregables:** `release.yml`; `docs/RELEASE.md`; feature `egui-app` (Tauri sin egui); versión 2.0.0; perfil `release-small`.

**Aceptación:**

- [x] `cargo tauri build --profile release-small` genera bundle en macOS.
- [x] Tauri usa `diffplayerqc` con `default-features = false` (sin egui en el .app).
- [x] Tag `v*` dispara workflow y sube artefactos a GitHub Releases (draft).
- [x] Firmado macOS documentado vía secrets `APPLE_*` (opcional).
- [x] CI avisa si DMG arm64 > 25 MB.

**Rama:** `m7/release`

---

## 16. Roadmap post-release M8–M16 [SHOULD]

Tras v2.0.0. Detalle completo, dependencias y estimaciones: **[`docs/ROADMAP_M8_M16.md`](ROADMAP_M8_M16.md)**.

| Bloque | Hitos | Foco |
|--------|-------|------|
| A | M8, M9, M10 | Métricas VMAF/SSIM, loudness EBU R128, eventos y notas QC |
| B | M11, M12 | Decode HW, ProRes/DNxHR 10–12 bit |
| C | M13, M14, M15 | Timecode SMPTE, modo Difference QC, reportes PDF/HTML |
| D | M16 | Atajos personalizables, multi-ventana |

**Resumen de aceptación por hito (extracto):**

- **M8:** [x] gráfico SSIM/MS-SSIM/VMAF + heatmap timeline + export JSON/CSV; escaneo en background; VMAF si `ffmpeg` incluye libvmaf.
- **M9:** [x] LUFS integrado EBU, true peak, LRA, `lufs_buckets` en waveform, alertas silencio/clipping; panel Audio post-escaneo.
- **M10:** [x] tabla eventos + notas persistentes + seek desde evento + marcadores en timeline.
- **M11:** [x] decode HW (VideoToolbox/VAAPI/D3D11…) con fallback software; etiqueta en Fuentes; `DIFFPLAYERQC_HW_DECODE=0`.
- **M12:** ProRes/DNxHR frame-accurate; color Rec.709/2020 correcto.
- **M13:** display y salto SMPTE NDF/DF.
- **M14:** difference shader con umbral; toggles de vista.
- **M15:** reporte PDF/HTML con snapshots y métricas.
- **M16:** mapa de teclas editable; ventana clean feed.

No se exige completar M8–M16 para etiquetar parches `2.x`; cada hito es un incremento versionable (`2.1.0` = M8, etc., según acuerdo de release).

## 24. Checklist agente (M0)

- Rama `egui-legacy` con v1 intacta.
- Este documento en `docs/SPEC_V2.md`.
- Mockups en `docs/design/` (referencia Binder1.pdf).
- Rama `m0/bootstrap`.
- PR template con “Acceptance criteria covered”.
- Issue de tracking “Migración v2” enlazando a § 15.

## Referencias

- v1: rama `egui-legacy` / binario `diffplayerqc`.
- Tauri 2: [https://v2.tauri.app](https://v2.tauri.app)
- Svelte 5 runes: [https://svelte.dev/docs/svelte/what-are-runes](https://svelte.dev/docs/svelte/what-are-runes)

