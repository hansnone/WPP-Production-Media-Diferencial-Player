# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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