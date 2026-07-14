use std::sync::Arc;

use super::DiffPlayerApp;
use crate::renderer::RenderCallback;
use crate::types::{CompareMode, Language, SafeZoneMode};

pub(super) fn show_canvas(ui: &mut egui::Ui, app: &mut DiffPlayerApp, _frame: &mut eframe::Frame) {
    let available = ui.available_rect_before_wrap();
    app.view.canvas_rect = available;

    let response = ui.allocate_rect(available, egui::Sense::click_and_drag());

    // -- Mouse wheel zoom ---------------------------------------------------
    let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
    if response.hovered() && scroll_delta != 0.0 {
        let zoom_factor = if scroll_delta > 0.0 {
            1.1f32
        } else {
            1.0 / 1.1
        };
        app.view.zoom = (app.view.zoom * zoom_factor).clamp(0.25, 32.0);
    }

    // -- Drag to pan OR drag split line (Available in all modes) -------------
    // Pan is only active when zoomed in (zoom > 1.0). At fit-to-frame only the
    // split divider can be dragged. Split line is vertical or horizontal per split_horizontal.
    if response.drag_started() {
        let pos = response.interact_pointer_pos().unwrap_or_default();
        let near_split = if app.view.split_horizontal {
            let split_y = available.top() + app.view.split_pos * available.height();
            (pos.y - split_y).abs() < 15.0
        } else {
            let split_x = available.left() + app.view.split_pos * available.width();
            (pos.x - split_x).abs() < 15.0
        };
        if near_split {
            app.dragging_split = true;
        } else {
            app.dragging_split = false;
            if app.view.zoom > 1.0 {
                app.drag_start = Some((pos, app.view.pan_u, app.view.pan_v));
            }
        }
    }

    if response.dragged() {
        if app.dragging_split {
            let pos = response.interact_pointer_pos().unwrap_or_default();
            if app.view.split_horizontal {
                let relative_y = (pos.y - available.top()) / available.height();
                app.view.split_pos = relative_y.clamp(0.0, 1.0);
            } else {
                let relative_x = (pos.x - available.left()) / available.width();
                app.view.split_pos = relative_x.clamp(0.0, 1.0);
            }
            ui.ctx().request_repaint();
        } else if let Some((start_pos, start_pu, start_pv)) = app.drag_start {
            let delta = response.interact_pointer_pos().unwrap_or_default() - start_pos;
            let uv_delta_u = -delta.x / available.width() / app.view.zoom;
            let uv_delta_v = -delta.y / available.height() / app.view.zoom;
            app.view.pan_u = (start_pu + uv_delta_u).clamp(-0.5, 0.5);
            app.view.pan_v = (start_pv + uv_delta_v).clamp(-0.5, 0.5);
            ui.ctx().request_repaint();
        }
    }

    if response.drag_stopped() {
        app.drag_start = None;
        app.dragging_split = false;
    }

    // -- Cursor hint for dragging split (Available in all modes) ------------
    if let Some(ptr) = ui.ctx().pointer_hover_pos() {
        let near_split = if app.view.split_horizontal {
            let split_y = available.top() + app.view.split_pos * available.height();
            available.contains(ptr) && (ptr.y - split_y).abs() < 10.0
        } else {
            let split_x = available.left() + app.view.split_pos * available.width();
            available.contains(ptr) && (ptr.x - split_x).abs() < 10.0
        };
        if near_split {
            ui.ctx().set_cursor_icon(if app.view.split_horizontal {
                egui::CursorIcon::ResizeVertical
            } else {
                egui::CursorIcon::ResizeHorizontal
            });
        }
    }

    // -- Double-click to reset zoom -----------------------------------------
    if response.double_clicked() {
        app.view.zoom = 1.0;
        app.view.pan_u = 0.0;
        app.view.pan_v = 0.0;
    }

    // -- Draw the wgpu render callback into this rect ----------------------
    // Skip on first frame so macOS window can appear (first Wgpu present can block).
    if app.frame_count > 1 {
        let renderer_clone = Arc::clone(&app.renderer);
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            available,
            RenderCallback {
                renderer: renderer_clone,
            },
        ));
    } else {
        ui.painter()
            .rect_filled(available, 0.0, egui::Color32::from_rgb(0, 0, 0));
    }

    // -- Safe zones overlay (video_rect + zoom/pan) -------------------------
    // In SideBySide mode draw on both halves (A left, B right); otherwise once on full canvas.
    if app.view.safe_zone != SafeZoneMode::None {
        let zoom = app.view.zoom;
        let visible_left = 0.5 - 0.5 / zoom + app.view.pan_u;
        let visible_right = 0.5 + 0.5 / zoom + app.view.pan_u;
        let visible_top = 0.5 - 0.5 / zoom + app.view.pan_v;
        let visible_bottom = 0.5 + 0.5 / zoom + app.view.pan_v;

        let draw_safe_zones = |container: egui::Rect, vw: f32, vh: f32| {
            let cw = container.width();
            let ch = container.height();
            let video_aspect = vw / vh;
            let container_aspect = cw / ch;
            let video_rect = if video_aspect >= container_aspect {
                let h = cw / video_aspect;
                let top = container.center().y - h * 0.5;
                egui::Rect::from_min_max(
                    egui::Pos2::new(container.left(), top),
                    egui::Pos2::new(container.right(), top + h),
                )
            } else {
                let w = ch * video_aspect;
                let left = container.center().x - w * 0.5;
                egui::Rect::from_min_max(
                    egui::Pos2::new(left, container.top()),
                    egui::Pos2::new(left + w, container.bottom()),
                )
            };
            let uv_to_screen = |u: f32, v: f32| {
                let x = video_rect.left()
                    + (u - visible_left) / (visible_right - visible_left) * video_rect.width();
                let y = video_rect.top()
                    + (v - visible_top) / (visible_bottom - visible_top) * video_rect.height();
                egui::Pos2::new(x, y)
            };

            match app.view.safe_zone {
                SafeZoneMode::None => {}
                SafeZoneMode::TvEbu => {
                    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 200, 255));
                    let action_min = uv_to_screen(0.035, 0.035);
                    let action_max = uv_to_screen(0.965, 0.965);
                    let action_rect = egui::Rect::from_min_max(action_min, action_max);
                    ui.painter().rect_stroke(action_rect, 0.0, stroke);
                    let title_min = uv_to_screen(0.10, 0.05);
                    let title_max = uv_to_screen(0.90, 0.95);
                    let title_rect = egui::Rect::from_min_max(title_min, title_max);
                    ui.painter().rect_stroke(title_rect, 0.0, stroke);
                    let center = uv_to_screen(0.5, 0.5);
                    let cross_half = 10.0;
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(center.x - cross_half, center.y),
                            egui::Pos2::new(center.x + cross_half, center.y),
                        ],
                        stroke,
                    );
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(center.x, center.y - cross_half),
                            egui::Pos2::new(center.x, center.y + cross_half),
                        ],
                        stroke,
                    );
                }
                SafeZoneMode::Social => {
                    let danger_fill = egui::Color32::from_black_alpha(150);
                    let top_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.0), uv_to_screen(1.0, 0.15));
                    let bottom_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.78), uv_to_screen(1.0, 1.0));
                    let right_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.85, 0.0), uv_to_screen(1.0, 1.0));
                    let left_danger =
                        egui::Rect::from_min_max(uv_to_screen(0.0, 0.0), uv_to_screen(0.05, 1.0));
                    ui.painter().rect_filled(top_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(bottom_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(right_danger, 0.0, danger_fill);
                    ui.painter().rect_filled(left_danger, 0.0, danger_fill);
                    let safe_min = uv_to_screen(0.05, 0.15);
                    let safe_max = uv_to_screen(0.85, 0.78);
                    let safe_rect = egui::Rect::from_min_max(safe_min, safe_max);
                    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 200, 0));
                    ui.painter().rect_stroke(safe_rect, 0.0, stroke);
                }
            }
        };

        if app.view.mode == CompareMode::SideBySide {
            let mid_x = available.center().x;
            let left_rect =
                egui::Rect::from_min_max(available.min, egui::pos2(mid_x, available.max.y));
            let right_rect =
                egui::Rect::from_min_max(egui::pos2(mid_x, available.min.y), available.max);
            let (vw_a, vh_a) = app
                .decoder_a_meta()
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            let (vw_b, vh_b) = app
                .decoder_b_meta()
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            draw_safe_zones(left_rect, vw_a, vh_a);
            draw_safe_zones(right_rect, vw_b, vh_b);
        } else {
            let (vw, vh) = app
                .decoder_a_meta()
                .or_else(|| app.decoder_b_meta())
                .map(|m| (m.width as f32, m.height as f32))
                .unwrap_or((16.0, 9.0));
            draw_safe_zones(available, vw, vh);
        }
    }

    // -- OS file drag-and-drop handling ------------------------------------
    if super::drag_drop::handle_canvas_drag_drop(ui, app, available) {
        return;
    }

    // -- Overlay: "No video" message when nothing is loaded ----------------
    let has_a = app.decoder_a.is_some();
    let has_b = app.decoder_b.is_some();
    if !has_a || !has_b {
        let center = available.center();
        let is_es = app.view.lang == Language::Es;
        let text = if !has_a && !has_b {
            if is_es {
                "Abre el Vídeo A y el Vídeo B para empezar la comparación"
            } else {
                "Open Video A and Video B to begin comparison"
            }
        } else if !has_a {
            if is_es {
                "Abre el Vídeo A  ←  (panel izquierdo)"
            } else {
                "Open Video A  ←  (left panel)"
            }
        } else {
            if is_es {
                "Abre el Vídeo B  →  (panel izquierdo)"
            } else {
                "Open Video B  →  (left panel)"
            }
        };
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(20.0),
            ui.visuals().text_color().gamma_multiply(0.5),
        );
    }

    // -- Zoom indicator overlay (top-right of canvas) ----------------------
    if (app.view.zoom - 1.0).abs() > 0.01 {
        let zoom_text = format!("{:.1}×", app.view.zoom);
        let pos = egui::pos2(available.right() - 8.0, available.top() + 8.0);
        ui.painter().text(
            pos,
            egui::Align2::RIGHT_TOP,
            &zoom_text,
            egui::FontId::monospace(13.0),
            egui::Color32::from_rgba_premultiplied(200, 200, 100, 200),
        );
    }

    // -- Frame counter overlay (bottom-left of canvas, unobtrusive) --------
    // Shows permanently, including during screenshots.
    {
        let fps_a = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
        let current_pts = app.playback().current_pts;
        let frame_num = (current_pts * fps_a).round() as u64;
        let is_es = app.view().lang == Language::Es;

        let frame_text = format!("{} {}", if is_es { "Fr." } else { "Frame" }, frame_num);
        let pos = egui::pos2(available.left() + 8.0, available.bottom() - 8.0);
        ui.painter().text(
            pos,
            egui::Align2::LEFT_BOTTOM,
            &frame_text,
            egui::FontId::monospace(14.0),
            egui::Color32::from_black_alpha(150), // Subtle shadow
        );
        ui.painter().text(
            pos - egui::Vec2::new(1.0, 1.0),
            egui::Align2::LEFT_BOTTOM,
            &frame_text,
            egui::FontId::monospace(14.0),
            egui::Color32::from_white_alpha(150), // Unobtrusive text
        );
    }
}
