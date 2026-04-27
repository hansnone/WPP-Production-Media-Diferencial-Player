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