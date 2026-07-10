// ui/timeline.rs — Scrubber / playhead widget

use egui::{Pos2, Rect, Sense, Ui, Vec2};

use crate::app::DiffPlayerApp;
use crate::types::Language;
use crate::ui::design::{tr, ACCENT_PRIMARY, FONT_MONO, FONT_MONO_SMALL, TIMELINE_HEIGHT};

fn timecode_to_secs(tc: &str, fps: f64) -> f64 {
    let parts: Vec<&str> = tc.split(|c| c == ':' || c == ';').collect();
    if parts.len() == 4 {
        let h: f64 = parts[0].parse().unwrap_or(0.0);
        let m: f64 = parts[1].parse().unwrap_or(0.0);
        let s: f64 = parts[2].parse().unwrap_or(0.0);
        let f: f64 = parts[3].parse().unwrap_or(0.0);
        h * 3600.0 + m * 60.0 + s + f / fps.max(1.0)
    } else {
        0.0
    }
}

/// Draw the timeline scrubber at the bottom of the window.
pub fn show(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    let duration = app.playback().duration_a.max(app.playback().duration_b);
    let fps = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
    let start_tc = app.decoder_a_meta().and_then(|m| m.start_timecode.clone());
    let start_tc_secs = if let Some(tc) = start_tc {
        timecode_to_secs(&tc, fps)
    } else {
        0.0
    };

    // Always reserve the full width
    let available_width = ui.available_width();
    let desired_size = Vec2::new(available_width, TIMELINE_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    let painter = ui.painter().clone();

    // ── Background track (Thumbnails) ──────────────────────────────────────────────────
    let track_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 4.0, rect.top() + 20.0), // Below timecode
        Pos2::new(rect.right() - 4.0, rect.bottom() - 4.0),
    );
    painter.rect_filled(track_rect, 4.0, ui.visuals().faint_bg_color);

    let thumb_count = app.thumbs_a.len();
    if thumb_count > 0 {
        let thumb_width = track_rect.width() / thumb_count as f32;
        for (i, thumb_opt) in app.thumbs_a.iter().enumerate() {
            if let Some(tex) = thumb_opt {
                let x0 = track_rect.left() + i as f32 * thumb_width;
                let x1 = x0 + thumb_width;
                let thumb_rect = Rect::from_min_max(
                    Pos2::new(x0, track_rect.top()),
                    Pos2::new(x1, track_rect.bottom()),
                );
                let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
                painter.image(tex.id(), thumb_rect, uv, egui::Color32::WHITE);
            }
        }
    }

    let mut dragging_handle = false;
    // ── Loop Range ────────────────────────────────────────────────────────
    let loop_in = app.playback().loop_in;
    let loop_out = app.playback().loop_out;

    if let (Some(in_pts), Some(out_pts)) = (loop_in, loop_out) {
        let fraction_in = if duration > 0.0 { (in_pts / duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let fraction_out = if duration > 0.0 { (out_pts / duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let x_in = track_rect.left() + track_rect.width() * fraction_in;
        let x_out = track_rect.left() + track_rect.width() * fraction_out;

        let range_rect = Rect::from_min_max(
            Pos2::new(x_in, track_rect.top()),
            Pos2::new(x_out, track_rect.bottom()),
        );
        painter.rect_filled(
            range_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(255, 200, 0, 60),
        );

        // Handle In
        let in_rect = Rect::from_center_size(
            Pos2::new(x_in, track_rect.center().y),
            Vec2::new(10.0, track_rect.height() + 8.0),
        );
        let in_resp = ui.allocate_rect(in_rect, Sense::drag());
        if in_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }
        if in_resp.dragged() {
            dragging_handle = true;
            if let Some(pos) = in_resp.interact_pointer_pos() {
                let fraction = ((pos.x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
                app.playback_mut().loop_in = Some((fraction as f64 * duration).min(out_pts));
            }
        }
        painter.rect_filled(in_rect, 2.0, egui::Color32::from_rgb(255, 200, 0));

        // Handle Out
        let out_rect = Rect::from_center_size(
            Pos2::new(x_out, track_rect.center().y),
            Vec2::new(10.0, track_rect.height() + 8.0),
        );
        let out_resp = ui.allocate_rect(out_rect, Sense::drag());
        if out_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }
        if out_resp.dragged() {
            dragging_handle = true;
            if let Some(pos) = out_resp.interact_pointer_pos() {
                let fraction = ((pos.x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
                app.playback_mut().loop_out = Some((fraction as f64 * duration).max(in_pts));
            }
        }
        painter.rect_filled(out_rect, 2.0, egui::Color32::from_rgb(255, 200, 0));
    }

    // ── Markers ───────────────────────────────────────────────────────────
    for marker in &app.session.markers {
        let fraction = if duration > 0.0 { (marker.pts / duration).clamp(0.0, 1.0) as f32 } else { 0.0 };
        let x = track_rect.left() + track_rect.width() * fraction;
        let p0 = Pos2::new(x, track_rect.top() - 6.0);
        let p1 = Pos2::new(x + 4.0, track_rect.center().y);
        let p2 = Pos2::new(x, track_rect.bottom() + 6.0);
        let p3 = Pos2::new(x - 4.0, track_rect.center().y);

        let color = egui::Color32::from_rgb(
            (marker.color[0] * 255.0) as u8,
            (marker.color[1] * 255.0) as u8,
            (marker.color[2] * 255.0) as u8,
        );

        painter.add(egui::Shape::convex_polygon(
            vec![p0, p1, p2, p3],
            color,
            egui::Stroke::new(1.0, egui::Color32::BLACK),
        ));
    }

    // ── Interaction ───────────────────────────────────────────────────────
    if (response.dragged() || response.clicked()) && !dragging_handle {
        if let Some(pos) = response.interact_pointer_pos() {
            let x = pos.x.clamp(track_rect.left(), track_rect.right());
            let fraction = (x - track_rect.left()) / track_rect.width();
            let new_pts = (fraction as f64 * duration).max(0.0);
            app.do_seek(new_pts, ui.ctx());
        }
    }

    // ── Played portion ────────────────────────────────────────────────────
    let current_pts = app.playback().current_pts;
    let fraction = if duration > 0.0 {
        (current_pts / duration).clamp(0.0, 1.0) as f32
    } else {
        0.0
    };
    let played_right = track_rect.left() + track_rect.width() * fraction;

    painter.rect_filled(
        Rect::from_min_max(track_rect.min, Pos2::new(played_right, track_rect.max.y)),
        4.0,
        ACCENT_PRIMARY,
    );

    // ── Playhead ──────────────────────────────────────────────────────────

    let handle_x = track_rect.left() + fraction * track_rect.width();
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
    let current_label = format_timecode(current_pts, fps, start_tc_secs);
    let duration_label = format_timecode(duration, fps, start_tc_secs);
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

/// Format `secs` as HH:MM:SS:FF using real fps and optional start offset
fn format_timecode(secs: f64, fps: f64, start_tc_secs: f64) -> String {
    let total_secs = secs + start_tc_secs;
    let total = total_secs.max(0.0) as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    let fps_val = fps.max(1.0);
    let f = ((total_secs.fract()) * fps_val).round() as u64 % (fps_val.round() as u64).max(1);
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
