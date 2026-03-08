# DiffPlayerQC

Frame-accurate differential video QC player

DiffPlayerQC es un reproductor avanzado multiplataforma (Windows, macOS, Linux) desarrollado en Rust para la comparación diferencial de video y control de calidad (QC) con precisión por cuadro. Utiliza FFmpeg para decodificación, WGPU para renderizado acelerado por hardware y eframe/egui para una interfaz de usuario moderna y responsiva.

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