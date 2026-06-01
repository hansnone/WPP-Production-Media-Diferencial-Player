//! Decodificación de vídeo con aceleración por hardware (M11) y fallback a software.
//!
//! Usa la API `hwdevice` / `hwframe` de FFmpeg. Tras `av_hwframe_transfer_data` el pipeline
//! sigue con `sws_scale` → RGBA (misma ruta que decode por CPU).

use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{anyhow, Result};
use ffmpeg_sys_next as ffi;
use log::{info, warn};

/// Desactivar HW: `DIFFPLAYERQC_HW_DECODE=0` o `off`.
static HW_GLOBAL_OFF: AtomicBool = AtomicBool::new(false);

fn hw_desactivado_por_entorno() -> bool {
    if HW_GLOBAL_OFF.load(Ordering::Relaxed) {
        return true;
    }
    match std::env::var("DIFFPLAYERQC_HW_DECODE").ok().as_deref() {
        Some("0") | Some("off") | Some("false") => {
            HW_GLOBAL_OFF.store(true, Ordering::Relaxed);
            true
        }
        _ => false,
    }
}

/// Contexto pasado a `codec_ctx.opaque` para el callback `get_format`.
struct ContextoFormatoHw {
    hw_pix_fmt: ffi::AVPixelFormat,
}

/// Estado activo tras inicializar HW en un `AVCodecContext`.
pub struct EstadoHwDecode {
    pub hw_pix_fmt: ffi::AVPixelFormat,
    /// Formato CPU tras `av_hwframe_transfer_data` (p. ej. NV12).
    pub formato_cpu: ffi::AVPixelFormat,
    pub nombre_dispositivo: String,
    /// Frame reutilizable para la descarga HW → CPU.
    frame_cpu: *mut ffi::AVFrame,
}

impl Drop for EstadoHwDecode {
    fn drop(&mut self) {
        unsafe {
            if !self.frame_cpu.is_null() {
                ffi::av_frame_free(&mut self.frame_cpu);
            }
        }
    }
}

unsafe extern "C" fn callback_formato_hw(
    ctx: *mut ffi::AVCodecContext,
    pix_fmts: *const ffi::AVPixelFormat,
) -> ffi::AVPixelFormat {
    if ctx.is_null() || pix_fmts.is_null() {
        return ffi::AVPixelFormat::AV_PIX_FMT_NONE;
    }
    let opaque = (*ctx).opaque as *mut ContextoFormatoHw;
    if opaque.is_null() {
        return *pix_fmts;
    }
    let objetivo = (*opaque).hw_pix_fmt;
    let mut p = pix_fmts;
    while *p != ffi::AVPixelFormat::AV_PIX_FMT_NONE {
        if *p == objetivo {
            return *p;
        }
        p = p.add(1);
    }
    warn!("get_format: formato HW no listado; primer formato software");
    *pix_fmts
}

/// Tipos de dispositivo a probar, en orden, según el SO de compilación.
fn tipos_dispositivo_preferidos() -> &'static [ffi::AVHWDeviceType] {
    #[cfg(target_os = "macos")]
    {
        &[ffi::AVHWDeviceType::AV_HWDEVICE_TYPE_VIDEOTOOLBOX]
    }
    #[cfg(target_os = "windows")]
    {
        &[
            ffi::AVHWDeviceType::AV_HWDEVICE_TYPE_D3D11VA,
            ffi::AVHWDeviceType::AV_HWDEVICE_TYPE_DXVA2,
            ffi::AVHWDeviceType::AV_HWDEVICE_TYPE_CUDA,
        ]
    }
    #[cfg(target_os = "linux")]
    {
        &[
            ffi::AVHWDeviceType::AV_HWDEVICE_TYPE_VAAPI,
            ffi::AVHWDeviceType::AV_HWDEVICE_TYPE_CUDA,
        ]
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        &[]
    }
}

fn nombre_tipo_dispositivo(tipo: ffi::AVHWDeviceType) -> String {
    unsafe {
        let ptr = ffi::av_hwdevice_get_type_name(tipo);
        if ptr.is_null() {
            return format!("hw-{tipo:?}");
        }
        std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

/// Busca `hw_pix_fmt` compatible con el códec y el tipo de dispositivo.
unsafe fn buscar_pix_fmt_hw(
    codec: *const ffi::AVCodec,
    tipo_dispositivo: ffi::AVHWDeviceType,
) -> Option<ffi::AVPixelFormat> {
    let mut i = 0i32;
    loop {
        let cfg = ffi::avcodec_get_hw_config(codec, i);
        if cfg.is_null() {
            break;
        }
        if (*cfg).device_type == tipo_dispositivo
            && ((*cfg).methods & ffi::AV_CODEC_HW_CONFIG_METHOD_HW_DEVICE_CTX as i32) != 0
        {
            return Some((*cfg).pix_fmt);
        }
        i += 1;
    }
    None
}

/// Intenta configurar `codec_ctx` para decode HW. Si falla, deja el contexto sin HW.
pub unsafe fn intentar_inicializar_hw(
    codec_ctx: *mut ffi::AVCodecContext,
    codec: *const ffi::AVCodec,
) -> Result<Option<EstadoHwDecode>> {
    if hw_desactivado_por_entorno() || tipos_dispositivo_preferidos().is_empty() {
        return Ok(None);
    }

    for &tipo in tipos_dispositivo_preferidos() {
        let Some(hw_pix_fmt) = buscar_pix_fmt_hw(codec, tipo) else {
            continue;
        };

        let mut hw_device_ctx: *mut ffi::AVBufferRef = ptr::null_mut();
        let nombre = nombre_tipo_dispositivo(tipo);
        let ret = ffi::av_hwdevice_ctx_create(&mut hw_device_ctx, tipo, ptr::null(), ptr::null_mut(), 0);
        if ret < 0 {
            warn!("av_hwdevice_ctx_create({nombre}): {ret}");
            continue;
        }

        let contexto = Box::new(ContextoFormatoHw { hw_pix_fmt });
        (*codec_ctx).opaque = Box::into_raw(contexto) as *mut _;
        (*codec_ctx).get_format = Some(callback_formato_hw);
        (*codec_ctx).hw_device_ctx = ffi::av_buffer_ref(hw_device_ctx);
        ffi::av_buffer_unref(&mut hw_device_ctx);

        let mut hw_frames_ref = ffi::av_hwframe_ctx_alloc((*codec_ctx).hw_device_ctx);
        if hw_frames_ref.is_null() {
            warn!("av_hwframe_ctx_alloc falló ({nombre})");
            liberar_parcial_hw(codec_ctx);
            continue;
        }

        let frames_ctx = (*hw_frames_ref).data as *mut ffi::AVHWFramesContext;
        (*frames_ctx).format = hw_pix_fmt;
        (*frames_ctx).sw_format = ffi::AVPixelFormat::AV_PIX_FMT_NV12;
        (*frames_ctx).width = (*codec_ctx).width;
        (*frames_ctx).height = (*codec_ctx).height;

        let ret_frames = ffi::av_hwframe_ctx_init(hw_frames_ref);
        if ret_frames < 0 {
            warn!("av_hwframe_ctx_init({nombre}): {ret_frames}");
            ffi::av_buffer_unref(&mut hw_frames_ref);
            liberar_parcial_hw(codec_ctx);
            continue;
        }

        (*codec_ctx).hw_frames_ctx = ffi::av_buffer_ref(hw_frames_ref);
        ffi::av_buffer_unref(&mut hw_frames_ref);

        // Menos hilos CPU cuando el códec corre en GPU.
        (*codec_ctx).thread_count = 1;

        let frame_cpu = ffi::av_frame_alloc();
        if frame_cpu.is_null() {
            liberar_parcial_hw(codec_ctx);
            return Err(anyhow!("av_frame_alloc (hw frame_cpu)"));
        }

        info!(
            "Decode HW activo: {nombre} (pix_fmt={hw_pix_fmt:?}, salida CPU NV12)"
        );

        return Ok(Some(EstadoHwDecode {
            hw_pix_fmt,
            formato_cpu: ffi::AVPixelFormat::AV_PIX_FMT_NV12,
            nombre_dispositivo: nombre,
            frame_cpu,
        }));
    }

    Ok(None)
}

/// Libera recursos HW si `avcodec_open2` falla tras configurar el contexto.
pub unsafe fn liberar_parcial_hw(codec_ctx: *mut ffi::AVCodecContext) {
    if codec_ctx.is_null() {
        return;
    }
    if !(*codec_ctx).hw_frames_ctx.is_null() {
        ffi::av_buffer_unref(&mut (*codec_ctx).hw_frames_ctx);
    }
    if !(*codec_ctx).hw_device_ctx.is_null() {
        ffi::av_buffer_unref(&mut (*codec_ctx).hw_device_ctx);
    }
    (*codec_ctx).get_format = None;
    if !(*codec_ctx).opaque.is_null() {
        let _ = Box::from_raw((*codec_ctx).opaque as *mut ContextoFormatoHw);
        (*codec_ctx).opaque = ptr::null_mut();
    }
}

/// Libera `opaque` antes de `avcodec_free_context` (FFmpeg no libera ese puntero).
pub unsafe fn liberar_opaque(codec_ctx: *mut ffi::AVCodecContext) {
    if codec_ctx.is_null() {
        return;
    }
    if !(*codec_ctx).opaque.is_null() {
        let _ = Box::from_raw((*codec_ctx).opaque as *mut ContextoFormatoHw);
        (*codec_ctx).opaque = ptr::null_mut();
    }
}

/// Si `frame` está en memoria HW, copia a `estado.frame_cpu` y devuelve ese puntero.
pub unsafe fn frame_para_escala<'a>(
    estado: &'a EstadoHwDecode,
    frame: *mut ffi::AVFrame,
) -> Result<*mut ffi::AVFrame> {
    if frame.is_null() {
        return Err(anyhow!("frame nulo"));
    }
    if (*frame).format != estado.hw_pix_fmt as i32 {
        return Ok(frame);
    }

    ffi::av_frame_unref(estado.frame_cpu);
    let ret = ffi::av_hwframe_transfer_data(estado.frame_cpu, frame, 0);
    if ret < 0 {
        return Err(anyhow!("av_hwframe_transfer_data: {ret}"));
    }
    Ok(estado.frame_cpu)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn hw_off_respeta_entorno() {
        std::env::set_var("DIFFPLAYERQC_HW_DECODE", "0");
        assert!(hw_desactivado_por_entorno());
        std::env::remove_var("DIFFPLAYERQC_HW_DECODE");
    }
}
