#![allow(dead_code)]
use egui::{Color32, FontFamily, FontId, Rounding, Stroke, Style, TextStyle, Visuals};

pub const BG_APP: Color32 = Color32::from_rgb(18, 18, 20);
pub const BG_PANEL: Color32 = Color32::from_rgb(25, 25, 28);
pub const BG_PANEL_ALT: Color32 = Color32::from_rgb(31, 31, 35);
pub const BG_RAISED: Color32 = Color32::from_rgb(38, 38, 43);

pub const STROKE_SUBTLE: Color32 = Color32::from_rgb(52, 52, 58);
pub const STROKE_STRONG: Color32 = Color32::from_rgb(75, 75, 84);

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(225, 225, 230);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 160, 170);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(105, 105, 115);

pub const ACCENT: Color32 = Color32::from_rgb(78, 156, 255);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(105, 174, 255);

pub const PLAYHEAD: Color32 = Color32::from_rgb(235, 70, 70);

pub const OK: Color32 = Color32::from_rgb(66, 185, 120);
pub const WARN: Color32 = Color32::from_rgb(230, 180, 64);
pub const ERROR: Color32 = Color32::from_rgb(235, 80, 80);

pub const A_COLOR: Color32 = Color32::from_rgb(83, 160, 255);
pub const B_COLOR: Color32 = Color32::from_rgb(255, 153, 82);

pub const VU_BG: Color32 = Color32::from_rgb(12, 13, 15);
pub const VU_GREEN: Color32 = Color32::from_rgb(60, 190, 115);
pub const VU_YELLOW: Color32 = Color32::from_rgb(230, 180, 60);
pub const VU_RED: Color32 = Color32::from_rgb(235, 75, 75);

pub fn apply_professional_dark_theme(ctx: &egui::Context) {
    let mut style: Style = (*ctx.style()).clone();

    style.visuals = Visuals::dark();

    style.visuals.window_fill = Color32::from_rgb(18, 18, 20);
    style.visuals.panel_fill = Color32::from_rgb(18, 18, 20);
    style.visuals.extreme_bg_color = Color32::from_rgb(12, 12, 14);
    style.visuals.faint_bg_color = Color32::from_rgb(25, 25, 28);

    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(25, 25, 28);
    style.visuals.widgets.noninteractive.bg_stroke =
        Stroke::new(1.0, Color32::from_rgb(52, 52, 58));

    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(34, 34, 38);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(58, 58, 64));
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::from_rgb(215, 215, 220));

    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(45, 45, 52);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(85, 85, 96));

    style.visuals.widgets.active.bg_fill = Color32::from_rgb(55, 68, 85);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(78, 156, 255));

    style.visuals.selection.bg_fill = Color32::from_rgb(45, 95, 155);
    style.visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(120, 190, 255));

    style.visuals.hyperlink_color = Color32::from_rgb(105, 174, 255);
    style.visuals.warn_fg_color = Color32::from_rgb(230, 180, 64);
    style.visuals.error_fg_color = Color32::from_rgb(235, 80, 80);

    style.visuals.window_rounding = Rounding::same(6.0);
    style.visuals.menu_rounding = Rounding::same(6.0);

    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.menu_margin = egui::Margin::same(8.0);
    style.spacing.window_margin = egui::Margin::same(10.0);

    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(18.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(13.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(13.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(11.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(12.0, FontFamily::Monospace),
        ),
    ]
    .into();

    ctx.set_style(style);
}

pub fn section_card(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(BG_PANEL_ALT)
        .stroke(egui::Stroke::new(1.0, STROKE_SUBTLE))
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(egui::Margin::same(10.0))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(title)
                    .size(11.0)
                    .strong()
                    .color(TEXT_SECONDARY),
            );
            ui.add_space(6.0);
            add_contents(ui);
        });
}

pub fn status_chip(ui: &mut egui::Ui, text: &str, color: egui::Color32) {
    egui::Frame::none()
        .fill(color.gamma_multiply(0.18))
        .stroke(egui::Stroke::new(1.0, color.gamma_multiply(0.7)))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::symmetric(8.0, 3.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).color(color).strong().size(11.0));
        });
}

pub fn file_chip(ui: &mut egui::Ui, label: &str, text: &str, color: egui::Color32) {
    let chip = format!("{label}: {text}");

    egui::Frame::none()
        .fill(egui::Color32::from_rgb(32, 32, 36))
        .stroke(egui::Stroke::new(1.0, color.gamma_multiply(0.6)))
        .rounding(egui::Rounding::same(4.0))
        .inner_margin(egui::Margin::symmetric(8.0, 3.0))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(chip)
                    .color(egui::Color32::from_rgb(215, 215, 220))
                    .size(12.0),
            );
        });
}

pub fn toggle_chip(ui: &mut egui::Ui, active: bool, text: &str) -> egui::Response {
    let fill = if active {
        ACCENT.gamma_multiply(0.25)
    } else {
        BG_RAISED
    };

    let stroke = if active {
        egui::Stroke::new(1.0, ACCENT)
    } else {
        egui::Stroke::new(1.0, STROKE_SUBTLE)
    };

    ui.add(
        egui::Button::new(egui::RichText::new(text).size(12.0).color(if active {
            ACCENT_HOVER
        } else {
            TEXT_SECONDARY
        }))
        .fill(fill)
        .stroke(stroke)
        .rounding(egui::Rounding::same(12.0)),
    )
}

pub fn transport_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add_sized(
        egui::vec2(32.0, 28.0),
        egui::Button::new(egui::RichText::new(label).size(15.0).color(TEXT_PRIMARY)),
    )
}
