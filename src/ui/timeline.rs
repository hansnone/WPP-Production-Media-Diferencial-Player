// ui/timeline.rs — Scrubber / playhead widget

use egui::{Pos2, Rect, Sense, Ui, Vec2};

use crate::app::DiffPlayerApp;
use crate::ui::design::FONT_MONO;

const RULER_HEIGHT: f32 = 24.0;
const THUMB_STRIP_HEIGHT: f32 = 96.0;
const TRACK_HEIGHT: f32 = 22.0;
const TIMELINE_PADDING_Y: f32 = 4.0;

pub const TIMELINE_PANEL_HEIGHT: f32 =
    RULER_HEIGHT + THUMB_STRIP_HEIGHT + TRACK_HEIGHT + TIMELINE_PADDING_Y * 2.0;

const PLAYHEAD_HEAD_W: f32 = 13.0;

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

fn nice_time_step(raw: f64) -> f64 {
    let candidates = [
        0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 15.0, 30.0, 60.0, 120.0, 300.0, 600.0,
    ];

    for c in candidates {
        if c >= raw {
            return c;
        }
    }
    1200.0
}

pub fn format_timecode_simple(seconds: f64) -> String {
    let total = seconds.max(0.0).floor() as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;

    if h > 0 {
        format!("{h:02}:{m:02}:{s:02}")
    } else {
        format!("{m:02}:{s:02}")
    }
}

fn time_to_x(t: f64, rect: Rect, duration: f64) -> f32 {
    let span = duration.max(0.001);
    rect.left() + (t / span) as f32 * rect.width()
}

fn x_to_time(x: f32, rect: Rect, duration: f64) -> f64 {
    let k = ((x - rect.left()) / rect.width()).clamp(0.0, 1.0) as f64;
    k * duration
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

fn draw_timeline_background(
    ui: &mut Ui,
    outer_rect: Rect,
    ruler_rect: Rect,
    thumb_rect: Rect,
    track_rect: Rect,
) {
    let painter = ui.painter();

    painter.rect_filled(outer_rect, 4.0, egui::Color32::from_rgb(24, 24, 26));
    painter.rect_filled(ruler_rect, 0.0, egui::Color32::from_rgb(31, 31, 34));
    painter.rect_filled(thumb_rect, 0.0, egui::Color32::from_rgb(18, 18, 20));
    painter.rect_filled(track_rect, 0.0, egui::Color32::from_rgb(35, 35, 38));

    let sep_color = egui::Color32::from_rgb(58, 58, 62);
    painter.line_segment(
        [ruler_rect.left_bottom(), ruler_rect.right_bottom()],
        egui::Stroke::new(1.0, sep_color),
    );
    painter.line_segment(
        [thumb_rect.left_bottom(), thumb_rect.right_bottom()],
        egui::Stroke::new(1.0, sep_color),
    );
}

fn draw_time_ruler(ui: &mut Ui, rect: Rect, duration: f64) {
    let painter = ui.painter();
    let span = duration.max(0.001);

    let approx_tick_px = 90.0;
    let approx_ticks = (rect.width() / approx_tick_px).max(1.0);
    let raw_step = span / approx_ticks as f64;
    let step = nice_time_step(raw_step);

    let mut t = 0.0;
    while t <= duration + step {
        let x = time_to_x(t, rect, duration);
        let is_major = ((t / step).round() as i64) % 2 == 0;
        let tick_h = if is_major { 11.0 } else { 6.0 };

        painter.line_segment(
            [
                Pos2::new(x, rect.bottom()),
                Pos2::new(x, rect.bottom() - tick_h),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(145, 145, 150)),
        );

        if is_major {
            painter.text(
                Pos2::new(x + 4.0, rect.top() + 3.0),
                egui::Align2::LEFT_TOP,
                format_timecode_simple(t),
                egui::FontId::monospace(10.0),
                egui::Color32::from_rgb(190, 190, 195),
            );
        }
        t += step;
    }
}

fn cover_uv_for_texture(tex_size: Vec2, dst_rect: Rect) -> Rect {
    let tex_w = tex_size.x.max(1.0);
    let tex_h = tex_size.y.max(1.0);

    let src_aspect = tex_w / tex_h;
    let dst_aspect = dst_rect.width().max(1.0) / dst_rect.height().max(1.0);

    if dst_aspect > src_aspect {
        // Destination is wider: crop vertically
        let visible_h = src_aspect / dst_aspect;
        let v0 = (1.0 - visible_h) * 0.5;
        let v1 = v0 + visible_h;

        Rect::from_min_max(Pos2::new(0.0, v0), Pos2::new(1.0, v1))
    } else {
        // Destination is narrower: crop horizontally
        let visible_w = dst_aspect / src_aspect;
        let u0 = (1.0 - visible_w) * 0.5;
        let u1 = u0 + visible_w;

        Rect::from_min_max(Pos2::new(u0, 0.0), Pos2::new(u1, 1.0))
    }
}

fn desired_thumb_w(rect: Rect) -> f32 {
    // Assume 16:9 aspect ratio for video visually
    (rect.height() * 16.0 / 9.0).clamp(96.0, 160.0)
}

fn draw_thumbnail_strip(ui: &mut Ui, app: &DiffPlayerApp, thumb_rect: Rect) {
    let thumbs = match app.view().timeline_thumbs_channel {
        crate::types::Channel::A => &app.thumbs_a,
        crate::types::Channel::B => &app.thumbs_b,
    };
    if thumbs.is_empty() || thumb_rect.width() <= 1.0 || thumb_rect.height() <= 1.0 {
        return;
    }

    let painter = ui.painter();

    let desired_w = desired_thumb_w(thumb_rect);
    let max_visible = (thumb_rect.width() / desired_w).floor().max(1.0) as usize;

    let stride = ((thumbs.len() as f32) / (max_visible as f32))
        .ceil()
        .max(1.0) as usize;

    let visible_count = ((thumbs.len() + stride - 1) / stride).max(1);
    let cell_w = thumb_rect.width() / visible_count as f32;

    let mut visual_idx = 0usize;

    for i in (0..thumbs.len()).step_by(stride) {
        if let Some(tex) = &thumbs[i] {
            let x0 = thumb_rect.left() + visual_idx as f32 * cell_w;
            let x1 = thumb_rect.left() + (visual_idx + 1) as f32 * cell_w;

            let cell_rect = Rect::from_min_max(
                Pos2::new(x0, thumb_rect.top()),
                Pos2::new(x1, thumb_rect.bottom()),
            );

            let image_rect = cell_rect.shrink2(Vec2::new(1.0, 1.0));
            let uv = cover_uv_for_texture(tex.size_vec2(), image_rect);

            painter.image(tex.id(), image_rect, uv, egui::Color32::WHITE);

            painter.line_segment(
                [
                    Pos2::new(cell_rect.right(), cell_rect.top()),
                    Pos2::new(cell_rect.right(), cell_rect.bottom()),
                ],
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 90)),
            );
        }
        visual_idx += 1;
    }
}

fn draw_bottom_track(ui: &mut Ui, rect: Rect, duration: f64) {
    let painter = ui.painter();

    let bar_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 6.0, rect.center().y - 3.0),
        Pos2::new(rect.right() - 6.0, rect.center().y + 3.0),
    );

    painter.rect_filled(bar_rect, 3.0, egui::Color32::from_rgb(75, 75, 82));

    if duration > 0.0 {
        let end_x = time_to_x(duration, rect, duration);
        let played_rect = Rect::from_min_max(
            Pos2::new(bar_rect.left(), bar_rect.top()),
            Pos2::new(
                end_x.clamp(bar_rect.left(), bar_rect.right()),
                bar_rect.bottom(),
            ),
        );

        painter.rect_filled(played_rect, 3.0, egui::Color32::from_rgb(95, 95, 105));
    }
}

fn draw_loop_region(ui: &mut Ui, track_rect: Rect, duration: f64, app: &mut DiffPlayerApp) -> bool {
    let painter = ui.painter().clone();
    let mut dragging_handle = false;

    let loop_in = app.playback().loop_in;
    let loop_out = app.playback().loop_out;

    if let (Some(in_pts), Some(out_pts)) = (loop_in, loop_out) {
        let x_in = time_to_x(in_pts, track_rect, duration);
        let x_out = time_to_x(out_pts, track_rect, duration);

        let range_rect = Rect::from_min_max(
            Pos2::new(x_in, track_rect.top()),
            Pos2::new(x_out, track_rect.bottom()),
        );
        painter.rect_filled(
            range_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(255, 200, 0, 60),
        );

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
                let t = x_to_time(pos.x, track_rect, duration);
                app.playback_mut().loop_in = Some(t.clamp(0.0, out_pts));
            }
        }
        painter.rect_filled(in_rect, 2.0, egui::Color32::from_rgb(255, 200, 0));

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
                let t = x_to_time(pos.x, track_rect, duration);
                app.playback_mut().loop_out = Some(t.clamp(in_pts, duration));
            }
        }
        painter.rect_filled(out_rect, 2.0, egui::Color32::from_rgb(255, 200, 0));
    }
    dragging_handle
}

fn draw_playhead(
    ui: &mut Ui,
    outer_rect: Rect,
    ruler_rect: Rect,
    current_pts: f64,
    duration: f64,
    app: &DiffPlayerApp,
) {
    let painter = ui.painter();

    if current_pts < 0.0 || current_pts > duration.max(0.0) {
        return;
    }

    let x = time_to_x(current_pts, outer_rect, duration);

    let red = egui::Color32::from_rgb(232, 64, 64);
    let dark_red = egui::Color32::from_rgb(140, 28, 28);

    painter.line_segment(
        [
            Pos2::new(x, ruler_rect.bottom()),
            Pos2::new(x, outer_rect.bottom()),
        ],
        egui::Stroke::new(1.5, red),
    );

    let head_top = ruler_rect.top() + 2.0;
    let head_bottom = ruler_rect.bottom() - 1.0;
    let half_w = PLAYHEAD_HEAD_W * 0.5;

    let points = vec![
        Pos2::new(x - half_w, head_top),
        Pos2::new(x + half_w, head_top),
        Pos2::new(x + half_w, head_bottom - 4.0),
        Pos2::new(x, head_bottom),
        Pos2::new(x - half_w, head_bottom - 4.0),
    ];

    painter.add(egui::Shape::convex_polygon(
        points,
        red,
        egui::Stroke::new(1.0, dark_red),
    ));

    // Timecode label
    let fps = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
    let start_tc_secs = app
        .decoder_a_meta()
        .and_then(|m| m.start_timecode.clone())
        .map(|tc| timecode_to_secs(&tc, fps))
        .unwrap_or(0.0);
    let current_label = format_timecode(current_pts, fps, start_tc_secs);

    // Background for label
    let font = egui::FontId::monospace(FONT_MONO);
    let text_pos = Pos2::new(x + 6.0, ruler_rect.top() + 4.0);
    painter.text(
        text_pos,
        egui::Align2::LEFT_TOP,
        &current_label,
        font,
        egui::Color32::WHITE,
    );
}

/// Draw the timeline scrubber at the bottom of the window.
pub fn show(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let duration = app.playback().duration_a.max(app.playback().duration_b);

    let mut timeline_height =
        RULER_HEIGHT + THUMB_STRIP_HEIGHT + TRACK_HEIGHT + TIMELINE_PADDING_Y * 2.0;
    timeline_height = timeline_height.clamp(80.0, 180.0);
    let desired_size = Vec2::new(ui.available_width(), timeline_height);
    let (outer_rect, outer_resp) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    let ruler_rect = Rect::from_min_max(
        outer_rect.min,
        Pos2::new(outer_rect.max.x, outer_rect.min.y + RULER_HEIGHT),
    );

    let thumb_rect = Rect::from_min_max(
        Pos2::new(outer_rect.min.x, ruler_rect.max.y),
        Pos2::new(outer_rect.max.x, ruler_rect.max.y + THUMB_STRIP_HEIGHT),
    );

    let track_rect = Rect::from_min_max(
        Pos2::new(outer_rect.min.x, thumb_rect.max.y),
        Pos2::new(outer_rect.max.x, thumb_rect.max.y + TRACK_HEIGHT),
    );

    draw_timeline_background(ui, outer_rect, ruler_rect, thumb_rect, track_rect);
    draw_thumbnail_strip(ui, app, thumb_rect);
    draw_time_ruler(ui, ruler_rect, duration);

    draw_bottom_track(ui, track_rect, duration);
    let dragging_handle = draw_loop_region(ui, track_rect, duration, app);

    if (outer_resp.dragged() || outer_resp.clicked()) && !dragging_handle {
        if let Some(pos) = outer_resp.interact_pointer_pos() {
            let t = x_to_time(pos.x, outer_rect, duration);
            app.do_seek(t.clamp(0.0, duration), ui.ctx());
        }
    }

    draw_playhead(
        ui,
        outer_rect,
        ruler_rect,
        app.playback().current_pts,
        duration,
        app,
    );
}
