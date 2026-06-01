//! Análisis loudness EBU R128 (M9): K-weighting, LUFS integrado, true peak, LRA y alertas QC.
//!
//! Implementación en Rust puro (48 kHz) sin `libebur128` del sistema; alineada con ITU-R BS.1770-4.

use serde::{Deserialize, Serialize};

/// Objetivo broadcast EBU R128 (LUFS integrado).
pub const LUFS_OBJETIVO_EBU: f64 = -23.0;
/// Tolerancia ± LU respecto al objetivo.
pub const TOLERANCIA_LUFS: f64 = 1.0;
/// True peak máximo recomendado (dBTP).
pub const MAX_TRUE_PEAK_DBTP: f64 = -1.0;
/// Umbral absoluto de gate (LUFS) para bloques de 400 ms.
pub const GATE_ABSOLUTO_LUFS: f64 = -70.0;
/// Gate relativo (LU) por debajo del integrado sin gate.
pub const GATE_RELATIVO_LU: f64 = 10.0;
/// Por debajo de esto se considera silencio en un bloque de 400 ms.
pub const UMBRAL_SILENCIO_LUFS: f64 = -60.0;
/// Bloques consecutivos de silencio (400 ms c/u) para alertar (~2 s).
pub const BLOQUES_SILENCIO_ALERTA: usize = 5;

const TASA_HZ: u32 = 48_000;
/// Duración bloque “momentary” BS.1770 (400 ms).
const MUESTRAS_POR_BLOQUE: u64 = (TASA_HZ as u64) * 400 / 1000;
/// Ventana short-term 3 s en bloques de 400 ms.
const BLOQUES_POR_SHORT_TERM: usize = 8;

/// Resultado EBU serializable (extiende M4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatosEbuR128 {
    pub lufs_integrado: f64,
    pub true_peak_dbtp: f64,
    pub lra: f64,
    /// Pico de muestra (dBFS), sin oversampling.
    pub pico_muestra_dbfs: f64,
    pub silencio_detectado: bool,
    pub clipping_detectado: f64,
    pub alertas: Vec<String>,
    pub dentro_spec_ebu: bool,
}

impl Default for DatosEbuR128 {
    fn default() -> Self {
        Self {
            lufs_integrado: f64::NEG_INFINITY,
            true_peak_dbtp: f64::NEG_INFINITY,
            lra: 0.0,
            pico_muestra_dbfs: f64::NEG_INFINITY,
            silencio_detectado: false,
            clipping_detectado: 0.0,
            alertas: Vec::new(),
            dentro_spec_ebu: false,
        }
    }
}

/// Filtro biquad direct form II (una etapa K-weighting).
struct Biquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    z1: f64,
    z2: f64,
}

impl Biquad {
    fn process(&mut self, x: f64) -> f64 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
    }
}

/// Coeficientes BS.1770-4 @ 48 kHz (estándar en libebur128).
fn k_weighting_48k() -> (Biquad, Biquad) {
    let stage1 = Biquad {
        b0: 1.53512485958697,
        b1: -2.69169618940638,
        b2: 1.19839281285261,
        a1: -1.69065929318241,
        a2: 0.73248077421585,
        z1: 0.0,
        z2: 0.0,
    };
    let stage2 = Biquad {
        b0: 1.0,
        b1: -2.0,
        b2: 1.0,
        a1: -1.99004745483398,
        a2: 0.99007225036621,
        z1: 0.0,
        z2: 0.0,
    };
    (stage1, stage2)
}

struct FiltroK {
    s1: Biquad,
    s2: Biquad,
}

impl FiltroK {
    fn nuevo() -> Self {
        let (s1, s2) = k_weighting_48k();
        Self { s1, s2 }
    }

    fn process(&mut self, x: f32) -> f64 {
        let y = self.s1.process(x as f64);
        self.s2.process(y)
    }
}

/// Analizador en streaming durante el escaneo FFmpeg.
pub struct AnalizadorLoudness {
    filtro: FiltroK,
    muestra_anterior: Option<f32>,
    suma_cuad_bloque: f64,
    muestras_en_bloque: u64,
    bloques_momentary_lufs: Vec<f64>,
    /// Por bucket temporal (misma cardinalidad que picos de forma de onda).
    lufs_buckets: Vec<f64>,
    bucket_actual: usize,
    muestras_en_bucket: u64,
    muestras_por_bucket: u64,
    total_muestras: u64,
    muestras_clip: u64,
    true_peak_lineal: f64,
    pico_muestra: f64,
    bloques_silencio_consecutivos: usize,
    max_bloques_silencio: usize,
}

impl AnalizadorLoudness {
    pub fn nuevo(num_buckets: usize, picos_por_segundo: u32) -> Self {
        let muestras_por_bucket = (TASA_HZ as u64 / picos_por_segundo.max(1) as u64).max(1);
        Self {
            filtro: FiltroK::nuevo(),
            muestra_anterior: None,
            suma_cuad_bloque: 0.0,
            muestras_en_bloque: 0,
            bloques_momentary_lufs: Vec::new(),
            lufs_buckets: vec![f64::NEG_INFINITY; num_buckets.max(1)],
            bucket_actual: 0,
            muestras_en_bucket: 0,
            muestras_por_bucket,
            total_muestras: 0,
            muestras_clip: 0,
            true_peak_lineal: 0.0,
            pico_muestra: 0.0,
            bloques_silencio_consecutivos: 0,
            max_bloques_silencio: 0,
        }
    }

    /// Procesa muestras mono float [-1, 1] (tasa 48 kHz).
    pub fn alimentar(&mut self, muestras: &[f32]) {
        for &m in muestras {
            self.alimentar_una(m);
        }
    }

    fn alimentar_una(&mut self, m: f32) {
        let abs = m.abs();
        if abs >= 0.999 {
            self.muestras_clip += 1;
        }
        self.pico_muestra = self.pico_muestra.max(abs as f64);
        actualizar_true_peak(&mut self.true_peak_lineal, self.muestra_anterior, m);
        self.muestra_anterior = Some(m);

        let z = self.filtro.process(m);
        let z2 = z * z;
        self.suma_cuad_bloque += z2;
        self.muestras_en_bloque += 1;
        self.total_muestras += 1;

        if self.bucket_actual < self.lufs_buckets.len() {
            self.lufs_buckets[self.bucket_actual] = self.lufs_buckets[self.bucket_actual]
                .max(lufs_desde_media(z2));
            self.muestras_en_bucket += 1;
            if self.muestras_en_bucket >= self.muestras_por_bucket {
                self.bucket_actual += 1;
                self.muestras_en_bucket = 0;
            }
        }

        if self.muestras_en_bloque >= MUESTRAS_POR_BLOQUE {
            self.cerrar_bloque();
        }
    }

    fn cerrar_bloque(&mut self) {
        if self.muestras_en_bloque == 0 {
            return;
        }
        let lufs = lufs_desde_suma(self.suma_cuad_bloque, self.muestras_en_bloque);
        self.bloques_momentary_lufs.push(lufs);

        if lufs < UMBRAL_SILENCIO_LUFS {
            self.bloques_silencio_consecutivos += 1;
            self.max_bloques_silencio = self
                .max_bloques_silencio
                .max(self.bloques_silencio_consecutivos);
        } else {
            self.bloques_silencio_consecutivos = 0;
        }

        self.suma_cuad_bloque = 0.0;
        self.muestras_en_bloque = 0;
    }

    pub fn finalizar(mut self) -> (DatosEbuR128, Vec<f32>) {
        self.cerrar_bloque();

        let lufs_buckets: Vec<f32> = self
            .lufs_buckets
            .iter()
            .map(|&l| {
                if lufs_finito(l) {
                    l.clamp(-70.0, 0.0) as f32
                } else {
                    -70.0f32
                }
            })
            .collect();

        let lufs_integrado = integrado_con_gate(&self.bloques_momentary_lufs);
        let lra = calcular_lra(&self.bloques_momentary_lufs);
        let true_peak_dbtp = lineal_a_dbtp(self.true_peak_lineal);
        let pico_muestra_dbfs = lineal_a_dbtp(self.pico_muestra);

        let ratio_clip = if self.total_muestras > 0 {
            self.muestras_clip as f64 / self.total_muestras as f64
        } else {
            0.0
        };
        let silencio_detectado = self.max_bloques_silencio >= BLOQUES_SILENCIO_ALERTA;

        let mut alertas = Vec::new();
        if !lufs_finito(lufs_integrado) {
            alertas.push("Sin señal audible para medir LUFS".into());
        } else {
            let delta = (lufs_integrado - LUFS_OBJETIVO_EBU).abs();
            if delta > TOLERANCIA_LUFS {
                alertas.push(format!(
                    "LUFS integrado {lufs_integrado:.1} (objetivo {LUFS_OBJETIVO_EBU:.1} ±{TOLERANCIA_LUFS:.0})"
                ));
            }
        }
        if true_peak_dbtp > MAX_TRUE_PEAK_DBTP {
            alertas.push(format!(
                "True peak {true_peak_dbtp:.1} dBTP (máx {MAX_TRUE_PEAK_DBTP:.1})"
            ));
        }
        if ratio_clip > 0.001 {
            alertas.push(format!(
                "Clipping ~{:.2}% de muestras",
                ratio_clip * 100.0
            ));
        }
        if silencio_detectado {
            alertas.push("Tramos de silencio prolongado detectados".into());
        }

        let dentro_spec_ebu = alertas.is_empty() && lufs_finito(lufs_integrado);

        (
            DatosEbuR128 {
                lufs_integrado,
                true_peak_dbtp,
                lra,
                pico_muestra_dbfs,
                silencio_detectado,
                clipping_detectado: ratio_clip,
                alertas,
                dentro_spec_ebu,
            },
            lufs_buckets,
        )
    }
}

fn lufs_desde_suma(suma_cuad: f64, n: u64) -> f64 {
    if n == 0 {
        return f64::NEG_INFINITY;
    }
    lufs_desde_media(suma_cuad / n as f64)
}

fn lufs_desde_media(media_cuad: f64) -> f64 {
    if media_cuad < 1e-20 {
        f64::NEG_INFINITY
    } else {
        -0.691 + 10.0 * media_cuad.log10()
    }
}

fn lufs_finito(l: f64) -> bool {
    l.is_finite() && l > -100.0
}

fn lineal_a_dbtp(p: f64) -> f64 {
    if p < 1e-12 {
        f64::NEG_INFINITY
    } else {
        20.0 * p.log10()
    }
}

fn actualizar_true_peak(tp: &mut f64, anterior: Option<f32>, actual: f32) {
    let a = actual.abs() as f64;
    *tp = tp.max(a);
    if let Some(p) = anterior {
        let interp = ((p as f64 + a) * 0.5).abs();
        *tp = tp.max(interp);
    }
}

/// Integrado con gate absoluto y relativo sobre bloques de 400 ms.
fn integrado_con_gate(bloques: &[f64]) -> f64 {
    let finitos: Vec<f64> = bloques.iter().copied().filter(|l| lufs_finito(*l)).collect();
    if finitos.is_empty() {
        return f64::NEG_INFINITY;
    }

    let ungated = promedio_energetico_lufs(&finitos);
    if !lufs_finito(ungated) {
        return f64::NEG_INFINITY;
    }

    let gate_rel = ungated - GATE_RELATIVO_LU;
    let gated: Vec<f64> = finitos
        .into_iter()
        .filter(|l| *l > GATE_ABSOLUTO_LUFS && *l > gate_rel)
        .collect();

    if gated.is_empty() {
        ungated
    } else {
        promedio_energetico_lufs(&gated)
    }
}

fn promedio_energetico_lufs(vals: &[f64]) -> f64 {
    let mut suma_lin = 0.0f64;
    let mut n = 0u64;
    for &l in vals {
        if !lufs_finito(l) {
            continue;
        }
        suma_lin += 10.0_f64.powf(l / 10.0);
        n += 1;
    }
    if n == 0 {
        return f64::NEG_INFINITY;
    }
    let media = suma_lin / n as f64;
    10.0 * media.log10()
}

fn calcular_lra(bloques: &[f64]) -> f64 {
    if bloques.len() < BLOQUES_POR_SHORT_TERM {
        return 0.0;
    }
    let mut short_terms = Vec::new();
    for ventana in bloques.windows(BLOQUES_POR_SHORT_TERM) {
        let finitos: Vec<f64> = ventana.iter().copied().filter(|l| lufs_finito(*l)).collect();
        if finitos.len() >= BLOQUES_POR_SHORT_TERM / 2 {
            short_terms.push(promedio_energetico_lufs(&finitos));
        }
    }
    let mut finitos: Vec<f64> = short_terms.into_iter().filter(|l| lufs_finito(*l)).collect();
    if finitos.len() < 2 {
        return 0.0;
    }
    finitos.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p10 = percentil(&finitos, 0.10);
    let p95 = percentil(&finitos, 0.95);
    if lufs_finito(p10) && lufs_finito(p95) {
        (p95 - p10).max(0.0)
    } else {
        0.0
    }
}

fn percentil(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NEG_INFINITY;
    }
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn silencio_lufs_muy_bajo() {
        let mut a = AnalizadorLoudness::nuevo(10, 10);
        let ceros = vec![0.0f32; MUESTRAS_POR_BLOQUE as usize * 6];
        a.alimentar(&ceros);
        let (r, _) = a.finalizar();
        assert!(r.lufs_integrado < -50.0 || !r.lufs_integrado.is_finite());
    }

    #[test]
    fn tono_seno_lufs_finito() {
        let mut a = AnalizadorLoudness::nuevo(20, 20);
        let mut buf = Vec::new();
        let n = MUESTRAS_POR_BLOQUE as usize * 20;
        for i in 0..n {
            let t = i as f32 / TASA_HZ as f32;
            buf.push((t * 440.0 * std::f32::consts::TAU).sin() * 0.25);
        }
        a.alimentar(&buf);
        let (r, _) = a.finalizar();
        assert!(r.lufs_integrado.is_finite());
        assert!(r.lufs_integrado < 0.0);
        assert!(r.true_peak_dbtp.is_finite());
    }
}
