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
