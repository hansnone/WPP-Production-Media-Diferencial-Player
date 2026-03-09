# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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