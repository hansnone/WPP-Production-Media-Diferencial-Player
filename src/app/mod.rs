//! Aplicación egui/eframe: estado global, bucle `update`, decoders, audio y proxy EXR.
//!
//! Submódulos: [`playback`] (temporización de repintado), [`proxy_bridge`] (ruta al proxy.mkv).
//! Ver `docs/ARQUITECTURA.md` en el repositorio para el flujo completo.

mod audio;
mod canvas;
mod drag_drop;
mod keyboard;
mod layout;
pub mod playback;
mod proxy;
mod proxy_bridge;

use crossbeam_channel::{Receiver, Sender};
use parking_lot::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::decoder;
use crate::renderer::{RenderCallback, ShaderUniforms, VideoRenderer};
use crate::types::{
    AudioFrame, Channel, ColorMetadata, CompareMode, DecoderCommand, DiffMode, Language,
    PlaybackState, VideoFrame,
};
use rodio::{OutputStream, Sink};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
//  View state — zoom / pan / mode / sliders
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewState {
    pub mode: CompareMode,
    pub diff_mode: DiffMode,
    pub lang: Language,
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
    pub show_vu_meter: bool,
    /// Canvas rect in egui screen-space (for coordinate transform)
    #[serde(skip, default = "default_rect")]
    pub canvas_rect: egui::Rect,
    pub mute_a: bool,
    pub mute_b: bool,
    pub vol_a: f32,
    pub vol_b: f32,
    pub timeline_thumbs_channel: crate::types::Channel,
    /// Split curtain orientation: false = vertical (X), true = horizontal (Y).
    pub split_horizontal: bool,
    /// Safe zone overlay: None, TV (EBU R95), or Social (9:16).
    pub safe_zone: crate::types::SafeZoneMode,
    /// EBU R128 loudness metrics for channel A (not persisted).
    #[serde(skip, default)]
    pub loudness_a: LoudnessResult,
    /// EBU R128 loudness metrics for channel B (not persisted).
    #[serde(skip, default)]
    pub loudness_b: LoudnessResult,
    #[serde(default = "default_true")]
    pub loop_playback: bool,
    #[serde(skip)]
    pub saved_loop_playback: Option<bool>,
    #[serde(skip)]
    pub pending_play_after_delay: Option<std::time::Instant>,
    #[serde(skip)]
    pub last_psnr: Option<f64>,
}

use crate::types::LoudnessResult;

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
            show_vu_meter: true,
            canvas_rect: egui::Rect::NOTHING,
            mute_a: false,
            mute_b: false,
            timeline_thumbs_channel: crate::types::Channel::A,
            vol_a: 1.0,
            vol_b: 1.0,
            split_horizontal: false,
            safe_zone: crate::types::SafeZoneMode::None,
            loudness_a: LoudnessResult {
                momentary: 0.0,
                short_term: 0.0,
                integrated: 0.0,
                true_peak: [0.0, 0.0],
            },
            loudness_b: LoudnessResult {
                momentary: 0.0,
                short_term: 0.0,
                integrated: 0.0,
                true_peak: [0.0, 0.0],
            },
            loop_playback: true,
            saved_loop_playback: None,
            pending_play_after_delay: None,
            last_psnr: None,
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
    loudness_arc: Arc<Mutex<crate::types::LoudnessResult>>,
}

// ---------------------------------------------------------------------------
//  Main application struct
// ---------------------------------------------------------------------------

pub struct DiffPlayerApp {
    decoder_a: Option<DecoderHandle>,
    decoder_b: Option<DecoderHandle>,

    view: ViewState,
    playback: PlaybackState,
    pub session: crate::types::SessionState,

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

    /// Deferred play/pause action: process at start of next update to avoid re-entrancy deadlock.
    pending_transport_action: PendingTransportAction,

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
    /// Error string from proxy background thread.
    proxy_error: Arc<Mutex<Option<String>>>,

    pub thumb_rx_a: Option<crossbeam_channel::Receiver<crate::thumbnail::ThumbnailFrame>>,
    pub thumb_rx_b: Option<crossbeam_channel::Receiver<crate::thumbnail::ThumbnailFrame>>,
    pub thumb_running_a: Arc<AtomicBool>,
    pub thumb_running_b: Arc<AtomicBool>,
    pub thumbs_a: Vec<Option<egui::TextureHandle>>,
    pub thumbs_b: Vec<Option<egui::TextureHandle>>,
}

/// Key actions deferred from ctx.input() to start of update() to avoid re-entrancy on macOS.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(super) enum PendingKeyAction {
    #[default]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PendingTransportAction {
    #[default]
    None,
    Play,
    Pause,
    Toggle,
}

impl DiffPlayerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let render_state = match cc.wgpu_render_state.as_ref() {
            Some(rs) => rs,
            None => {
                log::error!("CRITICAL: Wgpu render state is missing from CreationContext! Make sure eframe is configured with eframe::Renderer::Wgpu.");
                panic!("Wgpu render state missing");
            }
        };

        let target_format = render_state.target_format;
        let renderer = Arc::new(Mutex::new(VideoRenderer::new(
            &render_state.device,
            target_format,
        )));

        let view = ViewState::load();
        crate::ui::theme::apply_professional_dark_theme(&cc.egui_ctx);

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
            session: crate::types::SessionState::default(),
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
            pending_transport_action: PendingTransportAction::None,
            pending_key_action: PendingKeyAction::None,

            proxy_progress: Arc::new(Mutex::new(0.0)),
            proxy_running: Arc::new(AtomicBool::new(false)),
            proxy_temp_dir: None,
            proxy_target_channel: None,
            proxy_temp_dirs: Vec::new(),
            proxy_error: Arc::new(Mutex::new(None)),
            thumb_rx_a: None,
            thumb_rx_b: None,
            thumb_running_a: Arc::new(AtomicBool::new(false)),
            thumb_running_b: Arc::new(AtomicBool::new(false)),
            thumbs_a: Vec::new(),
            thumbs_b: Vec::new(),
        }
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
        match decoder::spawn_decoder(&path_str, 48000, 2) {
            Ok((cmd_tx, frame_rx, audio_rx, meta, loudness_arc)) => {
                let handle = DecoderHandle {
                    cmd_tx,
                    frame_rx,
                    audio_rx,
                    last_frame: None,
                    next_frame: None,
                    meta,
                    path: path_str,
                    loudness_arc,
                };

                match chan {
                    Channel::A => {
                        self.session.video_a_path = Some(handle.path.clone());
                        // Stop old decoder if any
                        if let Some(old) = self.decoder_a.take() {
                            let _ = old.cmd_tx.send(DecoderCommand::Stop);
                        }
                        if let Some(sink) = &self.sink_a {
                            sink.clear();
                        }
                        self.playback.duration_a = handle.meta.duration_secs;
                        let (rx, running) = crate::thumbnail::spawn_thumbnail_generator(
                            handle.path.clone(),
                            handle.meta.duration_secs,
                            100, // Fixed count for now
                        );
                        self.thumb_rx_a = Some(rx);
                        self.thumb_running_a = running;
                        self.thumbs_a = vec![None; 100];
                        self.decoder_a = Some(handle);
                        self.do_seek(0.0, ctx);
                        // No need for repaint here as do_seek handles it
                    }
                    Channel::B => {
                        self.session.video_b_path = Some(handle.path.clone());
                        if let Some(old) = self.decoder_b.take() {
                            let _ = old.cmd_tx.send(DecoderCommand::Stop);
                        }
                        if let Some(sink) = &self.sink_b {
                            sink.clear();
                        }
                        self.playback.duration_b = handle.meta.duration_secs;
                        let (rx, running) = crate::thumbnail::spawn_thumbnail_generator(
                            handle.path.clone(),
                            handle.meta.duration_secs,
                            100,
                        );
                        self.thumb_rx_b = Some(rx);
                        self.thumb_running_b = running;
                        self.thumbs_b = vec![None; 100];
                        self.decoder_b = Some(handle);
                        crate::ui::vu_meter::reset_meter_state(1);
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
    fn drain_thumbnails(&mut self, ctx: &egui::Context) {
        let process_thumbs =
            |rx: &crossbeam_channel::Receiver<crate::thumbnail::ThumbnailFrame>,
             thumbs: &mut Vec<Option<egui::TextureHandle>>| {
                while let Ok(frame) = rx.try_recv() {
                    let img = egui::ColorImage::from_rgba_unmultiplied(
                        [frame.width as usize, frame.height as usize],
                        &frame.rgba_data,
                    );
                    let handle = ctx.load_texture(
                        format!("thumb_{}", frame.index),
                        img,
                        egui::TextureOptions::LINEAR,
                    );
                    if frame.index < thumbs.len() {
                        thumbs[frame.index] = Some(handle);
                    }
                }
            };

        if let Some(rx) = &self.thumb_rx_a {
            process_thumbs(rx, &mut self.thumbs_a);
        }
        if let Some(rx) = &self.thumb_rx_b {
            process_thumbs(rx, &mut self.thumbs_b);
        }
    }

    fn drain_frames(&mut self, render_state: &egui_wgpu::RenderState) -> bool {
        let device = &render_state.device;
        let queue = &render_state.queue;
        let current_pts = self.playback.current_pts;
        let is_playing = self.playback.is_playing;
        let mut repainted = false;

        let mut process_dec = |dec: &mut DecoderHandle, is_a: bool| {
            let fps = dec.meta.fps.max(1.0);
            let tolerance = 0.5 / fps;
            let incoming = std::iter::from_fn(|| dec.frame_rx.try_recv().ok());
            let (best_frame, next) = playback::select_best_frame(
                dec.next_frame.take(),
                incoming,
                current_pts,
                tolerance,
            );
            dec.next_frame = next;

            if let Some(frame) = best_frame {
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
                dec.last_frame = Some(frame);
                repainted = true;
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
    pub fn set_loop_in(&mut self) {
        self.playback.loop_in = Some(self.playback.current_pts);
        if self.playback.loop_out.is_some() {
            self.playback.loop_range_active = true;
        }
    }
    pub fn set_loop_out(&mut self) {
        self.playback.loop_out = Some(self.playback.current_pts);
        if self.playback.loop_in.is_some() {
            self.playback.loop_range_active = true;
        }
    }
    pub fn toggle_loop_range(&mut self) {
        if self.playback.loop_in.is_some() && self.playback.loop_out.is_some() {
            self.playback.loop_range_active = !self.playback.loop_range_active;
        }
    }
    fn swap_videos_inner(&mut self, ctx: &egui::Context) {
        std::mem::swap(&mut self.decoder_a, &mut self.decoder_b);
        std::mem::swap(&mut self.playback.duration_a, &mut self.playback.duration_b);
        std::mem::swap(&mut self.view.mute_a, &mut self.view.mute_b);
        std::mem::swap(&mut self.view.vol_a, &mut self.view.vol_b);
        std::mem::swap(&mut self.sink_a, &mut self.sink_b);
        std::mem::swap(
            &mut self.session.video_a_path,
            &mut self.session.video_b_path,
        );
        std::mem::swap(&mut self.thumb_rx_a, &mut self.thumb_rx_b);
        std::mem::swap(&mut self.thumb_running_a, &mut self.thumb_running_b);
        std::mem::swap(&mut self.thumbs_a, &mut self.thumbs_b);
        // Force rendering buffers to swap next frame
        if let Some(dec) = &mut self.decoder_a {
            dec.next_frame = dec.frame_rx.try_recv().ok();
        }
        if let Some(dec) = &mut self.decoder_b {
            dec.next_frame = dec.frame_rx.try_recv().ok();
        }
        ctx.request_repaint();
    }

    fn update_master_clock_and_repaint(&mut self, ctx: &egui::Context, is_first_frame: bool) {
        if !self.playback.is_playing {
            return;
        }
        if let Some(start) = self.playback.playback_start_instant {
            let elapsed = start.elapsed().as_secs_f64();
            self.playback.current_pts = self.playback.playback_start_pts + elapsed;
            let max_duration = self.playback.duration_a.max(self.playback.duration_b);

            if self.playback.loop_range_active {
                if let (Some(in_pts), Some(out_pts)) =
                    (self.playback.loop_in, self.playback.loop_out)
                {
                    if self.playback.current_pts >= out_pts {
                        self.do_seek(in_pts, ctx);
                    }
                }
            } else if max_duration > 0.0 {
                if self.view.loop_playback && self.playback.current_pts >= max_duration {
                    self.do_seek(0.0, ctx);
                } else if self.playback.current_pts >= max_duration {
                    self.pause_both(ctx);
                    self.playback.current_pts = max_duration;
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
            if fps > 0.0 {
                ctx.request_repaint();
            } else {
                ctx.request_repaint();
            }
        }
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
                    ui.painter()
                        .rect_filled(bg_rect, 4.0, egui::Color32::from_black_alpha(150));
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
                if ui
                    .button(egui::RichText::new(format!("   {ok}   ")).strong())
                    .clicked()
                {
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
        // Process deferred play/pause action so we don't run play_both/pause_both from inside ctx.input() (avoids re-entrancy/deadlock on macOS).
        match std::mem::take(&mut self.pending_transport_action) {
            PendingTransportAction::None => {}
            PendingTransportAction::Play => {
                if !self.playback.is_playing {
                    self.play_both(ctx);
                }
            }
            PendingTransportAction::Pause => {
                if self.playback.is_playing {
                    self.pause_both(ctx);
                }
            }
            PendingTransportAction::Toggle => {
                if self.playback.is_playing {
                    self.pause_both(ctx);
                } else {
                    self.play_both(ctx);
                }
            }
        }

        if let Some(play_time) = self.view.pending_play_after_delay {
            if std::time::Instant::now() >= play_time {
                self.view.pending_play_after_delay = None;
                if let Some(saved_loop) = self.view.saved_loop_playback.take() {
                    self.view.loop_playback = saved_loop;
                }
                self.play_both(ctx);
            } else {
                ctx.request_repaint(); // ensure we keep ticking to hit the delay exactly
            }
        }
        // Process deferred key action (arrows, Home, Y, L, Num1–9, R, S) so we never call ctx or decoder from inside ctx.input().
        match std::mem::take(&mut self.pending_key_action) {
            PendingKeyAction::None => {}
            PendingKeyAction::StepFwd => self.do_step_fwd_inner(ctx),
            PendingKeyAction::StepBck => self.do_step_bck_inner(ctx),
            PendingKeyAction::Seek(t) => self.do_seek_inner(t, ctx),
            PendingKeyAction::CycleMode => {
                self.view.mode = match self.view.mode {
                    CompareMode::SplitScreen => CompareMode::AbsDiff,
                    CompareMode::AbsDiff => CompareMode::Heatmap,
                    CompareMode::Heatmap => CompareMode::SideBySide,
                    CompareMode::SideBySide => CompareMode::SplitScreen,
                };
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
        self.drain_thumbnails(ctx);
        self.drain_audio_and_update_levels();
        self.apply_sink_volumes();

        // ── Handle screenshot events ────────────────────────────────────────
        let events = ctx.input(|i| i.raw.events.clone());
        // (Wgpu Screenshot event listener removed to use OS-native xcap instead)
        for _event in events {
            // Processing other events...
        }

        self.handle_keyboard_input(ctx);

        self.show_main_layout(ctx, frame, is_first_frame);

        self.show_clean_feed_viewport(ctx);
        self.show_proxy_progress_window(ctx);
        crate::ui::vu_meter::show_vu_meter_window(ctx, self);
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

fn default_rect() -> egui::Rect {
    egui::Rect::NOTHING
}
fn default_true() -> bool {
    true
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
    pub fn playback_mut(&mut self) -> &mut PlaybackState {
        &mut self.playback
    }
    pub fn playback(&self) -> &PlaybackState {
        &self.playback
    }
    pub fn calculate_psnr(&mut self) {
        if let (Some(dec_a), Some(dec_b)) = (&mut self.decoder_a, &mut self.decoder_b) {
            if let (Some(fa), Some(fb)) = (&dec_a.next_frame, &dec_b.next_frame) {
                if let Some(psnr) = crate::metrics::compute_psnr(&fa.rgba_data, &fb.rgba_data) {
                    self.view.last_psnr = Some(psnr);
                }
            }
        }
    }
    pub fn save_session(&mut self) {}
    pub fn load_session(&mut self, _ctx: &egui::Context) {}
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
        self.pending_transport_action = PendingTransportAction::Play;
    }
    pub fn do_pause(&mut self, _ctx: &egui::Context) {
        self.pending_transport_action = PendingTransportAction::Pause;
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
