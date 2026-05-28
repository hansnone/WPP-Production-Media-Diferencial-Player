// ui/timeline.rs — Scrubber / playhead widget

use egui::{Pos2, Rect, Sense, Ui, Vec2};

use crate::app::DiffPlayerApp;
use crate::types::Language;
use crate::ui::design::{tr, ACCENT_PRIMARY, FONT_MONO, FONT_MONO_SMALL, TIMELINE_HEIGHT};

/// Draw the timeline scrubber at the bottom of the window.
pub fn show(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    let duration = app.playback().duration_a.max(app.playback().duration_b);

    // Always reserve the full width
    let available_width = ui.available_width();
    let desired_size = Vec2::new(available_width, TIMELINE_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    let painter = ui.painter();

    // ── Background track ──────────────────────────────────────────────────
    let track_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 4.0, rect.center().y - 4.0),
        Pos2::new(rect.right() - 4.0, rect.center().y + 4.0),
    );
    painter.rect_filled(track_rect, 2.0, ui.visuals().widgets.noninteractive.bg_fill);

    if duration <= 0.0 {
        // No video loaded — draw a disabled track
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            tr(
                lang,
                "──── línea de tiempo ────",
                "──── timeline ────",
                "──── línë —───",
            ),
            egui::FontId::proportional(FONT_MONO),
            ui.visuals().text_color().gamma_multiply(0.3),
        );
        return;
    }

    // ── Played portion ────────────────────────────────────────────────────
    let current_pts = app.playback().current_pts;
    let progress = (current_pts / duration).clamp(0.0, 1.0) as f32;
    let played_right = track_rect.left() + track_rect.width() * progress;

    painter.rect_filled(
        Rect::from_min_max(track_rect.min, Pos2::new(played_right, track_rect.max.y)),
        2.0,
        ACCENT_PRIMARY,
    );

    // ── Playhead handle ───────────────────────────────────────────────────
    let handle_x = track_rect.left() + track_rect.width() * progress;
    let handle_center = Pos2::new(handle_x, track_rect.center().y);

    let is_hovered = response.hovered();
    let handle_radius = if is_hovered { 9.0 } else { 7.0 };
    painter.circle_filled(
        handle_center,
        handle_radius + 1.5,
        ui.visuals().window_fill(),
    );
    painter.circle_filled(handle_center, handle_radius, ACCENT_PRIMARY);

    // ── Timecode labels ───────────────────────────────────────────────────
    let current_label = format_timecode(current_pts);
    let duration_label = format_timecode(duration);
    let font = egui::FontId::monospace(FONT_MONO);
    let dim = ui.visuals().text_color().gamma_multiply(0.7);

    painter.text(
        Pos2::new(rect.left() + 6.0, rect.top() + 4.0),
        egui::Align2::LEFT_TOP,
        &current_label,
        font.clone(),
        dim,
    );
    painter.text(
        Pos2::new(rect.right() - 6.0, rect.top() + 4.0),
        egui::Align2::RIGHT_TOP,
        &duration_label,
        font,
        dim,
    );

    // ── Frame number ──────────────────────────────────────────────────────
    let fps_a = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
    let frame_num = (current_pts * fps_a).round() as u64;
    let frame_prefix = match lang {
        Language::Es => "Cuad.",
        Language::En => "Frm.",
        Language::Quenya => "Fr.",
    };
    painter.text(
        handle_center - Vec2::new(0.0, 18.0),
        egui::Align2::CENTER_CENTER,
        format!("{frame_prefix}{frame_num}"),
        egui::FontId::monospace(FONT_MONO_SMALL),
        ACCENT_PRIMARY,
    );

    // ── Seek on click / drag ──────────────────────────────────────────────
    let interact = response.interact_pointer_pos();
    if response.clicked() || response.dragged() {
        if let Some(pos) = interact {
            let t = ((pos.x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
            let seek_secs = t as f64 * duration;
            app.do_seek(seek_secs, ui.ctx());
        }
    }
}

/// Format `secs` as HH:MM:SS:FF (assuming 25 fps for the frame counter display).
fn format_timecode(secs: f64) -> String {
    let total = secs.max(0.0) as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    let f = ((secs.fract()) * 25.0).round() as u64 % 25;
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
