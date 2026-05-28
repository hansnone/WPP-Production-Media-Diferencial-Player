//! Lógica de dominio DiffPlayerQC (v2): sin egui, wgpu ni FFmpeg.
//!
//! Extraída de la v1 para compartir entre el binario legacy y `src-tauri`.

pub mod jugador;
pub mod marcador;
pub mod modos;
pub mod persistencia_layout;
pub mod reproduccion;
pub mod workspace;

pub use jugador::PlayerState;
pub use marcador::{ListaMarcadores, Marcador};
pub use modos::{
    ciclar_modo_comparacion, modos_diferencia_validos, normalizar_modo_diferencia, CompareMode,
    DiffMode,
};
pub use persistencia_layout::{DisposicionPaneles, LayoutPersistido};
pub use reproduccion::{
    next_frame_repaint_delay, PlaybackState, REPINT_AUDIO_MAX_MS, REPINT_IDLE_MAX_MS,
};
pub use workspace::WorkspaceLayout;
