use crate::types::Language;
use std::path::PathBuf;

pub(super) fn handle_canvas_drag_drop(
    ui: &mut egui::Ui,
    app: &mut super::DiffPlayerApp,
    available: egui::Rect,
) -> bool {
    let hovered_files = ui.ctx().input(|i| i.raw.hovered_files.clone());
    let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());

    // IMPORTANT: Handle the actual drop FIRST, before we potentially clear
    // drag_drop_hover_pos in the else branch below. On the drop frame,
    // hovered_files is already empty but drag_drop_hover_pos still holds
    // the last valid cursor position from the previous frame.
    if !dropped_files.is_empty() {
        // Collect paths for EXR or video handling
        let paths: Vec<PathBuf> = dropped_files
            .iter()
            .filter_map(|f| f.path.as_ref().map(PathBuf::from))
            .collect();

        // EXR: single directory -> proxy from folder; all .exr files -> proxy from list. Target channel from drop position.
        let mid_x = available.center().x;
        let hover_x = app
            .drag_drop_hover_pos
            .or_else(|| ui.ctx().pointer_hover_pos())
            .unwrap_or(available.center())
            .x;
        let target_chan = if hover_x < mid_x {
            crate::types::Channel::A
        } else {
            crate::types::Channel::B
        };
        if paths.len() == 1 && paths[0].is_dir() {
            app.start_proxy_from_exr_input_dir(paths[0].clone(), target_chan, ui.ctx());
            app.drag_drop_hover_pos = None;
            return true;
        }
        let all_exr = !paths.is_empty()
            && paths.iter().all(|p| {
                p.extension()
                    .map(|e| {
                        e.to_str()
                            .map(|s| s.eq_ignore_ascii_case("exr"))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            });
        if all_exr {
            app.start_proxy_from_exr_input_files(paths, target_chan, ui.ctx());
            app.drag_drop_hover_pos = None;
            return true;
        }

        // Video handling
        let valid_extensions = [
            "mp4", "mov", "mxf", "mkv", "avi", "prores", "mts", "mpg", "mpeg", "ts",
        ];
        let mut valid_paths = Vec::new();
        let mut invalid_files = Vec::new();

        for path in &paths {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if valid_extensions.contains(&ext.as_str()) {
                valid_paths.push(path.to_string_lossy().to_string());
            } else {
                invalid_files.push(
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }

        if !invalid_files.is_empty() {
            app.error_title = Some("Formato no soportado".to_string());
            app.error_message = Some(format!(
                "Los siguientes archivos no son formatos soportados:\n{}",
                invalid_files.join(", ")
            ));
        } else if valid_paths.len() > 2 {
            app.error_title = Some("Máximo 2 videos".to_string());
            app.error_message =
                Some("Solo puedes arrastrar un máximo de 2 videos a la vez.".to_string());
        } else if valid_paths.len() == 2 {
            valid_paths.sort(); // A goes to Slot A, B goes to Slot B alphabetically
            app.open_video_a_from_path(valid_paths[0].clone(), ui.ctx());
            app.open_video_b_from_path(valid_paths[1].clone(), ui.ctx());
        } else if !valid_paths.is_empty() {
            let mid_x = available.center().x;
            let hover_x = app
                .drag_drop_hover_pos
                .or_else(|| ui.ctx().pointer_hover_pos())
                .unwrap_or(available.center())
                .x;
            if hover_x < mid_x {
                app.open_video_a_from_path(valid_paths[0].clone(), ui.ctx());
            } else {
                app.open_video_b_from_path(valid_paths[0].clone(), ui.ctx());
            }
        }

        app.drag_drop_hover_pos = None;
    } else if !hovered_files.is_empty() {
        // Files are being dragged over — update position and draw overlay
        if let Some(ptr) = ui.ctx().pointer_hover_pos() {
            app.drag_drop_hover_pos = Some(ptr);
        }

        let mid_x = available.center().x;
        let hover_x = app.drag_drop_hover_pos.map(|p| p.x).unwrap_or(mid_x);
        let targeting_a = hover_x < mid_x;

        let (a_alpha, b_alpha) = if targeting_a {
            (80u8, 30u8)
        } else {
            (30u8, 80u8)
        };

        let left_rect = egui::Rect::from_min_max(available.min, egui::pos2(mid_x, available.max.y));
        let right_rect =
            egui::Rect::from_min_max(egui::pos2(mid_x, available.min.y), available.max);

        ui.painter().rect_filled(
            left_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(80, 180, 100, a_alpha),
        );
        ui.painter().rect_filled(
            right_rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(80, 130, 220, b_alpha),
        );

        let is_es = app.view.lang == Language::Es;
        let label_a = if is_es {
            "Soltar aquí → VIDEO A"
        } else {
            "Drop here → VIDEO A"
        };
        let label_b = if is_es {
            "Soltar aquí → VIDEO B"
        } else {
            "Drop here → VIDEO B"
        };
        ui.painter().text(
            left_rect.center(),
            egui::Align2::CENTER_CENTER,
            label_a,
            egui::FontId::proportional(22.0),
            egui::Color32::from_rgba_premultiplied(220, 255, 220, 230),
        );
        ui.painter().text(
            right_rect.center(),
            egui::Align2::CENTER_CENTER,
            label_b,
            egui::FontId::proportional(22.0),
            egui::Color32::from_rgba_premultiplied(200, 220, 255, 230),
        );
        ui.painter().vline(
            mid_x,
            available.y_range(),
            egui::Stroke::new(
                2.0,
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 120),
            ),
        );

        ui.ctx().request_repaint();
    } else {
        // Nothing dragged — clear stored position
        app.drag_drop_hover_pos = None;
    }

    false
}
