//! Escaneo offline de forma de onda y loudness (M4 + M9 EBU R128).
//!
//! Decodifica la pista de audio con FFmpeg, picos por bucket, K-weighting y
//! LUFS integrado / true peak / LRA según BS.1770.

use crate::analisis_loudness::{AnalizadorLoudness, DatosEbuR128};

use anyhow::{anyhow, Context, Result};
use ffmpeg_sys_next as ffi;
use std::ffi::{CStr, CString};
use std::ptr;

/// Forma de onda precomputada para pintar en el workspace Audio.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FormaOnda {
    /// Picos normalizados 0..1, uno por bucket temporal.
    pub picos: Vec<f32>,
    pub duracion_secs: f64,
    /// LUFS integrado EBU R128 (K-weighting + gate). Igual que `ebu.lufs_integrado`.
    pub lufs_integrado: f64,
    pub picos_por_segundo: u32,
    /// LUFS momentáneo por bucket (overlay waveform).
    #[serde(default)]
    pub lufs_buckets: Vec<f32>,
    /// Métricas EBU R128 completas (M9).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebu: Option<DatosEbuR128>,
}

/// Tasa de salida fija para simplificar el mapeo temporal bucket ↔ muestra.
const TASA_MUESTRAS_SALIDA: i32 = 48_000;

/// Picos por segundo por defecto (~1200 puntos en un clip de 30 s).
pub const PICOS_POR_SEGUNDO_DEFECTO: u32 = 40;

/// Escanea la pista de audio de `ruta` y devuelve picos + LUFS estimado.
pub fn escanear(ruta: &str, picos_por_segundo: u32) -> Result<FormaOnda> {
    let pps = picos_por_segundo.max(1);
    unsafe { escanear_inseguro(ruta, pps) }
}

unsafe fn escanear_inseguro(ruta: &str, picos_por_segundo: u32) -> Result<FormaOnda> {
    let c_path = CString::new(ruta).context("ruta inválida")?;
    let mut fmt_ctx = ptr::null_mut();

    if ffi::avformat_open_input(&mut fmt_ctx, c_path.as_ptr(), ptr::null(), ptr::null_mut()) != 0 {
        return Err(anyhow!("avformat_open_input: {ruta}"));
    }

    if ffi::avformat_find_stream_info(fmt_ctx, ptr::null_mut()) < 0 {
        ffi::avformat_close_input(&mut fmt_ctx);
        return Err(anyhow!("avformat_find_stream_info"));
    }

    let duracion_secs = duracion_desde_formato(fmt_ctx);

    let nb_streams = (*fmt_ctx).nb_streams as usize;
    let streams = std::slice::from_raw_parts((*fmt_ctx).streams, nb_streams);
    let audio_idx = buscar_stream_audio(streams);

    if audio_idx < 0 {
        ffi::avformat_close_input(&mut fmt_ctx);
        return Ok(FormaOnda {
            picos: Vec::new(),
            duracion_secs,
            lufs_integrado: f64::NEG_INFINITY,
            picos_por_segundo,
            lufs_buckets: Vec::new(),
            ebu: None,
        });
    }

    let stream = streams[audio_idx as usize];
    let par = (*stream).codecpar;
    let time_base = (*stream).time_base;

    let codec = ffi::avcodec_find_decoder((*par).codec_id);
    if codec.is_null() {
        ffi::avformat_close_input(&mut fmt_ctx);
        return Err(anyhow!("codec de audio no encontrado"));
    }

    let mut codec_ctx = ffi::avcodec_alloc_context3(codec);
    if codec_ctx.is_null() {
        ffi::avformat_close_input(&mut fmt_ctx);
        return Err(anyhow!("avcodec_alloc_context3"));
    }

    if ffi::avcodec_parameters_to_context(codec_ctx, par) < 0
        || ffi::avcodec_open2(codec_ctx, codec, ptr::null_mut()) < 0
    {
        ffi::avcodec_free_context(&mut codec_ctx);
        ffi::avformat_close_input(&mut fmt_ctx);
        return Err(anyhow!("no se pudo abrir decoder de audio"));
    }

    let mut swr_ctx = ptr::null_mut();
    let mut layout_mono: ffi::AVChannelLayout = std::mem::zeroed();
    ffi::av_channel_layout_default(&mut layout_mono, 1);

    let ret = ffi::swr_alloc_set_opts2(
        &mut swr_ctx,
        &layout_mono,
        ffi::AVSampleFormat::AV_SAMPLE_FMT_FLT,
        TASA_MUESTRAS_SALIDA,
        &(*par).ch_layout,
        std::mem::transmute((*par).format),
        (*par).sample_rate,
        0,
        ptr::null_mut(),
    );

    if ret < 0 || swr_ctx.is_null() || ffi::swr_init(swr_ctx) < 0 {
        if !swr_ctx.is_null() {
            ffi::swr_free(&mut swr_ctx);
        }
        ffi::avcodec_free_context(&mut codec_ctx);
        ffi::avformat_close_input(&mut fmt_ctx);
        return Err(anyhow!("swr_init audio: {}", av_err(ret)));
    }

    // Buckets: uno por fracción de segundo.
    let num_buckets = ((duracion_secs * picos_por_segundo as f64).ceil() as usize).max(1);
    let mut picos = vec![0.0f32; num_buckets];
    let mut analizador = AnalizadorLoudness::nuevo(num_buckets, picos_por_segundo);
    let mut muestras_acumuladas = 0i64;

    let mut packet = ffi::av_packet_alloc();
    let mut frame = ffi::av_frame_alloc();
    if packet.is_null() || frame.is_null() {
        liberar_recursos(&mut fmt_ctx, &mut codec_ctx, &mut swr_ctx, &mut packet, &mut frame);
        return Err(anyhow!("av_packet_alloc / av_frame_alloc"));
    }

    // Bucle de lectura: solo paquetes del stream de audio.
    while ffi::av_read_frame(fmt_ctx, packet) >= 0 {
        if (*packet).stream_index != audio_idx {
            ffi::av_packet_unref(packet);
            continue;
        }

        let envio = ffi::avcodec_send_packet(codec_ctx, packet);
        if envio < 0 {
            ffi::av_packet_unref(packet);
            continue;
        }

        while ffi::avcodec_receive_frame(codec_ctx, frame) >= 0 {
            let nb = (*frame).nb_samples as i32;
            if nb <= 0 {
                continue;
            }

            let out_samples_cap = ffi::swr_get_out_samples(swr_ctx, nb);
            let mut out_samples_data: *mut u8 = ptr::null_mut();
            ffi::av_samples_alloc(
                &mut out_samples_data,
                ptr::null_mut(),
                1,
                out_samples_cap,
                ffi::AVSampleFormat::AV_SAMPLE_FMT_FLT,
                0,
            );

            let converted = ffi::swr_convert(
                swr_ctx,
                &mut out_samples_data,
                out_samples_cap,
                (*frame).data.as_ptr() as *mut *const u8,
                nb,
            );
            if converted <= 0 {
                if !out_samples_data.is_null() {
                    ffi::av_freep(&mut out_samples_data as *mut _ as *mut _);
                }
                continue;
            }

            let slice = std::slice::from_raw_parts(
                out_samples_data as *const f32,
                converted as usize,
            );

            // PTS del primer sample del frame (segundos).
            let pts_frame = if (*frame).best_effort_timestamp != ffi::AV_NOPTS_VALUE {
                (*frame).best_effort_timestamp as f64 * time_base.num as f64
                    / time_base.den as f64
            } else {
                muestras_acumuladas as f64 / TASA_MUESTRAS_SALIDA as f64
            };

            analizador.alimentar(slice);

            for (i, &muestra) in slice.iter().enumerate() {
                let t = pts_frame + i as f64 / TASA_MUESTRAS_SALIDA as f64;
                let bucket = (t * picos_por_segundo as f64).floor() as usize;
                let abs = muestra.abs();
                if bucket < picos.len() {
                    picos[bucket] = picos[bucket].max(abs);
                }
            }
            muestras_acumuladas += converted as i64;
            ffi::av_freep(&mut out_samples_data as *mut _ as *mut _);
        }
        // Tras send_packet exitoso FFmpeg toma ownership del paquete; no hacer unref.
    }

    // Normalizar picos al máximo global para usar todo el alto del canvas.
    let max_pico = picos.iter().copied().fold(0.0f32, f32::max);
    if max_pico > 1e-9 {
        for p in &mut picos {
            *p /= max_pico;
        }
    }

    let (ebu, lufs_buckets) = analizador.finalizar();
    let lufs_integrado = ebu.lufs_integrado;

    liberar_recursos(&mut fmt_ctx, &mut codec_ctx, &mut swr_ctx, &mut packet, &mut frame);

    Ok(FormaOnda {
        picos,
        duracion_secs,
        lufs_integrado,
        picos_por_segundo,
        lufs_buckets,
        ebu: Some(ebu),
    })
}

unsafe fn duracion_desde_formato(fmt_ctx: *mut ffi::AVFormatContext) -> f64 {
    if (*fmt_ctx).duration > 0 {
        return (*fmt_ctx).duration as f64 / ffi::AV_TIME_BASE as f64;
    }
    0.0
}

unsafe fn buscar_stream_audio(streams: &[*mut ffi::AVStream]) -> i32 {
    for (idx, &stream) in streams.iter().enumerate() {
        if (*(*stream).codecpar).codec_type == ffi::AVMediaType::AVMEDIA_TYPE_AUDIO {
            return idx as i32;
        }
    }
    -1
}

unsafe fn liberar_recursos(
    fmt_ctx: &mut *mut ffi::AVFormatContext,
    codec_ctx: &mut *mut ffi::AVCodecContext,
    swr_ctx: &mut *mut ffi::SwrContext,
    packet: &mut *mut ffi::AVPacket,
    frame: &mut *mut ffi::AVFrame,
) {
    if !packet.is_null() {
        ffi::av_packet_free(packet);
    }
    if !frame.is_null() {
        ffi::av_frame_free(frame);
    }
    if !swr_ctx.is_null() {
        ffi::swr_free(swr_ctx);
    }
    if !codec_ctx.is_null() {
        ffi::avcodec_free_context(codec_ctx);
    }
    if !fmt_ctx.is_null() {
        ffi::avformat_close_input(fmt_ctx);
    }
}

fn av_err(code: i32) -> String {
    let mut buf = [0i8; 256];
    unsafe {
        ffi::av_strerror(code, buf.as_mut_ptr(), buf.len());
        CStr::from_ptr(buf.as_ptr())
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn escaneo_mp4_muestra() {
        let ruta = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/videos-muestra/30s_C-HR_Hybrid.mp4"
        );
        if !std::path::Path::new(ruta).exists() {
            return;
        }
        let f = escanear(ruta, PICOS_POR_SEGUNDO_DEFECTO).expect("escaneo");
        assert!(f.duracion_secs > 0.0, "duración");
        assert!(!f.picos.is_empty(), "picos vacíos");
        assert!(f.picos.iter().any(|&p| p > 0.0), "picos todos cero");
    }
}
