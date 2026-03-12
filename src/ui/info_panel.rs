// ui/info_panel.rs — Left side panel: metadata, color info, status

use egui::{Color32, RichText, Ui};

use crate::app::DiffPlayerApp;
use crate::types::{ColorMetadata, Language};

pub fn show(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let is_es = app.view().lang == Language::Es;
    let playback = app.playback().clone();
    let meta_a = app.decoder_a_meta().cloned();
    let meta_b = app.decoder_b_meta().cloned();
    let path_a = app.decoder_a_path().map(|s| s.to_string());
    let path_b = app.decoder_b_path().map(|s| s.to_string());
    let zoom = app.view().zoom;
    let pan_u = app.view().pan_u;
    let pan_v = app.view().pan_v;

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(6.0);

        // ── App title ─────────────────────────────────────────────────────
        ui.label(
            RichText::new("WPP Production media diferencial player")
                .size(17.0)
                .strong()
                .color(Color32::from_rgb(80, 160, 230)),
        );
        ui.label(
            RichText::new(if is_es {
                "Control de Calidad Frame a Frame"
            } else {
                "Frame-Accurate Video QC"
            })
            .size(11.0)
            .weak(),
        );
        ui.separator();

        // ── Current playback info ─────────────────────────────────────────
        let pts = playback.current_pts;
        let fps_a = meta_a.as_ref().map(|m| m.fps).unwrap_or(0.0);
        let frame_n = (pts * fps_a).round() as u64;

        egui::Grid::new("playback_grid")
            .num_columns(2)
            .spacing([8.0, 3.0])
            .show(ui, |ui| {
                kv(ui, "PTS", &format!("{pts:.4} s"));
                kv(
                    ui,
                    if is_es { "Cuadro" } else { "Frame" },
                    &frame_n.to_string(),
                );
                kv(
                    ui,
                    if is_es { "Zoom" } else { "Zoom" },
                    &format!("{zoom:.2}×"),
                );
                kv(
                    ui,
                    if is_es { "Paneo" } else { "Pan UV" },
                    &format!("({pan_u:.3}, {pan_v:.3})"),
                );
            });

        ui.add_space(8.0);
        ui.separator();

        let dark_mode = ui.visuals().dark_mode;
        let color_a = if dark_mode {
            Color32::from_rgb(100, 200, 120)
        } else {
            Color32::from_rgb(20, 110, 50)
        };
        let color_b = if dark_mode {
            Color32::from_rgb(100, 160, 240)
        } else {
            Color32::from_rgb(30, 70, 180)
        };

        // ── Video A info ──────────────────────────────────────────────────
        channel_section(
            ui,
            if is_es { "VÍDEO A" } else { "VIDEO A" },
            color_a,
            path_a.as_deref(),
            meta_a.as_ref(),
            is_es,
        );

        ui.add_space(8.0);
        ui.separator();

        // ── Video B info ──────────────────────────────────────────────────
        channel_section(
            ui,
            if is_es { "VÍDEO B" } else { "VIDEO B" },
            color_b,
            path_b.as_deref(),
            meta_b.as_ref(),
            is_es,
        );

        ui.add_space(8.0);
        ui.separator();

        // ── Color mismatch warning ────────────────────────────────────────
        if let (Some(ma), Some(mb)) = (meta_a.as_ref(), meta_b.as_ref()) {
            if ma.colorspace != mb.colorspace
                || ma.color_transfer != mb.color_transfer
                || ma.color_primaries != mb.color_primaries
            {
                ui.add_space(6.0);
                egui::Frame::none()
                    .fill(Color32::from_rgba_premultiplied(180, 80, 0, 60))
                    .inner_margin(egui::Margin::symmetric(8.0, 5.0))
                    .rounding(4.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(if is_es {
                                "⚠ ¡Discrepancia de metadatos de color!"
                            } else {
                                "⚠ Color metadata mismatch detected!"
                            })
                            .color(Color32::from_rgb(255, 180, 60))
                            .size(11.5)
                            .strong(),
                        );
                        if ma.colorspace != mb.colorspace {
                            ui.label(
                                RichText::new(if is_es {
                                    format!("  Espacio: {} ≠ {}", ma.colorspace, mb.colorspace)
                                } else {
                                    format!("  Colorspace: {} ≠ {}", ma.colorspace, mb.colorspace)
                                })
                                .size(10.5)
                                .color(Color32::LIGHT_GRAY),
                            );
                        }
                        if ma.color_transfer != mb.color_transfer {
                            ui.label(
                                RichText::new(if is_es {
                                    format!(
                                        "  Transferencia: {} ≠ {}",
                                        ma.color_transfer, mb.color_transfer
                                    )
                                } else {
                                    format!(
                                        "  Transfer: {} ≠ {}",
                                        ma.color_transfer, mb.color_transfer
                                    )
                                })
                                .size(10.5)
                                .color(Color32::LIGHT_GRAY),
                            );
                        }
                        if ma.color_primaries != mb.color_primaries {
                            ui.label(
                                RichText::new(if is_es {
                                    format!(
                                        "  Primarios: {} ≠ {}",
                                        ma.color_primaries, mb.color_primaries
                                    )
                                } else {
                                    format!(
                                        "  Primaries: {} ≠ {}",
                                        ma.color_primaries, mb.color_primaries
                                    )
                                })
                                .size(10.5)
                                .color(Color32::LIGHT_GRAY),
                            );
                        }
                    });
            }
        }

        ui.add_space(6.0);

        // ── Usage hints ───────────────────────────────────────────────────
        ui.separator();
        ui.label(
            RichText::new(if is_es { "Atajos" } else { "Shortcuts" })
                .size(11.0)
                .strong()
                .weak(),
        );
        let mut hints: Vec<(&str, &str)> = vec![];
        if is_es {
            hints.push(("Espacio", "Reproducir / Pausa"));
            hints.push(("← →", "Avanzar frame"));
            hints.push(("Rueda", "Acercar / Alejar"));
            hints.push(("Arrastrar", "Desplazar"));
            hints.push(("Doble clk / R", "Restaurar zoom"));
            hints.push(("Inicio", "Ir al principio"));
            hints.push(("S", "Intercambiar A y B"));
            hints.push(("F", "Capturar pantalla (PNG)"));
            hints.push(("3", "Ocultar / Mostrar Interfaz"));
            hints.push(("4..9", "Ajustes rápidos de zoom"));
        } else {
            hints.push(("Space", "Play / Pause"));
            hints.push(("← →", "Step frame"));
            hints.push(("Scroll", "Zoom in / out"));
            hints.push(("Drag", "Pan"));
            hints.push(("Dbl-clk / R", "Reset zoom"));
            hints.push(("Home", "Go to start"));
            hints.push(("S", "Swap A and B"));
            hints.push(("F", "Take screenshot (PNG)"));
            hints.push(("3", "Toggle UI / HUD"));
            hints.push(("4..9", "Quick zoom presets"));
        }
        egui::Grid::new("hints_grid")
            .num_columns(2)
            .spacing([6.0, 2.0])
            .show(ui, |ui| {
                for (key, desc) in hints {
                    ui.label(
                        RichText::new(key)
                            .monospace()
                            .size(10.5)
                            .color(Color32::from_rgb(150, 200, 255)),
                    );
                    ui.label(RichText::new(desc).size(10.5).weak());
                    ui.end_row();
                }
            });
    });
}

// ---------------------------------------------------------------------------

fn channel_section(
    ui: &mut Ui,
    label: &str,
    accent: Color32,
    path: Option<&str>,
    meta: Option<&ColorMetadata>,
    is_es: bool,
) {
    ui.label(RichText::new(label).size(12.0).strong().color(accent));

    if let Some(path) = path {
        let filename = std::path::Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        ui.label(RichText::new(&filename).size(11.5).strong());
        ui.label(RichText::new(path).size(9.5).weak().italics());
    } else {
        ui.label(
            RichText::new(if is_es {
                "Ningún archivo cargado"
            } else {
                "No file loaded"
            })
            .weak()
            .italics()
            .size(11.0),
        );
        return;
    }

    if let Some(m) = meta {
        ui.add_space(3.0);
        egui::Grid::new(format!("meta_{label}"))
            .num_columns(2)
            .spacing([8.0, 2.0])
            .show(ui, |ui| {
                kv(
                    ui,
                    if is_es { "Resolución" } else { "Resolution" },
                    &format!("{}×{}", m.width, m.height),
                );
                kv(ui, "FPS", &format!("{:.4}", m.fps));
                kv(
                    ui,
                    if is_es { "Duración" } else { "Duration" },
                    &format_dur(m.duration_secs),
                );
                kv(
                    ui,
                    if is_es { "Tasa bits" } else { "Bitrate" },
                    &format!("{} kbps", m.bitrate_kbps),
                );
                kv(
                    ui,
                    if is_es { "Fmt Píxel" } else { "Pixel Fmt" },
                    &m.pixel_format,
                );
                kv(
                    ui,
                    if is_es { "Espacio" } else { "Colorspace" },
                    &m.colorspace,
                );
                kv(
                    ui,
                    if is_es { "Transfer" } else { "Transfer" },
                    &m.color_transfer,
                );
                kv(
                    ui,
                    if is_es { "Primarios" } else { "Primaries" },
                    &m.color_primaries,
                );
            });
    }
}

fn kv(ui: &mut Ui, key: &str, value: &str) {
    ui.label(RichText::new(key).size(10.5).weak());
    ui.label(RichText::new(value).size(10.5).monospace());
    ui.end_row();
}

fn format_dur(secs: f64) -> String {
    let total = secs as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    format!("{h:02}:{m:02}:{s:02}")
}
