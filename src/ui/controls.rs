// ui/controls.rs — Menu bar (drop-down menus + inline toolbar controls)

use egui::{Color32, RichText, Ui};

use crate::app::DiffPlayerApp;
use crate::types::{Channel, CompareMode, DiffMode, Language, SafeZoneMode};
use crate::ui::theme::apply_theme;

/// Renders the full menu bar: classic dropdown menus followed by an inline
/// compact toolbar row, all in a single top panel.
pub fn show_menu_bar(ui: &mut Ui, app: &mut DiffPlayerApp) {
    egui::menu::bar(ui, |ui| {
        // ── Dropdown menus ──────────────────────────────────────────────────

        ui.menu_button(
            match app.view().lang {
                Language::Es => "Archivo",
                Language::En => "File",
                Language::Quenya => "Parma",
            },
            |ui| {
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Abrir VÍDEO A…",
                        Language::En => "Open VIDEO A…",
                        Language::Quenya => "Panya VÍDEO A…",
                    })
                    .clicked()
                {
                    app.open_video_a(ui.ctx());
                    ui.close_menu();
                }

                if ui
                    .button(match app.view().lang {
                        Language::Es => "Abrir VÍDEO B…",
                        Language::En => "Open VIDEO B…",
                        Language::Quenya => "Panya VÍDEO B…",
                    })
                    .clicked()
                {
                    app.open_video_b(ui.ctx());
                    ui.close_menu();
                }

                if ui
                    .button(match app.view().lang {
                        Language::Es => "Abrir secuencia EXR (A)…",
                        Language::En => "Open EXR sequence (A)…",
                        Language::Quenya => "Panya EXR sequence (A)…",
                    })
                    .clicked()
                {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        app.start_proxy_from_exr_input_dir(folder, Channel::A, ui.ctx());
                    }
                    ui.close_menu();
                }
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Abrir secuencia EXR (B)…",
                        Language::En => "Open EXR sequence (B)…",
                        Language::Quenya => "Panya EXR sequence (B)…",
                    })
                    .clicked()
                {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        app.start_proxy_from_exr_input_dir(folder, Channel::B, ui.ctx());
                    }
                    ui.close_menu();
                }
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Abrir archivos EXR (A)…",
                        Language::En => "Open EXR files (A)…",
                        Language::Quenya => "Panya EXR files (A)…",
                    })
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
                    .button(match app.view().lang {
                        Language::Es => "Abrir archivos EXR (B)…",
                        Language::En => "Open EXR files (B)…",
                        Language::Quenya => "Panya EXR files (B)…",
                    })
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
                    .button(match app.view().lang {
                        Language::Es => "Guardar Frame como PNG  (F)",
                        Language::En => "Save Frame as PNG  (F)",
                        Language::Quenya => "Marta Frame ve PNG  (F)",
                    })
                    .clicked()
                {
                    ui.ctx()
                        .send_viewport_cmd(egui::ViewportCommand::Screenshot);
                    ui.close_menu();
                }

                if ui
                    .button(match app.view().lang {
                        Language::Es => "Elegir carpeta de capturas…",
                        Language::En => "Set Screenshot Folder…",
                        Language::Quenya => "Cilta Screenshot Nómë…",
                    })
                    .clicked()
                {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        app.view_mut().screenshot_dir = Some(folder);
                    }
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Salir  (Esc)",
                        Language::En => "Quit  (Esc)",
                        Language::Quenya => "Vanya  (Esc)",
                    })
                    .clicked()
                {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            },
        );

        ui.menu_button(
            match app.view().lang {
                Language::Es => "Vista",
                Language::En => "View",
                Language::Quenya => "Cén",
            },
            |ui| {
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Ocultar/Mostrar Interfaz  (3)",
                        Language::En => "Toggle HUD  (3)",
                        Language::Quenya => "Halya/Tanë HUD  (3)",
                    })
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
                        match app.view().lang {
                            Language::Es => "Barra izquierda (datos del vídeo)",
                            Language::En => "Left panel (video data)",
                            Language::Quenya => "Parma left (video data)",
                        },
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
                        match app.view().lang {
                            Language::Es => "Barra derecha (controles y audio)",
                            Language::En => "Right panel (controls & audio)",
                            Language::Quenya => "Parma right (controls & audio)",
                        },
                    )
                    .changed()
                {
                    app.view_mut().show_right_panel = right;
                    ui.close_menu();
                }
                ui.separator();
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Restaurar Zoom  (R)",
                        Language::En => "Reset Zoom  (R)",
                        Language::Quenya => "En-panya Zoom  (R)",
                    })
                    .clicked()
                {
                    app.view_mut().zoom = 1.0;
                    app.view_mut().pan_u = 0.0;
                    app.view_mut().pan_v = 0.0;
                    ui.close_menu();
                }
                if ui.button("Zoom 50%  (5)").clicked() {
                    app.view_mut().zoom = 0.5;
                    ui.close_menu();
                }
                if ui.button("Zoom 100%  (6)").clicked() {
                    app.view_mut().zoom = 1.0;
                    ui.close_menu();
                }
                if ui.button("Zoom 200%  (7)").clicked() {
                    app.view_mut().zoom = 2.0;
                    ui.close_menu();
                }
                if ui.button("Zoom 400%  (8)").clicked() {
                    app.view_mut().zoom = 4.0;
                    ui.close_menu();
                }
                if ui.button("Zoom 800%  (9)").clicked() {
                    app.view_mut().zoom = 8.0;
                    ui.close_menu();
                }
            },
        );

        ui.menu_button(
            match app.view().lang {
                Language::Es => "Reproducción",
                Language::En => "Playback",
                Language::Quenya => "Lirë",
            },
            |ui| {
                let is_p = app.playback().is_playing;
                if ui
                    .button(match (is_p, app.view().lang) {
                        (true, Language::Es) => "Pausar  (Espacio)",
                        (true, Language::En) => "Pause  (Space)",
                        (true, Language::Quenya) => "Talta  (Espacio)",
                        (false, Language::Es) => "Reproducir  (Espacio)",
                        (false, Language::En) => "Play  (Space)",
                        (false, Language::Quenya) => "Lir  (Espacio)",
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
                    .button(match app.view().lang {
                        Language::Es => "Retroceder Frame (Izquierda / Left)",
                        Language::En => "Step Backward (Left)",
                        Language::Quenya => "Nánë Frame (Left)",
                    })
                    .clicked()
                {
                    app.do_step_bck(ui.ctx());
                    ui.close_menu();
                }
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Avanzar Frame (Derecha / Right)",
                        Language::En => "Step Forward (Right)",
                        Language::Quenya => "Pónë Frame (Right)",
                    })
                    .clicked()
                {
                    app.do_step_fwd(ui.ctx());
                    ui.close_menu();
                }
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Ir al inicio  (Home)",
                        Language::En => "Go to Start  (Home)",
                        Language::Quenya => "Mena Yessë  (Home)",
                    })
                    .clicked()
                {
                    app.do_seek(0.0, ui.ctx());
                    ui.close_menu();
                }
            },
        );

        ui.menu_button(
            match app.view().lang {
                Language::Es => "Opciones",
                Language::En => "Options",
                Language::Quenya => "Cilmë",
            },
            |ui| {
                if ui
                    .button(match app.view().lang {
                        Language::Es => "Intercambiar A y B  (S)",
                        Language::En => "Swap A and B  (S)",
                        Language::Quenya => "Quista A ar B  (S)",
                    })
                    .clicked()
                {
                    app.swap_videos(ui.ctx());
                    ui.close_menu();
                }

                ui.separator();

                // Canvas background colour
                ui.horizontal(|ui| {
                    ui.label(match app.view().lang {
                        Language::Es => "Color fondo:",
                        Language::En => "Canvas color:",
                        Language::Quenya => "Talan cala:",
                    });
                    let mut bg = app.view().canvas_bg_color;
                    if ui.color_edit_button_rgb(&mut bg).changed() {
                        app.view_mut().canvas_bg_color = bg;
                    }
                });

                ui.separator();
                ui.menu_button(
                    match app.view().lang {
                        Language::Es => "Idioma / Language",
                        Language::En => "Language / Idioma",
                        Language::Quenya => "Lambë",
                    },
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
                ui.menu_button(
                    match app.view().lang {
                        Language::Es => "Tema / Theme",
                        Language::En => "Theme / Tema",
                        Language::Quenya => "Cala",
                    },
                    |ui| {
                        let mut current_theme = app.view().theme;
                        let themes = [
                            (crate::types::Theme::Dark, "Dark"),
                            (crate::types::Theme::Light, "Light"),
                            (crate::types::Theme::Rust, "Rust"),
                            (crate::types::Theme::SolarizedDark, "Solarized Dark"),
                            (crate::types::Theme::SolarizedLight, "Solarized Light"),
                            (crate::types::Theme::Dracula, "Dracula"),
                            (crate::types::Theme::Gruvbox, "Gruvbox"),
                            (crate::types::Theme::Nord, "Nord"),
                            (crate::types::Theme::Monokai, "Monokai"),
                            (crate::types::Theme::OneDark, "One Dark"),
                            (crate::types::Theme::OneLight, "One Light"),
                            (crate::types::Theme::Catppuccin, "Catppuccin"),
                            (crate::types::Theme::TokyoNight, "Tokyo Night"),
                            (crate::types::Theme::NightOwl, "Night Owl"),
                            (crate::types::Theme::Ayc, "Ayc"),
                            (crate::types::Theme::MaterialDesign, "Material Design"),
                            (crate::types::Theme::Everforest, "Everforest"),
                            (crate::types::Theme::TomorrowNight, "Tomorrow Night"),
                            (crate::types::Theme::RosePine, "Rose Pine"),
                            (crate::types::Theme::SynthWave84, "SynthWave '84"),
                            (crate::types::Theme::Nordic, "Nordic"),
                            (crate::types::Theme::OceanicNext, "Oceanic Next"),
                            (crate::types::Theme::Palenight, "Palenight"),
                            (crate::types::Theme::Powerlevel10k, "Powerlevel10k"),
                            (crate::types::Theme::Snazzy, "Snazzy"),
                        ];
                        egui::ScrollArea::vertical()
                            .max_height(400.0)
                            .show(ui, |ui| {
                                for (theme_val, name) in themes {
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
                    },
                );
            },
        );

        ui.menu_button(
            match app.view().lang {
                Language::Es => "Emisión",
                Language::En => "Broadcast",
                Language::Quenya => "Sirë",
            },
            |ui| {
                let mut enabled = app.view().show_clean_feed_window;
                if ui
                    .checkbox(
                        &mut enabled,
                        match app.view().lang {
                            Language::Es => "Ventana de Salida  (OBS)",
                            Language::En => "Clean Feed Window  (OBS)",
                            Language::Quenya => "Vëa Cén  (OBS)",
                        },
                    )
                    .clicked()
                {
                    app.view_mut().show_clean_feed_window = enabled;
                    ui.close_menu();
                }
                ui.label(
                    RichText::new(match app.view().lang {
                        Language::Es => "Capturar ventana en OBS",
                        Language::En => "Capture window in OBS",
                        Language::Quenya => "Mapa vëa mi OBS",
                    })
                    .weak()
                    .size(10.0),
                );
                ui.separator();
                ui.label(
                    RichText::new(match app.view().lang {
                        Language::Es => "Zonas seguras",
                        Language::En => "Safe Zones",
                        Language::Quenya => "Safe zones",
                    })
                    .weak()
                    .size(10.0),
                );
                let mut safe_zone = app.view().safe_zone;
                if ui
                    .radio_value(
                        &mut safe_zone,
                        SafeZoneMode::None,
                        match app.view().lang {
                            Language::Es => "Desactivado",
                            Language::En => "Off",
                            Language::Quenya => "Off",
                        },
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
                        match app.view().lang {
                            Language::Es => "Móvil: Redes Sociales (9:16)",
                            Language::En => "Mobile: Social (9:16)",
                            Language::Quenya => "Social (9:16)",
                        },
                    )
                    .clicked()
                {
                    ui.close_menu();
                }
                app.view_mut().safe_zone = safe_zone;
            },
        );

        // ── Separator before inline controls ───────────────────────────────
        ui.separator();

        // ── Inline compact controls ─────────────────────────────────────────

        // File open buttons
        let has_a = app.decoder_a_path().is_some();
        let has_b = app.decoder_b_path().is_some();

        let a_label =
            app.decoder_a_path()
                .map(short_name)
                .unwrap_or_else(|| match app.view().lang {
                    Language::Es => "A…".to_owned(),
                    Language::En => "A…".to_owned(),
                    Language::Quenya => "A…".to_owned(),
                });
        let b_label =
            app.decoder_b_path()
                .map(short_name)
                .unwrap_or_else(|| match app.view().lang {
                    Language::Es => "B…".to_owned(),
                    Language::En => "B…".to_owned(),
                    Language::Quenya => "B…".to_owned(),
                });

        let a_tooltip = app
            .decoder_a_path()
            .map(|p| p.to_owned())
            .unwrap_or_else(|| match app.view().lang {
                Language::Es => "Abrir Vídeo A".to_owned(),
                Language::En => "Open Video A".to_owned(),
                Language::Quenya => "Panya A".to_owned(),
            });
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
            .unwrap_or_else(|| match app.view().lang {
                Language::Es => "Abrir Vídeo B".to_owned(),
                Language::En => "Open Video B".to_owned(),
                Language::Quenya => "Panya B".to_owned(),
            });
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
        let is_playing = app.playback().is_playing;
        if ui
            .button(RichText::new("|<").size(16.0))
            .on_hover_text(match app.view().lang {
                Language::Es => "Inicio",
                Language::En => "Start",
                Language::Quenya => "Yessë",
            })
            .clicked()
        {
            app.do_seek(0.0, ui.ctx());
        }
        if ui
            .button(RichText::new("<<").size(16.0))
            .on_hover_text(match app.view().lang {
                Language::Es => "Retroceder (Izquierda)",
                Language::En => "Step back (Left)",
                Language::Quenya => "Nánë (Left)",
            })
            .clicked()
        {
            app.do_step_bck(ui.ctx());
        }
        if ui
            .button(RichText::new(if is_playing { "||" } else { ">" }).size(16.0))
            .on_hover_text(match app.view().lang {
                Language::Es => "Reproducir/Pausar (Espacio)",
                Language::En => "Play/Pause (Space)",
                Language::Quenya => "Lir/Talta",
            })
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
            .on_hover_text(match app.view().lang {
                Language::Es => "Avanzar (Derecha)",
                Language::En => "Step fwd (Right)",
                Language::Quenya => "Pónë (Right)",
            })
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
    let c_mode = app.view().mode;
    let split = app.view().split_pos;
    let is_a = c_mode == CompareMode::SplitScreen && split > 0.95;
    let is_b = c_mode == CompareMode::SplitScreen && split < 0.05;
    let is_split = c_mode == CompareMode::SplitScreen && !is_a && !is_b;
    let active = Color32::from_rgb(80, 130, 200);

    ui.vertical(|ui| {
        ui.set_min_width(90.0);
        // Display mode buttons (stacked for narrow sidebar)
        if ui
            .add(
                egui::Button::new(match app.view().lang {
                    Language::Es => "Solo A",
                    Language::En => "A Only",
                    Language::Quenya => "Erya A",
                })
                .fill(if is_a { active } else { Color32::TRANSPARENT }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SplitScreen;
            app.view_mut().split_pos = 1.0;
        }
        if ui
            .add(
                egui::Button::new(match app.view().lang {
                    Language::Es => "Solo B",
                    Language::En => "B Only",
                    Language::Quenya => "Erya B",
                })
                .fill(if is_b { active } else { Color32::TRANSPARENT }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::SplitScreen;
            app.view_mut().split_pos = 0.0;
        }
        if ui
            .add(
                egui::Button::new(match app.view().lang {
                    Language::Es => "Cortina",
                    Language::En => "Split",
                    Language::Quenya => "Hyanda",
                })
                .fill(if is_split {
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
                egui::Button::new(match app.view().lang {
                    Language::Es => "Diferencia",
                    Language::En => "Diff",
                    Language::Quenya => "Winya",
                })
                .fill(if c_mode == CompareMode::AbsDiff {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::AbsDiff;
        }
        if ui
            .add(
                egui::Button::new(match app.view().lang {
                    Language::Es => "Mapa Calor",
                    Language::En => "Heatmap",
                    Language::Quenya => "Úrë",
                })
                .fill(if c_mode == CompareMode::Heatmap {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
            )
            .clicked()
        {
            app.view_mut().mode = CompareMode::Heatmap;
        }
        if ui
            .add(
                egui::Button::new(match app.view().lang {
                    Language::Es => "Lado a Lado",
                    Language::En => "Side×Side",
                    Language::Quenya => "Ara",
                })
                .fill(if c_mode == CompareMode::SideBySide {
                    active
                } else {
                    Color32::TRANSPARENT
                }),
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
                        match app.view().lang {
                            Language::Es => "Cortina H",
                            Language::En => "Split H",
                            Language::Quenya => "Hya H",
                        }
                    } else {
                        match app.view().lang {
                            Language::Es => "Cortina V",
                            Language::En => "Split V",
                            Language::Quenya => "Hya V",
                        }
                    })
                    .clicked()
                {
                    app.view_mut().split_horizontal = !app.view().split_horizontal;
                }
                ui.label(if is_h { "Cort. (Y):" } else { "Cort. (X):" });
                let mut sp = app.view().split_pos;
                if ui
                    .add(egui::Slider::new(&mut sp, 0.0..=1.0).fixed_decimals(2))
                    .changed()
                {
                    app.view_mut().split_pos = sp;
                }
            }
            CompareMode::Heatmap | CompareMode::AbsDiff => {
                ui.label("Amp:");
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
                        .selected_text(match d_mode {
                            DiffMode::LegacyAbs => "Legacy",
                            DiffMode::AbsLinear => "Linear",
                            DiffMode::AbsSqrt => "Sqrt",
                            DiffMode::SignedDiverging => "Signed",
                            DiffMode::None => "—",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut d_mode, DiffMode::LegacyAbs, "Legacy");
                            ui.selectable_value(&mut d_mode, DiffMode::AbsLinear, "Linear");
                            ui.selectable_value(&mut d_mode, DiffMode::AbsSqrt, "Sqrt");
                            ui.selectable_value(&mut d_mode, DiffMode::SignedDiverging, "Signed");
                        });
                    if d_mode != app.view().diff_mode {
                        app.view_mut().diff_mode = d_mode;
                    }
                }
            }
            CompareMode::SideBySide => {
                ui.label("Amp:");
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
                        DiffMode::None => match app.view().lang {
                            Language::Es => "Sin Filtro".to_string(),
                            Language::En => "No Filter".to_string(),
                            Language::Quenya => "Munca".to_string(),
                        },
                        _ => format!("{:?}", d_mode),
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut d_mode, DiffMode::LegacyAbs, "Legacy");
                        ui.selectable_value(&mut d_mode, DiffMode::AbsLinear, "Linear");
                        ui.selectable_value(&mut d_mode, DiffMode::AbsSqrt, "Sqrt");
                        ui.selectable_value(&mut d_mode, DiffMode::SignedDiverging, "Signed");
                        ui.selectable_value(
                            &mut d_mode,
                            DiffMode::None,
                            match app.view().lang {
                                Language::Es => "Sin Filtro",
                                Language::En => "No Filter",
                                Language::Quenya => "Munca",
                            },
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
            if ui.button(format!("Zoom {:.1}×", zoom)).clicked() {
                app.view_mut().zoom = 1.0;
                app.view_mut().pan_u = 0.0;
                app.view_mut().pan_v = 0.0;
            }
        }
    });
}

pub fn show_audio_panel(ui: &mut Ui, app: &mut DiffPlayerApp) {
    ui.vertical_centered(|ui| {
        show_mode_toolbar(ui, app);
        ui.separator();
        ui.heading("Audio");
        ui.add_space(6.0);

        ui.label(
            RichText::new("A")
                .color(Color32::from_rgb(100, 200, 120))
                .strong(),
        );
        let mut mute_a = app.view().mute_a;
        let resp_a = ui.button(if mute_a { "Unmute" } else { "Mute" });
        if resp_a.clicked() {
            mute_a = !mute_a;
            app.view_mut().mute_a = mute_a;
            ui.ctx().request_repaint();
        }
        resp_a.on_hover_text(if mute_a {
            "Canal A silenciado (clic para activar)"
        } else {
            "Canal A con sonido (clic para silenciar)"
        });
        ui.add_space(5.0);
        let mut vol_a = app.view().vol_a;
        if ui
            .add(
                egui::Slider::new(&mut vol_a, 0.0..=2.0)
                    .vertical()
                    .show_value(false),
            )
            .changed()
        {
            app.view_mut().vol_a = vol_a;
        }
        let level_a = app.view().audio_level_a.clamp(0.0, 1.0);
        let color_a = if level_a < 0.5 {
            Color32::from_rgb(80, 200, 100)
        } else if level_a < 0.85 {
            Color32::from_rgb(220, 180, 0)
        } else {
            Color32::from_rgb(220, 60, 60)
        };
        ui.add(
            egui::ProgressBar::new(level_a)
                .fill(color_a)
                .desired_width(40.0),
        );

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        ui.label(
            RichText::new("B")
                .color(Color32::from_rgb(100, 160, 240))
                .strong(),
        );
        let mut mute_b = app.view().mute_b;
        let resp_b = ui.button(if mute_b { "Unmute" } else { "Mute" });
        if resp_b.clicked() {
            mute_b = !mute_b;
            app.view_mut().mute_b = mute_b;
            ui.ctx().request_repaint();
        }
        resp_b.on_hover_text(if mute_b {
            "Canal B silenciado (clic para activar)"
        } else {
            "Canal B con sonido (clic para silenciar)"
        });
        ui.add_space(5.0);
        let mut vol_b = app.view().vol_b;
        if ui
            .add(
                egui::Slider::new(&mut vol_b, 0.0..=2.0)
                    .vertical()
                    .show_value(false),
            )
            .changed()
        {
            app.view_mut().vol_b = vol_b;
        }
        let level_b = app.view().audio_level_b.clamp(0.0, 1.0);
        let color_b = if level_b < 0.5 {
            Color32::from_rgb(80, 160, 220)
        } else if level_b < 0.85 {
            Color32::from_rgb(220, 180, 0)
        } else {
            Color32::from_rgb(220, 60, 60)
        };
        ui.add(
            egui::ProgressBar::new(level_b)
                .fill(color_b)
                .desired_width(40.0),
        );
    });
}
