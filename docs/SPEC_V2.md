# DiffPlayerQC v2 — Especificación técnica de migración a Tauri + Svelte

**Versión objetivo:** 2.0.0  
**Base actual:** 1.3.0 (Rust + egui + wgpu + FFmpeg)  
**Stack destino:** Tauri 2 + Svelte 5 + TypeScript + Rust (sin egui)

Documento de implementación para agente de programación. Los mockups de referencia están en `Binder1.pdf` (concept board Compare / Audio).

## 0. Cómo leer este documento

- Las secciones marcadas **[MUST]** son requisitos no negociables.
- Las marcadas **[SHOULD]** son fuertemente recomendadas.
- Las marcadas **[MAY]** son opcionales.
- Cada hito (**M0–M7**) tiene criterios de aceptación: no se avanza al siguiente hasta que todos pasan.
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

- **`crates/core/`**: dominio puro (`PlayerState`, modos, reloj, workspaces, marcadores, layout).
- **`src-tauri/`**: shell nativa, permisos, empaquetado.
- **`frontend/`**: UI Svelte 5 (sin acceso directo a FFmpeg).
- **v1 legacy**: binario `diffplayerqc` (egui); rama `egui-legacy`.

## 3. Migración desde v1 (fases)

| Fase | Contenido |
|------|-----------|
| A | Extraer `core/` de v1; v1 compila contra `core` |
| B | Shell Tauri 2 + Svelte 5 |
| C | UI Svelte (shell, menús, workspaces) |
| D | Render wgpu embebido en WebView |
| E | Audio (rodio / análisis) |
| F | Acabado, firmado, release |

## 15. Hitos y criterios de aceptación [MUST]

### M0 — Bootstrap (1 sem)

**Entregables:** repo Tauri 2 + Svelte 5 compila; CI verde; `core/` extraído de v1.

**Aceptación:**

- [ ] `cargo build` && `pnpm build` funcionan en macOS, Windows, Linux.
- [ ] `cargo test -p diffplayerqc-core` con cobertura **> 70 %** sobre `crates/core/`.
- [ ] Workflow `.github/workflows/ci.yml` corre `fmt` + `clippy` + tests + `pnpm test`.

**Rama:** `m0/bootstrap`

### M1 — Apertura y reproducción (1–2 sem)

**Entregables:** abrir A y B, reproducir, seek, step; canvas placeholder negro (`#canvas-slot`).

**Aceptación:**

- [ ] Cargar 4 archivos de test (mp4, mov ProRes, mkv h264, mxf) sin error.
- [ ] Eventos `playback-tick` ~60 Hz en reproducción (ver `docs/IPC_PROTOCOL.md`).
- [ ] Audio (rodio) sin underruns perceptibles 60 s seguidos.

**Rama:** `m1/playback`

### M2 — UI shell completa (2 sem)

**Entregables:** menubar, toolbar, paneles colapsables, workspace tabs, command palette, atajos.

**Aceptación:**

- [ ] Layout Compare / Audio / Inspect alineado con mockups (desviación pixel ≤ 2 % en M2+).
- [ ] Cambio de workspace &lt; 100 ms (sin animación de transform en tabs).
- [ ] Command palette `Ctrl/Cmd+K` con navegación teclado.
- [ ] Tests Playwright: cada workspace visible (`frontend/e2e/workspaces.spec.ts`).

**Rama:** `m2/ui-shell`

### M3 — Render wgpu (1–2 sem)

**Entregables:** crate `diffplayerqc-render` (`compare.wgsl`); ventana overlay Tauri alineada con `#canvas-slot`; panel de modos en workspace Compare.

**Aceptación:**

- [ ] Con A y B cargados, el overlay muestra vídeo sincronizado al slot (ResizeObserver → `sincronizar_viewport`) — QA manual.
- [x] Modos `SplitScreen`, `AbsDiff`, `Heatmap`, `SideBySide` y `DiffMode` vía `establecer_vista_compare`.
- [x] `cargo build --workspace` y `pnpm build` sin error.
- [x] E2E: panel modos compare visible (`compare-mode-panel`).

**Rama:** `m3/render`

### M4 — Audio nuevo (2 sem)

Workspace Audio: waveforms A/B + diff + LUFS.

### M5 — Inspect + scopes (2 sem)

Histograma, vectorscope, waveform monitor.

### M6 — Persistencia y polish (1 sem)

Layouts por workspace, Es/En, recent files.

### M7 — Release (1 sem)

Builds firmados; bundle macOS < 25 MB; CI release en GitHub Releases.

## 24. Checklist agente (M0)

- [x] Rama `egui-legacy` con v1 intacta.
- [x] Este documento en `docs/SPEC_V2.md`.
- [ ] Mockups en `docs/design/` (referencia Binder1.pdf).
- [x] Rama `m0/bootstrap`.
- [x] PR template con “Acceptance criteria covered”.
- [ ] Issue de tracking “Migración v2” enlazando a § 15.

## Referencias

- v1: rama `egui-legacy` / binario `diffplayerqc`.
- Tauri 2: https://v2.tauri.app
- Svelte 5 runes: https://svelte.dev/docs/svelte/what-are-runes
