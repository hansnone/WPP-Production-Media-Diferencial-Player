//! Vista previa JPEG para el webview (vía `playback-tick` y evento opcional).

use std::io::Cursor;

use base64::{engine::general_purpose::STANDARD, Engine};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, RgbaImage};
use tauri::{AppHandle, Emitter};

const ANCHO_MAX_PREVIA: u32 = 960;
/// Resolución del fallback canvas (mientras la overlay GPU arranca).
const ANCHO_MAX_REPRODUCCION: u32 = 1280;
const CALIDAD_JPEG_REPRODUCCION: u8 = 88;

/// Payload del evento `vista-previa` hacia el frontend.
#[derive(Clone, serde::Serialize)]
pub struct VistaPrevia {
    pub b64: String,
    pub ancho: u32,
    pub alto: u32,
}

/// Dimensiones tras redimensionar (reproducción o vista fija).
pub fn dimensiones_salida_reproduccion(ancho: u32, alto: u32) -> (u32, u32) {
    dimensiones_con_tope(ancho, alto, ANCHO_MAX_REPRODUCCION)
}

/// JPEG base64 optimizado para reproducción en canvas (menor resolución).
pub fn codificar_base64_jpeg_reproduccion(
    rgba: &[u8],
    ancho: u32,
    alto: u32,
) -> Option<(String, u32, u32)> {
    codificar_base64_jpeg_interno(rgba, ancho, alto, ANCHO_MAX_REPRODUCCION, CALIDAD_JPEG_REPRODUCCION)
}

/// Codifica RGBA → JPEG base64 para el snapshot de reproducción.
pub fn codificar_base64_jpeg(rgba: &[u8], ancho: u32, alto: u32) -> Option<(String, u32, u32)> {
    codificar_base64_jpeg_interno(rgba, ancho, alto, ANCHO_MAX_PREVIA, 85)
}

fn codificar_base64_jpeg_interno(
    rgba: &[u8],
    ancho: u32,
    alto: u32,
    ancho_max: u32,
    calidad: u8,
) -> Option<(String, u32, u32)> {
    if ancho == 0 || alto == 0 {
        return None;
    }
    let esperado = (ancho as usize) * (alto as usize) * 4;
    if rgba.len() < esperado {
        log::warn!(
            "vista-previa: buffer RGBA corto ({} < {esperado}) para {ancho}x{alto}",
            rgba.len()
        );
        return None;
    }
    let jpeg = codificar_jpeg_redimensionado(rgba, ancho, alto, ancho_max, calidad)?;
    let (ancho_out, alto_out) = dimensiones_con_tope(ancho, alto, ancho_max);
    Some((STANDARD.encode(&jpeg), ancho_out, alto_out))
}

fn dimensiones_salida(ancho: u32, alto: u32) -> (u32, u32) {
    dimensiones_con_tope(ancho, alto, ANCHO_MAX_PREVIA)
}

fn dimensiones_con_tope(ancho: u32, alto: u32, ancho_max: u32) -> (u32, u32) {
    if ancho > ancho_max {
        let escala = ancho_max as f32 / ancho as f32;
        (
            ancho_max,
            (alto as f32 * escala).round().max(1.0) as u32,
        )
    } else {
        (ancho, alto)
    }
}

/// Emite JPEG al webview (respaldo; el canal principal es `playback-tick`).
pub fn emitir_si_corresponde(app: &AppHandle, rgba: &[u8], ancho: u32, alto: u32) {
    let Some((b64, ancho_out, alto_out)) = codificar_base64_jpeg(rgba, ancho, alto) else {
        return;
    };
    let payload = VistaPrevia {
        b64,
        ancho: ancho_out,
        alto: alto_out,
    };
    if let Err(e) = app.emit("vista-previa", payload) {
        log::warn!("vista-previa: emit falló: {e}");
    }
}

fn codificar_jpeg_redimensionado(
    rgba: &[u8],
    ancho: u32,
    alto: u32,
    ancho_max: u32,
    calidad: u8,
) -> Option<Vec<u8>> {
    let img: RgbaImage = ImageBuffer::from_raw(ancho, alto, rgba.to_vec())?;
    let (ancho_dst, alto_dst) = dimensiones_con_tope(ancho, alto, ancho_max);
    let redim = image::imageops::resize(&img, ancho_dst, alto_dst, FilterType::Triangle);
    let rgb = DynamicImage::ImageRgba8(redim).into_rgb8();
    let mut salida = Vec::new();
    let mut enc = JpegEncoder::new_with_quality(Cursor::new(&mut salida), calidad);
    enc.encode(
        rgb.as_raw(),
        ancho_dst,
        alto_dst,
        image::ExtendedColorType::Rgb8,
    )
    .ok()?;
    Some(salida)
}
