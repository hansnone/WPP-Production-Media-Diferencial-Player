use crate::app::DiffPlayerApp;
use crate::ui::design::{tr, FONT_MONO_SMALL};
use egui::{ScrollArea, Window};

pub fn show(ctx: &egui::Context, app: &mut DiffPlayerApp) {
    let mut show_panel = true; // We might want a toggle in view state for this
                               // Let's assume there's a view state toggle `show_markers_panel`.
    if !app.view().show_hud {
        // Defaulting to hide with HUD for now, but usually it's its own window
        return;
    }

    Window::new(tr(app.view().lang, "Marcadores", "Markers", "Markers"))
        .open(&mut show_panel)
        .resizable(true)
        .default_width(300.0)
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let mut markers_to_remove = Vec::new();
                let mut seek_to = None;
                let fps = app.decoder_a_meta().map(|m| m.fps).unwrap_or(25.0);
                for (idx, marker) in app.session.markers.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("▶").on_hover_text("Ir a marcador").clicked() {
                                seek_to = Some(marker.pts);
                            }

                            let tc = format_timecode(marker.pts, fps);
                            ui.label(
                                egui::RichText::new(tc)
                                    .font(egui::FontId::monospace(FONT_MONO_SMALL)),
                            );

                            if ui.button("🗑").on_hover_text("Eliminar").clicked() {
                                markers_to_remove.push(idx);
                            }
                        });
                        ui.text_edit_multiline(&mut marker.note);
                    });
                }

                for idx in markers_to_remove.into_iter().rev() {
                    app.session.markers.remove(idx);
                }

                if let Some(pts) = seek_to {
                    app.do_seek(pts, ctx);
                }
            });
        });
}

pub fn format_timecode(secs: f64, fps: f64) -> String {
    let total = secs.max(0.0) as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    let fps_val = fps.max(1.0);
    let f = ((secs.fract()) * fps_val).round() as u64 % (fps_val.round() as u64).max(1);
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
