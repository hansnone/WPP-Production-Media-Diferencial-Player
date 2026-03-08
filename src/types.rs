// types.rs — Shared data types across all modules

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
    /// Seek back one frame from the current position.
    StepBack,
    /// Terminate the decoder thread.
    Stop,
    /// Change volume output
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
}

/// Current display mode for the comparison shader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CompareMode {
    SplitScreen = 0,
    AbsDiff = 1,
    Heatmap = 2,
    SideBySide = 3,
}

impl Default for CompareMode {
    fn default() -> Self { Self::SplitScreen }
}

/// The specific algorithm used when evaluating `CompareMode::AbsDiff`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DiffMode {
    LegacyAbs = 0,
    AbsLinear = 1,
    AbsSqrt = 2,
    SignedDiverging = 3,
    None = 4,
}

impl Default for DiffMode {
    fn default() -> Self { Self::AbsLinear }
}





#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    En,
    Es,
    Quenya,
}

impl Default for Language {
    fn default() -> Self { Self::Es }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn default() -> Self { Self::Dark }
}

/// Playback state shared between the UI and decoder coordination logic.
#[derive(Debug, Clone, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_pts: f64,
    pub duration_a: f64,
    pub duration_b: f64,
}

/// Which video channel (A or B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    A,
    B,
}
