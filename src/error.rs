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
