//! Aplicación egui/eframe: estado global, bucle `update`, decoders, audio y proxy EXR.
//!
//! Submódulos: [`playback`] (temporización de repintado), [`proxy_bridge`] (ruta al proxy.mkv).
//! Ver `docs/ARQUITECTURA.md` en el repositorio para el flujo completo.

mod playback;
mod proxy_bridge;

use crossbeam_channel::{Receiver, Sender};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::decoder;
use crate::renderer::{RenderCallback, ShaderUniforms, VideoRenderer};
use crate::types::{
    AudioFrame, Channel, ColorMetadata, CompareMode, DecoderCommand, DiffMode, Language,
    PlaybackState, SafeZoneMode, VideoFrame,
};
use rodio::{OutputStream, Sink};
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
//  View state — zoom / pan / mode / sliders
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewState {
    pub mode: CompareMode,
    pub diff_mode: DiffMode,
    pub lang: Language,
    pub theme: crate::types::Theme,
    pub show_hud: bool,
    /// Show left sidebar (video metadata / info).
    pub show_left_panel: bool,
    /// Show right sidebar (mode + audio controls).
    pub show_right_panel: bool,
    pub split_pos: f32,
    pub screenshot_dir: Option<PathBuf>,
    pub amplifier: f32,
    pub zoom: f32,
    pub pan_u: f32,
    pub pan_v: f32,
    pub canvas_bg_color: [f32; 3],
    pub show_clean_feed_window: bool,
    /// Canvas rect in egui screen-space (for coordinate transform)
    #[serde(skip, default = "default_rect")]
    pub canvas_rect: egui::Rect,
    pub mute_a: bool,
    pub mute_b: bool,
    pub vol_a: f32,
    pub vol_b: f32,
    /// Split curtain orientation: false = vertical (X), true = horizontal (Y).
    pub split_horizontal: bool,
    /// Safe zone overlay: None, TV (EBU R95), or Social (9:16).
    pub safe_zone: crate::types::SafeZoneMode,
    /// Current audio level 0..1 for channel A (not persisted).
    #[serde(skip, default)]
    pub audio_level_a: f32,
    /// Current audio level 0..1 for channel B (not persisted).
    #[serde(skip, default)]
    pub audio_level_b: f32,
}

impl ViewState {
    pub fn config_path() -> Option<PathBuf> {
        directories::ProjectDirs::from("com", "diffplayerqc", "diffplayerqc")
            .map(|proj| proj.config_dir().join("config.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            log::info!("Loading config from: {:?}", path);
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(mut loaded) = serde_json::from_str::<Self>(&content) {
                    log::info!("Config loaded successfully");
                    // Ensure screenshot_dir exists or fall back to desktop
                    if let Some(dir) = &loaded.screenshot_dir {
                        if !dir.exists() {
                            loaded.screenshot_dir = directories::UserDirs::new()
                                .and_then(|d| d.desktop_dir().map(|p| p.to_path_buf()));
                        }
                    }
                    return loaded;
                } else {
                    log::warn!("Failed to parse config.json, using defaults");
                }
            } else {
                log::info!(
                    "No existing config.json found at {:?}, using defaults",
                    path
                );
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            log::info!("Saving config to: {:?}", path);
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(content) = serde_json::to_string_pretty(self) {
                let tmp_path = path.with_extension("json.tmp");
                if let Err(e) = std::fs::write(&tmp_path, &content) {
                    log::error!("Failed to write config.json.tmp: {e}");
                    return;
                }
                if let Err(e) = std::fs::rename(&tmp_path, &path) {
                    log::error!("Failed to rename config.json.tmp to config.json: {e}");
                } else {
                    log::info!("Config saved successfully ({} bytes)", content.len());
                }
            } else {
                log::error!("Failed to serialize ViewState to JSON");
            }
        }
    }
}

impl Default for ViewState {
    fn default() -> Self {
        let desk_dir =
            directories::UserDirs::new().and_then(|d| d.desktop_dir().map(|p| p.to_path_buf()));
        Self {
            mode: CompareMode::SplitScreen,
            diff_mode: DiffMode::AbsLinear,
            lang: Language::Es,
            theme: crate::types::Theme::Dark,
            show_hud: true,
            show_left_panel: true,
            show_right_panel: true,
            split_pos: 0.5,
            screenshot_dir: desk_dir,
            amplifier: 5.0,
            zoom: 1.0,
            pan_u: 0.0,
            pan_v: 0.0,
            canvas_bg_color: [0.0, 0.0, 0.0],
            show_clean_feed_window: false,
            canvas_rect: egui::Rect::NOTHING,
            mute_a: true,
            mute_b: true,
            vol_a: 1.0,
            vol_b: 1.0,
            split_horizontal: false,
            safe_zone: crate::types::SafeZoneMode::None,
            audio_level_a: 0.0,
            audio_level_b: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
//  Per-channel decoder handle
// ---------------------------------------------------------------------------

struct DecoderHandle {
    cmd_tx: Sender<DecoderCommand>,
    frame_rx: Receiver<VideoFrame>,
    audio_rx: Receiver<AudioFrame>,
    last_frame: Option<VideoFrame>,
    next_frame: Option<VideoFrame>,
    meta: ColorMetadata,
    path: String,
}

// ---------------------------------------------------------------------------
//  Main application struct
// ---------------------------------------------------------------------------

pub struct DiffPlayerApp {
    decoder_a: Option<DecoderHandle>,
    decoder_b: Option<DecoderHandle>,

    view: ViewState,
    playback: PlaybackState,

    renderer: Arc<Mutex<VideoRenderer>>,

    drag_start: Option<(egui::Pos2, f32, f32)>,
    dragging_split: bool,
    drag_drop_hover_pos: Option<egui::Pos2>,

    // DE VUELTA AL ORIGINAL
    _audio_stream: Option<OutputStream>,
    sink_a: Option<Sink>,
    sink_b: Option<Sink>,

    error_title: Option<String>,
    error_message: Option<String>,
    last_step_time: f64,

    /// Request viewport focus/visible for first N frames (macOS window not showing workaround).
    focus_visible_frames_left: u32,

    /// Incremented each frame. Frame 0 skips all Wgpu work so the window can appear on macOS.
    frame_count: u64,

    /// Deferred play/pause toggle (Space): process at start of next update to avoid re-entrancy deadlock.
    pending_play_pause_toggle: bool,

    /// Deferred key action: process at start of next update to avoid re-entrancy/deadlock when called from ctx.input().
    pending_key_action: PendingKeyAction,

    /// Proxy generation: progress 0.0..=1.0.
    proxy_progress: Arc<Mutex<f32>>,
    /// Proxy generation: true while background thread is running.
    proxy_running: Arc<AtomicBool>,
    /// Temp directory for current proxy run (PNGs + concat); cleared when run finishes or new run starts.
    proxy_temp_dir: Option<PathBuf>,
    /// Channel to load the proxy sequence into when generation finishes.
    proxy_target_channel: Option<Channel>,
    /// All proxy temp dirs to remove on exit.
    proxy_temp_dirs: Vec<PathBuf>,
}

/// Key actions deferred from ctx.input() to start of update() to avoid re-entrancy on macOS.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PendingKeyAction {
    None,
    StepFwd,
    StepBck,
    Seek(f64),
    CycleMode,
    SideBySide,
    SplitPos0,
    SplitPos1,
    ToggleHud,
    Zoom(f32),
    ResetZoomPan,
    SwapVideos,
}

impl Default for PendingKeyAction {
    fn default() -> Self {
        PendingKeyAction::None
    }
}

impl DiffPlayerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let render_state = match cc.wgpu_render_state.as_ref() {
            Some(rs) => rs,
            None => panic!("Wgpu render state missing"),
        };

        let target_format = render_state.target_format;
        let renderer = Arc::new(Mutex::new(VideoRenderer::new(
            &render_state.device,
            target_format,
        )));

        let view = ViewState::load();
        crate::ui::theme::apply_theme(&cc.egui_ctx, view.theme);

        // INICIALIZACIÓN DIRECTA (¡Ya no hay dark_light que moleste!)

        log::info!("Inicializando Audio en el hilo principal...");

        let (audio_stream, sink_a, sink_b) = match rodio::OutputStream::try_default() {
            Ok((stream, handle)) => {
                let s_a = rodio::Sink::try_new(&handle).ok();
                let s_b = rodio::Sink::try_new(&handle).ok();
                if let (Some(sa), Some(sb)) = (&s_a, &s_b) {
                    sa.set_volume(0.0);
                    sb.set_volume(0.0);
                }
                (Some(stream), s_a, s_b)
            }
            Err(e) => {
                log::error!("Error al inicializar audio: {}", e);
                (None, None, None)
            }
        };
        if sink_a.is_some() && sink_b.is_some() {
            log::info!("Audio inicializado correctamente (canales A y B).");
        }

        crate::trace_log::log("App initialized");

        Self {
            decoder_a: None,
            decoder_b: None,
            view,
            playback: PlaybackState::default(),
            renderer,
            drag_start: None,
            dragging_split: false,
            drag_drop_hover_pos: None,

            _audio_stream: audio_stream,
            sink_a,
            sink_b,

            error_title: None,
            error_message: None,
            last_step_time: 0.0,
            focus_visible_frames_left: 15,
            frame_count: 0,
            pending_play_pause_toggle: false,
            pending_key_action: PendingKeyAction::None,

            proxy_progress: Arc::new(Mutex::new(0.0)),
            proxy_running: Arc::new(AtomicBool::new(false)),
            proxy_temp_dir: None,
            proxy_target_channel: None,
            proxy_temp_dirs: Vec::new(),
        }
    }

    /// Start EXR→PNG proxy generation from a directory (lists .exr inside). When done, loads sequence into `channel`.
    pub fn start_proxy_from_exr_input_dir(
        &mut self,
        src_dir: PathBuf,
        channel: Channel,
        _ctx: &egui::Context,
    ) {
        if self.proxy_running() {
            return;
        }
        let name = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis().to_string())
            .unwrap_or_else(|_| "proxy".to_string());
        let temp_dir = std::env::temp_dir().join("diffplayerqc_proxies").join(name);
        if let Err(e) = std::fs::create_dir_all(&temp_dir) {
            log::error!("Failed to create proxy temp dir: {e}");
            return;
        }
        self.proxy_temp_dir = Some(temp_dir.clone());
        self.proxy_target_channel = Some(channel);
        *self.proxy_progress.lock() = 0.0;
        crate::proxy::run_from_directory_in_background(
            src_dir,
            temp_dir,
            Arc::clone(&self.proxy_progress),
            Arc::clone(&self.proxy_running),
        );
    }

    /// Start EXR→PNG proxy generation from a list of EXR file paths. When done, loads sequence into `channel`.
    pub fn start_proxy_from_exr_input_files(
        &mut self,
        exr_paths: Vec<PathBuf>,
        channel: Channel,
        _ctx: &egui::Context,
    ) {
        if self.proxy_running() || exr_paths.is_empty() {
            return;
        }
        let name = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis().to_string())
            .unwrap_or_else(|_| "proxy".to_string());
        let temp_dir = std::env::temp_dir().join("diffplayerqc_proxies").join(name);
        if let Err(e) = std::fs::create_dir_all(&temp_dir) {
            log::error!("Failed to create proxy temp dir: {e}");
            return;
        }
        self.proxy_temp_dir = Some(temp_dir.clone());
        self.proxy_target_channel = Some(channel);
        *self.proxy_progress.lock() = 0.0;
        crate::proxy::run_from_files_in_background(
            exr_paths,
            temp_dir,
            Arc::clone(&self.proxy_progress),
            Arc::clone(&self.proxy_running),
        );
    }

    /// True if proxy generation is currently running.
    pub fn proxy_running(&self) -> bool {
        self.proxy_running.load(Ordering::Relaxed)
    }

    /// Current proxy progress 0.0..=1.0.
    pub fn proxy_progress(&self) -> f32 {
        *self.proxy_progress.lock()
    }

    // -----------------------------------------------------------------------
    //  Open a video file for one channel
    // -----------------------------------------------------------------------
    fn open_video(&mut self, chan: Channel, ctx: &egui::Context) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter(
                "Video",
                &[
                    "mp4", "mov", "mxf", "mkv", "avi", "prores", "mts", "mpg", "mpeg", "ts",
                ],
            )
            .add_filter("All files", &["*"])
            .pick_file()
        else {
            return;
        };

        let path_str = path.to_string_lossy().to_string();
        self.open_video_from_path(path_str, chan, ctx);
    }

    /// Load a video from a filesystem path into the given channel, replacing any existing video.
    pub fn open_video_from_path(&mut self, path_str: String, chan: Channel, ctx: &egui::Context) {
        match decoder::spawn_decoder(&path_str) {
            Ok((cmd_tx, frame_rx, audio_rx, meta)) => {
                let handle = DecoderHandle {
                    cmd_tx,
                    frame_rx,
                    audio_rx,
                    last_frame: None,
                    next_frame: None,
                    meta,
                    path: path_str,
                };

                match chan {
                    Channel::A => {
                        // Stop old decoder if any
                        if let Some(old) = self.decoder_a.take() {
                            let _ = old.cmd_tx.send(DecoderCommand::Stop);
                        }
                        self.playback.duration_a = handle.meta.duration_secs;
                        self.decoder_a = Some(handle);
                        self.do_seek(0.0, ctx);
                        // No need for repaint here as do_seek handles it
                    }
                    Channel::B => {
                        if let Some(old) = self.decoder_b.take() {
                            let _ = old.cmd_tx.send(DecoderCommand::Stop);
                        }
                        self.playback.duration_b = handle.meta.duration_secs;
                        self.decoder_b = Some(handle);
                        self.do_seek(0.0, ctx);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to open video: {e}");
            }
        }
    }

    // -----------------------------------------------------------------------
    //  Drain frame channels and upload new frames to GPU
    // -----------------------------------------------------------------------
    /// Drain at most one frame per decoder that is at or before current_pts (master clock).
    /// Future frames stay in next_frame or channel; no blind turbo-drain, no clock resync.
    fn drain_frames(&mut self, render_state: &egui_wgpu::RenderState) -> bool {
        let device = &render_state.device;
        let queue = &render_state.queue;
        let current_pts = self.playback.current_pts;
        let is_playing = self.playback.is_playing;
        let mut repainted = false;
        const PTS_TOLERANCE: f64 = 0.005;

        let mut process_dec = |dec: &mut DecoderHandle, is_a: bool| {
            // Candidate: next_frame (peek) or one try_recv
            let mut candidate = dec.next_frame.take();
            if candidate.is_none() {
                candidate = dec.frame_rx.try_recv().ok();
            }

            if let Some(frame) = candidate {
                let show = if is_playing {
                    frame.pts <= current_pts + PTS_TOLERANCE
                } else {
                    // Paused: show latest frame we have (step/seek)
                    true
                };

                if show {
                    let (w, h) = (frame.width, frame.height);
                    let data = frame.rgba_data.clone();
                    {
                        let mut rend = self.renderer.lock();
                        if is_a {
                            rend.update_texture_a(device, queue, &data, w, h);
                        } else {
                            rend.update_texture_b(device, queue, &data, w, h);
                        }
                    }
                    dec.last_frame = Some(frame.clone());
                    dec.next_frame = dec.frame_rx.try_recv().ok();
                    repainted = true;
                } else {
                    // Future frame: keep for next cycle
                    dec.next_frame = Some(frame);
                }
            }
        };

        if let Some(dec) = &mut self.decoder_a {
            process_dec(dec, true);
        }
        if let Some(dec) = &mut self.decoder_b {
            process_dec(dec, false);
        }

        if !is_playing {
            // When paused, current_pts tracks the last shown frame (from last_frame)
            if let Some(dec) = &self.decoder_a {
                if let Some(ref f) = dec.last_frame {
                    self.playback.current_pts = f.pts;
                }
            }
            if let Some(dec) = &self.decoder_b {
                if let Some(ref f) = dec.last_frame {
                    self.playback.current_pts = self.playback.current_pts.max(f.pts);
                }
            }
        }

        repainted
    }

    // -----------------------------------------------------------------------
    //  Sync uniform buffer from current view state
    // ---------------------------------------------------------------------------
    fn sync_uniforms(&self) {
        let (mut scale_u, mut scale_v) = (1.0, 1.0);

        let mut canvas_w = self.view.canvas_rect.width();
        let canvas_h = self.view.canvas_rect.height();

        if self.view.mode == CompareMode::SideBySide {
            canvas_w /= 2.0;
        }

        if canvas_w > 0.0 && canvas_h > 0.0 {
            let mut vid_w: f32 = 0.0;
            let mut vid_h: f32 = 0.0;
            if let Some(meta) = self.decoder_a_meta() {
                vid_w = vid_w.max(meta.width as f32);
                vid_h = vid_h.max(meta.height as f32);
            }
            if let Some(meta) = self.decoder_b_meta() {
                vid_w = vid_w.max(meta.width as f32);
                vid_h = vid_h.max(meta.height as f32);
            }

            if vid_w > 0.0 && vid_h > 0.0 {
                let canvas_aspect = canvas_w / canvas_h;
                let video_aspect = vid_w / vid_h;

                if canvas_aspect > video_aspect {
                    // Window is wider than video (pillarbox)
                    scale_u = canvas_aspect / video_aspect;
                } else {
                    // Window is taller than video (letterbox)
                    scale_v = video_aspect / canvas_aspect;
                }
            }
        }

        let mut rend = self.renderer.lock();
        rend.uniforms = ShaderUniforms {
            split_pos: self.view.split_pos,
            mode: self.view.mode as u32,
            diff_mode: self.view.diff_mode as u32,
            amplifier: self.view.amplifier,
            zoom: self.view.zoom,
            pan_u: self.view.pan_u,
            pan_v: self.view.pan_v,
            scale_u,
            scale_v,
            bg_color: self.view.canvas_bg_color,
            split_horizontal: if self.view.split_horizontal { 1 } else { 0 },
        };
    }

    // -----------------------------------------------------------------------
    //  Send seek command to both decoders
    // -----------------------------------------------------------------------
    fn seek_both(&self, pts: f64, ctx: &egui::Context) {
        if let Some(dec) = &self.decoder_a {
            let _ = dec.cmd_tx.send(DecoderCommand::Seek(pts));
        }
        if let Some(dec) = &self.decoder_b {
            let _ = dec.cmd_tx.send(DecoderCommand::Seek(pts));
        }
        ctx.request_repaint();
    }

    fn play_both(&mut self, ctx: &egui::Context) {
        crate::trace_log::log("Play");
        self.playback.is_playing = true;
        self.playback.playback_start_instant = Some(Instant::now());
        self.playback.playback_start_pts = self.playback.current_pts;
        if let Some(s) = &self.sink_a {
            s.play();
        }
        if let Some(s) = &self.sink_b {
            s.play();
        }
        if let Some(dec) = &self.decoder_a {
            let _ = dec.cmd_tx.send(DecoderCommand::Play);
        }
        if let Some(dec) = &self.decoder_b {
            let _ = dec.cmd_tx.send(DecoderCommand::Play);
        }
        ctx.request_repaint();
    }

    fn pause_both(&mut self, ctx: &egui::Context) {
        crate::trace_log::log("Pause");
        self.playback.is_playing = false;
        self.playback.playback_start_instant = None;
        if let Some(s) = &self.sink_a {
            s.pause();
        }
        if let Some(s) = &self.sink_b {
            s.pause();
        }
        if let Some(dec) = &self.decoder_a {
            let _ = dec.cmd_tx.send(DecoderCommand::Pause);
        }
        if let Some(dec) = &self.decoder_b {
            let _ = dec.cmd_tx.send(DecoderCommand::Pause);
        }
        ctx.request_repaint();
    }

    fn step_forward(&self, ctx: &egui::Context) {
        if let Some(dec) = &self.decoder_a {
            let _ = dec.cmd_tx.send(DecoderCommand::StepForward);
        }
        if let Some(dec) = &self.decoder_b {
            let _ = dec.cmd_tx.send(DecoderCommand::StepForward);
        }
        ctx.request_repaint();
    }

    pub fn swap_videos(&mut self, ctx: &egui::Context) {
        self.swap_videos_inner(ctx);
    }
    fn swap_videos_inner(&mut self, ctx: &egui::Context) {
        std::mem::swap(&mut self.decoder_a, &mut self.decoder_b);
        std::mem::swap(&mut self.playback.duration_a, &mut self.playback.duration_b);
        std::mem::swap(&mut self.view.mute_a, &mut self.view.mute_b);
        std::mem::swap(&mut self.view.vol_a, &mut self.view.vol_b);
        std::mem::swap(&mut self.sink_a, &mut self.sink_b);
        // Force rendering buffers to swap next frame
        if let Some(dec) = &mut self.decoder_a {
            dec.next_frame = dec.frame_rx.try_recv().ok();
        }
        if let Some(dec) = &mut self.decoder_b {
            dec.next_frame = dec.frame_rx.try_recv().ok();
        }
        ctx.request_repaint();
    }

    fn complete_proxy_if_ready(&mut self, ctx: &egui::Context) {
        if self.proxy_running() || self.proxy_target_channel.is_none() || self.proxy_temp_dir.is_none() {
            return;
        }
        let dir = self.proxy_temp_dir.take().unwrap();
        let channel = self.proxy_target_channel.take().unwrap();
        let proxy_video = proxy_bridge::proxy_video_path(&dir);
        if proxy_video.exists() {
            self.proxy_temp_dirs.push(dir);
            let path_str = proxy_video.to_string_lossy().to_string();
            self.open_video_from_path(path_str, channel, ctx);
        }
    }

    fn update_master_clock_and_repaint(&mut self, ctx: &egui::Context, is_first_frame: bool) {
        if !self.playback.is_playing {
            return;
        }
        if let Some(start) = self.playback.playback_start_instant {
            let elapsed = start.elapsed().as_secs_f64();
            self.playback.current_pts = self.playback.playback_start_pts + elapsed;
            let max_duration = self.playback.duration_a.max(self.playback.duration_b);
            if max_duration > 0.0 {
                if self.playback.current_pts >= max_duration {
                    self.do_seek(0.0, ctx);
                } else {
                    self.playback.current_pts = self.playback.current_pts.clamp(0.0, max_duration);
                }
            }
        }
        // Repintado: intervalo corto con audio activo (rodio) para evitar underruns.
        if !is_first_frame {
            let fps = self
                .decoder_a_meta()
                .or_else(|| self.decoder_b_meta())
                .map(|m| m.fps)
                .unwrap_or(25.0);
            let max_delay_ms = if self.sink_a.is_some() || self.sink_b.is_some() {
                playback::REPINT_AUDIO_MAX_MS
            } else {
                playback::REPINT_IDLE_MAX_MS
            };
            if fps > 0.0 {
                let delay =
                    playback::next_frame_repaint_delay(fps, self.playback.current_pts, max_delay_ms);
                ctx.request_repaint_after(delay);
            } else {
                ctx.request_repaint();
            }
        }
    }

    fn drain_audio_and_update_levels(&mut self) {
        const LEVEL_DECAY: f32 = 0.92;
        if !self.playback.is_playing {
            return;
        }
        let mut received_a = false;
        let mut received_b = false;
        if let Some(dec) = &mut self.decoder_a {
            if let Some(sink) = &self.sink_a {
                while let Ok(audio) = dec.audio_rx.try_recv() {
                    received_a = true;
                    let peak = audio.samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
                    self.view.audio_level_a =
                        (self.view.audio_level_a * LEVEL_DECAY + peak).max(peak).min(1.0);
                    let buf = rodio::buffer::SamplesBuffer::new(
                        audio.channels,
                        audio.sample_rate,
                        audio.samples,
                    );
                    sink.append(buf);
                }
            }
        }
        if !received_a {
            self.view.audio_level_a *= LEVEL_DECAY;
        }
        if let Some(dec) = &mut self.decoder_b {
            if let Some(sink) = &self.sink_b {
                while let Ok(audio) = dec.audio_rx.try_recv() {
                    received_b = true;
                    let peak = audio.samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
                    self.view.audio_level_b =
                        (self.view.audio_level_b * LEVEL_DECAY + peak).max(peak).min(1.0);
                    let buf = rodio::buffer::SamplesBuffer::new(
                        audio.channels,
                        audio.sample_rate,
                        audio.samples,
                    );
                    sink.append(buf);
                }
            }
        }
        if !received_b {
            self.view.audio_level_b *= LEVEL_DECAY;
        }
    }

    fn apply_sink_volumes(&mut self) {
        if let Some(sink) = &self.sink_a {
            if self.view.mute_a {
                sink.set_volume(0.0);
            } else {
                sink.set_volume(self.view.vol_a);
            }
        }
        if let Some(sink) = &self.sink_b {
            if self.view.mute_b {
                sink.set_volume(0.0);
            } else {
                sink.set_volume(self.view.vol_b);
            }
        }
    }

    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.pending_play_pause_toggle = true;
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                self.pending_key_action = PendingKeyAction::StepFwd;
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.pending_key_action = PendingKeyAction::StepBck;
            }
            if i.key_pressed(egui::Key::Home) {
                self.pending_key_action = PendingKeyAction::Seek(0.0);
            }
            if i.key_pressed(egui::Key::Y) {
                self.pending_key_action = PendingKeyAction::CycleMode;
            }
            if i.key_pressed(egui::Key::L) {
                self.pending_key_action = PendingKeyAction::SideBySide;
            }
            if i.key_pressed(egui::Key::Num1) {
                self.pending_key_action = PendingKeyAction::SplitPos0;
            }
            if i.key_pressed(egui::Key::Num2) {
                self.pending_key_action = PendingKeyAction::SplitPos1;
            }
            if i.key_pressed(egui::Key::Num3) {
                self.pending_key_action = PendingKeyAction::ToggleHud;
            }
            if i.key_pressed(egui::Key::Num4) {
                self.pending_key_action = PendingKeyAction::Zoom(1.0);
            }
            if i.key_pressed(egui::Key::Num5) {
                self.pending_key_action = PendingKeyAction::Zoom(0.5);
            }
            if i.key_pressed(egui::Key::Num6) {
                self.pending_key_action = PendingKeyAction::Zoom(1.0);
            }
            if i.key_pressed(egui::Key::Num7) {
                self.pending_key_action = PendingKeyAction::Zoom(2.0);
            }
            if i.key_pressed(egui::Key::Num8) {
                self.pending_key_action = PendingKeyAction::Zoom(4.0);
            }
            if i.key_pressed(egui::Key::Num9) {
                self.pending_key_action = PendingKeyAction::Zoom(8.0);
            }
            if i.key_pressed(egui::Key::F) {
                log::trace!("Key 'F': xcap OS-native capture");
                let dir_for_thread = self.view.screenshot_dir.clone();

                std::thread::spawn(move || {
                    let mut success = false;
                    log::trace!("xcap: scanning OS windows");
                    if let Ok(windows) = xcap::Window::all() {
                        for window in windows {
                            if let Ok(title) = window.title() {
                                if title.contains("Production Media") || title.contains("Diferencial") {
                                    log::trace!("xcap: window -> {}", title);
                                    if let Ok(img_buf) = window.capture_image() {
                                        if let Some(dir) = dir_for_thread.as_ref() {
                                            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                                            let filename = format!("WPP_QC_{timestamp}.png");
                                            let path = dir.join(filename);
                                            log::trace!("xcap: writing PNG to {:?}", path);

                                            if let Err(e) = img_buf.save(&path) {
                                                log::error!("xcap disk write error: {}", e);
                                            } else {
                                                log::trace!("xcap: screenshot saved");
                                                success = true;
                                            }
                                        }
                                    } else {
                                        log::error!("xcap failed to read window buffer");
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    if !success {
                        log::error!("xcap: target WPP window not found or capture failed");
                    }
                });
            }
            if i.key_pressed(egui::Key::R) {
                self.pending_key_action = PendingKeyAction::ResetZoomPan;
            }
            if i.key_pressed(egui::Key::S) {
                self.pending_key_action = PendingKeyAction::SwapVideos;
            }

            let now = i.time;
            let repeat_delay = 0.25;
            let repeat_interval = 0.05;

            if i.key_down(egui::Key::ArrowRight) {
                if i.key_pressed(egui::Key::ArrowRight)
                    || (now - self.last_step_time) > repeat_interval
                {
                    let delay_ok = (now - self.last_step_time) > repeat_delay;
                    if i.key_pressed(egui::Key::ArrowRight) || delay_ok {
                        self.pending_key_action = PendingKeyAction::StepFwd;
                        self.last_step_time = now;
                    }
                }
            } else if i.key_down(egui::Key::ArrowLeft) {
                if i.key_pressed(egui::Key::ArrowLeft)
                    || (now - self.last_step_time) > repeat_interval
                {
                    let delay_ok = (now - self.last_step_time) > repeat_delay;
                    if i.key_pressed(egui::Key::ArrowLeft) || delay_ok {
                        self.pending_key_action = PendingKeyAction::StepBck;
                        self.last_step_time = now;
                    }
                }
            } else {
                self.last_step_time = 0.0;
            }
        });
    }

    fn show_hud_panels(&mut self, ctx: &egui::Context, is_first_frame: bool) {
        if !self.view.show_hud || is_first_frame {
            return;
        }
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            crate::ui::controls::show_menu_bar(ui, self);
        });
        if self.view.show_left_panel {
            egui::SidePanel::left("info_panel")
                .resizable(true)
                .default_width(260.0)
                .min_width(200.0)
                .max_width(340.0)
                .show(ctx, |ui| {
                    crate::ui::info_panel::show(ui, self);
                });
        }
        if self.view.show_right_panel {
            egui::SidePanel::right("audio_panel")
                .resizable(true)
                .default_width(110.0)
                .min_width(90.0)
                .max_width(220.0)
                .show(ctx, |ui| {
                    crate::ui::controls::show_audio_panel(ui, self);
                });
        }
        egui::TopBottomPanel::bottom("timeline").show(ctx, |ui| {
            crate::ui::timeline::show(ui, self);
        });
    }

    fn show_clean_feed_viewport(&mut self, ctx: &egui::Context) {
        if !self.view.show_clean_feed_window {
            return;
        }
        let mut show = self.view.show_clean_feed_window;
        let renderer_clone = Arc::clone(&self.renderer);
        let title = crate::ui::controls::clean_feed_window_title(self.view.lang);

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("clean_feed_viewport"),
            egui::ViewportBuilder::default()
                .with_title(title)
                .with_inner_size([1280.0, 720.0])
                .with_always_on_top(),
            |ctx, _class| {
                if ctx.input(|i| i.viewport().close_requested()) {
                    show = false;
                }

                let fps = self
                    .decoder_a_meta()
                    .map(|m| m.fps)
                    .filter(|f| *f > 0.0)
                    .unwrap_or(24.0);
                let overlay_text = crate::ui::controls::clean_feed_overlay_text(
                    self.view.lang,
                    self.view.mode,
                    self.view.split_pos,
                    self.playback.current_pts,
                    fps,
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    let available = ui.available_rect_before_wrap();
                    ui.allocate_rect(available, egui::Sense::hover());
                    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                        available,
                        RenderCallback {
                            renderer: renderer_clone.clone(),
                        },
                    ));

                    let text_pos = available.min + egui::vec2(20.0, 20.0);
                    let galley = ui.painter().layout_no_wrap(
                        overlay_text,
                        egui::FontId::proportional(22.0),
                        egui::Color32::WHITE,
                    );
                    let bg_rect = galley.rect.translate(text_pos.to_vec2()).expand(6.0);
                    ui.painter().rect_filled(
                        bg_rect,
                        4.0,
                        egui::Color32::from_black_alpha(150),
                    );
                    ui.painter().galley(text_pos, galley, egui::Color32::WHITE);
                });
            },
        );

        self.view.show_clean_feed_window = show;
    }

    fn show_proxy_progress_window(&mut self, ctx: &egui::Context) {
        if !self.proxy_running() {
            return;
        }
        let progress = self.proxy_progress();
        let cap = crate::ui::controls::proxy_loading_caption(self.view.lang);
        egui::Window::new(cap)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(12.0);
                    ui.label(cap);
                    ui.add_space(8.0);
                    ui.add(egui::ProgressBar::new(progress.clamp(0.0, 1.0)).show_percentage());
                    ui.add_space(12.0);
                });
            });
    }

    fn show_error_modal_if_any(&mut self, ctx: &egui::Context) {
        let (title, msg) = match (&self.error_title, &self.error_message) {
            (Some(t), Some(m)) => (t.clone(), m.clone()),
            _ => return,
        };
        let lang = self.view.lang;
        let mut open = true;
        egui::Window::new(
            egui::RichText::new(&title)
                .color(egui::Color32::from_rgb(255, 100, 100))
                .strong(),
        )
        .collapsible(false)
        .resizable(false)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.screen_rect().center())
        .fixed_size(egui::vec2(400.0, 150.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.label(egui::RichText::new(&msg).size(15.0));
                ui.add_space(25.0);
                let ok = crate::ui::design::dialog_ok(lang);
                if ui.button(egui::RichText::new(format!("   {ok}   ")).strong()).clicked() {
                    open = false;
                }
                ui.add_space(10.0);
            });
        });
        if !open {
            self.error_title = None;
            self.error_message = None;
        }
    }
}

impl eframe::App for DiffPlayerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let is_first_frame = self.frame_count == 0;
        if self.frame_count == 0 {
            self.frame_count = 1;
        } else {
            self.frame_count = self.frame_count.saturating_add(1);
        }
        // macOS: repeatedly send Focus + Visible for first frames so window appears and comes to front.
        if self.focus_visible_frames_left > 0 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            self.focus_visible_frames_left -= 1;
        }
        // Process deferred play/pause toggle (Space) so we don't run play_both/pause_both from inside ctx.input() (avoids re-entrancy/deadlock on macOS).
        if self.pending_play_pause_toggle {
            self.pending_play_pause_toggle = false;
            if self.playback.is_playing {
                self.pause_both(ctx);
            } else {
                self.play_both(ctx);
            }
        }
        // Process deferred key action (arrows, Home, Y, L, Num1–9, R, S) so we never call ctx or decoder from inside ctx.input().
        match std::mem::take(&mut self.pending_key_action) {
            PendingKeyAction::None => {}
            PendingKeyAction::StepFwd => self.do_step_fwd_inner(ctx),
            PendingKeyAction::StepBck => self.do_step_bck_inner(ctx),
            PendingKeyAction::Seek(t) => self.do_seek_inner(t, ctx),
            PendingKeyAction::CycleMode => {
                self.view.mode = diffplayerqc_core::ciclar_modo_comparacion(self.view.mode);
                self.view.diff_mode = diffplayerqc_core::normalizar_modo_diferencia(
                    self.view.mode,
                    self.view.diff_mode,
                );
                ctx.request_repaint();
            }
            PendingKeyAction::SideBySide => {
                self.view.mode = CompareMode::SideBySide;
                ctx.request_repaint();
            }
            PendingKeyAction::SplitPos0 => {
                self.view.mode = CompareMode::SplitScreen;
                self.view.split_pos = if self.view.split_pos < 0.05 { 0.5 } else { 0.0 };
                ctx.request_repaint();
            }
            PendingKeyAction::SplitPos1 => {
                self.view.mode = CompareMode::SplitScreen;
                self.view.split_pos = if self.view.split_pos > 0.95 { 0.5 } else { 1.0 };
                ctx.request_repaint();
            }
            PendingKeyAction::ToggleHud => {
                self.view.show_hud = !self.view.show_hud;
            }
            PendingKeyAction::Zoom(z) => {
                self.view.zoom = z;
            }
            PendingKeyAction::ResetZoomPan => {
                self.view.zoom = 1.0;
                self.view.pan_u = 0.0;
                self.view.pan_v = 0.0;
            }
            PendingKeyAction::SwapVideos => self.swap_videos_inner(ctx),
        }
        // When proxy generation just finished: load proxy.mkv into the target channel.
        self.complete_proxy_if_ready(ctx);
        log::trace!("App::update() tick");
        // First frame: don't request repaint so macOS can finish present.
        // Later: only schedule repaint when playing (request_repaint_after); when paused, input triggers repaint.

        // Master clock and repaint cadence.
        self.update_master_clock_and_repaint(ctx, is_first_frame);
        // Keep UI updating while proxy generation is running
        if self.proxy_running() {
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        // Skip Wgpu work on first frame so window can appear on macOS (avoids first-frame block).
        if !is_first_frame {
            if let Some(rs) = frame.wgpu_render_state() {
                if self.drain_frames(rs) {
                    ctx.request_repaint();
                }
            }
            self.sync_uniforms();
        }
        self.drain_audio_and_update_levels();
        self.apply_sink_volumes();

        // ── Handle screenshot events ────────────────────────────────────────
        let events = ctx.input(|i| i.raw.events.clone());
        // (Wgpu Screenshot event listener removed to use OS-native xcap instead)
        for _event in events {
            // Processing other events...
        }

        self.handle_keyboard_input(ctx);

        // ── UI Overlay conditionally rendered ───────────────────────────────
        // Skip HUD on first frame (macOS Metal): minimal first frame so present can complete.
        self.show_hud_panels(ctx, is_first_frame);

        // ── Central canvas ──────────────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            show_canvas(ui, self, frame);
        });

        self.show_clean_feed_viewport(ctx);
        self.show_proxy_progress_window(ctx);
        self.show_error_modal_if_any(ctx);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.view.save();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        log::info!("Application exiting, triggering final save");
        self.view.save();
        for dir in &self.proxy_temp_dirs {
            if let Err(e) = std::fs::remove_dir_all(dir) {
                log::warn!("Failed to remove proxy temp dir {:?}: {}", dir, e);
            } else {
                log::info!("Removed proxy temp dir: {:?}", dir);
            }
        }
        self.proxy_temp_dirs.clear();
        if let Some(ref dir) = self.proxy_temp_dir {
            if let Err(e) = std::fs::remove_dir_all(dir) {
                log::warn!("Failed to remove proxy temp dir {:?}: {}", dir, e);
            }
            self.proxy_temp_dir = None;
        }
    }
}

// ---------------------------------------------------------------------------
//  Video canvas with zoom / pan interaction
// ---------------------------------------------------------------------------

fn show_canvas(ui: &mut egui::Ui, app: &mut DiffPlayerApp, _frame: &mut eframe::Frame) {
    let available = ui.available_rect_before_wrap();
    app.view.canvas_rect = available;

    let response = ui.allocate_rect(available, egui::Sense::click_and_drag());

    // -- Mouse wheel zoom ---------------------------------------------------
    let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
    if response.hovered() && scroll_delta != 0.0 {
        let zoom_factor = if scroll_delta > 0.0 {
            1.1f32
        } else {
            1.0 / 1.1
        };
        app.view.zoom = (app.view.zoom * zoom_factor).clamp(0.25, 32.0);
    }

    // -- Drag to pan OR drag split line (Available in all modes) -------------
    // Pan is only active when zoomed in (zoom > 1.0). At fit-to-frame only the
    // split divider can be dragged. Split line is vertical or horizontal per split_horizontal.
    if response.drag_started() {
        let pos = response.interact_pointer_pos().unwrap_or_default();
        let near_split = if app.view.split_horizontal {
            let split_y = available.top() + app.view.split_pos * available.height();
            (pos.y - split_y).abs() < 15.0
        } else {
            let split_x = available.left() + app.view.split_pos * available.width();
            (pos.x - split_x).abs() < 15.0
        };
        if near_split {
            app.dragging_split = true;
        } else {
            app.dragging_split = false;
            if app.view.zoom > 1.0 {
                app.drag_start = Some((pos, app.view.pan_u, app.view.pan_v));
            }
        }
    }

    if response.dragged() {
        if app.dragging_split {
            let pos = response.interact_pointer_pos().unwrap_or_default();
            if app.view.split_horizontal {
                let relative_y = (pos.y - available.top()) / available.height();
                app.view.split_pos = relative_y.clamp(0.0, 1.0);
            } else {
                let relative_x = (pos.x - available.left()) / available.width();
                app.view.split_pos = relative_x.clamp(0.0, 1.0);
            }
            ui.ctx().request_repaint();
        } else if let Some((start_pos, start_pu, start_pv)) = app.drag_start {
            let delta = response.interact_pointer_pos().unwrap_or_default() - start_pos;
            let uv_delta_u = -delta.x / available.width() / app.view.zoom;
            let uv_delta_v = -delta.y / available.height() / app.view.zoom;
            app.view.pan_u = (start_pu + uv_delta_u).clamp(-0.5, 0.5);
            app.view.pan_v = (start_pv + uv_delta_v).clamp(-0.5, 0.5);
            ui.ctx().request_repaint();
        }
    }

    if response.drag_stopped() {
        app.drag_start = None;
        app.dragging_split = false;
    }

    // -- Cursor hint for dragging split (Available in all modes) ------------
    if let Some(ptr) = ui.ctx().pointer_hover_pos() {
        let near_split = if app.view.split_horizontal {
            let split_y = available.top() + app.view.split_pos * available.height();
            available.contains(ptr) && (ptr.y - split_y).abs() < 10.0
        } else {
            let split_x = available.left() + app.view.split_pos * available.width();
            available.contains(ptr) && (ptr.x - split_x).abs() < 10.0
        };
        if near_split {
            ui.ctx().set_cursor_icon(if app.view.split_horizontal {
                egui::CursorIcon::ResizeVertical
            } else {
                egui::CursorIcon::ResizeHorizontal
            });
        }
    }

    // -- Double-click to reset zoom -----------------------------------------
    if response.double_clicked() {
        app.view.zoom = 1.0;
        app.view.pan_u = 0.0;
        app.view.pan_v = 0.0;
    }

    // -- Draw the wgpu render callback into this rect ----------------------
    // Skip on first frame so macOS window can appear (first Wgpu present can block).
    if app.frame_count > 1 {
        let renderer_clone = Arc::clone(&app.renderer);
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            available,
            RenderCallback {
                renderer: renderer_clone,
            },
        ));
    } else {
        ui.painter()
            .rect_filled(available, 0.0, egui::Color32::from_rgb(0, 0, 0));
    }

    // -- Safe zones overlay (video_rect + zoom/pan) -------------------------
    // In SideBySide mode draw on both halves (A left, B right); otherwise once on full canvas.
    if app.view.safe_zone != SafeZoneMode::None {
        let zoom = app.view.zoom;
        let visible_left = 0.5 - 0.5 / zoom + app.view.pan_u;
        let visible_right = 0.5 + 0.5 / zoom + app.view.pan_u;
        let visible_top = 0.5 - 0.5 / zoom + app.view.pan_v;
        let visible_bottom = 0.5 + 0.5 / zoom + app.view.pan_v;

        let draw_safe_zones = |container: egui::Rect, vw: f32, vh: f32| {
            let cw = container.width();
            let ch = container.height();
            let video_aspect = vw / vh;
            let container_aspect = cw / ch;
            let video_rect = if video_aspect >= container_aspect {
                let h = cw / video_aspect;
                let top = container.center().y - h * 0.5;
                egui::Rect::from_min_max(
                    egui::Pos2::new(container.left(), top),
                    egui::Pos2::new(container.right(), top + h),
                )
            } else {
                let w = ch * video_aspect;
                let left = container.center().x - w * 0.5;
                egui::Rect::from_min_max(
                    egui::Pos2::new(left, container.top()),
                    egui::Pos2::new(left + w, container.bottom()),
                )
            };
            let uv_to_screen = |u: f32, v: f32| {
                let x = video_rect.left()
                    + (u - visible_left) / (visible_right - visible_left) * video_rect.width();
                let y = video_rect.top()
                    + (v - visible_top) / (visible_bottom - visible_top) * video_rect.height();
                egui::Pos2::new(x, y)
            };

            match app.view.safe_zone {
                SafeZoneMode::None => {}
                SafeZoneMode::TvEbu => {
                    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 200, 255));
                    let action_min = uv_to_screen(0.035, 0.035);
                    let action_max = uv_to_screen(0.965, 0.965);
                    let action_rect = egui::Rect::from_min_max(action_min, action_max);
                    ui.painter().rect_stroke(action_rect, 0.0, stroke);
                    let title_min = uv_to_screen(0.10, 0.05);
                    let title_max = uv_to_screen(0.90, 0.95);
                    let title_rect = egui::Rect::from_min_max(title_min, title_max);
                    ui.painter().rect_stroke(title_rect, 0.0, stroke);
                    let center = uv_to_screen(0.5, 0.5);
                    let cross_half = 10.0;
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(center.x - cross_half, center.y),
                            egui::Pos2::new(center.x + cross_half, center.y),
                        ],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(center.x, center.y - cross_half),
                            egui::Pos2::new(center.x, center.y + cross_half),
                        ],
                        stroke,
                    );
                }
                SafeZoneMode::Social => {
                    let danger_fill = egui::Color32::from_black_alpha(150);
                    let top_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.0), uv_to_screen(1.0, 0.15));
                    let bottom_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.78), uv_to_screen(1.0, 1.0));
                    let right_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.85, 0.0), uv_to_screen(1.0, 1.0));
                    let left_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.0), uv_to_screen(0.05, 1.0));
                    ui.painter().rect_filled(top_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(bottom_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(right_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(left_danger, 0.0, danger_fill);
                    let safe_min = uv_to_screen(0.05, 0.15);
                    let safe_max = uv_to_screen(0.85, 0.78);
                    let safe_rect = egui::Rect::from_min_max(safe_min, safe_max);
                    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 200, 0));
                    ui.painter().rect_stroke(safe_rect, 0.0, stroke);
                }
            }
        };

        if app.view.mode == CompareMode::SideBySide {
            let mid_x = available.center().x;
            let left_rect =
                egui::Rect::from_min_max(available.min, egui::pos2(mid_x, available.max.y));
            let right_rect =
                egui::Rect::from_min_max(egui::pos2(mid_x, available.min.y), available.max);
            let (vw_a, vh_a) = app
                .decoder_a_meta()
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            let (vw_b, vh_b) = app
                .decoder_b_meta()
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            draw_safe_zones(left_rect, vw_a, vh_a);
            draw_safe_zones(right_rect, vw_b, vh_b);
        } else {
            let (vw, vh) = app
                .decoder_a_meta()
                .or_else(|| app.decoder_b_meta())
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            draw_safe_zones(available, vw, vh);
        }
    }

    // -- OS file drag-and-drop handling ------------------------------------
    let hovered_files = ui.ctx().input(|i| i.raw.hovered_files.clone());
    let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());

    // IMPORTANT: Handle the actual drop FIRST, before we potentially clear
    // drag_drop_hover_pos in the else branch below. On the drop frame,
    // hovered_files is already empty but drag_drop_hover_pos still holds
    // the last valid cursor position from the previous frame.
    if !dropped_files.is_empty() {
        // Collect paths for EXR or video handling
        let paths: Vec<PathBuf> = dropped_files
            .iter()
            .filter_map(|f| f.path.as_ref().map(PathBuf::from))
            .collect();

        // EXR: single directory -> proxy from folder; all .exr files -> proxy from list. Target channel from drop position.
        let mid_x = available.center().x;
        let hover_x = app
            .drag_drop_hover_pos
            .or_else(|| ui.ctx().pointer_hover_pos())
            .unwrap_or(available.center())
            .x;
        let target_chan = if hover_x < mid_x {
            crate::types::Channel::A
        } else {
            crate::types::Channel::B
        };
        if paths.len() == 1 && paths[0].is_dir() {
            app.start_proxy_from_exr_input_dir(paths[0].clone(), target_chan, ui.ctx());
            app.drag_drop_hover_pos = None;
            return;
        }
        let all_exr = !paths.is_empty()
            && paths.iter().all(|p| {
                p.extension()
                    .map(|e| {
                        e.to_str()
                            .map(|s| s.eq_ignore_ascii_case("exr"))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            });
        if all_exr {
            app.start_proxy_from_exr_input_files(paths, target_chan, ui.ctx());
            app.drag_drop_hover_pos = None;
            return;
        }

        // Video handling
        let valid_extensions = [
            "mp4", "mov", "mxf", "mkv", "avi", "prores", "mts", "mpg", "mpeg", "ts",
        ];
        let mut valid_paths = Vec::new();
        let mut invalid_files = Vec::new();

        for path in &paths {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if valid_extensions.contains(&ext.as_str()) {
                valid_paths.push(path.to_string_lossy().to_string());
            } else {
                invalid_files.push(
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }

        if !invalid_files.is_empty() {
            app.error_title = Some("Formato no soportado".to_string());
            app.error_message = Some(format!(
                "Los siguientes archivos no son formatos soportados:\n{}",
                invalid_files.join(", ")
            ));
        } else if valid_paths.len() > 2 {
            app.error_title = Some("Máximo 2 videos".to_string());
            app.error_message =
                Some("Solo puedes arrastrar un máximo de 2 videos a la vez.".to_string());
        } else if valid_paths.len() == 2 {
            valid_paths.sort(); // A goes to Slot A, B goes to Slot B alphabetically
            app.open_video_a_from_path(valid_paths[0].clone(), ui.ctx());
            app.open_video_b_from_path(valid_paths[1].clone(), ui.ctx());
        } else if !valid_paths.is_empty() {
            let mid_x = available.center().x;
            let hover_x = app
                .drag_drop_hover_pos
                .or_else(|| ui.ctx().pointer_hover_pos())
                .unwrap_or(available.center())
                .x;
            if hover_x < mid_x {
                app.open_video_a_from_path(valid_paths[0].clone(), ui.ctx());
            } else {
                app.open_video_b_from_path(valid_paths[0].clone(), ui.ctx());
            }
        }

        app.drag_drop_hover_pos = None;
    } else if !hovered_files.is_empty() {
        // Files are being dragged over — update position and draw overlay
        if let Some(ptr) = ui.ctx().pointer_hover_pos() {
            app.drag_drop_hover_pos = Some(ptr);
        }

        let mid_x = available.center().x;
        let hover_x = app.drag_drop_hover_pos.map(|p| p.x).unwrap_or(mid_x);
        let targeting_a = hover_x < mid_x;

        let (a_alpha, b_alpha) = if targeting_a {
            (80u8, 30u8)
        } else {
            (30u8, 80u8)
        };

        let left_rect = egui::Rect::from_min_max(available.min, egui::pos2(mid_x, available.max.y));
        let right_rect =
            egui::Rect::from_min_max(egui::pos2(mid_x, available.min.y), available.max);

        ui.painter().rect_filled(
            left_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(80, 180, 100, a_alpha),
        );
        ui.painter().rect_filled(
            right_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(80, 130, 220, b_alpha),
        );

        let is_es = app.view.lang == Language::Es;
        let label_a = if is_es {
            "Soltar aquí → VIDEO A"
        } else {
            "Drop here → VIDEO A"
        };
        let label_b = if is_es {
            "Soltar aquí → VIDEO B"
        } else {
            "Drop here → VIDEO B"
        };
        ui.painter().text(
            left_rect.center(),
            egui::Align2::CENTER_CENTER,
            label_a,
            egui::FontId::proportional(22.0),
            egui::Color32::from_rgba_premultiplied(220, 255, 220, 230),
        );
        ui.painter().text(
            right_rect.center(),
            egui::Align2::CENTER_CENTER,
            label_b,
            egui::FontId::proportional(22.0),
            egui::Color32::from_rgba_premultiplied(200, 220, 255, 230),
        );
        ui.painter().vline(
            mid_x,
            available.y_range(),
            egui::Stroke::new(
                2.0,
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 120),
            ),
        );

        ui.ctx().request_repaint();
    } else {
        // Nothing dragged — clear stored position
        app.drag_drop_hover_pos = None;
    }

    // -- Overlay: "No video" message when nothing is loaded ----------------
    let has_a = app.decoder_a.is_some();
    let has_b = app.decoder_b.is_some();
    if !has_a || !has_b {
        let center = available.center();
        let is_es = app.view.lang == Language::Es;
        let text = if !has_a && !has_b {
            if is_es {
                "Abre el Vídeo A y el Vídeo B para empezar la comparación"
            } else {
                "Open Video A and Video B to begin comparison"
            }
        } else if !has_a {
            if is_es {
                "Abre el Vídeo A  ←  (panel izquierdo)"
            } else {
                "Open Video A  ←  (left panel)"
            }
        } else {
            if is_es {
                "Abre el Vídeo B  →  (panel izquierdo)"
            } else {
                "Open Video B  →  (left panel)"
            }
        };
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(20.0),
            ui.visuals().text_color().gamma_multiply(0.5),
        );
    }

    // -- Zoom indicator overlay (top-right of canvas) ----------------------
    if (app.view.zoom - 1.0).abs() > 0.01 {
        let zoom_text = format!("{:.1}×", app.view.zoom);
        let pos = egui::pos2(available.right() - 8.0, available.top() + 8.0);
        ui.painter().text(
            pos,
            egui::Align2::RIGHT_TOP,
            &zoom_text,
            egui::FontId::monospace(13.0),
            egui::Color32::from_rgba_premultiplied(200, 200, 100, 200),
        );
    }

    // -- Frame counter overlay (bottom-left of canvas, unobtrusive) --------
    // Shows permanently, including during screenshots.
    {
        let fps_a = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
        let current_pts = app.playback().current_pts;
        let frame_num = (current_pts * fps_a).round() as u64;
        let is_es = app.view().lang == Language::Es;

        let frame_text = format!("{} {}", if is_es { "Fr." } else { "Frame" }, frame_num);
        let pos = egui::pos2(available.left() + 8.0, available.bottom() - 8.0);
        ui.painter().text(
            pos,
            egui::Align2::LEFT_BOTTOM,
            &frame_text,
            egui::FontId::monospace(14.0),
            egui::Color32::from_black_alpha(150), // Subtle shadow
        );
        ui.painter().text(
            pos - egui::Vec2::new(1.0, 1.0),
            egui::Align2::LEFT_BOTTOM,
            &frame_text,
            egui::FontId::monospace(14.0),
            egui::Color32::from_white_alpha(150), // Unobtrusive text
        );
    }
}

fn default_rect() -> egui::Rect {
    egui::Rect::NOTHING
}

// ---------------------------------------------------------------------------
//  Font setup
// ---------------------------------------------------------------------------

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // List of common font paths for different OSes
    let font_paths = [
        "C:/Windows/Fonts/arial.ttf",                   // Windows
        "/Library/Fonts/Arial.ttf",                     // macOS
        "/System/Library/Fonts/Supplemental/Arial.ttf", // macOS Supplemental
        "/Library/Fonts/Helvetica.ttc",                 // macOS Helvetica fallback
    ];

    for path in font_paths {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert("DefaultFont".to_owned(), egui::FontData::from_owned(bytes));
            // Insert at the front of the proportional list
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "DefaultFont".to_owned());
            // Also use as monospace fallback
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("DefaultFont".to_owned());
            log::info!("Loaded font from: {:?}", path);
            break;
        }
    }

    ctx.set_fonts(fonts);

    // Apply overall style tweaks
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.slider_width = 120.0;
    ctx.set_style(style);
}

// Expose app fields the UI modules need
impl DiffPlayerApp {
    pub fn view_mut(&mut self) -> &mut ViewState {
        &mut self.view
    }
    pub fn view(&self) -> &ViewState {
        &self.view
    }
    pub fn playback(&self) -> &PlaybackState {
        &self.playback
    }
    pub fn decoder_a_meta(&self) -> Option<&ColorMetadata> {
        self.decoder_a.as_ref().map(|d| &d.meta)
    }
    pub fn decoder_b_meta(&self) -> Option<&ColorMetadata> {
        self.decoder_b.as_ref().map(|d| &d.meta)
    }
    pub fn decoder_a_path(&self) -> Option<&str> {
        self.decoder_a.as_ref().map(|d| d.path.as_str())
    }
    pub fn decoder_b_path(&self) -> Option<&str> {
        self.decoder_b.as_ref().map(|d| d.path.as_str())
    }
    pub fn open_video_a(&mut self, ctx: &egui::Context) {
        self.open_video(Channel::A, ctx);
    }
    pub fn open_video_b(&mut self, ctx: &egui::Context) {
        self.open_video(Channel::B, ctx);
    }
    pub fn open_video_a_from_path(&mut self, path: String, ctx: &egui::Context) {
        self.open_video_from_path(path, Channel::A, ctx);
    }
    pub fn open_video_b_from_path(&mut self, path: String, ctx: &egui::Context) {
        self.open_video_from_path(path, Channel::B, ctx);
    }
    pub fn do_play(&mut self, _ctx: &egui::Context) {
        self.pending_play_pause_toggle = true;
    }
    pub fn do_pause(&mut self, _ctx: &egui::Context) {
        self.pending_play_pause_toggle = true;
    }
    /// Enqueue step forward; processed at start of next update (avoids re-entrancy from keyboard/UI).
    pub fn do_step_fwd(&mut self, _ctx: &egui::Context) {
        self.pending_key_action = PendingKeyAction::StepFwd;
    }
    /// Enqueue step back; processed at start of next update (avoids re-entrancy from keyboard/UI).
    pub fn do_step_bck(&mut self, _ctx: &egui::Context) {
        self.pending_key_action = PendingKeyAction::StepBck;
    }
    /// Called from start of update() when pending_key_action was StepFwd.
    fn do_step_fwd_inner(&mut self, ctx: &egui::Context) {
        if self.playback.is_playing {
            self.pause_both(ctx);
        }
        self.step_forward(ctx);
    }
    /// Called from start of update() when pending_key_action was StepBck.
    fn do_step_bck_inner(&mut self, ctx: &egui::Context) {
        if self.playback.is_playing {
            self.pause_both(ctx);
        }
        let fps = match (self.decoder_a_meta(), self.decoder_b_meta()) {
            (Some(a), _) if a.fps > 0.0 => a.fps,
            (_, Some(b)) if b.fps > 0.0 => b.fps,
            _ => 25.0,
        };
        let t = (self.playback.current_pts - 1.0 / fps).max(0.0);
        self.do_seek_inner(t, ctx);
    }
    pub fn do_seek(&mut self, t: f64, ctx: &egui::Context) {
        self.do_seek_inner(t, ctx);
    }
    fn do_seek_inner(&mut self, t: f64, ctx: &egui::Context) {
        crate::trace_log::log(&format!("Seek to {:.3}s", t));
        self.seek_both(t, ctx);
        self.playback.current_pts = t;

        // Clear audio sink buffers since we are jumping in time
        if let Some(s) = &self.sink_a {
            s.clear();
            s.play();
        }
        if let Some(s) = &self.sink_b {
            s.clear();
            s.play();
        }
        if !self.playback.is_playing {
            if let Some(s) = &self.sink_a {
                s.pause();
            }
            if let Some(s) = &self.sink_b {
                s.pause();
            }
        }

        // Discard any trailing frames in the pipeline so the next frame is exactly the requested one
        if let Some(dec) = &mut self.decoder_a {
            dec.next_frame = None;
            while dec.frame_rx.try_recv().is_ok() {}
            while dec.audio_rx.try_recv().is_ok() {}
        }
        if let Some(dec) = &mut self.decoder_b {
            dec.next_frame = None;
            while dec.frame_rx.try_recv().is_ok() {}
            while dec.audio_rx.try_recv().is_ok() {}
        }

        // Restore decoder playback state if we were playing. Decoder threads pause automatically on seek.
        if self.playback.is_playing {
            self.play_both(ctx);
        }
        ctx.request_repaint();
    }
}
