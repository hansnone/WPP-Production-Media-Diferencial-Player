//! Tipos compartidos entre UI, decoders y renderer: frames, comandos y re-exports de `core`.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub use diffplayerqc_core::{CompareMode, DiffMode, PlaybackState};

/// A decoded video frame ready for GPU upload.
#[derive(Clone)]
pub struct VideoFrame {
    /// Presentation timestamp in seconds.
    pub pts: f64,
    /// Raw RGBA bytes, row-major, no padding (compartido vía Arc para evitar clones en caliente).
    pub rgba_data: Arc<Vec<u8>>,
    pub width: u32,
    pub height: u32,
}

/// A decoded audio frame ready for playback.
#[derive(Clone)]
pub struct AudioFrame {
    /// Raw f32 interleaved PCM samples.
    pub samples: Vec<f32>,
    /// Number of channels.
    pub channels: u16,
    /// Sample rate in Hz.
    pub sample_rate: u32,
}

/// Commands sent from the UI thread to a decoder thread.
#[derive(Debug, Clone)]
pub enum DecoderCommand {
    /// Start continuous playback from the current position.
    Play,
    /// Pause decoding (decoder stays alive, waiting for next command).
    Pause,
    /// Seek to the given PTS (seconds). The decoder will find the nearest
    /// keyframe and then step forward to the exact target.
    Seek(f64),
    /// Decode exactly one frame forward from the current position.
    StepForward,
    /// Retroceso vía decoder (seek interno); la UI usa seek por FPS (`do_step_bck_inner`).
    #[allow(dead_code)]
    StepBack,
    /// Terminate the decoder thread.
    Stop,
    /// Reservado: el volumen se aplica en `rodio` desde la UI, no en el hilo decoder.
    #[allow(dead_code)]
    SetVolume(f32),
}

/// Color metadata extracted from the video stream header.
#[derive(Debug, Clone, Default)]
pub struct ColorMetadata {
    pub colorspace: String,
    pub color_transfer: String,
    pub color_primaries: String,
    pub pixel_format: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub duration_secs: f64,
    pub bitrate_kbps: i64,
    pub video_codec: String,
    pub audio_codec: String,
    pub major_brand: String,
    pub video_stream_metadata: String,
    pub audio_stream_metadata: String,
    /// `hw:…` (p. ej. `hw:videotoolbox`) o `software` (M11).
    pub decode_ruta: String,
}

/// Safe zone overlay mode: none, TV (EBU R95), or social/mobile (9:16).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SafeZoneMode {
    #[default]
    None,
    /// TV 16:9 — Action Safe 93%, Title Safe (5% top/bottom, 10% sides), centre cross.
    TvEbu,
    /// Social 9:16 — Safe zone + danger zones (top 15%, bottom 22%, right 15%, left 5%) shaded.
    Social,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    En,
    Es,
    Quenya,
}

impl Default for Language {
    fn default() -> Self {
        Self::Es
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    Rust,
    SolarizedDark,
    SolarizedLight,
    Dracula,
    Gruvbox,
    Nord,
    Monokai,
    OneDark,
    OneLight,
    Catppuccin,
    TokyoNight,
    NightOwl,
    Ayc,
    MaterialDesign,
    Everforest,
    TomorrowNight,
    RosePine,
    SynthWave84,
    Nordic,
    OceanicNext,
    Palenight,
    Powerlevel10k,
    Snazzy,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

/// Which video channel (A or B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    A,
    B,
}
