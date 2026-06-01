//! VMAF opcional vía FFmpeg `libvmaf` (si el binario del sistema lo incluye).
//!
//! No es dependencia de compilación: si `ffmpeg -filters` no lista `libvmaf`,
//! el escaneo M8 sigue con SSIM/MS-SSIM únicamente.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

/// Comprueba si `ffmpeg` en PATH expone el filtro libvmaf.
pub fn vmaf_disponible() -> bool {
    let salida = Command::new("ffmpeg").args(["-hide_banner", "-filters"]).output();
    match salida {
        Ok(o) => {
            let texto = String::from_utf8_lossy(&o.stdout);
            texto.contains("libvmaf")
        }
        Err(_) => false,
    }
}

#[derive(Debug, Deserialize)]
struct RaizLogVmaf {
    frames: Option<Vec<FrameVmaf>>,
    #[serde(alias = "pooled_metrics")]
    aggregated: Option<PooledVmaf>,
}

#[derive(Debug, Deserialize)]
struct FrameVmaf {
    #[serde(rename = "frameNum")]
    frame_num: u64,
    metrics: MetricasFrameVmaf,
}

#[derive(Debug, Deserialize)]
struct MetricasFrameVmaf {
    vmaf: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct PooledVmaf {
    vmaf: Option<f64>,
}

/// Ejecuta libvmaf entre `ruta_referencia` (A) y `ruta_distorsionado` (B).
/// Devuelve puntuación por número de frame y media pooled (0–100).
pub fn ejecutar_vmaf(
    ruta_referencia: &str,
    ruta_distorsionado: &str,
    log_json: &Path,
) -> Result<(Vec<(u64, f32)>, Option<f32>)> {
    if log_json.exists() {
        let _ = std::fs::remove_file(log_json);
    }

    let filtro = format!(
        "libvmaf=log_fmt=json:log_path={}:n_threads=2",
        log_json.display()
    );

    let estado = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-i",
            ruta_distorsionado,
            "-i",
            ruta_referencia,
            "-lavfi",
            &filtro,
            "-f",
            "null",
            "-",
        ])
        .status()
        .context("ejecutar ffmpeg libvmaf")?;

    if !estado.success() {
        return Err(anyhow!("ffmpeg libvmaf terminó con código {:?}", estado.code()));
    }

    let texto = std::fs::read_to_string(log_json).context("leer log vmaf")?;
    parsear_log_vmaf(&texto)
}

fn parsear_log_vmaf(texto: &str) -> Result<(Vec<(u64, f32)>, Option<f32>)> {
    let raiz: RaizLogVmaf = serde_json::from_str(texto).context("parsear JSON vmaf")?;
    let mut por_frame = Vec::new();
    if let Some(frames) = raiz.frames {
        for f in frames {
            if let Some(v) = f.metrics.vmaf {
                por_frame.push((f.frame_num, v.clamp(0.0, 100.0) as f32));
            }
        }
    }
    let media = raiz
        .aggregated
        .and_then(|p| p.vmaf)
        .map(|v| v.clamp(0.0, 100.0) as f32);
    Ok((por_frame, media))
}

/// Fusiona puntuaciones VMAF (por índice de frame) en puntos muestreados por PTS.
pub fn fusionar_vmaf_en_serie(
    puntos: &mut [crate::metricas_video::PuntoMetrica],
    por_frame: &[(u64, f32)],
    fps: f64,
) {
    if puntos.is_empty() || por_frame.is_empty() || fps <= 0.0 {
        return;
    }
    for p in puntos.iter_mut() {
        let idx = (p.pts * fps).round() as u64;
        let vmaf = por_frame
            .iter()
            .find(|(n, _)| *n == idx)
            .map(|(_, v)| *v)
            .or_else(|| {
                // Interpolación por vecino más cercano en frameNum
                por_frame
                    .iter()
                    .min_by_key(|(n, _)| (*n as i64 - idx as i64).unsigned_abs())
                    .map(|(_, v)| *v)
            });
        p.vmaf = vmaf;
    }
}
