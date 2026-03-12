use egui::{Color32, Context, Stroke, Visuals};

pub fn apply_theme(ctx: &Context, theme: crate::types::Theme) {
    match theme {
        crate::types::Theme::Dark => ctx.set_visuals(Visuals::dark()),
        crate::types::Theme::Light => ctx.set_visuals(Visuals::light()),
        _ => {
            let (is_dark, bg, panel, accent, text) = match theme {
                crate::types::Theme::Rust => (
                    true,
                    Color32::from_rgb(43, 43, 43),
                    Color32::from_rgb(32, 32, 32),
                    Color32::from_rgb(252, 60, 20),
                    Color32::from_rgb(240, 230, 220),
                ),
                crate::types::Theme::SolarizedDark => (
                    true,
                    Color32::from_rgb(0, 43, 54),
                    Color32::from_rgb(7, 54, 66),
                    Color32::from_rgb(181, 137, 0),
                    Color32::from_rgb(131, 148, 150),
                ),
                crate::types::Theme::SolarizedLight => (
                    false,
                    Color32::from_rgb(253, 246, 227),
                    Color32::from_rgb(238, 232, 213),
                    Color32::from_rgb(38, 139, 210),
                    Color32::from_rgb(101, 123, 131),
                ),
                crate::types::Theme::Dracula => (
                    true,
                    Color32::from_rgb(40, 42, 54),
                    Color32::from_rgb(68, 71, 90),
                    Color32::from_rgb(189, 147, 249),
                    Color32::from_rgb(248, 248, 242),
                ),
                crate::types::Theme::Gruvbox => (
                    true,
                    Color32::from_rgb(40, 40, 40),
                    Color32::from_rgb(60, 56, 54),
                    Color32::from_rgb(250, 189, 47),
                    Color32::from_rgb(235, 219, 178),
                ),
                crate::types::Theme::Nord => (
                    true,
                    Color32::from_rgb(46, 52, 64),
                    Color32::from_rgb(59, 66, 82),
                    Color32::from_rgb(136, 192, 208),
                    Color32::from_rgb(236, 239, 244),
                ),
                crate::types::Theme::Monokai => (
                    true,
                    Color32::from_rgb(39, 40, 34),
                    Color32::from_rgb(62, 61, 50),
                    Color32::from_rgb(249, 38, 114),
                    Color32::from_rgb(248, 248, 242),
                ),
                crate::types::Theme::OneDark => (
                    true,
                    Color32::from_rgb(40, 44, 52),
                    Color32::from_rgb(44, 49, 58),
                    Color32::from_rgb(97, 175, 239),
                    Color32::from_rgb(171, 178, 191),
                ),
                crate::types::Theme::OneLight => (
                    false,
                    Color32::from_rgb(250, 250, 250),
                    Color32::from_rgb(240, 240, 240),
                    Color32::from_rgb(82, 111, 255),
                    Color32::from_rgb(56, 58, 66),
                ),
                crate::types::Theme::Catppuccin => (
                    true,
                    Color32::from_rgb(30, 30, 46),
                    Color32::from_rgb(24, 24, 37),
                    Color32::from_rgb(203, 166, 247),
                    Color32::from_rgb(205, 214, 244),
                ),
                crate::types::Theme::TokyoNight => (
                    true,
                    Color32::from_rgb(26, 27, 38),
                    Color32::from_rgb(22, 22, 30),
                    Color32::from_rgb(122, 162, 247),
                    Color32::from_rgb(192, 202, 245),
                ),
                crate::types::Theme::NightOwl => (
                    true,
                    Color32::from_rgb(1, 22, 39),
                    Color32::from_rgb(11, 41, 66),
                    Color32::from_rgb(130, 170, 255),
                    Color32::from_rgb(214, 222, 235),
                ),
                crate::types::Theme::Ayc => (
                    true,
                    Color32::from_rgb(15, 20, 25),
                    Color32::from_rgb(20, 25, 31),
                    Color32::from_rgb(230, 180, 80),
                    Color32::from_rgb(191, 186, 176),
                ),
                crate::types::Theme::MaterialDesign => (
                    true,
                    Color32::from_rgb(38, 50, 56),
                    Color32::from_rgb(55, 71, 79),
                    Color32::from_rgb(128, 203, 196),
                    Color32::from_rgb(236, 239, 241),
                ),
                crate::types::Theme::Everforest => (
                    true,
                    Color32::from_rgb(43, 51, 57),
                    Color32::from_rgb(50, 60, 65),
                    Color32::from_rgb(167, 192, 128),
                    Color32::from_rgb(211, 198, 170),
                ),
                crate::types::Theme::TomorrowNight => (
                    true,
                    Color32::from_rgb(29, 31, 33),
                    Color32::from_rgb(40, 42, 46),
                    Color32::from_rgb(129, 162, 190),
                    Color32::from_rgb(197, 200, 198),
                ),
                crate::types::Theme::RosePine => (
                    true,
                    Color32::from_rgb(25, 23, 36),
                    Color32::from_rgb(31, 29, 46),
                    Color32::from_rgb(196, 167, 231),
                    Color32::from_rgb(224, 222, 244),
                ),
                crate::types::Theme::SynthWave84 => (
                    true,
                    Color32::from_rgb(38, 35, 58),
                    Color32::from_rgb(43, 33, 58),
                    Color32::from_rgb(255, 126, 219),
                    Color32::from_rgb(249, 42, 173),
                ),
                crate::types::Theme::Nordic => (
                    true,
                    Color32::from_rgb(36, 41, 51),
                    Color32::from_rgb(46, 52, 64),
                    Color32::from_rgb(143, 188, 187),
                    Color32::from_rgb(216, 222, 233),
                ),
                crate::types::Theme::OceanicNext => (
                    true,
                    Color32::from_rgb(27, 43, 52),
                    Color32::from_rgb(52, 61, 70),
                    Color32::from_rgb(102, 153, 204),
                    Color32::from_rgb(192, 197, 206),
                ),
                crate::types::Theme::Palenight => (
                    true,
                    Color32::from_rgb(41, 45, 62),
                    Color32::from_rgb(50, 55, 77),
                    Color32::from_rgb(199, 146, 234),
                    Color32::from_rgb(191, 199, 213),
                ),
                crate::types::Theme::Powerlevel10k => (
                    true,
                    Color32::from_rgb(0, 0, 0),
                    Color32::from_rgb(28, 28, 28),
                    Color32::from_rgb(0, 135, 255),
                    Color32::from_rgb(255, 255, 255),
                ),
                crate::types::Theme::Snazzy => (
                    true,
                    Color32::from_rgb(40, 42, 54),
                    Color32::from_rgb(52, 53, 65),
                    Color32::from_rgb(255, 92, 87),
                    Color32::from_rgb(239, 240, 235),
                ),
                _ => (
                    true,
                    Color32::from_rgb(43, 43, 43),
                    Color32::from_rgb(32, 32, 32),
                    Color32::from_rgb(252, 60, 20),
                    Color32::from_rgb(240, 230, 220),
                ),
            };

            let mut visuals = if is_dark {
                Visuals::dark()
            } else {
                Visuals::light()
            };
            visuals.widgets.noninteractive.bg_fill = if is_dark {
                bg.linear_multiply(1.5)
            } else {
                bg.linear_multiply(0.9)
            };
            visuals.widgets.noninteractive.bg_stroke = Stroke::new(
                1.0,
                if is_dark {
                    panel
                } else {
                    panel.linear_multiply(0.8)
                },
            );
            visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text);

            visuals.widgets.inactive.bg_fill = panel;
            visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text);

            visuals.widgets.hovered.bg_fill = accent.gamma_multiply(0.2);
            visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, accent);
            visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, accent);

            visuals.widgets.active.bg_fill = accent.gamma_multiply(0.4);
            visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            visuals.widgets.active.bg_stroke = Stroke::new(1.0, accent);

            visuals.selection.bg_fill = accent;
            visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);

            visuals.panel_fill = panel;
            visuals.window_fill = bg;
            ctx.set_visuals(visuals);
        }
    }
}
