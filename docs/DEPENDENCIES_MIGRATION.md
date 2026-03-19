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
