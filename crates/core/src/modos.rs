//! Modos de comparación visual y algoritmos de diferencia (alineados con `compare.wgsl`).

use serde::{Deserialize, Serialize};

/// Modo de visualización A/B en el workspace Compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum CompareMode {
    SplitScreen = 0,
    AbsDiff = 1,
    Heatmap = 2,
    SideBySide = 3,
}

impl Default for CompareMode {
    fn default() -> Self {
        Self::SplitScreen
    }
}

/// Algoritmo de diferencia cuando `CompareMode::AbsDiff` o filtros en SideBySide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
pub enum DiffMode {
    LegacyAbs = 0,
    AbsLinear = 1,
    AbsSqrt = 2,
    SignedDiverging = 3,
    None = 4,
}

impl Default for DiffMode {
    fn default() -> Self {
        Self::AbsLinear
    }
}

/// Avanza al siguiente modo de comparación (atajo de teclado / toolbar).
#[must_use]
pub fn ciclar_modo_comparacion(actual: CompareMode) -> CompareMode {
    match actual {
        CompareMode::SplitScreen => CompareMode::AbsDiff,
        CompareMode::AbsDiff => CompareMode::Heatmap,
        CompareMode::Heatmap => CompareMode::SideBySide,
        CompareMode::SideBySide => CompareMode::SplitScreen,
    }
}

/// Lista de modos de diff aplicables según el modo de comparación activo.
#[must_use]
pub fn modos_diferencia_validos(modo: CompareMode) -> &'static [DiffMode] {
    match modo {
        CompareMode::AbsDiff => &[
            DiffMode::LegacyAbs,
            DiffMode::AbsLinear,
            DiffMode::AbsSqrt,
            DiffMode::SignedDiverging,
        ],
        CompareMode::SideBySide => &[
            DiffMode::None,
            DiffMode::LegacyAbs,
            DiffMode::AbsLinear,
            DiffMode::AbsSqrt,
            DiffMode::SignedDiverging,
        ],
        CompareMode::SplitScreen | CompareMode::Heatmap => &[],
    }
}

/// Si el diff actual no es válido para el modo, devuelve el primero válido o el default global.
#[must_use]
pub fn normalizar_modo_diferencia(modo_comparacion: CompareMode, diff: DiffMode) -> DiffMode {
    let validos = modos_diferencia_validos(modo_comparacion);
    if validos.is_empty() {
        return diff;
    }
    if validos.contains(&diff) {
        diff
    } else {
        validos[0]
    }
}

/// Índice uniforme para el shader (`compare.wgsl`).
#[must_use]
pub fn indice_shader_modo_comparacion(modo: CompareMode) -> u32 {
    modo as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminantes_shader() {
        assert_eq!(CompareMode::SplitScreen as u32, 0);
        assert_eq!(CompareMode::AbsDiff as u32, 1);
        assert_eq!(CompareMode::Heatmap as u32, 2);
        assert_eq!(CompareMode::SideBySide as u32, 3);
    }

    #[test]
    fn ciclo_comparacion_cuatro_pasos() {
        let mut m = CompareMode::SplitScreen;
        for _ in 0..4 {
            m = ciclar_modo_comparacion(m);
        }
        assert_eq!(m, CompareMode::SplitScreen);
    }

    #[test]
    fn normalizar_diff_en_side_by_side() {
        assert_eq!(
            normalizar_modo_diferencia(CompareMode::AbsDiff, DiffMode::None),
            DiffMode::LegacyAbs
        );
        assert_eq!(
            normalizar_modo_diferencia(CompareMode::SideBySide, DiffMode::None),
            DiffMode::None
        );
    }
}
