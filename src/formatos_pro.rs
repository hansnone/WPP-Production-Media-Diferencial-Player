//! Formatos profesionales ProRes / DNxHR y metadatos de color (M12).

use ffmpeg_sys_next as ffi;

/// Códecs de mastering que suelen ir en 10/12 bit y requieren rango legal/full correcto.
pub fn es_codec_profesional(codec_id: ffi::AVCodecID) -> bool {
    matches!(
        codec_id,
        ffi::AVCodecID::AV_CODEC_ID_PRORES | ffi::AVCodecID::AV_CODEC_ID_DNXHD
    )
}

/// Etiqueta corta para logs y UI (DNxHR comparte `AV_CODEC_ID_DNXHD` en FFmpeg).
pub fn etiqueta_codec_profesional(codec_id: ffi::AVCodecID) -> Option<&'static str> {
    match codec_id {
        ffi::AVCodecID::AV_CODEC_ID_PRORES => Some("ProRes"),
        ffi::AVCodecID::AV_CODEC_ID_DNXHD => Some("DNxHD/DNxHR"),
        _ => None,
    }
}

/// ¿El `AVPixelFormat` es plano alto bit depth (10/12/16)?
pub fn es_pix_fmt_alto_bitdepth(pix_fmt: ffi::AVPixelFormat) -> bool {
    let desc = unsafe { ffi::av_pix_fmt_desc_get(pix_fmt) };
    if desc.is_null() {
        return false;
    }
    unsafe { (*desc).comp[0].depth > 8 }
}

/// Nombre legible del rango de color (MPEG = legal/TV, JPEG = full).
pub fn color_range_str(range: ffi::AVColorRange) -> String {
    match range {
        ffi::AVColorRange::AVCOL_RANGE_MPEG => "mpeg".into(),
        ffi::AVColorRange::AVCOL_RANGE_JPEG => "jpeg".into(),
        ffi::AVColorRange::AVCOL_RANGE_UNSPECIFIED => "unspecified".into(),
        _ => "unknown".into(),
    }
}

/// Coeficientes swscale según primarias (Rec.709 vs Rec.2020).
pub fn coeficientes_sws(primaries: ffi::AVColorPrimaries) -> i32 {
    match primaries {
        ffi::AVColorPrimaries::AVCOL_PRI_BT2020 => ffi::SWS_CS_BT2020,
        _ => ffi::SWS_CS_ITU709,
    }
}

/// Aplica rango legal/full y matriz de color al contexto `sws` (después de `sws_getContext`).
///
/// # Safety
/// `sws` debe ser un `SwsContext` válido recién creado.
pub unsafe fn aplicar_detalles_color_sws(
    sws: *mut ffi::SwsContext,
    primaries: ffi::AVColorPrimaries,
    range: ffi::AVColorRange,
) {
    if sws.is_null() {
        return;
    }
    let coefs = coeficientes_sws(primaries);
  // 1 = JPEG/full, 0 = MPEG/legal en destino RGBA queremos full
    let src_range = if range == ffi::AVColorRange::AVCOL_RANGE_JPEG {
        1
    } else {
        0
    };
    let dst_range = 1;
    let ret = ffi::sws_setColorspaceDetails(
        sws,
        ffi::sws_getCoefficients(coefs),
        src_range,
        ffi::sws_getCoefficients(coefs),
        dst_range,
        0,
        1 << 16,
        1 << 16,
    );
    if ret < 0 {
        log::warn!("sws_setColorspaceDetails falló ({ret}); color puede verse lavado");
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn detecta_prores() {
        assert!(es_codec_profesional(ffi::AVCodecID::AV_CODEC_ID_PRORES));
        assert!(!es_codec_profesional(ffi::AVCodecID::AV_CODEC_ID_H264));
    }

    #[test]
    fn rango_mpeg_vs_jpeg() {
        assert_eq!(
            color_range_str(ffi::AVColorRange::AVCOL_RANGE_MPEG),
            "mpeg"
        );
        assert_eq!(
            color_range_str(ffi::AVColorRange::AVCOL_RANGE_JPEG),
            "jpeg"
        );
    }
}
