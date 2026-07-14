//! Decodificación de vídeo y audio en un hilo dedicado (API C de FFmpeg vía `ffmpeg-sys-next`).
//!
//! El bucle recibe [`DecoderCommand`](crate::types::DecoderCommand), emite [`VideoFrame`](crate::types::VideoFrame)
//! (YUV→RGBA con libswscale, ver `convert_frame`) y [`AudioFrame`](crate::types::AudioFrame) vía `swr`.
//! El hilo de UI **no** debe bloquearse en estas operaciones.

use anyhow::{anyhow, Context, Result};
use crossbeam_channel::{Receiver, Sender};
use ebur128::{EbuR128, Mode};
use parking_lot::Mutex;
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Arc;

use ffmpeg_sys_next as ffi;

/// `SWS_FAST_BILINEAR` (libswscale): más barato que `SWS_BILINEAR` para QC en tiempo real.
const SWS_FAST_BILINEAR: i32 = 1 << 2;

use crate::trace_log;
use crate::types::{AudioFrame, ColorMetadata, DecoderCommand, VideoFrame};

/// Spawn a decoder thread for the given file path.
pub fn spawn_decoder(
    path: &str,
    target_sample_rate: i32,
    target_channels: i32,
) -> Result<(
    Sender<DecoderCommand>,
    Receiver<VideoFrame>,
    Receiver<AudioFrame>,
    ColorMetadata,
    Arc<Mutex<crate::types::LoudnessResult>>,
)> {
    // Initialise FFmpeg (safe to call multiple times)
    unsafe {
        ffi::av_log_set_level(ffi::AV_LOG_ERROR);
    }

    let path_owned = path.to_owned();

    // Extract metadata synchronously before spawning thread
    let meta = extract_metadata(&path_owned)?;

    let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded::<DecoderCommand>();
    let (frame_tx, frame_rx) = crossbeam_channel::bounded::<VideoFrame>(8);
    let (audio_tx, audio_rx) = crossbeam_channel::bounded::<AudioFrame>(256);
    let (ebu_tx, ebu_rx) = crossbeam_channel::unbounded::<AudioFrame>();

    let loudness_arc = Arc::new(Mutex::new(crate::types::LoudnessResult::default()));
    let ebu_loudness_arc = Arc::clone(&loudness_arc);

    std::thread::Builder::new()
        .name(format!("ebu:{}", &path_owned))
        .spawn(move || {
            let mut ebu: Option<EbuR128> = None;
            let mut ebu_channels = 0;
            let mut ebu_rate = 0;

            while let Ok(frame) = ebu_rx.recv() {
                let channels = frame.channels as u32;
                let sample_rate = frame.sample_rate as u32;
                let ebu_target_channels = if channels == 1 { 2 } else { channels };

                if ebu.is_none() || ebu_channels != ebu_target_channels || ebu_rate != sample_rate {
                    if let Ok(new_ebu) = EbuR128::new(
                        ebu_target_channels,
                        sample_rate,
                        Mode::I | Mode::M | Mode::S | Mode::TRUE_PEAK,
                    ) {
                        ebu = Some(new_ebu);
                        ebu_channels = ebu_target_channels;
                        ebu_rate = sample_rate;
                    }
                }

                if let Some(ebu) = ebu.as_mut() {
                    if channels == 1 {
                        let mut stereo = Vec::with_capacity(frame.samples.len() * 2);
                        for &s in &frame.samples {
                            stereo.push(s);
                            stereo.push(s);
                        }
                        let _ = ebu.add_frames_f32(&stereo);
                    } else {
                        let _ = ebu.add_frames_f32(&frame.samples);
                    }

                    if let Ok(m) = ebu.loudness_momentary() {
                        let mut l = ebu_loudness_arc.lock();
                        l.momentary = m;
                        l.short_term = ebu.loudness_shortterm().unwrap_or(-120.0);
                        l.integrated = ebu.loudness_global().unwrap_or(-120.0);
                        l.true_peak[0] = l.true_peak[0].max(ebu.prev_true_peak(0).unwrap_or(0.0));
                        if ebu_channels > 1 {
                            l.true_peak[1] =
                                l.true_peak[1].max(ebu.prev_true_peak(1).unwrap_or(0.0));
                        }
                    }
                }
            }
        })?;

    std::thread::Builder::new()
        .name(format!("decoder:{}", &path_owned))
        .spawn(move || {
            if let Err(e) = decoder_loop(
                &path_owned,
                target_sample_rate,
                target_channels,
                cmd_rx,
                frame_tx,
                audio_tx,
                ebu_tx,
            ) {
                log::error!("Decoder thread error: {e:#}");
            }
        })?;

    Ok((cmd_tx, frame_rx, audio_rx, meta, loudness_arc))
}

// ---------------------------------------------------------------------------
// Metadata extraction helpers (AVDictionary)
// ---------------------------------------------------------------------------

/// Read a single metadata value by key from an AVDictionary. Returns empty string if not found or null.
unsafe fn dict_get(m: *const ffi::AVDictionary, key: &str) -> String {
    if m.is_null() {
        return String::new();
    }
    let c_key = match CString::new(key) {
        Ok(k) => k,
        Err(_) => return String::new(),
    };
    let entry = ffi::av_dict_get(m, c_key.as_ptr(), ptr::null_mut(), 0);
    if entry.is_null() {
        return String::new();
    }
    let val = (*entry).value;
    if val.is_null() {
        String::new()
    } else {
        CStr::from_ptr(val).to_string_lossy().into_owned()
    }
}

/// Iterate all AVDictionary entries and format as "key: value\n" lines.
unsafe fn dict_to_string(m: *const ffi::AVDictionary) -> String {
    if m.is_null() {
        return String::new();
    }
    let mut out = String::new();
    let mut prev = ptr::null_mut::<ffi::AVDictionaryEntry>();
    loop {
        let entry = ffi::av_dict_get(m, ptr::null(), prev, 0);
        if entry.is_null() {
            break;
        }
        let key: String = if (*entry).key.is_null() {
            String::new()
        } else {
            CStr::from_ptr((*entry).key).to_string_lossy().into_owned()
        };
        let val: String = if (*entry).value.is_null() {
            String::new()
        } else {
            CStr::from_ptr((*entry).value)
                .to_string_lossy()
                .into_owned()
        };
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(&key);
        out.push_str(": ");
        out.push_str(&val);
        prev = entry;
    }
    out
}

// ---------------------------------------------------------------------------
// Metadata extraction (open file, read stream headers, close)
// ---------------------------------------------------------------------------

fn extract_metadata(path: &str) -> Result<ColorMetadata> {
    let c_path = CString::new(path).context("invalid path")?;
    unsafe {
        let mut fmt_ctx: *mut ffi::AVFormatContext = ptr::null_mut();

        let ret =
            ffi::avformat_open_input(&mut fmt_ctx, c_path.as_ptr(), ptr::null(), ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("avformat_open_input: {}", av_err(ret)));
        }

        let ret = ffi::avformat_find_stream_info(fmt_ctx, ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("avformat_find_stream_info: {}", av_err(ret)));
        }

        let nb = (*fmt_ctx).nb_streams as usize;
        let streams = std::slice::from_raw_parts((*fmt_ctx).streams, nb);

        let video_idx = find_video_stream(streams);
        if video_idx < 0 {
            return Err(anyhow!("no video stream in '{path}'"));
        }

        let stream = *streams[video_idx as usize];
        let par = *stream.codecpar;

        let fps = {
            let r = stream.avg_frame_rate;
            if r.den == 0 {
                0.0
            } else {
                r.num as f64 / r.den as f64
            }
        };

        let duration_secs = if stream.duration > 0 {
            let tb = stream.time_base;
            stream.duration as f64 * tb.num as f64 / tb.den as f64
        } else {
            (*fmt_ctx).duration as f64 / ffi::AV_TIME_BASE as f64
        };

        let colorspace = color_space_str(par.color_space);
        let color_transfer = color_trc_str(par.color_trc);
        let color_primaries = color_primaries_str(par.color_primaries);

        // Pixel format name
        let pix_name = ffi::av_get_pix_fmt_name(std::mem::transmute(par.format));
        let pixel_format = if pix_name.is_null() {
            "unknown".to_owned()
        } else {
            CStr::from_ptr(pix_name).to_string_lossy().into_owned()
        };

        // Video codec name
        let video_codec = {
            let name_ptr = ffi::avcodec_get_name(par.codec_id);
            if name_ptr.is_null() {
                "unknown".to_owned()
            } else {
                CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
            }
        };

        // Audio codec name (if present)
        let audio_codec = {
            let a_idx = find_audio_stream(streams);
            if a_idx >= 0 {
                let a_stream = *streams[a_idx as usize];
                let a_par = *a_stream.codecpar;
                let name_ptr = ffi::avcodec_get_name(a_par.codec_id);
                if name_ptr.is_null() {
                    "—".to_owned()
                } else {
                    CStr::from_ptr(name_ptr).to_string_lossy().into_owned()
                }
            } else {
                "—".to_owned()
            }
        };

        // Format-level metadata: major_brand
        let major_brand = {
            let s = dict_get((*fmt_ctx).metadata, "major_brand");
            if s.is_empty() {
                "—".to_owned()
            } else {
                s
            }
        };

        // Video stream metadata (Stream #0:0)
        let video_stream_metadata = dict_to_string(stream.metadata);

        // Timecode extraction
        let mut start_timecode = dict_get(stream.metadata, "timecode");
        if start_timecode.is_empty() {
            start_timecode = dict_get((*fmt_ctx).metadata, "timecode");
        }
        let start_timecode_opt = if start_timecode.is_empty() {
            None
        } else {
            Some(start_timecode)
        };

        // Audio stream metadata (Stream #0:1) if present
        let audio_stream_metadata = {
            let a_idx = find_audio_stream(streams);
            if a_idx >= 0 {
                let a_stream = *streams[a_idx as usize];
                let s = dict_to_string(a_stream.metadata);
                if s.is_empty() {
                    "—".to_owned()
                } else {
                    s
                }
            } else {
                "—".to_owned()
            }
        };

        let meta = ColorMetadata {
            colorspace,
            color_transfer,
            color_primaries,
            pixel_format,
            width: par.width as u32,
            height: par.height as u32,
            fps,
            duration_secs,
            bitrate_kbps: (*fmt_ctx).bit_rate / 1000,
            video_codec,
            audio_codec,
            major_brand,
            start_timecode: start_timecode_opt,
            video_stream_metadata,
            audio_stream_metadata,
        };

        ffi::avformat_close_input(&mut fmt_ctx);
        Ok(meta)
    }
}

// ---------------------------------------------------------------------------
// Decoder loop — runs on its own thread
// ---------------------------------------------------------------------------

struct DecoderCtx {
    fmt_ctx: *mut ffi::AVFormatContext,
    codec_ctx: *mut ffi::AVCodecContext,
    sws_ctx: *mut ffi::SwsContext,
    /// Búfer RGBA reutilizable para `sws_scale` (evita `av_frame_alloc` por fotograma).
    rgba_scratch: *mut ffi::AVFrame,
    stream_idx: i32,
    time_base: ffi::AVRational,
    width: u32,
    height: u32,
    fps: f64,

    audio_stream_idx: i32,
    audio_codec_ctx: *mut ffi::AVCodecContext,
    swr_ctx: *mut ffi::SwrContext,
    /// Reservado para PTS de audio alineado al stream; hoy el PCM va a rodio sin marca temporal aquí.
    _audio_time_base: ffi::AVRational,
    target_sample_rate: i32,
    target_channels: i32,
    audio_scratch: *mut u8,
    audio_scratch_cap: i32,
    ebu_tx: Sender<AudioFrame>,
}

impl Drop for DecoderCtx {
    fn drop(&mut self) {
        unsafe {
            if !self.codec_ctx.is_null() {
                ffi::avcodec_free_context(&mut self.codec_ctx);
            }
            if !self.audio_codec_ctx.is_null() {
                ffi::avcodec_free_context(&mut self.audio_codec_ctx);
            }
            if !self.fmt_ctx.is_null() {
                ffi::avformat_close_input(&mut self.fmt_ctx);
            }
            if !self.sws_ctx.is_null() {
                ffi::sws_freeContext(self.sws_ctx);
            }
            if !self.rgba_scratch.is_null() {
                ffi::av_frame_free(&mut self.rgba_scratch);
            }
            if !self.swr_ctx.is_null() {
                ffi::swr_free(&mut self.swr_ctx);
            }
            if !self.audio_scratch.is_null() {
                ffi::av_freep(&mut self.audio_scratch as *mut _ as *mut _);
            }
        }
    }
}

fn open_decoder(
    path: &str,
    target_sample_rate: i32,
    target_channels: i32,
    ebu_tx: Sender<AudioFrame>,
) -> Result<DecoderCtx> {
    let c_path = CString::new(path)?;
    unsafe {
        let mut fmt_ctx: *mut ffi::AVFormatContext = ptr::null_mut();
        let ret =
            ffi::avformat_open_input(&mut fmt_ctx, c_path.as_ptr(), ptr::null(), ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("open: {}", av_err(ret)));
        }

        let ret = ffi::avformat_find_stream_info(fmt_ctx, ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("stream info: {}", av_err(ret)));
        }

        let nb = (*fmt_ctx).nb_streams as usize;
        let streams = std::slice::from_raw_parts((*fmt_ctx).streams, nb);
        let video_idx = find_video_stream(streams);
        if video_idx < 0 {
            return Err(anyhow!("no video stream"));
        }

        let stream = *streams[video_idx as usize];
        let par = stream.codecpar;
        let time_base = stream.time_base;

        let codec = ffi::avcodec_find_decoder((*par).codec_id);
        if codec.is_null() {
            return Err(anyhow!("codec not found"));
        }

        let mut codec_ctx = ffi::avcodec_alloc_context3(codec);
        if codec_ctx.is_null() {
            return Err(anyhow!("avcodec_alloc_context3"));
        }

        let ret = ffi::avcodec_parameters_to_context(codec_ctx, par);
        if ret < 0 {
            return Err(anyhow!("params_to_ctx: {}", av_err(ret)));
        }

        // Enable multithreaded decoding
        (*codec_ctx).thread_count = 0; // auto
        (*codec_ctx).thread_type = ffi::FF_THREAD_FRAME as i32;

        let ret = ffi::avcodec_open2(codec_ctx, codec, ptr::null_mut());
        if ret < 0 {
            return Err(anyhow!("avcodec_open2: {}", av_err(ret)));
        }

        let width = (*codec_ctx).width as u32;
        let height = (*codec_ctx).height as u32;
        let src_fmt = (*codec_ctx).pix_fmt;

        let fps = if stream.avg_frame_rate.den != 0 {
            stream.avg_frame_rate.num as f64 / stream.avg_frame_rate.den as f64
        } else {
            25.0
        };

        let sws_ctx = ffi::sws_getContext(
            width as i32,
            height as i32,
            src_fmt,
            width as i32,
            height as i32,
            ffi::AVPixelFormat::AV_PIX_FMT_RGBA,
            SWS_FAST_BILINEAR,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null(),
        );
        if sws_ctx.is_null() {
            return Err(anyhow!("sws_getContext failed"));
        }

        let mut rgba_scratch = ffi::av_frame_alloc();
        if rgba_scratch.is_null() {
            ffi::sws_freeContext(sws_ctx);
            ffi::avcodec_free_context(&mut codec_ctx);
            ffi::avformat_close_input(&mut fmt_ctx);
            return Err(anyhow!("av_frame_alloc (rgba scratch)"));
        }
        (*rgba_scratch).width = width as i32;
        (*rgba_scratch).height = height as i32;
        (*rgba_scratch).format = ffi::AVPixelFormat::AV_PIX_FMT_RGBA as i32;
        let buf_ret = ffi::av_frame_get_buffer(rgba_scratch, 0);
        if buf_ret < 0 {
            ffi::av_frame_free(&mut rgba_scratch);
            ffi::sws_freeContext(sws_ctx);
            ffi::avcodec_free_context(&mut codec_ctx);
            ffi::avformat_close_input(&mut fmt_ctx);
            return Err(anyhow!("av_frame_get_buffer (rgba): {}", av_err(buf_ret)));
        }

        let mut audio_stream_idx = -1;
        let mut audio_codec_ctx = ptr::null_mut();
        let mut swr_ctx = ptr::null_mut();
        let mut audio_time_base = ffi::AVRational { num: 0, den: 1 };

        let a_idx = find_audio_stream(streams);
        if a_idx >= 0 {
            audio_stream_idx = a_idx;
            let a_stream = *streams[a_idx as usize];
            let a_par = a_stream.codecpar;
            audio_time_base = a_stream.time_base;
            let a_codec = ffi::avcodec_find_decoder((*a_par).codec_id);
            if !a_codec.is_null() {
                audio_codec_ctx = ffi::avcodec_alloc_context3(a_codec);
                if ffi::avcodec_parameters_to_context(audio_codec_ctx, a_par) >= 0 {
                    if ffi::avcodec_open2(audio_codec_ctx, a_codec, ptr::null_mut()) >= 0 {
                        swr_ctx = ffi::swr_alloc();

                        let mut out_ch_layout: ffi::AVChannelLayout = std::mem::zeroed();
                        ffi::av_channel_layout_default(&mut out_ch_layout, target_channels);

                        let ret = ffi::swr_alloc_set_opts2(
                            &mut swr_ctx,
                            &out_ch_layout,
                            ffi::AVSampleFormat::AV_SAMPLE_FMT_FLT,
                            target_sample_rate,
                            &(*a_par).ch_layout,
                            std::mem::transmute((*a_par).format),
                            (*a_par).sample_rate,
                            0,
                            ptr::null_mut(),
                        );

                        if ret >= 0 {
                            ffi::swr_init(swr_ctx);
                            log::info!(
                                "SWR ctx initialized: out {} Hz, {} ch",
                                target_sample_rate,
                                target_channels
                            );
                        } else {
                            log::warn!("Failed to init SwrContext, audio disabled.");
                            ffi::swr_free(&mut swr_ctx);
                            swr_ctx = ptr::null_mut();
                            audio_stream_idx = -1;
                        }
                    } else {
                        audio_stream_idx = -1;
                    }
                } else {
                    audio_stream_idx = -1;
                }
            } else {
                audio_stream_idx = -1;
            }
        }

        Ok(DecoderCtx {
            fmt_ctx,
            codec_ctx,
            sws_ctx,
            rgba_scratch,
            stream_idx: video_idx,
            time_base,
            width,
            height,
            fps,
            audio_stream_idx,
            audio_codec_ctx,
            swr_ctx,
            _audio_time_base: audio_time_base,
            target_sample_rate,
            target_channels,
            audio_scratch: ptr::null_mut(),
            audio_scratch_cap: 0,
            ebu_tx,
        })
    }
}

fn decoder_loop(
    path: &str,
    target_sample_rate: i32,
    target_channels: i32,
    cmd_rx: Receiver<DecoderCommand>,
    frame_tx: Sender<VideoFrame>,
    audio_tx: Sender<AudioFrame>,
    ebu_tx: Sender<AudioFrame>,
) -> Result<()> {
    let mut ctx = open_decoder(path, target_sample_rate, target_channels, ebu_tx)?;
    log::info!(
        "Decoder open: '{path}' {}×{} @ {:.2}fps",
        ctx.width,
        ctx.height,
        ctx.fps
    );

    let mut is_playing = false;
    let mut current_pts: i64 = 0;
    let _frame_dur = if ctx.fps > 0.0 {
        secs_to_pts(1.0 / ctx.fps, ctx.time_base)
    } else {
        1
    };

    unsafe {
        let packet = ffi::av_packet_alloc();
        let frame = ffi::av_frame_alloc();
        let mut pending_frame: Option<VideoFrame> = None;

        loop {
            // Priority 1: drain existing commands without blocking
            let mut pending_seek = None;
            let mut pending_play_state: Option<bool> = None;

            while let Ok(cmd) = cmd_rx.try_recv() {
                match cmd {
                    DecoderCommand::Seek(secs) => {
                        pending_seek = Some(secs);
                    }
                    DecoderCommand::Play => {
                        pending_play_state = Some(true);
                    }
                    DecoderCommand::Pause => {
                        pending_play_state = Some(false);
                    }
                    DecoderCommand::StepForward => {
                        // Process pending Seek/Play/Pause before this step
                        if let Some(secs) = pending_seek.take() {
                            handle_cmd(
                                DecoderCommand::Seek(secs),
                                &mut ctx,
                                &frame_tx,
                                &audio_tx,
                                &mut is_playing,
                                &mut current_pts,
                            )?;
                        }
                        if let Some(play) = pending_play_state.take() {
                            handle_cmd(
                                if play {
                                    DecoderCommand::Play
                                } else {
                                    DecoderCommand::Pause
                                },
                                &mut ctx,
                                &frame_tx,
                                &audio_tx,
                                &mut is_playing,
                                &mut current_pts,
                            )?;
                        }
                        handle_cmd(
                            cmd,
                            &mut ctx,
                            &frame_tx,
                            &audio_tx,
                            &mut is_playing,
                            &mut current_pts,
                        )?;
                        if !is_playing {
                            pending_frame = None;
                        }
                        // Stop draining more commands this cycle to prevent thread-hogging if holding keys
                        break;
                    }
                    _ => {
                        handle_cmd(
                            cmd,
                            &mut ctx,
                            &frame_tx,
                            &audio_tx,
                            &mut is_playing,
                            &mut current_pts,
                        )?;
                        if !is_playing {
                            pending_frame = None;
                        }
                    }
                }
            }
            if let Some(secs) = pending_seek.take() {
                handle_cmd(
                    DecoderCommand::Seek(secs),
                    &mut ctx,
                    &frame_tx,
                    &audio_tx,
                    &mut is_playing,
                    &mut current_pts,
                )?;
                if !is_playing {
                    pending_frame = None;
                }
            }
            if let Some(play) = pending_play_state.take() {
                let state_cmd = if play {
                    DecoderCommand::Play
                } else {
                    DecoderCommand::Pause
                };
                handle_cmd(
                    state_cmd,
                    &mut ctx,
                    &frame_tx,
                    &audio_tx,
                    &mut is_playing,
                    &mut current_pts,
                )?;
            }

            // If we need a frame, decode one
            if is_playing && pending_frame.is_none() {
                if let Some(f) = decode_one_frame(&mut ctx, packet, frame, &audio_tx)? {
                    current_pts = secs_to_pts(f.pts, ctx.time_base);
                    pending_frame = Some(f);
                } else {
                    log::info!("Decoder EOF or stopped at end of file: '{path}'");
                    is_playing = false; // EOF
                }
            }

            if let Some(f) = pending_frame.take() {
                // If paused, we don't try to send to avoid filling channel and blocking,
                // wait, if paused we DO want to send one frame to show the current frame!
                // But if it's paused we only send it once.
                crossbeam_channel::select! {
                    send(frame_tx, f.clone()) -> res => {
                        if res.is_err() {
                            log::warn!("Decoder thread exiting: UI frame channel disconnected");
                            return Ok(());
                        }
                        // Throttle removed: we rely on channel bounds (frame_tx = 8) to pace the decoder naturally.
                        // This allows audio_tx to build a healthy buffer so rodio never starves.
                    }
                    recv(cmd_rx) -> msg => {
                        pending_frame = Some(f); // Put it back
                        if let Ok(cmd) = msg {
                            log::trace!("Decoder received command: {:?}", cmd);
                            handle_cmd(cmd, &mut ctx, &frame_tx, &audio_tx, &mut is_playing, &mut current_pts)?;
                            if !is_playing { pending_frame = None; }
                        } else {
                            log::warn!("Decoder thread exiting: Command channel disconnected");
                            return Ok(());
                        }
                    }
                }
            } else {
                // No pending frame (either EOF or paused without a frame). Block on commands.
                let msg = cmd_rx.recv();
                if let Ok(cmd) = msg {
                    log::trace!("Decoder received command (idle): {:?}", cmd);
                    handle_cmd(
                        cmd,
                        &mut ctx,
                        &frame_tx,
                        &audio_tx,
                        &mut is_playing,
                        &mut current_pts,
                    )?;
                } else {
                    log::warn!("Decoder thread exiting (idle): Command channel disconnected");
                    return Ok(());
                }
            }
        } // loop
    } // unsafe
}

unsafe fn handle_cmd(
    cmd: DecoderCommand,
    ctx: &mut DecoderCtx,
    frame_tx: &Sender<VideoFrame>,
    audio_tx: &Sender<AudioFrame>,
    is_playing: &mut bool,
    current_pts: &mut i64,
) -> Result<()> {
    match &cmd {
        DecoderCommand::Play => {
            trace_log::log("Decoder: Play");
            *is_playing = true;
        }
        DecoderCommand::Pause => {
            trace_log::log("Decoder: Pause");
            *is_playing = false;
        }
        DecoderCommand::Stop => {
            trace_log::log("Decoder: Stop");
            // Thread will exit because we don't have a way to gracefully return from here directly,
            // but we can close the channel or trigger disconnect.
        }
        DecoderCommand::Seek(secs) => {
            trace_log::log(&format!("Decoder: Seek {:.3}s", secs));
            *is_playing = false;
            let target = secs_to_pts(*secs, ctx.time_base);
            seek_exact(ctx, target, frame_tx, audio_tx)?;
            *current_pts = target;
        }
        DecoderCommand::StepForward => {
            trace_log::log("Decoder: StepForward");
            // For step forward, we decode one immediately and send it.
            // Using blocking send() to ensure the decoder and the UI clock stay perfectly in sync.
            let packet = ffi::av_packet_alloc();
            let frame = ffi::av_frame_alloc();
            if let Some(f) = decode_one_frame(ctx, packet, frame, audio_tx)? {
                let frame_pts_raw = secs_to_pts(f.pts, ctx.time_base);
                if frame_tx.try_send(f).is_ok() {
                    *current_pts = frame_pts_raw;
                }
            }
            ffi::av_packet_free(&mut (packet as *mut _));
            ffi::av_frame_free(&mut (frame as *mut _));
        }
        DecoderCommand::SetVolume(_) => {}
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Decode one frame and send it
// ---------------------------------------------------------------------------

unsafe fn decode_one_frame(
    ctx: &mut DecoderCtx,
    packet: *mut ffi::AVPacket,
    frame: *mut ffi::AVFrame,
    audio_tx: &Sender<AudioFrame>,
) -> Result<Option<VideoFrame>> {
    // Try to receive a buffered frame first
    let ret = ffi::avcodec_receive_frame(ctx.codec_ctx, frame);
    if ret == 0 {
        let pts = (*frame).best_effort_timestamp;
        let vf = convert_frame(ctx, frame, pts)?;
        ffi::av_frame_unref(frame);
        return Ok(Some(vf));
    } else if ret != ffi::AVERROR(ffi::EAGAIN) && ret != ffi::AVERROR_EOF {
        log::warn!("avcodec_receive_frame error code: {}", ret);
    }

    // Read packets until we get a frame
    loop {
        let ret = ffi::av_read_frame(ctx.fmt_ctx, packet);
        if ret < 0 {
            // EOF — flush decoder
            let ret_send = ffi::avcodec_send_packet(ctx.codec_ctx, ptr::null());
            if ret_send < 0 {
                log::warn!("avcodec_send_packet (flush video) error code: {}", ret_send);
            }
            let r2 = ffi::avcodec_receive_frame(ctx.codec_ctx, frame);
            if r2 == 0 {
                let pts = (*frame).best_effort_timestamp;
                let vf = convert_frame(ctx, frame, pts)?;
                ffi::av_frame_unref(frame);
                return Ok(Some(vf));
            }
            return Ok(None);
        }

        if (*packet).stream_index == ctx.stream_idx {
            let ret_send = ffi::avcodec_send_packet(ctx.codec_ctx, packet);
            if ret_send < 0 {
                log::warn!("avcodec_send_packet (video) error code: {}", ret_send);
            }
            ffi::av_packet_unref(packet);

            let r2 = ffi::avcodec_receive_frame(ctx.codec_ctx, frame);
            if r2 == 0 {
                let pts = (*frame).best_effort_timestamp;
                let vf = convert_frame(ctx, frame, pts)?;
                ffi::av_frame_unref(frame);
                return Ok(Some(vf));
            }
        } else if (*packet).stream_index == ctx.audio_stream_idx {
            let ret_send = ffi::avcodec_send_packet(ctx.audio_codec_ctx, packet);
            if ret_send < 0 {
                log::warn!("avcodec_send_packet (audio) error code: {}", ret_send);
            }
            ffi::av_packet_unref(packet);

            while ffi::avcodec_receive_frame(ctx.audio_codec_ctx, frame) == 0 {
                if let Some(audio) = convert_audio_frame(ctx, frame)? {
                    // Send blocking (with bounds) so we don't drop audio packets and cause glitches
                    let _ = audio_tx.send(audio.clone());
                    let _ = ctx.ebu_tx.send(audio);
                }
                ffi::av_frame_unref(frame);
            }
        } else {
            ffi::av_packet_unref(packet);
        }
    }
}

unsafe fn seek_exact(
    ctx: &mut DecoderCtx,
    target_pts: i64,
    frame_tx: &Sender<VideoFrame>,
    audio_tx: &Sender<AudioFrame>,
) -> Result<()> {
    // Seek to slightly before target to get the keyframe
    let ret_seek = ffi::av_seek_frame(
        ctx.fmt_ctx,
        ctx.stream_idx,
        target_pts,
        ffi::AVSEEK_FLAG_BACKWARD as i32,
    );
    if ret_seek < 0 {
        log::warn!("av_seek_frame error code: {}", ret_seek);
    }
    ffi::avcodec_flush_buffers(ctx.codec_ctx);

    let packet = ffi::av_packet_alloc();
    let frame = ffi::av_frame_alloc();

    loop {
        let ret = ffi::av_read_frame(ctx.fmt_ctx, packet);
        if ret < 0 {
            break;
        }

        if (*packet).stream_index == ctx.stream_idx {
            let ret_send = ffi::avcodec_send_packet(ctx.codec_ctx, packet);
            if ret_send < 0 {
                log::warn!("avcodec_send_packet (video seek) error code: {}", ret_send);
            }
            ffi::av_packet_unref(packet);

            let r2 = ffi::avcodec_receive_frame(ctx.codec_ctx, frame);
            if r2 == 0 {
                let pts = (*frame).best_effort_timestamp;
                if pts >= target_pts {
                    let vf = convert_frame(ctx, frame, pts)?;
                    let _ = frame_tx.send(vf);
                    ffi::av_frame_unref(frame);
                    break;
                }
                ffi::av_frame_unref(frame);
            }
        } else if (*packet).stream_index == ctx.audio_stream_idx {
            let ret_send = ffi::avcodec_send_packet(ctx.audio_codec_ctx, packet);
            if ret_send < 0 {
                log::warn!("avcodec_send_packet (audio seek) error code: {}", ret_send);
            }
            ffi::av_packet_unref(packet);

            while ffi::avcodec_receive_frame(ctx.audio_codec_ctx, frame) == 0 {
                let pts = (*frame).best_effort_timestamp;
                if pts >= target_pts {
                    if let Some(audio) = convert_audio_frame(ctx, frame)? {
                        let _ = audio_tx.send(audio.clone());
                        let _ = ctx.ebu_tx.send(audio);
                    }
                }
                ffi::av_frame_unref(frame);
            }
        } else {
            ffi::av_packet_unref(packet);
        }
    }

    ffi::av_packet_free(&mut (packet as *mut _));
    ffi::av_frame_free(&mut (frame as *mut _));
    Ok(())
}

unsafe fn convert_frame(
    ctx: &DecoderCtx,
    frame: *mut ffi::AVFrame,
    pts_raw: i64,
) -> Result<VideoFrame> {
    let w = ctx.width;
    let h = ctx.height;
    let pts = pts_to_secs(pts_raw, ctx.time_base);

    let dst_frame = ctx.rgba_scratch;
    if dst_frame.is_null() {
        return Err(anyhow!("rgba scratch frame null"));
    }

    let src_data: [*const u8; 4] = [
        (*frame).data[0],
        (*frame).data[1],
        (*frame).data[2],
        (*frame).data[3],
    ];

    ffi::sws_scale(
        ctx.sws_ctx,
        src_data.as_ptr(),
        (*frame).linesize.as_ptr(),
        0,
        h as i32,
        (*dst_frame).data.as_mut_ptr(),
        (*dst_frame).linesize.as_mut_ptr(),
    );

    let stride = (*dst_frame).linesize[0] as usize;
    let pixel_bytes = 4;
    let total_bytes = (w * h) as usize * pixel_bytes;
    let mut rgba_data = Vec::with_capacity(total_bytes);
    let src_ptr = (*dst_frame).data[0];

    if stride == w as usize * pixel_bytes {
        let all_slice = std::slice::from_raw_parts(src_ptr, total_bytes);
        rgba_data.extend_from_slice(all_slice);
    } else {
        for row in 0..h as usize {
            let row_start = src_ptr.add(row * stride);
            let row_slice = std::slice::from_raw_parts(row_start, w as usize * pixel_bytes);
            rgba_data.extend_from_slice(row_slice);
        }
    }

    Ok(VideoFrame {
        pts,
        rgba_data: rgba_data.into(),
        width: w,
        height: h,
    })
}

unsafe fn convert_audio_frame(
    ctx: &mut DecoderCtx,
    frame: *mut ffi::AVFrame,
) -> Result<Option<AudioFrame>> {
    if ctx.swr_ctx.is_null() {
        return Ok(None);
    }

    let nb_samples = (*frame).nb_samples;
    // Calculate out samples (allowing up to 10% more for resampling drift)
    let out_samples_cap = ffi::swr_get_out_samples(ctx.swr_ctx, nb_samples);

    if ctx.audio_scratch_cap < out_samples_cap {
        if !ctx.audio_scratch.is_null() {
            ffi::av_freep(&mut ctx.audio_scratch as *mut _ as *mut _);
        }
        ffi::av_samples_alloc(
            &mut ctx.audio_scratch,
            ptr::null_mut(),
            ctx.target_channels,
            out_samples_cap,
            ffi::AVSampleFormat::AV_SAMPLE_FMT_FLT,
            0,
        );
        ctx.audio_scratch_cap = out_samples_cap;
    }

    let out_samples_count = ffi::swr_convert(
        ctx.swr_ctx,
        &mut ctx.audio_scratch,
        out_samples_cap,
        (*frame).data.as_ptr() as *mut *const u8,
        nb_samples,
    );

    if out_samples_count < 0 {
        return Err(anyhow!("swr_convert failed"));
    }

    let byte_size = out_samples_count as usize * ctx.target_channels as usize * 4; // count * channels * sizeof(f32)
    let slice = std::slice::from_raw_parts(ctx.audio_scratch as *const f32, byte_size / 4);
    let mut samples = Vec::with_capacity(slice.len());
    samples.extend_from_slice(slice);

    Ok(Some(AudioFrame {
        samples,
        channels: ctx.target_channels as u16,
        sample_rate: ctx.target_sample_rate as u32,
    }))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

pub(crate) fn pts_to_secs(pts: i64, tb: ffi::AVRational) -> f64 {
    if tb.den == 0 {
        return 0.0;
    }
    pts as f64 * tb.num as f64 / tb.den as f64
}

pub(crate) fn secs_to_pts(secs: f64, tb: ffi::AVRational) -> i64 {
    if tb.den == 0 {
        return 0;
    }
    (secs * tb.den as f64 / tb.num as f64) as i64
}

pub(crate) unsafe fn find_video_stream(streams: &[*mut ffi::AVStream]) -> i32 {
    for (idx, &stream) in streams.iter().enumerate() {
        if (*(*stream).codecpar).codec_type == ffi::AVMediaType::AVMEDIA_TYPE_VIDEO {
            return idx as i32;
        }
    }
    -1
}

unsafe fn find_audio_stream(streams: &[*mut ffi::AVStream]) -> i32 {
    for (idx, &stream) in streams.iter().enumerate() {
        if (*(*stream).codecpar).codec_type == ffi::AVMediaType::AVMEDIA_TYPE_AUDIO {
            return idx as i32;
        }
    }
    -1
}

fn color_space_str(cs: ffi::AVColorSpace) -> String {
    unsafe {
        let ptr = ffi::av_color_space_name(cs);
        if ptr.is_null() {
            return "unknown".into();
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

fn color_trc_str(trc: ffi::AVColorTransferCharacteristic) -> String {
    unsafe {
        let ptr = ffi::av_color_transfer_name(trc);
        if ptr.is_null() {
            return "unknown".into();
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

fn color_primaries_str(prim: ffi::AVColorPrimaries) -> String {
    unsafe {
        let ptr = ffi::av_color_primaries_name(prim);
        if ptr.is_null() {
            return "unknown".into();
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub(crate) fn av_err(code: i32) -> String {
    let mut buf = vec![0u8; ffi::AV_ERROR_MAX_STRING_SIZE as usize];
    unsafe {
        ffi::av_strerror(code, buf.as_mut_ptr() as *mut i8, buf.len());
        let s = CStr::from_ptr(buf.as_ptr() as *const i8);
        s.to_string_lossy().into_owned()
    }
}

// SAFETY: AVFormatContext, AVCodecContext etc. pointers are only used on the decoder thread.
unsafe impl Send for DecoderCtx {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pts_to_secs() {
        let tb = ffi::AVRational { num: 1, den: 1000 };
        assert_eq!(pts_to_secs(500, tb), 0.5);
        assert_eq!(pts_to_secs(0, tb), 0.0);

        let tb_zero = ffi::AVRational { num: 1, den: 0 };
        assert_eq!(pts_to_secs(100, tb_zero), 0.0);
    }

    #[test]
    fn test_secs_to_pts() {
        let tb = ffi::AVRational { num: 1, den: 1000 };
        assert_eq!(secs_to_pts(0.5, tb), 500);
        assert_eq!(secs_to_pts(0.0, tb), 0);

        let tb_zero = ffi::AVRational { num: 1, den: 0 };
        assert_eq!(secs_to_pts(1.0, tb_zero), 0);
    }
}
