// app.rs — Main eframe application

use crossbeam_channel::{Receiver, Sender};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::decoder;
use crate::renderer::{RenderCallback, ShaderUniforms, VideoRenderer};
use crate::types::{
    Channel, ColorMetadata, CompareMode, DiffMode, DecoderCommand, PlaybackState, VideoFrame, AudioFrame, Language,
};
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

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
                log::info!("No existing config.json found at {:?}, using defaults", path);
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
        let desk_dir = directories::UserDirs::new().and_then(|d| d.desktop_dir().map(|p| p.to_path_buf()));
        Self {
            mode: CompareMode::SplitScreen,
            diff_mode: DiffMode::AbsLinear,
            lang: Language::Es,
            theme: crate::types::Theme::Dark,
            show_hud: true,
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

    /// Shared with egui_wgpu::Callback
    renderer: Arc<Mutex<VideoRenderer>>,

    drag_start: Option<(egui::Pos2, f32, f32)>, // pos, pan_u, pan_v at drag start
    dragging_split: bool,

    /// Last known pointer position while OS files are being dragged over the window
    drag_drop_hover_pos: Option<egui::Pos2>,

    _audio_stream: Option<OutputStream>,
    _audio_handle: Option<OutputStreamHandle>,
    sink_a: Option<Sink>,
    sink_b: Option<Sink>,

    /// Error alert state
    error_title: Option<String>,
    error_message: Option<String>,

    /// Keyboard repeat state
    last_step_time: f64,
}

impl DiffPlayerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // --- Load Arial font if available on this OS -----------------------
        setup_fonts(&cc.egui_ctx);

        // --- Create the wgpu VideoRenderer ---------------------------------
        let render_state = match cc.wgpu_render_state.as_ref() {
            Some(rs) => rs,
            None => {
                log::error!("CRITICAL: eframe did not provide Wgpu render state. The app cannot continue.");
                panic!("Wgpu render state missing"); // Still panic but with explicit log before
            }
        };

        let target_format = render_state.target_format;
        let renderer = Arc::new(Mutex::new(VideoRenderer::new(
            &render_state.device,
            target_format,
        )));

        let (audio_stream, audio_handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(_) => (None, None),
        };
        let sink_a = audio_handle.as_ref().and_then(|h| Sink::try_new(h).ok());
        let sink_b = audio_handle.as_ref().and_then(|h| Sink::try_new(h).ok());
        if let Some(s) = &sink_a { s.set_volume(0.0); }
        if let Some(s) = &sink_b { s.set_volume(0.0); }

        log::info!("Loading ViewState...");
        let view = ViewState::load();
        log::info!("Applying theme...");
        crate::ui::theme::apply_theme(&cc.egui_ctx, view.theme);

        log::info!("App struct construction finished.");
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
            _audio_handle: audio_handle,
            sink_a,
            sink_b,
            error_title: None,
            error_message: None,
            last_step_time: 0.0,
        }
    }

    // -----------------------------------------------------------------------
    //  Open a video file for one channel
    // -----------------------------------------------------------------------
    fn open_video(&mut self, chan: Channel, ctx: &egui::Context) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Video", &["mp4", "mov", "mxf", "mkv", "avi", "prores", "mts", "mpg", "mpeg", "ts"])
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
                        if let Some(old) = self.decoder_a.take() {
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
    fn drain_frames(&mut self, render_state: &egui_wgpu::RenderState) -> bool {
        let device = &render_state.device;
        let queue  = &render_state.queue;

        let mut current_pts = self.playback.current_pts;
        let is_playing = self.playback.is_playing;
        let mut repainted = false;

        // Process frames for one decoder
        let mut process_dec = |dec: &mut DecoderHandle, is_a: bool| {
            // TURBO DRAINING: Consume all available frames in the channel
            let mut latest_frame = dec.next_frame.take();
            while let Ok(f) = dec.frame_rx.try_recv() {
                if !is_playing {
                    // PAUSED/STEPPING: Just get the absolute latest frame available
                    latest_frame = Some(f);
                } else {
                    // PLAYING: Only consume frames up to current time
                    if f.pts <= current_pts + 0.005 {
                        latest_frame = Some(f);
                    } else if f.pts > current_pts + 2.0 {
                        // CLOCK RESYNC: If the decoder is way ahead of the UI clock (e.g. fast PC or low FPS), 
                        // force the UI clock to jump forward to avoid a long "freeze".
                        current_pts = f.pts;
                        self.playback.current_pts = f.pts;
                        latest_frame = Some(f);
                    } else {
                        // Future frame: save it and stop draining
                        dec.next_frame = Some(f);
                        break;
                    }
                }
            }

            if let Some(frame) = latest_frame {
                // Determine if we should show this specific frame
                let time_to_show = if is_playing {
                    // When playing: show if frame is in the past or exactly now
                    frame.pts <= current_pts || (frame.pts - current_pts).abs() < 0.005
                } else {
                    // When paused: always show the LATEST frame from the channel
                    // to ensure flow even when holding the step button.
                    true
                };

                if time_to_show {
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
                    
                    if !is_playing {
                        current_pts = frame.pts;
                    }
                } else {
                    // Stale frame in play mode
                    if frame.pts < current_pts {
                        dec.next_frame = dec.frame_rx.try_recv().ok();
                    } else {
                        dec.next_frame = Some(frame);
                    }
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
            // When paused, current_pts should track the latest frame shown
            // from whichever decoder just produced a frame.
            self.playback.current_pts = current_pts;
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
        self.playback.is_playing = true;
        if let Some(s) = &self.sink_a { s.play(); }
        if let Some(s) = &self.sink_b { s.play(); }
        if let Some(dec) = &self.decoder_a {
            let _ = dec.cmd_tx.send(DecoderCommand::Play);
        }
        if let Some(dec) = &self.decoder_b {
            let _ = dec.cmd_tx.send(DecoderCommand::Play);
        }
        ctx.request_repaint();
    }

    fn pause_both(&mut self, ctx: &egui::Context) {
        self.playback.is_playing = false;
        if let Some(s) = &self.sink_a { s.pause(); }
        if let Some(s) = &self.sink_b { s.pause(); }
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

    fn step_back(&self, ctx: &egui::Context) {
        if let Some(dec) = &self.decoder_a {
            let _ = dec.cmd_tx.send(DecoderCommand::StepBack);
        }
        if let Some(dec) = &self.decoder_b {
            let _ = dec.cmd_tx.send(DecoderCommand::StepBack);
        }
        ctx.request_repaint();
    }
    pub fn swap_videos(&mut self, ctx: &egui::Context) {
        std::mem::swap(&mut self.decoder_a, &mut self.decoder_b);
        std::mem::swap(&mut self.playback.duration_a, &mut self.playback.duration_b);
        std::mem::swap(&mut self.view.mute_a, &mut self.view.mute_b);
        std::mem::swap(&mut self.view.vol_a, &mut self.view.vol_b);
        std::mem::swap(&mut self.sink_a, &mut self.sink_b);
        // Force rendering buffers to swap next frame
        if let Some(dec) = &mut self.decoder_a { dec.next_frame = dec.frame_rx.try_recv().ok(); }
        if let Some(dec) = &mut self.decoder_b { dec.next_frame = dec.frame_rx.try_recv().ok(); }
        ctx.request_repaint();
    }
}

impl eframe::App for DiffPlayerApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        log::debug!("App::update() called");
        if self.playback.is_playing {
            let dt = ctx.input(|i| i.stable_dt).min(0.1); 
            self.playback.current_pts += dt as f64;
            
            let max_duration = self.playback.duration_a.max(self.playback.duration_b);
            if max_duration > 0.0 && self.playback.current_pts >= max_duration {
                // Loop video when reaching the end
                self.do_seek(0.0, ctx);
            }

            ctx.request_repaint(); // Repaint continuously while playing
        }

        // Drain new frames and upload to GPU
        if let Some(rs) = frame.wgpu_render_state() {
            if self.drain_frames(rs) {
                ctx.request_repaint();
            }
        }

        // Sync uniforms from view state every frame
        self.sync_uniforms();

        // Drain Audio 
        if self.playback.is_playing {
            if let Some(dec) = &mut self.decoder_a {
                if let Some(sink) = &self.sink_a {
                    while let Ok(audio) = dec.audio_rx.try_recv() {
                        let buf = rodio::buffer::SamplesBuffer::new(audio.channels, audio.sample_rate, audio.samples);
                        sink.append(buf);
                    }
                }
            }
            if let Some(dec) = &mut self.decoder_b {
                if let Some(sink) = &self.sink_b {
                    while let Ok(audio) = dec.audio_rx.try_recv() {
                        let buf = rodio::buffer::SamplesBuffer::new(audio.channels, audio.sample_rate, audio.samples);
                        sink.append(buf);
                    }
                }
            }
        }

        // Update Volumes
        if let Some(sink) = &self.sink_a {
            if self.view.mute_a { sink.set_volume(0.0); } else { sink.set_volume(self.view.vol_a); }
        }
        if let Some(sink) = &self.sink_b {
            if self.view.mute_b { sink.set_volume(0.0); } else { sink.set_volume(self.view.vol_b); }
        }

        // ── Handle screenshot events ────────────────────────────────────────
        let events = ctx.input(|i| i.raw.events.clone());
        // (Wgpu Screenshot event listener removed to use OS-native xcap instead)
        for _event in events {
            // Processing other events...
        }

        // ── Keyboard shortcuts ──────────────────────────────────────────────
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                if self.playback.is_playing {
                    self.pause_both(ctx);
                } else {
                    self.play_both(ctx);
                }
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                self.do_step_fwd(ctx);
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.do_step_bck(ctx);
            }
            if i.key_pressed(egui::Key::Home) {
                self.do_seek(0.0, ctx);
            }
            if i.key_pressed(egui::Key::Y) {
                self.view.mode = match self.view.mode {
                    CompareMode::SplitScreen => CompareMode::AbsDiff,
                    CompareMode::AbsDiff => CompareMode::Heatmap,
                    CompareMode::Heatmap => CompareMode::SideBySide,
                    CompareMode::SideBySide => CompareMode::SplitScreen,
                };
                ctx.request_repaint();
            }
            if i.key_pressed(egui::Key::L) {
                self.view.mode = CompareMode::SideBySide;
                ctx.request_repaint();
            }
            if i.key_pressed(egui::Key::Num1) {
                self.view.mode = CompareMode::SplitScreen;
                self.view.split_pos = if self.view.split_pos < 0.05 { 0.5 } else { 0.0 };
                ctx.request_repaint();
            }
            if i.key_pressed(egui::Key::Num2) {
                self.view.mode = CompareMode::SplitScreen;
                self.view.split_pos = if self.view.split_pos > 0.95 { 0.5 } else { 1.0 };
                ctx.request_repaint();
            }
            if i.key_pressed(egui::Key::Num3) { self.view.show_hud = !self.view.show_hud; }
            if i.key_pressed(egui::Key::Num4) { self.view.zoom = 1.0; }
            if i.key_pressed(egui::Key::Num5) { self.view.zoom = 0.5; }
            if i.key_pressed(egui::Key::Num6) { self.view.zoom = 1.0; }
            if i.key_pressed(egui::Key::Num7) { self.view.zoom = 2.0; }
            if i.key_pressed(egui::Key::Num8) { self.view.zoom = 4.0; }
            if i.key_pressed(egui::Key::Num9) { self.view.zoom = 8.0; }
            if i.key_pressed(egui::Key::F) {
                log::info!("DEBUG: Key 'F' pressed. Triggering xcap OS-native capture.");
                let dir_for_thread = self.view.screenshot_dir.clone();
                
                std::thread::spawn(move || {
                    let mut success = false;
                    log::info!("DEBUG: xcap background thread scanning for OS Windows...");
                    if let Ok(windows) = xcap::Window::all() {
                        for window in windows {
                            if let Ok(title) = window.title() {
                                if title.contains("Production Media") || title.contains("Diferencial") {
                                    log::info!("DEBUG: Located OS Window -> {}", title);
                                    if let Ok(img_buf) = window.capture_image() {
                                        if let Some(dir) = dir_for_thread.as_ref() {
                                            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                                            let filename = format!("WPP_QC_{timestamp}.png");
                                            let path = dir.join(filename);
                                            log::info!("DEBUG: Writing OS-extracted PNG to {:?}", path);
                                            
                                            if let Err(e) = img_buf.save(&path) {
                                                log::error!("DEBUG: Disk write error: {}", e);
                                            } else {
                                                log::info!("DEBUG: Screenshot successfully saved!");
                                                success = true;
                                            }
                                        }
                                    } else {
                                        log::error!("DEBUG: xcap failed to read window buffer.");
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    if !success {
                        log::error!("DEBUG: xcap failed to locate or read the target WPP window.");
                    }
                });
            }
            if i.key_pressed(egui::Key::R) {
                self.view.zoom = 1.0;
                self.view.pan_u = 0.0;
                self.view.pan_v = 0.0;
            }
            if i.key_pressed(egui::Key::S) { self.swap_videos(ctx); }

            // ── Precise continuous frame stepping (Keyboard Repeat) ─────────
            let now = ctx.input(|i| i.time);
            let repeat_delay = 0.25; // 250ms initial delay
            let repeat_interval = 0.05; // 50ms interval (20 fps)

            if i.key_down(egui::Key::ArrowRight) {
                if i.key_pressed(egui::Key::ArrowRight) || (now - self.last_step_time) > repeat_interval {
                    if i.key_pressed(egui::Key::ArrowRight) || (now - self.last_step_time) > repeat_delay || self.last_step_time > 0.0 {
                         self.do_step_fwd(ctx);
                         self.last_step_time = now;
                    }
                }
            } else if i.key_down(egui::Key::ArrowLeft) {
                if i.key_pressed(egui::Key::ArrowLeft) || (now - self.last_step_time) > repeat_interval {
                    if i.key_pressed(egui::Key::ArrowLeft) || (now - self.last_step_time) > repeat_delay || self.last_step_time > 0.0 {
                        self.do_step_bck(ctx);
                        self.last_step_time = now;
                    }
                }
            } else {
                self.last_step_time = 0.0;
            }
        });

        // ── UI Overlay conditionally rendered ───────────────────────────────
        if self.view.show_hud {
            // ── Menu bar (contains all controls inline) ─────────────────────────
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                crate::ui::controls::show_menu_bar(ui, self);
            });

            // ── Info / metadata panel (left side) ──────────────────────────────
            egui::SidePanel::left("info_panel")
                .resizable(true)
                .default_width(260.0)
                .min_width(200.0)
                .max_width(340.0)
                .show(ctx, |ui| {
                    crate::ui::info_panel::show(ui, self);
                });
                
            // ── Audio controls (right side) ────────────────────────────────────
            egui::SidePanel::right("audio_panel")
                .resizable(false)
                .default_width(60.0)
                .show(ctx, |ui| {
                    crate::ui::controls::show_audio_panel(ui, self);
                });

            // ── Timeline (bottom) ───────────────────────────────────────────────
            egui::TopBottomPanel::bottom("timeline").show(ctx, |ui| {
                crate::ui::timeline::show(ui, self);
            });
        }

        // ── Central canvas ──────────────────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            show_canvas(ui, self, frame);
        });

        // ── Clean Feed Secondary Window (OBS Capture) ──────────────────────
        if self.view.show_clean_feed_window {
            let mut show = self.view.show_clean_feed_window;
            let renderer_clone = Arc::clone(&self.renderer);
            
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("clean_feed_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("DiffPlayerQC - Clean Feed")
                    .with_inner_size([1280.0, 720.0])
                    .with_always_on_top(),
                |ctx, _class| {
                    if ctx.input(|i| i.viewport().close_requested()) {
                        show = false;
                    }
                    
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let available = ui.available_rect_before_wrap();
                        ui.allocate_rect(available, egui::Sense::hover());
                        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                            available,
                            RenderCallback { renderer: renderer_clone },
                        ));
                        
                        // Clean Feed Overlay
                        let mode_str = match self.view.mode {
                            CompareMode::SplitScreen => {
                                if self.view.split_pos <= 0.01 { "Solo B / B Only" }
                                else if self.view.split_pos >= 0.99 { "Solo A / A Only" }
                                else { "Separador / Split" }
                            },
                            CompareMode::AbsDiff => "Diferencia / Diff",
                            CompareMode::Heatmap => "Mapa Calor / Heatmap",
                            CompareMode::SideBySide => "Lado a Lado / Side-by-Side",
                        };
                        
                        let video_str = match self.view.mode {
                            CompareMode::SplitScreen => {
                                if self.view.split_pos <= 0.01 { "VIDEO A" }
                                else if self.view.split_pos >= 0.99 { "VIDEO B" }
                                else { "VIDEO A + B" }
                            },
                            _ => "VIDEO A + B",
                        };

                        let pts = self.playback.current_pts;
                        // For a rough frame estimate we assume 24 fps, as we don't have global fps easily exposed without reaching decoder
                        let rough_frame = (pts * 24.0).round() as u64;
                        let overlay_text = format!("{} | {} | PTS: {:.3}s | Frame ~= {}", video_str, mode_str, pts, rough_frame);
                        
                        // We draw a faint background to ensure readability on bright scenes
                        let text_pos = available.min + egui::vec2(20.0, 20.0);
                        let galley = ui.painter().layout_no_wrap(
                            overlay_text,
                            egui::FontId::proportional(22.0),
                            egui::Color32::WHITE,
                        );
                        let bg_rect = galley.rect.translate(text_pos.to_vec2()).expand(6.0);
                        ui.painter().rect_filled(bg_rect, 4.0, egui::Color32::from_black_alpha(150));
                        ui.painter().galley(text_pos, galley, egui::Color32::WHITE);
                    });
                },
            );
            
            self.view.show_clean_feed_window = show;
        }
        // ── Error Alert Modal ───────────────────────────────────────────────
        if let (Some(title), Some(msg)) = (&self.error_title, &self.error_message) {
            let mut open = true;
            egui::Window::new(egui::RichText::new(title).color(egui::Color32::from_rgb(255, 100, 100)).strong())
                .collapsible(false)
                .resizable(false)
                .pivot(egui::Align2::CENTER_CENTER)
                .default_pos(ctx.screen_rect().center())
                .fixed_size(egui::vec2(400.0, 150.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new(msg).size(15.0));
                        ui.add_space(25.0);
                        if ui.button(egui::RichText::new("   OK   ").strong()).clicked() {
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

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.view.save();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        log::info!("Application exiting, triggering final save");
        self.view.save();
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
        let zoom_factor = if scroll_delta > 0.0 { 1.1f32 } else { 1.0 / 1.1 };
        app.view.zoom = (app.view.zoom * zoom_factor).clamp(0.25, 32.0);
    }

    // -- Drag to pan OR drag split line (Available in all modes) -------------
    // Pan is only active when zoomed in (zoom > 1.0). At fit-to-frame only the
    // split divider can be dragged.
    if response.drag_started() {
        let pos = response.interact_pointer_pos().unwrap_or_default();
        let split_x = available.left() + app.view.split_pos * available.width();
        
        if (pos.x - split_x).abs() < 15.0 {
            app.dragging_split = true;
        } else {
            app.dragging_split = false;
            // Only allow panning when zoomed in
            if app.view.zoom > 1.0 {
                app.drag_start = Some((pos, app.view.pan_u, app.view.pan_v));
            }
        }
    }

    if response.dragged() {
        if app.dragging_split {
            let pos = response.interact_pointer_pos().unwrap_or_default();
            let relative_x = (pos.x - available.left()) / available.width();
            app.view.split_pos = relative_x.clamp(0.0, 1.0);
            ui.ctx().request_repaint();
        } else if let Some((start_pos, start_pu, start_pv)) = app.drag_start {
            let delta = response.interact_pointer_pos().unwrap_or_default() - start_pos;
            let uv_delta_u = -delta.x / available.width()  / app.view.zoom;
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
        let split_x = available.left() + app.view.split_pos * available.width();
        if available.contains(ptr) && (ptr.x - split_x).abs() < 10.0 {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }
    }

    // -- Double-click to reset zoom -----------------------------------------
    if response.double_clicked() {
        app.view.zoom  = 1.0;
        app.view.pan_u = 0.0;
        app.view.pan_v = 0.0;
    }

    // -- Draw the wgpu render callback into this rect ----------------------
    let renderer_clone = Arc::clone(&app.renderer);
    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        available,
        RenderCallback { renderer: renderer_clone },
    ));

    // -- OS file drag-and-drop handling ------------------------------------
    let hovered_files = ui.ctx().input(|i| i.raw.hovered_files.clone());
    let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());

    // IMPORTANT: Handle the actual drop FIRST, before we potentially clear
    // drag_drop_hover_pos in the else branch below. On the drop frame,
    // hovered_files is already empty but drag_drop_hover_pos still holds
    // the last valid cursor position from the previous frame.
    if !dropped_files.is_empty() {
        let valid_extensions = ["mp4", "mov", "mxf", "mkv", "avi", "prores", "mts", "mpg", "mpeg", "ts"];
        
        let mut valid_paths = Vec::new();
        let mut invalid_files = Vec::new();

        for file in &dropped_files {
            if let Some(path) = &file.path {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if valid_extensions.contains(&ext.as_str()) {
                    valid_paths.push(path.to_string_lossy().to_string());
                } else {
                    invalid_files.push(path.file_name().unwrap_or_default().to_string_lossy().to_string());
                }
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
            app.error_message = Some("Solo puedes arrastrar un máximo de 2 videos a la vez.".to_string());
        } else if valid_paths.len() == 2 {
            valid_paths.sort(); // A goes to Slot A, B goes to Slot B alphabetically
            app.open_video_from_path(valid_paths[0].clone(), crate::types::Channel::A, ui.ctx());
            app.open_video_from_path(valid_paths[1].clone(), crate::types::Channel::B, ui.ctx());
        } else if !valid_paths.is_empty() {
            let mid_x = available.center().x;
            let hover_x = app.drag_drop_hover_pos
                .or_else(|| ui.ctx().pointer_hover_pos())
                .unwrap_or(available.center()).x;
            let target_chan = if hover_x < mid_x {
                crate::types::Channel::A
            } else {
                crate::types::Channel::B
            };
            app.open_video_from_path(valid_paths[0].clone(), target_chan, ui.ctx());
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

        let (a_alpha, b_alpha) = if targeting_a { (80u8, 30u8) } else { (30u8, 80u8) };

        let left_rect  = egui::Rect::from_min_max(available.min, egui::pos2(mid_x, available.max.y));
        let right_rect = egui::Rect::from_min_max(egui::pos2(mid_x, available.min.y), available.max);

        ui.painter().rect_filled(left_rect,  0.0, egui::Color32::from_rgba_premultiplied(80, 180, 100, a_alpha));
        ui.painter().rect_filled(right_rect, 0.0, egui::Color32::from_rgba_premultiplied(80, 130, 220, b_alpha));

        let is_es = app.view.lang == Language::Es;
        let label_a = if is_es { "Soltar aquí → VIDEO A" } else { "Drop here → VIDEO A" };
        let label_b = if is_es { "Soltar aquí → VIDEO B" } else { "Drop here → VIDEO B" };
        ui.painter().text(left_rect.center(),  egui::Align2::CENTER_CENTER, label_a,
            egui::FontId::proportional(22.0), egui::Color32::from_rgba_premultiplied(220, 255, 220, 230));
        ui.painter().text(right_rect.center(), egui::Align2::CENTER_CENTER, label_b,
            egui::FontId::proportional(22.0), egui::Color32::from_rgba_premultiplied(200, 220, 255, 230));
        ui.painter().vline(mid_x, available.y_range(),
            egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 120)));

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
            if is_es { "Abre el Vídeo A y el Vídeo B para empezar la comparación" } else { "Open Video A and Video B to begin comparison" }
        } else if !has_a {
            if is_es { "Abre el Vídeo A  ←  (panel izquierdo)" } else { "Open Video A  ←  (left panel)" }
        } else {
            if is_es { "Abre el Vídeo B  →  (panel izquierdo)" } else { "Open Video B  →  (left panel)" }
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

fn default_rect() -> egui::Rect { egui::Rect::NOTHING }

// ---------------------------------------------------------------------------
//  Font setup
// ---------------------------------------------------------------------------

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // List of common font paths for different OSes
    let font_paths = [
        "C:/Windows/Fonts/arial.ttf",                   // Windows
        "/Library/Fonts/Arial.ttf",                      // macOS
        "/System/Library/Fonts/Supplemental/Arial.ttf",  // macOS Supplemental
        "/Library/Fonts/Helvetica.ttc",                  // macOS Helvetica fallback
    ];

    for path in font_paths {
        if let Ok(bytes) = std::fs::read(path) {
            fonts.font_data.insert(
                "DefaultFont".to_owned(),
                egui::FontData::from_owned(bytes),
            );
            // Insert at the front of the proportional list
            fonts.families.entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "DefaultFont".to_owned());
            // Also use as monospace fallback
            fonts.families.entry(egui::FontFamily::Monospace)
                .or_default()
                .push("DefaultFont".to_owned());
            log::info!("Loaded font from: {:?}", path);
            break;
        }
    }

    ctx.set_fonts(fonts);

    // Apply overall style tweaks
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing  = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.slider_width  = 120.0;
    ctx.set_style(style);
}

// Expose app fields the UI modules need
impl DiffPlayerApp {
    pub fn view_mut(&mut self) -> &mut ViewState { &mut self.view }
    pub fn view(&self) -> &ViewState { &self.view }
    pub fn playback(&self) -> &PlaybackState { &self.playback }
    pub fn decoder_a_meta(&self) -> Option<&ColorMetadata> { self.decoder_a.as_ref().map(|d| &d.meta) }
    pub fn decoder_b_meta(&self) -> Option<&ColorMetadata> { self.decoder_b.as_ref().map(|d| &d.meta) }
    pub fn decoder_a_path(&self) -> Option<&str> { self.decoder_a.as_ref().map(|d| d.path.as_str()) }
    pub fn decoder_b_path(&self) -> Option<&str> { self.decoder_b.as_ref().map(|d| d.path.as_str()) }
    pub fn open_video_a(&mut self, ctx: &egui::Context) { self.open_video(Channel::A, ctx); }
    pub fn open_video_b(&mut self, ctx: &egui::Context) { self.open_video(Channel::B, ctx); }
    pub fn open_video_a_from_path(&mut self, path: String, ctx: &egui::Context) { self.open_video_from_path(path, Channel::A, ctx); }
    pub fn open_video_b_from_path(&mut self, path: String, ctx: &egui::Context) { self.open_video_from_path(path, Channel::B, ctx); }
    pub fn do_play(&mut self, ctx: &egui::Context)  { self.play_both(ctx); }
    pub fn do_pause(&mut self, ctx: &egui::Context) { self.pause_both(ctx); }
    pub fn do_step_fwd(&mut self, ctx: &egui::Context) { 
        if self.playback.is_playing {
            self.pause_both(ctx);
        }
        self.step_forward(ctx);
    }
    pub fn do_step_bck(&mut self, ctx: &egui::Context) { 
        if self.playback.is_playing {
            self.pause_both(ctx);
        }
        let fps = match (self.decoder_a_meta(), self.decoder_b_meta()) {
            (Some(a), _) if a.fps > 0.0 => a.fps,
            (_, Some(b)) if b.fps > 0.0 => b.fps,
            _ => 25.0,
        };
        let t = (self.playback.current_pts - 1.0 / fps).max(0.0);
        self.do_seek(t, ctx);
    }
    pub fn do_seek(&mut self, t: f64, ctx: &egui::Context) { 
        self.seek_both(t, ctx); 
        self.playback.current_pts = t; 
        
        // Clear audio sink buffers since we are jumping in time
        if let Some(s) = &self.sink_a { s.clear(); s.play(); } 
        if let Some(s) = &self.sink_b { s.clear(); s.play(); }
        if !self.playback.is_playing {
            if let Some(s) = &self.sink_a { s.pause(); }
            if let Some(s) = &self.sink_b { s.pause(); }
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
