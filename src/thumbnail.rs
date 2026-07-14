//! Generación de miniaturas de vídeo en segundo plano para la timeline.
//!
//! Usa una apertura FFmpeg independiente del decoder de reproducción (misma
//! filosofía que `decoder.rs`, pero sin hilo continuo: abre, busca N puntos
//! temporales, decodifica 1 frame en cada uno, escala a baja resolución y
//! cierra). Pensado para ejecutarse una vez por vídeo cargado.

use crossbeam_channel::{Receiver, Sender};
use ffmpeg_sys_next as ffi;
use std::ffi::CString;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub const THUMB_WIDTH: u32 = 160;
pub const THUMB_HEIGHT: u32 = 90;
/// Niveles de densidad de miniaturas disponibles, de menor a mayor detalle.
/// Se generan de forma incremental: primero el más bajo (rápido, disponible
/// pronto), luego los siguientes en segundo plano sin bloquear al usuario.
pub const THUMB_LOD_LEVELS: &[usize] = &[40, 120, 300];

/// Ancho de celda mínimo deseable (px lógicos) antes de subir de nivel LOD.
/// Si cell_width < este valor, se solicita el siguiente nivel más denso.
pub const MIN_CELL_WIDTH_PX: f32 = 12.0;

/// Miniatura decodificada, lista para subir a una textura egui.
#[derive(Clone)]
pub struct ThumbnailFrame {
    /// Índice de la miniatura (0..total_requested).
    pub index: usize,
    /// PTS real en segundos del frame decodificado.
    pub pts: f64,
    pub rgba_data: std::sync::Arc<[u8]>,
    pub width: u32,
    pub height: u32,
}

pub struct LodLevel {
    pub count: usize,
    pub thumbs: Vec<Option<ThumbnailFrame>>,
    pub rx: Option<Receiver<ThumbnailFrame>>,
    pub running: Option<Arc<AtomicBool>>,
    /// true cuando el hilo generador ha terminado (running=false Y canal vacío/cerrado)
    pub complete: bool,
}

/// Gestiona la generación progresiva de varios niveles de densidad de
/// miniaturas para un mismo vídeo. Sustituye la generación única de Fase 1.
pub struct ThumbnailLodManager {
    path: String,
    duration_secs: f64,
    /// Miniaturas ya generadas, por nivel de LOD. Índice 0 = THUMB_LOD_LEVELS[0], etc.
    levels: Vec<LodLevel>,
    /// Nivel actualmente en generación (None si no hay ninguno en curso).
    generating_level_idx: Option<usize>,
    /// Nivel especial para vista con zoom: solo se usa cuando
    /// `TimelineViewport` no cubre el rango completo.
    zoom_level: Option<ZoomLevel>,
}

pub struct ZoomLevel {
    pub range_start: f64,
    pub range_end: f64,
    pub count: usize,
    pub thumbs: Vec<Option<ThumbnailFrame>>,
    pub rx: Option<Receiver<ThumbnailFrame>>,
    pub running: Option<Arc<AtomicBool>>,
}

/// Resultado de qué conjunto de miniaturas usar, junto con el rango temporal
/// que representan (necesario para el mapeo UV→tiempo en la UI).
pub enum ThumbSource<'a> {
    Global {
        level_idx: usize,
        thumbs: &'a [Option<ThumbnailFrame>],
    },
    Zoom {
        range_start: f64,
        range_end: f64,
        thumbs: &'a [Option<ThumbnailFrame>],
    },
}

impl ThumbnailLodManager {
    /// Umbral de cambio de rango (en fracción del span actual) que dispara
    /// una regeneración del nivel de zoom. Evita relanzar el hilo en cada
    /// pixel de arrastre/scroll.
    const ZOOM_REFRESH_THRESHOLD: f64 = 0.15;
    /// Crea el manager y arranca la generación del primer nivel (el más bajo,
    /// para tener algo visible cuanto antes).
    pub fn new(path: String, duration_secs: f64) -> Self {
        let levels: Vec<LodLevel> = THUMB_LOD_LEVELS
            .iter()
            .map(|&count| LodLevel {
                count,
                thumbs: vec![None; count],
                rx: None,
                running: None,
                complete: false,
            })
            .collect();

        let mut mgr = Self {
            path,
            duration_secs,
            levels,
            generating_level_idx: None,
            zoom_level: None,
        };
        mgr.start_level(0);
        mgr
    }

    fn start_level(&mut self, level_idx: usize) {
        if level_idx >= self.levels.len() {
            return;
        }
        let count = self.levels[level_idx].count;
        let (rx, running) = spawn_thumbnail_generator(self.path.clone(), self.duration_secs, count);
        self.levels[level_idx].rx = Some(rx);
        self.levels[level_idx].running = Some(running);
        self.generating_level_idx = Some(level_idx);
    }

    /// Llamar cada frame desde `DiffPlayerApp::update()`. Drena miniaturas
    /// pendientes del nivel en generación y avanza al siguiente nivel cuando
    /// el actual termina.
    /// Devuelve `true` si se subió alguna miniatura nueva (para request_repaint).
    pub fn drain_and_advance(
        &mut self,
        on_new_frame: &mut dyn FnMut(usize, &ThumbnailFrame),
    ) -> bool {
        let mut any_new = self.drain_global_levels(on_new_frame);

        if let Some(z) = &mut self.zoom_level {
            if let Some(rx) = &z.rx {
                for tf in rx.try_iter() {
                    if tf.index < z.thumbs.len() {
                        z.thumbs[tf.index] = Some(tf.clone());
                        any_new = true;
                    }
                }
            }
            let finished = z
                .running
                .as_ref()
                .map(|r| !r.load(Ordering::Relaxed))
                .unwrap_or(true);
            if finished {
                z.rx = None;
                z.running = None;
            }
        }
        any_new
    }

    fn drain_global_levels(
        &mut self,
        on_new_frame: &mut dyn FnMut(usize, &ThumbnailFrame),
    ) -> bool {
        let mut any_new = false;
        if let Some(idx) = self.generating_level_idx {
            let level = &mut self.levels[idx];
            if let Some(rx) = &level.rx {
                for tf in rx.try_iter() {
                    if tf.index < level.thumbs.len() {
                        level.thumbs[tf.index] = Some(tf.clone());
                        on_new_frame(idx, &tf);
                        any_new = true;
                    }
                }
            }
            let finished = level
                .running
                .as_ref()
                .map(|r| !r.load(Ordering::Relaxed))
                .unwrap_or(true);
            if finished {
                level.complete = true;
                level.rx = None;
                level.running = None;
                self.generating_level_idx = None;
            }
        }
        any_new
    }

    pub fn has_any_global_thumbnails(&self) -> bool {
        self.levels
            .iter()
            .any(|level| level.complete && !level.thumbs.is_empty())
    }

    pub fn ensure_initial_level_started(&mut self) {
        if self.levels.is_empty() {
            return;
        }

        let has_complete = self
            .levels
            .iter()
            .any(|level| level.complete && !level.thumbs.is_empty());

        let has_running = self.levels.iter().any(|level| level.rx.is_some());

        if !has_complete && !has_running {
            self.start_level(0);
        }
    }

    /// Solicita generar el siguiente nivel de detalle si aún no se ha
    /// arrancado y no hay ya uno en curso. Llamado desde la UI cuando detecta
    /// que el ancho disponible necesita más densidad.
    pub fn request_next_level(&mut self, playback_active: bool) {
        let already_has_some_thumbs = self.has_any_global_thumbnails();

        // Durante reproducción no subir a LOD más pesado,
        // pero sí permitir el primer nivel si aún no hay nada.
        if playback_active && already_has_some_thumbs {
            return;
        }

        if self.generating_level_idx.is_some() {
            return; // ya generando algo, no solapar hilos
        }
        if let Some(next_idx) = self
            .levels
            .iter()
            .position(|l| !l.complete && l.rx.is_none())
        {
            self.start_level(next_idx);
        }
    }

    /// Devuelve el mejor nivel disponible (el más denso ya completo o en
    /// generación con datos parciales) para un ancho de celda mínimo dado.
    pub fn best_level_for_width(&self, available_width: f32) -> (usize, &[Option<ThumbnailFrame>]) {
        for (i, level) in self.levels.iter().enumerate().rev() {
            let has_data = level.thumbs.iter().any(|t| t.is_some());
            if !has_data {
                continue;
            }
            let cell_w = available_width / level.count as f32;
            if cell_w >= MIN_CELL_WIDTH_PX || i == 0 {
                return (i, &level.thumbs);
            }
        }
        (0, &self.levels[0].thumbs)
    }

    pub fn duration_secs(&self) -> f64 {
        self.duration_secs
    }

    pub fn request_zoom_range_checked(
        &mut self,
        view_start: f64,
        view_end: f64,
        target_count: usize,
    ) -> bool {
        let needs_refresh = match &self.zoom_level {
            None => true,
            Some(z) => {
                let old_span = (z.range_end - z.range_start).max(0.001);
                let new_span = (view_end - view_start).max(0.001);
                let start_delta = (view_start - z.range_start).abs() / old_span;
                let span_ratio = (new_span / old_span - 1.0).abs();
                start_delta > Self::ZOOM_REFRESH_THRESHOLD
                    || span_ratio > Self::ZOOM_REFRESH_THRESHOLD
            }
        };

        if !needs_refresh {
            return false;
        }

        if let Some(z) = self.zoom_level.take() {
            if let Some(running) = z.running {
                running.store(false, Ordering::Relaxed);
            }
        }

        let (rx, running) =
            spawn_thumbnail_generator_range(self.path.clone(), view_start, view_end, target_count);
        self.zoom_level = Some(ZoomLevel {
            range_start: view_start,
            range_end: view_end,
            count: target_count,
            thumbs: vec![None; target_count],
            rx: Some(rx),
            running: Some(running),
        });
        true
    }

    pub fn thumbs_for_viewport(
        &self,
        view_start: f64,
        view_end: f64,
        available_width: f32,
    ) -> ThumbSource<'_> {
        let is_zoomed = (view_end - view_start) < self.duration_secs * 0.98;
        if is_zoomed {
            if let Some(z) = &self.zoom_level {
                return ThumbSource::Zoom {
                    range_start: z.range_start,
                    range_end: z.range_end,
                    thumbs: &z.thumbs,
                };
            }
        }
        let (level_idx, thumbs) = self.best_level_for_width(available_width);
        ThumbSource::Global { level_idx, thumbs }
    }

    pub fn cancel_all(&mut self) {
        for level in &mut self.levels {
            if let Some(running) = level.running.take() {
                running.store(false, Ordering::Relaxed);
            }
            level.rx = None;
        }
        if let Some(z) = self.zoom_level.take() {
            if let Some(running) = z.running {
                running.store(false, Ordering::Relaxed);
            }
        }
        self.generating_level_idx = None;
    }
}

pub fn spawn_thumbnail_generator(
    path: String,
    duration_secs: f64,
    count: usize,
) -> (Receiver<ThumbnailFrame>, Arc<AtomicBool>) {
    spawn_thumbnail_generator_range(path, 0.0, duration_secs, count)
}

pub fn spawn_thumbnail_generator_range(
    path: String,
    range_start_secs: f64,
    range_end_secs: f64,
    count: usize,
) -> (Receiver<ThumbnailFrame>, Arc<AtomicBool>) {
    let (tx, rx) = crossbeam_channel::unbounded::<ThumbnailFrame>();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    std::thread::Builder::new()
        .name(format!("thumbnails_range:{}", &path))
        .spawn(move || {
            if let Err(e) = generate_thumbnails_range(
                &path,
                range_start_secs,
                range_end_secs,
                count,
                &tx,
                &running_clone,
            ) {
                log::warn!("Range thumbnail generation failed for '{}': {:#}", path, e);
            }
            running_clone.store(false, Ordering::Relaxed);
        })
        .ok();

    (rx, running)
}

struct ThumbDecoderCtx {
    fmt_ctx: *mut ffi::AVFormatContext,
    video_stream_idx: i32,
    codec_ctx: *mut ffi::AVCodecContext,
    sws_ctx: *mut ffi::SwsContext,
    frame: *mut ffi::AVFrame,
    frame_rgb: *mut ffi::AVFrame,
    packet: *mut ffi::AVPacket,
    time_base: ffi::AVRational,
}

impl Drop for ThumbDecoderCtx {
    fn drop(&mut self) {
        unsafe {
            if !self.sws_ctx.is_null() {
                ffi::sws_freeContext(self.sws_ctx);
            }
            if !self.frame_rgb.is_null() {
                ffi::av_frame_free(&mut self.frame_rgb);
            }
            if !self.frame.is_null() {
                ffi::av_frame_free(&mut self.frame);
            }
            if !self.packet.is_null() {
                ffi::av_packet_free(&mut self.packet);
            }
            if !self.codec_ctx.is_null() {
                ffi::avcodec_free_context(&mut self.codec_ctx);
            }
            if !self.fmt_ctx.is_null() {
                ffi::avformat_close_input(&mut self.fmt_ctx);
            }
        }
    }
}

fn open_thumb_decoder(path: &str) -> anyhow::Result<ThumbDecoderCtx> {
    unsafe {
        let mut fmt_ctx: *mut ffi::AVFormatContext = ptr::null_mut();
        let c_path = CString::new(path)?;

        let mut res = ffi::avformat_open_input(
            &mut fmt_ctx,
            c_path.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if res < 0 {
            anyhow::bail!("avformat_open_input error: {}", crate::decoder::av_err(res));
        }

        res = ffi::avformat_find_stream_info(fmt_ctx, ptr::null_mut());
        if res < 0 {
            ffi::avformat_close_input(&mut fmt_ctx);
            anyhow::bail!(
                "avformat_find_stream_info error: {}",
                crate::decoder::av_err(res)
            );
        }

        let streams =
            std::slice::from_raw_parts((*fmt_ctx).streams, (*fmt_ctx).nb_streams as usize);
        let v_idx = crate::decoder::find_video_stream(streams);
        if v_idx < 0 {
            ffi::avformat_close_input(&mut fmt_ctx);
            anyhow::bail!("No video stream found");
        }

        let v_stream = streams[v_idx as usize];
        let codec_par = (*v_stream).codecpar;
        let codec = ffi::avcodec_find_decoder((*codec_par).codec_id);
        if codec.is_null() {
            ffi::avformat_close_input(&mut fmt_ctx);
            anyhow::bail!("Video codec not found");
        }

        let mut codec_ctx = ffi::avcodec_alloc_context3(codec);
        if codec_ctx.is_null() {
            ffi::avformat_close_input(&mut fmt_ctx);
            anyhow::bail!("avcodec_alloc_context3 failed");
        }

        res = ffi::avcodec_parameters_to_context(codec_ctx, codec_par);
        if res < 0 {
            ffi::avcodec_free_context(&mut codec_ctx);
            ffi::avformat_close_input(&mut fmt_ctx);
            anyhow::bail!(
                "avcodec_parameters_to_context error: {}",
                crate::decoder::av_err(res)
            );
        }

        res = ffi::avcodec_open2(codec_ctx, codec, ptr::null_mut());
        if res < 0 {
            ffi::avcodec_free_context(&mut codec_ctx);
            ffi::avformat_close_input(&mut fmt_ctx);
            anyhow::bail!("avcodec_open2 error: {}", crate::decoder::av_err(res));
        }

        let frame = ffi::av_frame_alloc();
        let frame_rgb = ffi::av_frame_alloc();
        let packet = ffi::av_packet_alloc();

        if frame.is_null() || frame_rgb.is_null() || packet.is_null() {
            if !frame.is_null() {
                ffi::av_frame_free(&mut (frame.clone()));
            }
            if !frame_rgb.is_null() {
                ffi::av_frame_free(&mut (frame_rgb.clone()));
            }
            if !packet.is_null() {
                ffi::av_packet_free(&mut (packet.clone()));
            }
            ffi::avcodec_free_context(&mut codec_ctx);
            ffi::avformat_close_input(&mut fmt_ctx);
            anyhow::bail!("Failed to allocate frames/packet");
        }

        // sws_getContext to scale directly to THUMB_WIDTH x THUMB_HEIGHT
        let sws_ctx = ffi::sws_getContext(
            (*codec_ctx).width,
            (*codec_ctx).height,
            (*codec_ctx).pix_fmt,
            THUMB_WIDTH as i32,
            THUMB_HEIGHT as i32,
            ffi::AVPixelFormat::AV_PIX_FMT_RGBA,
            ffi::SWS_FAST_BILINEAR as i32,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null(),
        );

        if sws_ctx.is_null() {
            ffi::av_frame_free(&mut (frame.clone()));
            ffi::av_frame_free(&mut (frame_rgb.clone()));
            ffi::av_packet_free(&mut (packet.clone()));
            ffi::avcodec_free_context(&mut codec_ctx);
            ffi::avformat_close_input(&mut fmt_ctx);
            anyhow::bail!("sws_getContext failed");
        }

        Ok(ThumbDecoderCtx {
            fmt_ctx,
            video_stream_idx: v_idx,
            codec_ctx,
            sws_ctx,
            frame,
            frame_rgb,
            packet,
            time_base: (*v_stream).time_base,
        })
    }
}

fn decode_thumb_at(
    ctx: &mut ThumbDecoderCtx,
    target_pts: i64,
    running: &Arc<AtomicBool>,
) -> anyhow::Result<Option<(f64, Vec<u8>, u32, u32)>> {
    unsafe {
        let res = ffi::av_seek_frame(
            ctx.fmt_ctx,
            ctx.video_stream_idx,
            target_pts,
            ffi::AVSEEK_FLAG_BACKWARD,
        );
        if res < 0 {
            return Ok(None);
        }
        ffi::avcodec_flush_buffers(ctx.codec_ctx);

        let mut frame_decoded = false;
        let mut decoded_pts = 0.0;

        while ffi::av_read_frame(ctx.fmt_ctx, ctx.packet) >= 0 {
            if !running.load(Ordering::Relaxed) {
                ffi::av_packet_unref(ctx.packet);
                return Ok(None);
            }

            if (*ctx.packet).stream_index == ctx.video_stream_idx {
                let mut ret = ffi::avcodec_send_packet(ctx.codec_ctx, ctx.packet);
                if ret >= 0 {
                    ret = ffi::avcodec_receive_frame(ctx.codec_ctx, ctx.frame);
                    if ret >= 0 {
                        frame_decoded = true;
                        decoded_pts = crate::decoder::pts_to_secs((*ctx.frame).pts, ctx.time_base);
                        ffi::av_packet_unref(ctx.packet);
                        break;
                    }
                }
            }
            ffi::av_packet_unref(ctx.packet);
        }

        if frame_decoded {
            let thumb_size = (THUMB_WIDTH * THUMB_HEIGHT * 4) as usize;
            let mut data = vec![0u8; thumb_size];
            let mut linesize = [0i32; 8];
            linesize[0] = (THUMB_WIDTH * 4) as i32;
            let mut data_ptrs = [ptr::null_mut::<u8>(); 8];
            data_ptrs[0] = data.as_mut_ptr();

            ffi::sws_scale(
                ctx.sws_ctx,
                (*ctx.frame).data.as_ptr() as *const *const u8,
                (*ctx.frame).linesize.as_ptr() as *const i32,
                0,
                (*ctx.codec_ctx).height,
                data_ptrs.as_mut_ptr(),
                linesize.as_mut_ptr(),
            );

            return Ok(Some((decoded_pts, data, THUMB_WIDTH, THUMB_HEIGHT)));
        }
        Ok(None)
    }
}

fn generate_thumbnails_range(
    path: &str,
    range_start: f64,
    range_end: f64,
    count: usize,
    tx: &Sender<ThumbnailFrame>,
    running: &Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let mut ctx = open_thumb_decoder(path)?;
    let span = (range_end - range_start).max(0.001);

    for i in 0..count {
        if !running.load(Ordering::Relaxed) {
            break;
        }
        let safe_span = (span - 0.001).max(0.0);
        let target_secs = if count <= 1 {
            range_start + safe_span * 0.5
        } else {
            range_start + safe_span * i as f64 / (count - 1) as f64
        };
        let target_pts = crate::decoder::secs_to_pts(target_secs, ctx.time_base);

        match decode_thumb_at(&mut ctx, target_pts, running) {
            Ok(Some((pts_secs, rgba, w, h))) => {
                let frame = ThumbnailFrame {
                    index: i,
                    pts: pts_secs,
                    rgba_data: rgba.into(),
                    width: w,
                    height: h,
                };
                if tx.send(frame).is_err() {
                    break;
                }
            }
            Ok(None) => log::debug!(
                "No frame decoded at {:.2}s for thumbnail {}",
                target_secs,
                i
            ),
            Err(e) => log::debug!("Thumbnail decode error at {:.2}s: {:#}", target_secs, e),
        }
    }
    Ok(())
}
