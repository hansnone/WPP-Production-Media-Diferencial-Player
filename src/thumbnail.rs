//! Generación de miniaturas de vídeo en segundo plano para la timeline.
//!
//! Usa una apertura FFmpeg independiente del decoder de reproducción (misma
//! filosofía que `decoder.rs`, pero sin hilo continuo: abre, busca N puntos
//! temporales, decodifica 1 frame en cada uno, escala a baja resolución y
//! cierra). Pensado para ejecutarse una vez por vídeo cargado.

use ffmpeg_sys_next as ffi;
use std::ffi::CString;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crossbeam_channel::{Receiver, Sender};

pub const THUMB_WIDTH: u32 = 160;
pub const THUMB_HEIGHT: u32 = 90;
/// Número fijo de miniaturas por vídeo en Fase 1 (independiente del ancho de ventana).
pub const THUMB_COUNT_DEFAULT: usize = 120;

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

pub fn spawn_thumbnail_generator(
    path: String,
    duration_secs: f64,
    count: usize,
) -> (Receiver<ThumbnailFrame>, Arc<AtomicBool>) {
    let (tx, rx) = crossbeam_channel::unbounded::<ThumbnailFrame>();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    std::thread::Builder::new()
        .name(format!("thumbnails:{}", &path))
        .spawn(move || {
            if let Err(e) = generate_thumbnails(&path, duration_secs, count, &tx, &running_clone) {
                log::warn!("Thumbnail generation failed for '{}': {}", path, e);
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

fn open_video_only_decoder(path: &str) -> anyhow::Result<ThumbDecoderCtx> {
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
            anyhow::bail!("avformat_find_stream_info error: {}", crate::decoder::av_err(res));
        }

        let streams = std::slice::from_raw_parts((*fmt_ctx).streams, (*fmt_ctx).nb_streams as usize);
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
            anyhow::bail!("avcodec_parameters_to_context error: {}", crate::decoder::av_err(res));
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
            if !frame.is_null() { ffi::av_frame_free(&mut (frame.clone())); }
            if !frame_rgb.is_null() { ffi::av_frame_free(&mut (frame_rgb.clone())); }
            if !packet.is_null() { ffi::av_packet_free(&mut (packet.clone())); }
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

fn generate_thumbnails(
    path: &str,
    duration_secs: f64,
    count: usize,
    tx: &Sender<ThumbnailFrame>,
    running: &Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let ctx = open_video_only_decoder(path)?;
    let thumb_size = (THUMB_WIDTH * THUMB_HEIGHT * 4) as usize;

    for i in 0..count {
        if !running.load(Ordering::Relaxed) {
            break;
        }

        let target_secs = (i as f64 / count as f64) * duration_secs;
        let target_pts = crate::decoder::secs_to_pts(target_secs, ctx.time_base);

        unsafe {
            // Seek
            let res = ffi::av_seek_frame(ctx.fmt_ctx, ctx.video_stream_idx, target_pts, ffi::AVSEEK_FLAG_BACKWARD);
            if res < 0 {
                log::debug!("Thumbnail seek failed at {}, index {}", target_secs, i);
                continue;
            }
            ffi::avcodec_flush_buffers(ctx.codec_ctx);

            let mut frame_decoded = false;
            let mut decoded_pts = 0.0;

            // Read packets until we decode one frame
            while ffi::av_read_frame(ctx.fmt_ctx, ctx.packet) >= 0 {
                if !running.load(Ordering::Relaxed) {
                    ffi::av_packet_unref(ctx.packet);
                    return Ok(());
                }

                if (*ctx.packet).stream_index == ctx.video_stream_idx {
                    let mut ret = ffi::avcodec_send_packet(ctx.codec_ctx, ctx.packet);
                    if ret >= 0 {
                        ret = ffi::avcodec_receive_frame(ctx.codec_ctx, ctx.frame);
                        if ret >= 0 {
                            // Frame decoded!
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
                // Scale
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

                let tf = ThumbnailFrame {
                    index: i,
                    pts: decoded_pts,
                    rgba_data: data.into(),
                    width: THUMB_WIDTH,
                    height: THUMB_HEIGHT,
                };

                if tx.send(tf).is_err() {
                    // Receiver closed
                    break;
                }
            } else {
                log::debug!("No frame decoded for index {}", i);
            }
        }
    }

    Ok(())
}
