//! Métricas objetivas A↔B (M8): SSIM, MSE y PSNR por fotograma alineado en PTS.
//!
//! El escaneo offline usa decoders temporales; la comparación en vivo usa buffers RGBA
//! del motor (mismas dimensiones tras escala en decode).

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::decoder;
use crate::types::{DecoderCommand, VideoFrame};

/// Muestras por segundo en escaneo offline (2 Hz ≈ 120 puntos/min).
pub const MUESTRAS_POR_SEGUNDO_DEFECTO: u32 = 2;

/// Ancho máximo al escanear (velocidad; no afecta al overlay de reproducción).
const ANCHO_ESCANEO: u32 = 320;

/// Umbral SSIM por debajo del cual se marca el punto como “caída” en UI.
pub const UMBRAL_SSIM_BAJO_DEFECTO: f32 = 0.92;

/// Un punto de la serie temporal.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PuntoMetrica {
    pub pts: f64,
    pub ssim: f32,
    /// MS-SSIM (tres escalas); correlaciona mejor con calidad percibida que SSIM global.
    pub ms_ssim: f32,
    pub psnr: f32,
    pub mse: f32,
    /// VMAF 0–100 si FFmpeg libvmaf estuvo disponible; `None` si no.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmaf: Option<f32>,
}

/// Serie completa tras escanear el par A/B.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerieMetricasVideo {
    pub puntos: Vec<PuntoMetrica>,
    pub duracion_secs: f64,
    pub muestras_por_segundo: u32,
    pub umbral_ssim_bajo: f32,
    /// Media VMAF pooled (FFmpeg); `None` si no se ejecutó libvmaf.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmaf_integrado: Option<f32>,
    pub vmaf_disponible_en_sistema: bool,
}

impl SerieMetricasVideo {
    pub fn vacia() -> Self {
        Self {
            puntos: Vec::new(),
            duracion_secs: 0.0,
            muestras_por_segundo: MUESTRAS_POR_SEGUNDO_DEFECTO,
            umbral_ssim_bajo: UMBRAL_SSIM_BAJO_DEFECTO,
            vmaf_integrado: None,
            vmaf_disponible_en_sistema: crate::vmaf_ffmpeg::vmaf_disponible(),
        }
    }

    /// PTS de fotogramas con SSIM por debajo del umbral.
    pub fn pts_caidas(&self) -> Vec<f64> {
        self.puntos
            .iter()
            .filter(|p| p.ssim < self.umbral_ssim_bajo)
            .map(|p| p.pts)
            .collect()
    }

    /// Índices de puntos con SSIM por debajo del umbral configurado.
    pub fn indices_caidas(&self) -> Vec<usize> {
        self.puntos
            .iter()
            .enumerate()
            .filter(|(_, p)| p.ssim < self.umbral_ssim_bajo)
            .map(|(i, _)| i)
            .collect()
    }
}

/// Compara dos fotogramas RGBA (se reescalan al tamaño común mínimo).
pub fn comparar_fotogramas(frame_a: &VideoFrame, frame_b: &VideoFrame) -> PuntoMetrica {
    let pts = frame_a.pts;
    let (buf_a, buf_b, w, h) = igualar_buffers(&frame_a.rgba_data, frame_a.width, frame_a.height, &frame_b.rgba_data, frame_b.width, frame_b.height);
    let (mse, psnr) = mse_psnr_rgba(&buf_a, &buf_b);
    let ssim = ssim_rgba(&buf_a, &buf_b, w, h);
    let ms_ssim = ms_ssim_rgba(&buf_a, &buf_b, w, h);
    PuntoMetrica {
        pts,
        ssim,
        ms_ssim,
        psnr,
        mse,
        vmaf: None,
    }
}

/// Escaneo offline: una muestra cada `1/muestras_por_segundo` hasta `min(duración A, B)`.
pub fn escanear_par(
    ruta_a: &str,
    ruta_b: &str,
    muestras_por_segundo: u32,
    mut al_progreso: Option<&mut dyn FnMut(f32)>,
) -> Result<SerieMetricasVideo> {
    let mps = muestras_por_segundo.max(1);
    let (tx_a, rx_a, _, meta_a, hw_a) =
        decoder::spawn_decoder(ruta_a, false, Some(ANCHO_ESCANEO)).context("decoder A métricas")?;
    let (tx_b, rx_b, _, meta_b, hw_b) =
        decoder::spawn_decoder(ruta_b, false, Some(ANCHO_ESCANEO)).context("decoder B métricas")?;
    drop(hw_a);
    drop(hw_b);

    let duracion = meta_a.duration_secs.min(meta_b.duration_secs).max(0.0);
    let fps = meta_a.fps.max(meta_b.fps).max(1.0);
    let num_muestras = ((duracion * mps as f64).ceil() as usize).max(1);
    let mut puntos = Vec::with_capacity(num_muestras);
    let vmaf_ok = crate::vmaf_ffmpeg::vmaf_disponible();

    for i in 0..num_muestras {
        let pts = (i as f64) / mps as f64;
        if let Some(cb) = al_progreso.as_deref_mut() {
            cb((i as f32 + 0.5) / num_muestras as f32);
        }

        let fa = fotograma_en_pts(&tx_a, &rx_a, pts)?;
        let fb = fotograma_en_pts(&tx_b, &rx_b, pts)?;
        if let (Some(a), Some(b)) = (fa, fb) {
            puntos.push(comparar_fotogramas(&a, &b));
        }
    }

    let _ = tx_a.send(DecoderCommand::Stop);
    let _ = tx_b.send(DecoderCommand::Stop);

    let mut vmaf_integrado = None;
    if vmaf_ok {
        let log = std::env::temp_dir().join(format!(
            "diffplayerqc-vmaf-{}.json",
            std::process::id()
        ));
        match crate::vmaf_ffmpeg::ejecutar_vmaf(ruta_a, ruta_b, &log) {
            Ok((por_frame, pooled)) => {
                crate::vmaf_ffmpeg::fusionar_vmaf_en_serie(&mut puntos, &por_frame, fps);
                vmaf_integrado = pooled;
            }
            Err(e) => {
                log::warn!("VMAF FFmpeg: {e:#}");
            }
        }
        let _ = std::fs::remove_file(&log);
    }

    Ok(SerieMetricasVideo {
        puntos,
        duracion_secs: duracion,
        muestras_por_segundo: mps,
        umbral_ssim_bajo: UMBRAL_SSIM_BAJO_DEFECTO,
        vmaf_integrado,
        vmaf_disponible_en_sistema: vmaf_ok,
    })
}

fn fotograma_en_pts(
    cmd_tx: &crossbeam_channel::Sender<DecoderCommand>,
    frame_rx: &crossbeam_channel::Receiver<VideoFrame>,
    pts: f64,
) -> Result<Option<VideoFrame>> {
    cmd_tx.send(DecoderCommand::Seek(pts))?;
    cmd_tx.send(DecoderCommand::StepForward)?;
    Ok(frame_rx
        .recv_timeout(Duration::from_secs(120))
        .ok())
}

fn igualar_buffers(
    a: &Arc<Vec<u8>>,
    aw: u32,
    ah: u32,
    b: &Arc<Vec<u8>>,
    bw: u32,
    bh: u32,
) -> (Vec<u8>, Vec<u8>, u32, u32) {
    let w = aw.min(bw).max(1);
    let h = ah.min(bh).max(1);
    let ra = reescalar_rgba(a.as_slice(), aw, ah, w, h);
    let rb = reescalar_rgba(b.as_slice(), bw, bh, w, h);
    (ra, rb, w, h)
}

/// Reescala RGBA8 con muestreo por vecino más cercano.
fn reescalar_rgba(origen: &[u8], ow: u32, oh: u32, dw: u32, dh: u32) -> Vec<u8> {
    if ow == dw && oh == dh {
        return origen.to_vec();
    }
    let mut out = vec![0u8; (dw * dh * 4) as usize];
    let ow = ow.max(1) as f64;
    let oh = oh.max(1) as f64;
    for y in 0..dh {
        for x in 0..dw {
            let sx = ((x as f64 + 0.5) * ow / dw as f64 - 0.5).clamp(0.0, ow - 1.0) as u32;
            let sy = ((y as f64 + 0.5) * oh / dh as f64 - 0.5).clamp(0.0, oh - 1.0) as u32;
            let si = ((sy * ow as u32 + sx) * 4) as usize;
            let di = ((y * dw + x) * 4) as usize;
            if si + 3 < origen.len() && di + 3 < out.len() {
                out[di..di + 4].copy_from_slice(&origen[si..si + 4]);
            }
        }
    }
    out
}

fn mse_psnr_rgba(a: &[u8], b: &[u8]) -> (f32, f32) {
    let n = a.len().min(b.len()) / 4;
    if n == 0 {
        return (0.0, f32::INFINITY);
    }
    let mut suma = 0.0f64;
    for i in 0..n {
        let j = i * 4;
        for c in 0..3 {
            let da = a[j + c] as f64 - b[j + c] as f64;
            suma += da * da;
        }
    }
    let mse = (suma / (n * 3) as f64) as f32;
    let psnr = if mse <= 1e-10 {
        f32::INFINITY
    } else {
        (255.0 * 255.0 / mse).log10() * 10.0
    };
    (mse, psnr)
}

/// SSIM global sobre canal de luma (Rec.709) en la imagen completa.
fn ssim_rgba(a: &[u8], b: &[u8], w: u32, h: u32) -> f32 {
    let n = (w * h) as usize;
    if n == 0 {
        return 1.0;
    }
    const C1: f64 = 6.5025; // (0.01 * 255)^2
    const C2: f64 = 58.5225; // (0.03 * 255)^2

    let mut sum_a = 0.0f64;
    let mut sum_b = 0.0f64;
    let mut sum_aa = 0.0f64;
    let mut sum_bb = 0.0f64;
    let mut sum_ab = 0.0f64;

    for i in 0..n {
        let j = i * 4;
        if j + 2 >= a.len() || j + 2 >= b.len() {
            break;
        }
        let ya = 0.2126 * a[j] as f64 + 0.7152 * a[j + 1] as f64 + 0.0722 * a[j + 2] as f64;
        let yb = 0.2126 * b[j] as f64 + 0.7152 * b[j + 1] as f64 + 0.0722 * b[j + 2] as f64;
        sum_a += ya;
        sum_b += yb;
        sum_aa += ya * ya;
        sum_bb += yb * yb;
        sum_ab += ya * yb;
    }

    let nf = n as f64;
    let mu_a = sum_a / nf;
    let mu_b = sum_b / nf;
    let var_a = (sum_aa / nf) - mu_a * mu_a;
    let var_b = (sum_bb / nf) - mu_b * mu_b;
    let cov = (sum_ab / nf) - mu_a * mu_b;

    let num = (2.0 * mu_a * mu_b + C1) * (2.0 * cov + C2);
    let den = (mu_a * mu_a + mu_b * mu_b + C1) * (var_a + var_b + C2);
    if den.abs() < 1e-12 {
        return 1.0;
    }
    (num / den).clamp(0.0, 1.0) as f32
}

/// MS-SSIM: producto de SSIM en tres escalas (imagen reducida ×½ cada vez).
fn ms_ssim_rgba(a: &[u8], b: &[u8], w: u32, h: u32) -> f32 {
    let mut buf_a = a.to_vec();
    let mut buf_b = b.to_vec();
    let mut cw = w.max(1);
    let mut ch = h.max(1);
    let mut producto = 1.0f32;
    let mut escalas = 0u32;

    for _ in 0..3 {
        if cw < 4 || ch < 4 {
            break;
        }
        producto *= ssim_rgba(&buf_a, &buf_b, cw, ch);
        escalas += 1;
        if cw < 8 || ch < 8 {
            break;
        }
        let nw = (cw / 2).max(2);
        let nh = (ch / 2).max(2);
        buf_a = reescalar_rgba(&buf_a, cw, ch, nw, nh);
        buf_b = reescalar_rgba(&buf_b, cw, ch, nw, nh);
        cw = nw;
        ch = nh;
    }

    if escalas == 0 {
        return 1.0;
    }
    producto.powf(1.0 / escalas as f32)
}

/// Serializa la serie a CSV (cabecera incluida).
pub fn exportar_csv(serie: &SerieMetricasVideo) -> String {
    let mut out = String::from("pts,ssim,ms_ssim,psnr,mse,vmaf\n");
    for p in &serie.puntos {
        let vmaf = p
            .vmaf
            .map(|v| format!("{v:.4}"))
            .unwrap_or_else(String::new);
        out.push_str(&format!(
            "{:.6},{:.6},{:.6},{:.6},{:.6},{}\n",
            p.pts, p.ssim, p.ms_ssim, p.psnr, p.mse, vmaf
        ));
    }
    out
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::sync::Arc;

    fn frame_solido(w: u32, h: u32, r: u8, g: u8, b: u8) -> VideoFrame {
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        for i in 0..(w * h) as usize {
            let j = i * 4;
            rgba[j] = r;
            rgba[j + 1] = g;
            rgba[j + 2] = b;
            rgba[j + 3] = 255;
        }
        VideoFrame {
            pts: 0.0,
            rgba_data: Arc::new(rgba),
            width: w,
            height: h,
        }
    }

    #[test]
    fn ssim_identico_es_uno() {
        let a = frame_solido(64, 64, 128, 64, 32);
        let b = frame_solido(64, 64, 128, 64, 32);
        let p = comparar_fotogramas(&a, &b);
        assert!(p.ssim > 0.99);
        assert!(p.mse < 1.0);
    }

    #[test]
    fn ssim_distinto_es_menor() {
        let a = frame_solido(64, 64, 0, 0, 0);
        let b = frame_solido(64, 64, 255, 255, 255);
        let p = comparar_fotogramas(&a, &b);
        assert!(p.ssim < 0.5);
        assert!(p.ms_ssim <= p.ssim + 0.01);
    }

    #[test]
    fn ms_ssim_identico_alto() {
        let a = frame_solido(128, 72, 100, 50, 20);
        let b = frame_solido(128, 72, 100, 50, 20);
        let p = comparar_fotogramas(&a, &b);
        assert!(p.ms_ssim > 0.99);
    }
}
