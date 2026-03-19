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
