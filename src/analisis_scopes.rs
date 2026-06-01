//! Análisis de imagen para scopes de QC (M5): histograma RGB, vectoscopio y monitor de luma.

use serde::{Deserialize, Serialize};

/// Resolución del mapa de densidad del vectoscopio (Cb × Cr).
pub const TAM_VECTOSCOPIO: usize = 128;

/// Columnas del monitor de forma de onda de luminancia.
pub const COLUMNAS_MONITOR_LUMA: usize = 256;

/// Resultado serializable enviado al frontend vía IPC/evento.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopesFrame {
    pub pts: f64,
    /// Canal de origen: "a" o "b".
    pub canal: String,
    pub histograma_r: Vec<u32>,
    pub histograma_g: Vec<u32>,
    pub histograma_b: Vec<u32>,
    /// Densidad aplanada fila×columna (128×128).
    pub vectoscopio: Vec<u32>,
    /// Máximo Y por columna (0..1), longitud 256.
    pub monitor_luma: Vec<f32>,
}

impl ScopesFrame {
    pub fn vacio(pts: f64, canal: &str) -> Self {
        Self {
            pts,
            canal: canal.to_string(),
            histograma_r: vec![0; 256],
            histograma_g: vec![0; 256],
            histograma_b: vec![0; 256],
            vectoscopio: vec![0; TAM_VECTOSCOPIO * TAM_VECTOSCOPIO],
            monitor_luma: vec![0.0; COLUMNAS_MONITOR_LUMA],
        }
    }
}

/// Calcula scopes a partir de un buffer RGBA8 (submuestreo para rendimiento en 1080p).
pub fn calcular_desde_rgba(
    rgba: &[u8],
    ancho: u32,
    alto: u32,
    pts: f64,
    canal: &str,
) -> ScopesFrame {
    let mut hist_r = [0u32; 256];
    let mut hist_g = [0u32; 256];
    let mut hist_b = [0u32; 256];
    let mut vect = vec![0u32; TAM_VECTOSCOPIO * TAM_VECTOSCOPIO];
    let mut monitor = vec![0.0f32; COLUMNAS_MONITOR_LUMA];

    let w = ancho.max(1) as usize;
    let h = alto.max(1) as usize;
    if rgba.is_empty() || w == 0 || h == 0 {
        return ScopesFrame::vacio(pts, canal);
    }

    let stride = w * 4;
    // Cada 4 píxeles en X/Y: ~16× menos trabajo en 1280×720.
    let paso = 4usize;

    for y in (0..h).step_by(paso) {
        let fila = y * stride;
        for x in (0..w).step_by(paso) {
            let i = fila + x * 4;
            if i + 3 >= rgba.len() {
                continue;
            }
            let r = rgba[i] as f32 / 255.0;
            let g = rgba[i + 1] as f32 / 255.0;
            let b = rgba[i + 2] as f32 / 255.0;

            hist_r[rgba[i] as usize] += 1;
            hist_g[rgba[i + 1] as usize] += 1;
            hist_b[rgba[i + 2] as usize] += 1;

            // Rec.709 luma
            let y709 = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            let col = (x * COLUMNAS_MONITOR_LUMA / w).min(COLUMNAS_MONITOR_LUMA - 1);
            if y709 > monitor[col] {
                monitor[col] = y709;
            }

            // Cb/Cr BT.709 para vectoscopio
            let cb = (-0.168736 * r - 0.331264 * g + 0.5 * b).clamp(-0.5, 0.5);
            let cr = (0.5 * r - 0.418688 * g - 0.081312 * b).clamp(-0.5, 0.5);
            let ix = ((cb + 0.5) * (TAM_VECTOSCOPIO as f32 - 0.01)) as usize;
            let iy = ((cr + 0.5) * (TAM_VECTOSCOPIO as f32 - 0.01)) as usize;
            let idx = iy.min(TAM_VECTOSCOPIO - 1) * TAM_VECTOSCOPIO
                + ix.min(TAM_VECTOSCOPIO - 1);
            vect[idx] = vect[idx].saturating_add(1);
        }
    }

    ScopesFrame {
        pts,
        canal: canal.to_string(),
        histograma_r: hist_r.to_vec(),
        histograma_g: hist_g.to_vec(),
        histograma_b: hist_b.to_vec(),
        vectoscopio: vect,
        monitor_luma: monitor,
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn histograma_pixel_blanco() {
        let rgba = [255u8, 255, 255, 255];
        let s = calcular_desde_rgba(&rgba, 1, 1, 0.0, "a");
        assert_eq!(s.histograma_r[255], 1);
        assert_eq!(s.histograma_g[255], 1);
        assert_eq!(s.histograma_b[255], 1);
        assert!(s.monitor_luma.iter().any(|&v| v > 0.99));
    }
}
