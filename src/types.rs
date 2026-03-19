//! Tipos compartidos entre UI, decoders y renderer: frames, comandos y estado de reproducción.

use serde::{Deserialize, Serialize};

/// A decoded video frame ready for GPU upload.
#[derive(Clone)]
pub struct VideoFrame {
    /// Presentation timestamp in seconds.
    pub pts: f64,
    /// Raw RGBA bytes, row-major, no padding.
    pub rgba_data: Vec<u8>,
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
#[derive(Debug)]
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
}

/// Current display mode for the comparison shader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum CompareMode {
    SplitScreen = 0,
    AbsDiff = 1,
    Heatmap = 2,
    SideBySide = 3,
}

impl Default for CompareMode {
    fn default() -> Self {
        Self::SplitScreen
    }
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

/// The specific algorithm used when evaluating `CompareMode::AbsDiff`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum DiffMode {
    LegacyAbs = 0,
    AbsLinear = 1,
    AbsSqrt = 2,
    SignedDiverging = 3,
    None = 4,
}

impl Default for DiffMode {
    fn default() -> Self {
        Self::AbsLinear
    }
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

/// Estado de reproducción compartido entre la UI y la coordinación con los decoders.
///
/// Reloj maestro: al reproducir, `current_pts = playback_start_pts + elapsed` desde `playback_start_instant`.
#[derive(Debug, Clone, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_pts: f64,
    pub duration_a: f64,
    pub duration_b: f64,
    /// When set, current_pts is derived from this instant + playback_start_pts (system-time master clock).
    pub playback_start_instant: Option<std::time::Instant>,
    /// PTS at the moment we started (or seeked during) playback.
    pub playback_start_pts: f64,
}

/// Which video channel (A or B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    A,
    B,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Los discriminantes deben coincidir con `compare.wgsl` / uniforms.
    #[test]
    fn compare_mode_shader_indices() {
        assert_eq!(CompareMode::SplitScreen as u32, 0);
        assert_eq!(CompareMode::AbsDiff as u32, 1);
        assert_eq!(CompareMode::Heatmap as u32, 2);
        assert_eq!(CompareMode::SideBySide as u32, 3);
    }
}
