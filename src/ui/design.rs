//! Design tokens shared across native UI panels.

use egui::Color32;

use crate::types::Language;

/// Short ES / EN / Quenya lookup for menu copy (extend as i18n grows).
pub fn tr(
    lang: Language,
    es: &'static str,
    en: &'static str,
    quenya: &'static str,
) -> &'static str {
    match lang {
        Language::Es => es,
        Language::En => en,
        Language::Quenya => quenya,
    }
}

pub const FONT_TITLE: f32 = 17.0;
pub const FONT_SUBTITLE: f32 = 12.0;
pub const FONT_LABEL: f32 = 11.0;
pub const FONT_VALUE: f32 = 11.0;
/// Monospace data (timecode, frame counters).
pub const FONT_MONO: f32 = 11.0;
pub const FONT_MONO_SMALL: f32 = 10.0;

/// Primary accent (timeline playhead, highlights) — keep aligned with info panel branding.
pub const ACCENT_PRIMARY: Color32 = Color32::from_rgb(80, 160, 230);

pub const TIMELINE_HEIGHT: f32 = 44.0;

pub fn dialog_ok(lang: Language) -> &'static str {
    tr(lang, "Aceptar", "OK", "Ná")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Language;

    #[test]
    fn tr_selects_branch_by_language() {
        assert_eq!(tr(Language::Es, "es", "en", "qy"), "es");
        assert_eq!(tr(Language::En, "es", "en", "qy"), "en");
        assert_eq!(tr(Language::Quenya, "es", "en", "qy"), "qy");
    }
}
