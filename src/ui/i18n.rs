//! Textos de interfaz agrupados por dominio (algoritmos de diferencia, temas, etc.).

use crate::types::{DiffMode, Language};
use crate::ui::design::tr;

/// Etiqueta corta para modos de diferencia en combos y barras laterales.
pub fn diff_mode_label(lang: Language, mode: DiffMode) -> &'static str {
    match mode {
        DiffMode::LegacyAbs => tr(lang, "Legado (abs)", "Legacy (abs)", "Yestë (abs)"),
        DiffMode::AbsLinear => tr(lang, "Lineal", "Linear", "Lina"),
        DiffMode::AbsSqrt => tr(lang, "Raíz", "Sqrt", "Súrt"),
        DiffMode::SignedDiverging => tr(lang, "Divergente signado", "Signed diverging", "Haina"),
        DiffMode::None => tr(lang, "—", "—", "—"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_mode_label_spanish() {
        assert_eq!(
            diff_mode_label(Language::Es, DiffMode::LegacyAbs),
            "Legado (abs)"
        );
        assert_eq!(diff_mode_label(Language::Es, DiffMode::AbsLinear), "Lineal");
        assert_eq!(diff_mode_label(Language::Es, DiffMode::AbsSqrt), "Raíz");
        assert_eq!(
            diff_mode_label(Language::Es, DiffMode::SignedDiverging),
            "Divergente signado"
        );
        assert_eq!(diff_mode_label(Language::Es, DiffMode::None), "—");
    }

    #[test]
    fn diff_mode_label_english() {
        assert_eq!(
            diff_mode_label(Language::En, DiffMode::LegacyAbs),
            "Legacy (abs)"
        );
        assert_eq!(
            diff_mode_label(Language::En, DiffMode::SignedDiverging),
            "Signed diverging"
        );
    }

    #[test]
    fn diff_mode_label_quenya() {
        assert_eq!(
            diff_mode_label(Language::Quenya, DiffMode::AbsLinear),
            "Lina"
        );
        assert_eq!(
            diff_mode_label(Language::Quenya, DiffMode::SignedDiverging),
            "Haina"
        );
    }
}
