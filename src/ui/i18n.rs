//! Textos de interfaz agrupados por dominio (algoritmos de diferencia, temas, etc.).

use crate::types::{DiffMode, Language, Theme};
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

/// Nombres mostrados en el submenú de temas (nombres de paletas reconocibles; mismo texto en todos los idiomas).
/// Al añadir un valor en [`Theme`](crate::types::Theme), actualizar esta lista.
pub const THEME_MENU_CHOICES: &[(Theme, &'static str)] = &[
    (Theme::Dark, "Dark"),
    (Theme::Light, "Light"),
    (Theme::Rust, "Rust"),
    (Theme::SolarizedDark, "Solarized Dark"),
    (Theme::SolarizedLight, "Solarized Light"),
    (Theme::Dracula, "Dracula"),
    (Theme::Gruvbox, "Gruvbox"),
    (Theme::Nord, "Nord"),
    (Theme::Monokai, "Monokai"),
    (Theme::OneDark, "One Dark"),
    (Theme::OneLight, "One Light"),
    (Theme::Catppuccin, "Catppuccin"),
    (Theme::TokyoNight, "Tokyo Night"),
    (Theme::NightOwl, "Night Owl"),
    (Theme::Ayc, "Ayc"),
    (Theme::MaterialDesign, "Material Design"),
    (Theme::Everforest, "Everforest"),
    (Theme::TomorrowNight, "Tomorrow Night"),
    (Theme::RosePine, "Rose Pine"),
    (Theme::SynthWave84, "SynthWave '84"),
    (Theme::Nordic, "Nordic"),
    (Theme::OceanicNext, "Oceanic Next"),
    (Theme::Palenight, "Palenight"),
    (Theme::Powerlevel10k, "Powerlevel10k"),
    (Theme::Snazzy, "Snazzy"),
];

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

    #[test]
    fn theme_menu_matches_theme_variant_count() {
        assert_eq!(
            THEME_MENU_CHOICES.len(),
            25,
            "actualizar THEME_MENU_CHOICES si cambia Theme en types.rs"
        );
        let mut seen = [false; 25];
        for (i, (t, _)) in THEME_MENU_CHOICES.iter().enumerate() {
            let idx = *t as usize;
            assert!(idx < 25, "índice Theme fuera de rango: {t:?} -> {idx}");
            assert!(
                !seen[idx],
                "variante Theme duplicada en THEME_MENU_CHOICES: {t:?} (pos {i})"
            );
            seen[idx] = true;
        }
    }
}
