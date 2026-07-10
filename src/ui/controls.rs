// ui/controls.rs — Menu bar (drop-down menus + inline toolbar controls)

use egui::{Color32, RichText, Ui};

use crate::app::DiffPlayerApp;
use crate::types::{Channel, CompareMode, DiffMode, Language, SafeZoneMode};
use crate::ui::design::{tr, ACCENT_PRIMARY, FONT_LABEL};
use crate::ui::i18n::{diff_mode_label, THEME_MENU_CHOICES};
use crate::ui::theme::apply_theme;

/// Title for the OBS clean-feed secondary viewport.
pub fn clean_feed_window_title(lang: Language) -> String {
    tr(
        lang,
        "DiffPlayerQC — Salida limpia",
        "DiffPlayerQC — Clean Feed",
        "DiffPlayerQC — Cén sirima",
    )
    .to_string()
}

/// Single-line overlay (mode, channel, PTS, frame) for the clean-feed window.
pub fn clean_feed_overlay_text(
    lang: Language,
    mode: CompareMode,
    split_pos: f32,
    pts: f64,
    fps: f64,
) -> String {
    let mode_str = match mode {
        CompareMode::SplitScreen => {
            if split_pos <= 0.01 {
                tr(lang, "Solo B", "B Only", "Erya B")
            } else if split_pos >= 0.99 {
                tr(lang, "Solo A", "A Only", "Erya A")
            } else {
                tr(lang, "Cortina", "Split", "Hyanda")
            }
        }
        CompareMode::AbsDiff => tr(lang, "Diferencia", "Diff", "Winya"),
        CompareMode::Heatmap => tr(lang, "Mapa de calor", "Heatmap", "Úrë"),
        CompareMode::SideBySide => tr(lang, "Lado a lado", "Side by side", "Ara"),
    };

    let video_str = match mode {
        CompareMode::SplitScreen => {
            if split_pos <= 0.01 {
                tr(lang, "VÍDEO B", "VIDEO B", "VÍDEO B")
            } else if split_pos >= 0.99 {
                tr(lang, "VÍDEO A", "VIDEO A", "VÍDEO A")
            } else {
                tr(lang, "VÍDEO A + B", "VIDEO A + B", "A + B")
            }
        }
        _ => tr(lang, "VÍDEO A + B", "VIDEO A + B", "A + B"),
    };

    let rough_frame = (pts * fps).round() as u64;
    format!(
        "{} | {} | {}: {:.3}s | {} {}",
        video_str,
        mode_str,
        tr(lang, "PTS", "PTS", "PTS"),
        pts,
        tr(lang, "Cuad.", "Frm.", "Fr."),
        rough_frame
    )
}

pub fn proxy_loading_caption(lang: Language) -> &'static str {
    tr(
        lang,
        "Cargando imágenes…",
        "Loading images…",
        "Cárala yando…",
    )
}

/// Renders the full menu bar: classic dropdown menus followed by an inline
/// compact toolbar row, all in a single top panel.
pub fn show_menu_bar(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    egui::menu::bar(ui, |ui| {
        // ── Dropdown menus ──────────────────────────────────────────────────

        ui.menu_button(tr(lang, "Archivo", "File", "Parma"), |ui| {
            if ui
                .button(tr(
                    lang,
                    "Abrir VÍDEO A…",
                    "Open VIDEO A…",
                    "Panya VÍDEO A…",
                ))
                .clicked()
            {
                app.open_video_a(ui.ctx());
                ui.close_menu();
            }

            if ui
                .button(tr(
                    lang,
                    "Abrir VÍDEO B…",
                    "Open VIDEO B…",
                    "Panya VÍDEO B…",
                ))
                .clicked()
            {
                app.open_video_b(ui.ctx());
                ui.close_menu();
            }

            ui.separator();

            if ui
                .button(tr(
                    lang,
                    "Cargar sesión (.dpqc)…",
                    "Load Session (.dpqc)…",
                    "Load Session (.dpqc)…",
                ))
                .clicked()
            {
                app.load_session(ui.ctx());
                ui.close_menu();
            }

            if ui
                .button(tr(
                    lang,
                    "Guardar sesión (.dpqc)…",
                    "Save Session (.dpqc)…",
                    "Save Session (.dpqc)…",
                ))
                .clicked()
            {
                app.save_session();
                ui.close_menu();
            }

            if ui
                .button(tr(
                    lang,
                    "Exportar marcadores a CSV…",
                    "Export markers to CSV…",
                    "Export markers to CSV…",
                ))
                .clicked()
            {
                app.export_csv();
                ui.close_menu();
            }

            ui.separator();

            if ui
                .button(tr(
                    lang,
                    "Abrir secuencia EXR (A)…",
                    "Open EXR sequence (A)…",
                    "Panya EXR sequence (A)…",
                ))
                .clicked()
            {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    app.start_proxy_from_exr_input_dir(folder, Channel::A, ui.ctx());
                }
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Abrir secuencia EXR (B)…",
                    "Open EXR sequence (B)…",
                    "Panya EXR sequence (B)…",
                ))
                .clicked()
            {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    app.start_proxy_from_exr_input_dir(folder, Channel::B, ui.ctx());
                }
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Abrir archivos EXR (A)…",
                    "Open EXR files (A)…",
                    "Panya EXR files (A)…",
                ))
                .clicked()
            {
                if let Some(files) = rfd::FileDialog::new()
                    .add_filter("EXR", &["exr"])
                    .pick_files()
                {
                    if !files.is_empty() {
                        app.start_proxy_from_exr_input_files(files, Channel::A, ui.ctx());
                    }
                }
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Abrir archivos EXR (B)…",
                    "Open EXR files (B)…",
                    "Panya EXR files (B)…",
                ))
                .clicked()
            {
                if let Some(files) = rfd::FileDialog::new()
                    .add_filter("EXR", &["exr"])
                    .pick_files()
                {
                    if !files.is_empty() {
                        app.start_proxy_from_exr_input_files(files, Channel::B, ui.ctx());
                    }
                }
                ui.close_menu();
            }

            ui.separator();
            if ui
                .button(tr(
                    lang,
                    "Guardar Frame como PNG  (F)",
                    "Save Frame as PNG  (F)",
                    "Marta Frame ve PNG  (F)",
                ))
                .clicked()
            {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Screenshot);
                ui.close_menu();
            }

            if ui
                .button(tr(
                    lang,
                    "Elegir carpeta de capturas…",
                    "Set Screenshot Folder…",
                    "Cilta Screenshot Nómë…",
                ))
                .clicked()
            {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    app.view_mut().screenshot_dir = Some(folder);
                }
                ui.close_menu();
            }
            ui.separator();
            if ui
                .button(tr(lang, "Salir  (Esc)", "Quit  (Esc)", "Vanya  (Esc)"))
                .clicked()
            {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        ui.menu_button(tr(lang, "Vista", "View", "Cén"), |ui| {
            if ui
                .button(tr(
                    lang,
                    "Ocultar/Mostrar Interfaz  (3)",
                    "Toggle HUD  (3)",
                    "Halya/Tanë HUD  (3)",
                ))
                .clicked()
            {
                let v = app.view().show_hud;
                app.view_mut().show_hud = !v;
                ui.close_menu();
            }
            ui.separator();
            let mut left = app.view().show_left_panel;
            if ui
                .checkbox(
                    &mut left,
                    tr(
                        lang,
                        "Barra izquierda (datos del vídeo)",
                        "Left panel (video data)",
                        "Parma left (video data)",
                    ),
                )
                .changed()
            {
                app.view_mut().show_left_panel = left;
                ui.close_menu();
            }
            let mut right = app.view().show_right_panel;
            if ui
                .checkbox(
                    &mut right,
                    tr(
                        lang,
                        "Barra derecha (controles y audio)",
                        "Right panel (controls & audio)",
                        "Parma right (controls & audio)",
                    ),
                )
                .changed()
            {
                app.view_mut().show_right_panel = right;
                ui.close_menu();
            }
            ui.separator();
            if ui
                .button(tr(
                    lang,
                    "Restaurar Zoom  (R)",
                    "Reset Zoom  (R)",
                    "En-panya Zoom  (R)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 1.0;
                app.view_mut().pan_u = 0.0;
                app.view_mut().pan_v = 0.0;
                ui.close_menu();
            }
            if ui
                .button(tr(lang, "Zoom 50%  (5)", "Zoom 50%  (5)", "Zoom 50%  (5)"))
                .clicked()
            {
                app.view_mut().zoom = 0.5;
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Zoom 100%  (6)",
                    "Zoom 100%  (6)",
                    "Zoom 100%  (6)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 1.0;
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Zoom 200%  (7)",
                    "Zoom 200%  (7)",
                    "Zoom 200%  (7)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 2.0;
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Zoom 400%  (8)",
                    "Zoom 400%  (8)",
                    "Zoom 400%  (8)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 4.0;
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Zoom 800%  (9)",
                    "Zoom 800%  (9)",
                    "Zoom 800%  (9)",
                ))
                .clicked()
            {
                app.view_mut().zoom = 8.0;
                ui.close_menu();
            }
        });

        ui.menu_button(tr(lang, "Reproducción", "Playback", "Lirë"), |ui| {
            let is_p = app.playback().is_playing;
            if ui
                .button(if is_p {
                    tr(
                        lang,
                        "Pausar  (Espacio)",
                        "Pause  (Space)",
                        "Talta  (Espacio)",
                    )
                } else {
                    tr(
                        lang,
                        "Reproducir  (Espacio)",
                        "Play  (Space)",
                        "Lir  (Espacio)",
                    )
                })
                .clicked()
            {
                if is_p {
                    app.do_pause(ui.ctx());
                } else {
                    app.do_play(ui.ctx());
                }
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Retroceder Frame (Izquierda / Left)",
                    "Step Backward (Left)",
                    "Nánë Frame (Left)",
                ))
                .clicked()
            {
                app.do_step_bck(ui.ctx());
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Avanzar Frame (Derecha / Right)",
                    "Step Forward (Right)",
                    "Pónë Frame (Right)",
                ))
                .clicked()
            {
                app.do_step_fwd(ui.ctx());
                ui.close_menu();
            }
            if ui
                .button(tr(
                    lang,
                    "Ir al inicio  (Home)",
                    "Go to Start  (Home)",
                    "Mena Yessë  (Home)",
                ))
                .clicked()
            {
                app.do_seek(0.0, ui.ctx());
                ui.close_menu();
            }
        });

        ui.menu_button(tr(lang, "Opciones", "Options", "Cilmë"), |ui| {
            if ui
                .button(tr(
                    lang,
                    "Intercambiar A y B  (S)",
                    "Swap A and B  (S)",
                    "Quista A ar B  (S)",
                ))
                .clicked()
            {
                app.swap_videos(ui.ctx());
                ui.close_menu();
            }

            ui.separator();

            // Canvas background colour
            ui.horizontal(|ui| {
                ui.label(tr(lang, "Color fondo:", "Canvas color:", "Talan cala:"));
                let mut bg = app.view().canvas_bg_color;
                if ui.color_edit_button_rgb(&mut bg).changed() {
                    app.view_mut().canvas_bg_color = bg;
                }
            });

            ui.separator();
            ui.menu_button(
                tr(lang, "Idioma / Language", "Language / Idioma", "Lambë"),
                |ui| {
                    if ui
                        .radio_value(&mut app.view_mut().lang, Language::En, "English")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(&mut app.view_mut().lang, Language::Es, "Español")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .radio_value(
                            &mut app.view_mut().lang,
                            Language::Quenya,
                            "Quenya (Elvish)",
                        )
                        .clicked()
                    {
                        ui.close_menu();
                    }
                },
            );
            ui.menu_button(tr(lang, "Tema / Theme", "Theme / Tema", "Cala"), |ui| {
                let mut current_theme = app.view().theme;
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        for &(theme_val, name) in THEME_MENU_CHOICES {
                            if ui
                                .radio_value(&mut current_theme, theme_val, name)
                                .clicked()
                            {
                                app.view_mut().theme = theme_val;
                                apply_theme(ui.ctx(), theme_val);
                                ui.close_menu();
                            }
                        }
                    });
            });
        });

        ui.menu_button(tr(lang, "Emisión", "Broadcast", "Sirë"), |ui| {
            let mut enabled = app.view().show_clean_feed_window;
            if ui
                .checkbox(
                    &mut enabled,
                    tr(
                        lang,
                        "Ventana de Salida  (OBS)",
                        "Clean Feed Window  (OBS)",
                        "Vëa Cén  (OBS)",
                    ),
                )
                .clicked()
            {
                app.view_mut().show_clean_feed_window = enabled;
                ui.close_menu();
            }
            ui.label(
                RichText::new(tr(
                    lang,
                    "Capturar ventana en OBS",
                    "Capture window in OBS",
                    "Mapa vëa mi OBS",
                ))
                .weak()
                .size(FONT_LABEL),
            );
            ui.separator();
            ui.label(
                RichText::new(tr(lang, "Zonas seguras", "Safe Zones", "Safe zones"))
                    .weak()
                    .size(FONT_LABEL),
            );
            let mut safe_zone = app.view().safe_zone;
            if ui
                .radio_value(
                    &mut safe_zone,
                    SafeZoneMode::None,
                    tr(lang, "Desactivado", "Off", "Off"),
                )
                .clicked()
            {
                ui.close_menu();
            }
            if ui
                .radio_value(&mut safe_zone, SafeZoneMode::TvEbu, "TV: EBU R95 (16:9)")
                .clicked()
            {
                ui.close_menu();
            }
            if ui
                .radio_value(
                    &mut safe_zone,
                    SafeZoneMode::Social,
                    tr(
                        lang,
                        "Móvil: Redes Sociales (9:16)",
                        "Mobile: Social (9:16)",
                        "Social (9:16)",
                    ),
                )
                .clicked()
            {
                ui.close_menu();
            }
            app.view_mut().safe_zone = safe_zone;
        });

        // ── Separator before inline controls ───────────────────────────────
        ui.separator();

        // ── Inline compact controls ─────────────────────────────────────────

        // File open buttons
        let has_a = app.decoder_a_path().is_some();
        let has_b = app.decoder_b_path().is_some();

        let a_label = app
            .decoder_a_path()
            .map(short_name)
            .unwrap_or_else(|| "A…".to_owned());
        let b_label = app
            .decoder_b_path()
            .map(short_name)
            .unwrap_or_else(|| "B…".to_owned());

        let a_tooltip = app
            .decoder_a_path()
            .map(|p| p.to_owned())
            .unwrap_or_else(|| tr(lang, "Abrir Vídeo A", "Open Video A", "Panya A").to_owned());
        if ui
            .add(egui::Button::new(
                RichText::new(format!("▶A {a_label}")).color(if has_a {
                    Color32::from_rgb(100, 200, 120)
                } else {
                    Color32::LIGHT_GRAY
                }),
            ))
            .on_hover_text(a_tooltip)
            .clicked()
        {
            app.open_video_a(ui.ctx());
        }

        let b_tooltip = app
            .decoder_b_path()
            .map(|p| p.to_owned())
            .unwrap_or_else(|| tr(lang, "Abrir Vídeo B", "Open Video B", "Panya B").to_owned());
        if ui
            .add(egui::Button::new(
                RichText::new(format!("▶B {b_label}")).color(if has_b {
                    Color32::from_rgb(100, 160, 240)
                } else {
                    Color32::LIGHT_GRAY
                }),
            ))
            .on_hover_text(b_tooltip)
            .clicked()
        {
            app.open_video_b(ui.ctx());
        }

        ui.separator();

        // Playback controls
        let mut loop_playback = app.view().loop_playback;
        if ui
            .checkbox(&mut loop_playback, tr(lang, "Bucle", "Loop", "Loop"))
            .changed()
        {
            app.view_mut().loop_playback = loop_playback;
            if loop_playback {
                app.playback_mut().loop_range_active = false;
            }
        }

        let mut loop_range = app.playback().loop_range_active;
        if ui
            .checkbox(
                &mut loop_range,
                tr(lang, "Bucle Rango", "Loop Range", "Loop Range"),
            )
            .changed()
        {
            if loop_range {
                app.toggle_loop_range(); // This handles turning it on and disabling `loop_playback`
            } else {
                app.playback_mut().loop_range_active = false;
            }
        }

        ui.add_space(4.0);
        if ui
            .button("[ I ]")
            .on_hover_text(tr(
                lang,
                "Marcar inicio de bucle",
                "Set Loop In",
                "Set Loop In",
            ))
            .clicked()
        {
            app.set_loop_in();
        }
        if ui
            .button("[ O ]")
            .on_hover_text(tr(
                lang,
                "Marcar fin de bucle",
                "Set Loop Out",
                "Set Loop Out",
            ))
            .clicked()
        {
            app.set_loop_out();
        }
        ui.add_space(4.0);

        let is_playing = app.playback().is_playing;
        if ui
            .button(RichText::new("|<").size(16.0))
            .on_hover_text(tr(lang, "Inicio", "Start", "Yessë"))
            .clicked()
        {
            app.do_seek(0.0, ui.ctx());
        }
        if ui
            .button(RichText::new("<<").size(16.0))
            .on_hover_text(tr(
                lang,
                "Retroceder (Izquierda)",
                "Step back (Left)",
                "Nánë (Left)",
            ))
            .clicked()
        {
            app.do_step_bck(ui.ctx());
        }
        if ui
            .button(RichText::new(if is_playing { "||" } else { ">" }).size(16.0))
            .on_hover_text(tr(
                lang,
                "Reproducir/Pausar (Espacio)",
                "Play/Pause (Space)",
                "Lir/Talta",
            ))
            .clicked()
        {
            if is_playing {
                app.do_pause(ui.ctx());
            } else {
                app.do_play(ui.ctx());
            }
        }
        if ui
            .button(RichText::new(">>").size(16.0))
            .on_hover_text(tr(
                lang,
                "Avanzar (Derecha)",
                "Step fwd (Right)",
                "Pónë (Right)",
            ))
            .clicked()
        {
            app.do_step_fwd(ui.ctx());
        }
        // Mode selector and contextual options moved to right sidebar (show_audio_panel) for low-res visibility.
    });
}

fn short_name(path: &str) -> String {
    // Show only the file stem (no extension) truncated to 18 chars for compactness
    let name = std::path::Path::new(path)
        .file_stem()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_owned());
    if name.len() > 18 {
        format!("{}…", &name[..18])
    } else {
        name
    }
}

/// Mode selector and contextual options (Cortina, Amp, Diff mode, Zoom). Used in the right sidebar.
pub fn show_mode_toolbar(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    let c_mode = app.view().mode;
    let split = app.view().split_pos;
    let is_a = c_mode == CompareMode::SplitScreen && split > 0.95;
    let is_b = c_mode == CompareMode::SplitScreen && split < 0.05;
    let is_split = c_mode == CompareMode::SplitScreen && !is_a && !is_b;
    let active = ACCENT_PRIMARY;

    ui.vertical(|ui| {
        ui.set_min_width(90.0);
        // Display mode buttons (stacked for narrow sidebar)
        if ui
            .add(
                egui::Button::new(tr(lang, "Solo A", "A Only", "Erya A")).fill(if is_a {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SplitScreen;
            app.view_mut().split_pos = 1.0;
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Solo B", "B Only", "Erya B")).fill(if is_b {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SplitScreen;
            app.view_mut().split_pos = 0.0;
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Cortina", "Split", "Hyanda")).fill(if is_split {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SplitScreen;
            if is_a || is_b {
                app.view_mut().split_pos = 0.5;
            }
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Diferencia", "Diff", "Winya")).fill(
                    if c_mode == CompareMode::AbsDiff {
                        active
                    } else {
                        Color32::TRANSPARENT
                    },
                ),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::AbsDiff;
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Mapa Calor", "Heatmap", "Úrë")).fill(
                    if c_mode == CompareMode::Heatmap {
                        active
                    } else {
                        Color32::TRANSPARENT
                    },
                ),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::Heatmap;
        }
        if ui
            .add(
                egui::Button::new(tr(lang, "Lado a Lado", "Side×Side", "Ara")).fill(
                    if c_mode == CompareMode::SideBySide {
                        active
                    } else {
                        Color32::TRANSPARENT
                    },
                ),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SideBySide;
        }

        ui.separator();

        match app.view().mode {
            CompareMode::SplitScreen => {
                let is_h = app.view().split_horizontal;
                if ui
                    .button(if is_h {
                        tr(lang, "Cortina H", "Split H", "Hya H")
                    } else {
                        tr(lang, "Cortina V", "Split V", "Hya V")
                    })
                    .clicked()
                {
                    app.view_mut().split_horizontal = !app.view().split_horizontal;
                }
                ui.label(if is_h {
                    tr(lang, "Cortina (Y):", "Split (Y):", "Hyanda (Y):")
                } else {
                    tr(lang, "Cortina (X):", "Split (X):", "Hyanda (X):")
                });
                let mut sp = app.view().split_pos;
                if ui
                    .add(egui::Slider::new(&mut sp, 0.0..=1.0).fixed_decimals(2))
                    .changed()
                {
                    app.view_mut().split_pos = sp;
                }
            }
            CompareMode::Heatmap | CompareMode::AbsDiff => {
                ui.label(tr(lang, "Amplificación:", "Amplification:", "Amp:"));
                let mut amp = app.view().amplifier;
                if ui
                    .add(
                        egui::Slider::new(&mut amp, 1.0..=50.0)
                            .step_by(0.5)
                            .suffix("×"),
                    )
                    .changed()
                {
                    app.view_mut().amplifier = amp;
                }
                if app.view().mode == CompareMode::AbsDiff {
                    ui.separator();
                    let mut d_mode = app.view().diff_mode;
                    egui::ComboBox::from_id_source("diff_mode_side")
                        .selected_text(diff_mode_label(lang, d_mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut d_mode,
                                DiffMode::LegacyAbs,
                                diff_mode_label(lang, DiffMode::LegacyAbs),
                            );
                            ui.selectable_value(
                                &mut d_mode,
                                DiffMode::AbsLinear,
                                diff_mode_label(lang, DiffMode::AbsLinear),
                            );
                            ui.selectable_value(
                                &mut d_mode,
                                DiffMode::AbsSqrt,
                                diff_mode_label(lang, DiffMode::AbsSqrt),
                            );
                            ui.selectable_value(
                                &mut d_mode,
                                DiffMode::SignedDiverging,
                                diff_mode_label(lang, DiffMode::SignedDiverging),
                            );
                        });
                    if d_mode != app.view().diff_mode {
                        app.view_mut().diff_mode = d_mode;
                    }
                }
            }
            CompareMode::SideBySide => {
                ui.label(tr(lang, "Amplificación:", "Amplification:", "Amp:"));
                let mut amp = app.view().amplifier;
                if ui
                    .add(
                        egui::Slider::new(&mut amp, 1.0..=50.0)
                            .step_by(0.5)
                            .suffix("×"),
                    )
                    .changed()
                {
                    app.view_mut().amplifier = amp;
                }
                ui.separator();
                let mut d_mode = app.view().diff_mode;
                egui::ComboBox::from_id_source("diff_mode_sbs_side")
                    .selected_text(match d_mode {
                        DiffMode::None => tr(lang, "Sin Filtro", "No Filter", "Munca").to_string(),
                        _ => diff_mode_label(lang, d_mode).to_string(),
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::LegacyAbs,
                            diff_mode_label(lang, DiffMode::LegacyAbs),
                        );
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::AbsLinear,
                            diff_mode_label(lang, DiffMode::AbsLinear),
                        );
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::AbsSqrt,
                            diff_mode_label(lang, DiffMode::AbsSqrt),
                        );
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::SignedDiverging,
                            diff_mode_label(lang, DiffMode::SignedDiverging),
                        );
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::None,
                            tr(lang, "Sin Filtro", "No Filter", "Munca"),
                        );
                    });
                if d_mode != app.view().diff_mode {
                    app.view_mut().diff_mode = d_mode;
                }
            }
        }

        let zoom = app.view().zoom;
        if (zoom - 1.0).abs() > 0.01 {
            ui.separator();
            if ui
                .button(format!("Zoom {:.1}×", zoom))
                .on_hover_text(tr(
                    lang,
                    "Restaurar zoom y paneo",
                    "Reset zoom and pan",
                    "En-panya hyanda ar pan",
                ))
                .clicked()
            {
                app.view_mut().zoom = 1.0;
                app.view_mut().pan_u = 0.0;
                app.view_mut().pan_v = 0.0;
            }
        }
    });
}

pub fn show_audio_panel(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let lang = app.view().lang;
    ui.vertical_centered(|ui| {
        show_mode_toolbar(ui, app);
        ui.separator();
        ui.heading(tr(lang, "Audio", "Audio", "Lind"));
        ui.add_space(6.0);

        ui.label(
            RichText::new("A")
                .color(Color32::from_rgb(100, 200, 120))
                .strong(),
        );
        let mut mute_a = app.view().mute_a;
        let resp_a = ui.button(if mute_a {
            tr(lang, "Activar", "Unmute", "Nanquet")
        } else {
            tr(lang, "Silenciar", "Mute", "Tamya")
        });
        if resp_a.clicked() {
            mute_a = !mute_a;
            app.view_mut().mute_a = mute_a;
            if !mute_a {
                app.view_mut().mute_b = true; // Mutuamente excluyentes
            }
            ui.ctx().request_repaint();
        }
        resp_a.on_hover_text(if mute_a {
            tr(
                lang,
                "Canal A silenciado (clic para activar el sonido)",
                "Channel A muted (click to unmute)",
                "A tamya (nanquet)",
            )
        } else {
            tr(
                lang,
                "Canal A con sonido (clic para silenciar)",
                "Channel A audible (click to mute)",
                "A lind (tamya)",
            )
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        ui.label(
            RichText::new("B")
                .color(Color32::from_rgb(100, 160, 240))
                .strong(),
        );
        let mut mute_b = app.view().mute_b;
        let resp_b = ui.button(if mute_b {
            tr(lang, "Activar", "Unmute", "Nanquet")
        } else {
            tr(lang, "Silenciar", "Mute", "Tamya")
        });
        if resp_b.clicked() {
            mute_b = !mute_b;
            app.view_mut().mute_b = mute_b;
            if !mute_b {
                app.view_mut().mute_a = true; // Mutuamente excluyentes
            }
            ui.ctx().request_repaint();
        }
        resp_b.on_hover_text(if mute_b {
            tr(
                lang,
                "Canal B silenciado (clic para activar el sonido)",
                "Channel B muted (click to unmute)",
                "B tamya (nanquet)",
            )
        } else {
            tr(
                lang,
                "Canal B con sonido (clic para silenciar)",
                "Channel B audible (click to mute)",
                "B lind (tamya)",
            )
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        #[cfg(target_os = "macos")]
        {
            if ui
                .button(tr(
                    lang,
                    "Audiometer (Abrir/Cerrar)",
                    "Audiometer (Toggle)",
                    "Audiometer",
                ))
                .on_hover_text(tr(
                    lang,
                    "Abre o cierra el medidor Audiometer (Youlean)",
                    "Opens or closes the Audiometer",
                    "Audiometer",
                ))
                .clicked()
            {
                let is_running = std::process::Command::new("osascript")
                    .arg("-e")
                    .arg("application \"Youlean Loudness Meter 2\" is running")
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "true")
                    .unwrap_or(false);

                if is_running {
                    if let Err(e) = std::process::Command::new("osascript")
                        .arg("-e")
                        .arg("tell application \"Youlean Loudness Meter 2\" to quit")
                        .spawn()
                    {
                        log::warn!("Failed to quit Youlean Loudness Meter 2: {}", e);
                    }
                    if let Some(saved) = app.view_mut().saved_loop_playback.take() {
                        app.view_mut().loop_playback = saved;
                    }
                } else {
                    if let Err(e) = std::process::Command::new("open")
                        .arg("-a")
                        .arg("Youlean Loudness Meter 2")
                        .spawn()
                    {
                        log::warn!("Failed to open Youlean Loudness Meter 2: {}", e);
                    }
                    app.view_mut().saved_loop_playback = Some(app.view().loop_playback);
                    app.view_mut().loop_playback = false;
                    app.do_seek(0.0, ui.ctx());

                    // Wait 2.5 seconds for Youlean GUI to open before playing
                    if app.playback().is_playing {
                        app.do_pause(ui.ctx());
                    }
                    app.view_mut().pending_play_after_delay =
                        Some(std::time::Instant::now() + std::time::Duration::from_millis(2500));
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            ui.add_enabled(
                false,
                egui::Button::new(tr(
                    lang,
                    "Audiometer (No soportado)",
                    "Audiometer (Unsupported)",
                    "Audiometer",
                )),
            )
            .on_hover_text(tr(
                lang,
                "Integración Youlean no soportada en esta plataforma.",
                "Youlean integration not supported on this platform.",
                "Youlean integration not supported on this platform.",
            ));
        }

        ui.add_space(10.0);

        if ui
            .button(tr(
                lang,
                "VU Meter (Abrir/Cerrar)",
                "VU Meter (Toggle)",
                "VU Meter",
            ))
            .on_hover_text(tr(
                lang,
                "Abre o cierra el vúmetro digital LED",
                "Opens or closes the digital LED VU Meter",
                "VU Meter",
            ))
            .clicked()
        {
            app.view_mut().show_vu_meter = !app.view().show_vu_meter;
        }
    });
}
