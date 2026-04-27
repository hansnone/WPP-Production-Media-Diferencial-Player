// ui/info_panel.rs — Left side panel: metadata, color info, status

use egui::{Color32, RichText, Ui};

use crate::app::DiffPlayerApp;
use crate::types::{ColorMetadata, Language};
use crate::ui::design::{tr, FONT_LABEL, FONT_SUBTITLE, FONT_TITLE, FONT_VALUE};

pub fn show(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
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
            RichText::new("WPP Production Media Differential Player")
                .size(FONT_TITLE)
                .strong()
                .color(Color32::from_rgb(80, 160, 230)),
        );
        ui.label(
            RichText::new(tr(
                lang,
                "Control de Calidad Frame a Frame",
                "Frame-Accurate Video QC",
                "QC vídeo nu per ranga",
            ))
            .size(FONT_SUBTITLE)
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
                kv(ui, tr(lang, "PTS", "PTS", "PTS"), &format!("{pts:.4} s"));
                kv(
                    ui,
                    tr(lang, "Cuadro", "Frame", "Quanta"),
                    &frame_n.to_string(),
                );
                kv(
                    ui,
                    tr(lang, "Zoom", "Zoom", "Hyanda"),
                    &format!("{zoom:.2}×"),
                );
                kv(
                    ui,
                    tr(lang, "Paneo", "Pan UV", "Pan"),
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
            tr(lang, "VÍDEO A", "VIDEO A", "VÍDEO A"),
            color_a,
            path_a.as_deref(),
            meta_a.as_ref(),
            lang,
        );

        ui.add_space(8.0);
        ui.separator();

        // ── Video B info ──────────────────────────────────────────────────
        channel_section(
            ui,
            tr(lang, "VÍDEO B", "VIDEO B", "VÍDEO B"),
            color_b,
            path_b.as_deref(),
            meta_b.as_ref(),
            lang,
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
                            RichText::new(tr(
                                lang,
                                "⚠ ¡Discrepancia de metadatos de color!",
                                "⚠ Color metadata mismatch detected!",
                                "⚠ Cala meta winya!",
                            ))
                            .color(Color32::from_rgb(255, 180, 60))
                            .size(11.5)
                            .strong(),
                        );
                        if ma.colorspace != mb.colorspace {
                            ui.label(
                                RichText::new(format!(
                                    "  {}: {} ≠ {}",
                                    tr(lang, "Espacio", "Colorspace", "Cala"),
                                    ma.colorspace,
                                    mb.colorspace
                                ))
                                .size(FONT_LABEL)
                                .color(Color32::LIGHT_GRAY),
                            );
                        }
                        if ma.color_transfer != mb.color_transfer {
                            ui.label(
                                RichText::new(format!(
                                    "  {}: {} ≠ {}",
                                    tr(lang, "Transferencia", "Transfer", "Tíra"),
                                    ma.color_transfer,
                                    mb.color_transfer
                                ))
                                .size(FONT_LABEL)
                                .color(Color32::LIGHT_GRAY),
                            );
                        }
                        if ma.color_primaries != mb.color_primaries {
                            ui.label(
                                RichText::new(format!(
                                    "  {}: {} ≠ {}",
                                    tr(lang, "Primarios", "Primaries", "Hairë"),
                                    ma.color_primaries,
                                    mb.color_primaries
                                ))
                                .size(FONT_LABEL)
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
            RichText::new(tr(lang, "Atajos", "Shortcuts", "Quanta ranga"))
                .size(FONT_SUBTITLE)
                .strong()
                .weak(),
        );
        let hints: Vec<(&str, &str)> = match lang {
            Language::Es => vec![
                ("Espacio", "Reproducir / Pausa"),
                ("← →", "Avanzar frame"),
                ("Rueda", "Acercar / Alejar"),
                ("Arrastrar", "Desplazar"),
                ("Doble clk / R", "Restaurar zoom"),
                ("Inicio", "Ir al principio"),
                ("S", "Intercambiar A y B"),
                ("F", "Capturar pantalla (PNG)"),
                ("3", "Ocultar / Mostrar Interfaz"),
                ("4..9", "Ajustes rápidos de zoom"),
            ],
            Language::En => vec![
                ("Space", "Play / Pause"),
                ("← →", "Step frame"),
                ("Scroll", "Zoom in / out"),
                ("Drag", "Pan"),
                ("Dbl-clk / R", "Reset zoom"),
                ("Home", "Go to start"),
                ("S", "Swap A and B"),
                ("F", "Take screenshot (PNG)"),
                ("3", "Toggle UI / HUD"),
                ("4..9", "Quick zoom presets"),
            ],
            Language::Quenya => vec![
                ("Space", "Lir / Talta"),
                ("← →", "Quanta ranga"),
                ("Scroll", "Hyanda"),
                ("Drag", "Pano"),
                ("Dbl-clk / R", "En-panya zoom"),
                ("Home", "Yessë"),
                ("S", "Quista A ar B"),
                ("F", "Harya PNG"),
                ("3", "HUD"),
                ("4..9", "Zoom ve"),
            ],
        };
        egui::Grid::new("hints_grid")
            .num_columns(2)
            .spacing([6.0, 2.0])
            .show(ui, |ui| {
                for (key, desc) in hints {
                    ui.label(
                        RichText::new(key)
                            .monospace()
                            .size(FONT_LABEL)
                            .color(Color32::from_rgb(150, 200, 255)),
                    );
                    ui.label(RichText::new(desc).size(FONT_LABEL).weak());
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
    lang: Language,
) {
    ui.label(RichText::new(label).size(12.0).strong().color(accent));

    if let Some(path) = path {
        let filename = std::path::Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        ui.label(RichText::new(&filename).size(11.5).strong());
        ui.label(RichText::new(path).size(FONT_VALUE).weak().italics());
    } else {
        ui.label(
            RichText::new(tr(
                lang,
                "Ningún archivo cargado",
                "No file loaded",
                "La parma",
            ))
            .weak()
            .italics()
            .size(FONT_VALUE),
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
                    tr(lang, "Resolución", "Resolution", "Palúrë"),
                    &format!("{}×{}", m.width, m.height),
                );
                kv(ui, tr(lang, "FPS", "FPS", "FPS"), &format!("{:.4}", m.fps));
                kv(
                    ui,
                    tr(lang, "Duración", "Duration", "Lúmë"),
                    &format_dur(m.duration_secs),
                );
                kv(
                    ui,
                    tr(lang, "Tasa bits", "Bitrate", "Tix"),
                    &format!("{} kbps", m.bitrate_kbps),
                );
                kv(
                    ui,
                    tr(lang, "Fmt Píxel", "Pixel Fmt", "Píxel"),
                    &m.pixel_format,
                );
                kv(
                    ui,
                    tr(lang, "Espacio", "Colorspace", "Cala"),
                    &m.colorspace,
                );
                kv(
                    ui,
                    tr(lang, "Transfer", "Transfer", "Tíra"),
                    &m.color_transfer,
                );
                kv(
                    ui,
                    tr(lang, "Primarios", "Primaries", "Hairë"),
                    &m.color_primaries,
                );
                kv(
                    ui,
                    tr(lang, "Códec video", "Video codec", "Códec vídeo"),
                    if m.video_codec.is_empty() {
                        "—"
                    } else {
                        m.video_codec.as_str()
                    },
                );
                kv(
                    ui,
                    tr(lang, "Códec audio", "Audio codec", "Códec audio"),
                    if m.audio_codec.is_empty() {
                        "—"
                    } else {
                        m.audio_codec.as_str()
                    },
                );
                kv(
                    ui,
                    tr(
                        lang,
                        "Marca contenedor",
                        "Major brand",
                        "Marca",
                    ),
                    if m.major_brand.is_empty() || m.major_brand == "—" {
                        "—"
                    } else {
                        m.major_brand.as_str()
                    },
                );
                {
                    let v = if m.video_stream_metadata.is_empty() {
                        "—".to_string()
                    } else {
                        truncate_meta(&m.video_stream_metadata, 50)
                    };
                    kv(
                        ui,
                        tr(
                            lang,
                            "Stream vídeo (meta)",
                            "Stream video (meta)",
                            "Stream vídeo",
                        ),
                        &v,
                    );
                }
                {
                    let v = if m.audio_stream_metadata.is_empty() || m.audio_stream_metadata == "—"
                    {
                        "—".to_string()
                    } else {
                        truncate_meta(&m.audio_stream_metadata, 50)
                    };
                    kv(
                        ui,
                        tr(
                            lang,
                            "Stream audio (meta)",
                            "Stream audio (meta)",
                            "Stream audio",
                        ),
                        &v,
                    );
                }
            });
    }
}

fn truncate_meta(s: &str, max_len: usize) -> String {
    let one_line: String = s.replace('\n', " ");
    if one_line.len() <= max_len {
        one_line
    } else {
        format!("{}…", one_line.chars().take(max_len).collect::<String>())
    }
}

fn kv(ui: &mut Ui, key: &str, value: &str) {
    ui.label(RichText::new(key).size(FONT_LABEL).weak());
    ui.add(egui::Label::new(RichText::new(value).size(FONT_VALUE).monospace()).wrap(true));
    ui.end_row();
}

fn format_dur(secs: f64) -> String {
    let total = secs as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    format!("{h:02}:{m:02}:{s:02}")
}
