// ui/controls.rs — Top toolbar: file open, playback, mode, sliders

use egui::{Color32, RichText, Ui};

use crate::app::DiffPlayerApp;
use crate::types::{CompareMode, Language};
use crate::ui::theme::apply_theme;

pub fn show_menu_bar(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let _is_es = app.view().lang == Language::Es;

    egui::menu::bar(ui, |ui| {
        ui.menu_button(match app.view().lang { Language::Es => "Archivo", Language::En => "File", Language::Quenya => "Parma" }, |ui| {
            if ui.button(match app.view().lang { Language::Es => "Abrir VÍDEO A...", Language::En => "Open VIDEO A...", Language::Quenya => "Panya VÍDEO A..." }).clicked() {
                app.open_video_a();
                ui.close_menu();
            }
            if ui.button(match app.view().lang { Language::Es => "Abrir VÍDEO B...", Language::En => "Open VIDEO B...", Language::Quenya => "Panya VÍDEO B..." }).clicked() {
                app.open_video_b();
                ui.close_menu();
            }
            ui.separator();
            if ui.button(match app.view().lang { Language::Es => "Guardar Frame como PNG (F)", Language::En => "Save Frame as PNG (F)", Language::Quenya => "Marta Frame ve PNG (F)" }).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Screenshot);
                ui.close_menu();
            }
            if ui.button(match app.view().lang { Language::Es => "Elegir carpeta de capturas...", Language::En => "Set Screenshot Folder...", Language::Quenya => "Cilta Screenshot Nómë..." }).clicked() {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    app.view_mut().screenshot_dir = Some(folder);
                }
                ui.close_menu();
            }
            ui.separator();
            if ui.button(match app.view().lang { Language::Es => "Salir (Esc)", Language::En => "Quit (Esc)", Language::Quenya => "Vanya (Esc)" }).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        ui.menu_button(match app.view().lang { Language::Es => "Vista", Language::En => "View", Language::Quenya => "Cén" }, |ui| {
            if ui.button(match app.view().lang { Language::Es => "Ocultar/Mostrar Interfaz (3)", Language::En => "Toggle HUD (3)", Language::Quenya => "Halya/Tanë HUD (3)" }).clicked() {
                let current = app.view().show_hud;
                app.view_mut().show_hud = !current;
                ui.close_menu();
            }
            ui.separator();
            if ui.button(match app.view().lang { Language::Es => "Restaurar Zoom (R)", Language::En => "Reset Zoom (R)", Language::Quenya => "En-panya Zoom (R)" }).clicked() {
                app.view_mut().zoom = 1.0;
                app.view_mut().pan_u = 0.0;
                app.view_mut().pan_v = 0.0;
            }
            if ui.button("Zoom 50% (5)").clicked() { app.view_mut().zoom = 0.5; }
            if ui.button("Zoom 100% (6)").clicked() { app.view_mut().zoom = 1.0; }
            if ui.button("Zoom 200% (7)").clicked() { app.view_mut().zoom = 2.0; }
            if ui.button("Zoom 400% (8)").clicked() { app.view_mut().zoom = 4.0; }
            if ui.button("Zoom 800% (9)").clicked() { app.view_mut().zoom = 8.0; }
        });

        ui.menu_button(match app.view().lang { Language::Es => "Reproducción", Language::En => "Playback", Language::Quenya => "Lirë" }, |ui| {
            let is_p = app.playback().is_playing;
            let play_text = match (is_p, app.view().lang) {
                (true, Language::Es) => "Pausar (Espacio)",
                (true, Language::En) => "Pause (Space)",
                (true, Language::Quenya) => "Talta (Espacio)",
                (false, Language::Es) => "Reproducir (Espacio)",
                (false, Language::En) => "Play (Space)",
                (false, Language::Quenya) => "Lir (Espacio)",
            };
            if ui.button(play_text).clicked() {
                if is_p { app.do_pause(); } else { app.do_play(); }
                ui.close_menu();
            }
            if ui.button(match app.view().lang { Language::Es => "Retroceder Frame (←)", Language::En => "Step Backward (←)", Language::Quenya => "Nánë Frame (←)" }).clicked() {
                app.do_step_bck();
            }
            if ui.button(match app.view().lang { Language::Es => "Avanzar Frame (→)", Language::En => "Step Forward (→)", Language::Quenya => "Pónë Frame (→)" }).clicked() {
                app.do_step_fwd();
            }
            if ui.button(match app.view().lang { Language::Es => "Ir al inicio (Home)", Language::En => "Go to Start (Home)", Language::Quenya => "Mena Yessë (Home)" }).clicked() {
                app.do_seek(0.0);
            }
        });

        ui.menu_button(match app.view().lang { Language::Es => "Opciones", Language::En => "Options", Language::Quenya => "Cilmë" }, |ui| {
            if ui.button(match app.view().lang { Language::Es => "Intercambiar A y B (S)", Language::En => "Swap A and B (S)", Language::Quenya => "Quista A ar B (S)" }).clicked() {
                app.swap_videos();
                ui.close_menu();
            }
            ui.separator();
            ui.menu_button(match app.view().lang { Language::Es => "Idioma / Language", Language::En => "Language / Idioma", Language::Quenya => "Lambë" }, |ui| {
                if ui.radio_value(&mut app.view_mut().lang, Language::En, "English").clicked() { ui.close_menu(); }
                if ui.radio_value(&mut app.view_mut().lang, Language::Es, "Español").clicked() { ui.close_menu(); }
                if ui.radio_value(&mut app.view_mut().lang, Language::Quenya, "Quenya (Elvish)").clicked() { ui.close_menu(); }
            });
            ui.menu_button(match app.view().lang { Language::Es => "Tema / Theme", Language::En => "Theme / Tema", Language::Quenya => "Cala" }, |ui| {
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
                egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                    for (theme_val, name) in themes {
                        if ui.radio_value(&mut current_theme, theme_val, name).clicked() {
                            app.view_mut().theme = theme_val;
                            apply_theme(ui.ctx(), theme_val);
                            ui.close_menu();
                        }
                    }
                });
            });
        });

        ui.menu_button(match app.view().lang { Language::Es => "Emisión", Language::En => "Broadcast", Language::Quenya => "Sirë" }, |ui| {
            let mut enabled = app.view().show_clean_feed_window;
            if ui.checkbox(&mut enabled, match app.view().lang { Language::Es => "Ventana de Salida (OBS)", Language::En => "Clean Feed Window (OBS)", Language::Quenya => "Vëa Cén (OBS)" }).clicked() {
                app.view_mut().show_clean_feed_window = enabled;
                ui.close_menu();
            }
            ui.label(RichText::new(match app.view().lang { Language::Es => "Capturar ventana en OBS", Language::En => "Capture window in OBS", Language::Quenya => "Mapa vëa mi OBS" }).weak().size(10.0));
        });
    });
}

pub fn show_toolbar(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let _is_es = app.view().lang == Language::Es;

    ui.horizontal_wrapped(|ui| {
        ui.set_height(42.0);

        // ── File open buttons ───────────────────────────────────────────
        let has_a = app.decoder_a_path().is_some();
        let has_b = app.decoder_b_path().is_some();

        let a_label = if let Some(p) = app.decoder_a_path() {
            short_name(p)
        } else {
            match app.view().lang { Language::Es => "Abrir A…".to_owned(), Language::En => "Open A…".to_owned(), Language::Quenya => "Panya A…".to_owned() }
        };
        let b_label = if let Some(p) = app.decoder_b_path() {
            short_name(p)
        } else {
            match app.view().lang { Language::Es => "Abrir B…".to_owned(), Language::En => "Open B…".to_owned(), Language::Quenya => "Panya B…".to_owned() }
        };

        let btn_a = egui::Button::new(
            RichText::new(format!("▶ A: {a_label}"))
                .color(if has_a { Color32::from_rgb(100, 200, 120) } else { Color32::LIGHT_GRAY }),
        );
        let btn_b = egui::Button::new(
            RichText::new(format!("▶ B: {b_label}"))
                .color(if has_b { Color32::from_rgb(100, 160, 240) } else { Color32::LIGHT_GRAY }),
        );

        if ui.add(btn_a).clicked() {
            app.open_video_a();
        }
        if ui.add(btn_b).clicked() {
            app.open_video_b();
        }

        ui.separator();

        // ── Playback controls ───────────────────────────────────────────
        let is_playing = app.playback().is_playing;

        if ui.button(RichText::new("⏮").size(18.0)).on_hover_text(match app.view().lang { Language::Es => "Ir al inicio", Language::En => "Go to start", Language::Quenya => "Mena yessë" }).clicked() {
            app.do_seek(0.0);
        }
        if ui
            .button(RichText::new("⏪").size(18.0))
            .on_hover_text(match app.view().lang { Language::Es => "Retroceder 1 frame (←)", Language::En => "Step back (←)", Language::Quenya => "Nánë 1 frame (←)" })
            .clicked()
        {
            app.do_step_bck();
        }

        let play_icon = if is_playing { "⏸" } else { "▶" };
        let play_tip  = match (is_playing, app.view().lang) {
            (true, Language::Es) => "Pausar (Espacio)",
            (true, Language::En) => "Pause (Space)",
            (true, Language::Quenya) => "Talta (Espacio)",
            (false, Language::Es) => "Reproducir (Espacio)",
            (false, Language::En) => "Play (Space)",
            (false, Language::Quenya) => "Lir (Espacio)",
        };
        if ui.button(RichText::new(play_icon).size(18.0)).on_hover_text(play_tip).clicked() {
            if is_playing { app.do_pause(); } else { app.do_play(); }
        }

        if ui
            .button(RichText::new("⏩").size(18.0))
            .on_hover_text(match app.view().lang { Language::Es => "Avanzar 1 frame (→)", Language::En => "Step forward (→)", Language::Quenya => "Pónë 1 frame (→)" })
            .clicked()
        {
            app.do_step_fwd();
        }

        ui.separator();

        // ── Display mode ────────────────────────────────────────────────
        ui.label(match app.view().lang { Language::Es => "Vista:", Language::En => "View:", Language::Quenya => "Cén:" });
        
        let c_mode = app.view().mode;
        let split = app.view().split_pos;
        let is_a = c_mode == CompareMode::SplitScreen && split > 0.95;
        let is_b = c_mode == CompareMode::SplitScreen && split < 0.05;
        let is_split = c_mode == CompareMode::SplitScreen && !is_a && !is_b;

        let active_color = Color32::from_rgb(80, 130, 200);

        if ui.add(egui::Button::new(match app.view().lang { Language::Es => "Solo A (1)", Language::En => "A Only (1)", Language::Quenya => "Erya A (1)" }).fill(if is_a { active_color } else { Color32::TRANSPARENT })).clicked() {
            app.view_mut().mode = CompareMode::SplitScreen;
            app.view_mut().split_pos = 1.0;
        }
        if ui.add(egui::Button::new(match app.view().lang { Language::Es => "Solo B (2)", Language::En => "B Only (2)", Language::Quenya => "Erya B (2)" }).fill(if is_b { active_color } else { Color32::TRANSPARENT })).clicked() {
            app.view_mut().mode = CompareMode::SplitScreen;
            app.view_mut().split_pos = 0.0;
        }
        if ui.add(egui::Button::new(match app.view().lang { Language::Es => "Cortina (Y)", Language::En => "Split (Y)", Language::Quenya => "Hyanda (Y)" }).fill(if is_split { active_color } else { Color32::TRANSPARENT })).clicked() {
            app.view_mut().mode = CompareMode::SplitScreen;
            if is_a || is_b { app.view_mut().split_pos = 0.5; }
        }
        if ui.add(egui::Button::new(match app.view().lang { Language::Es => "Diferencia (Y)", Language::En => "Diff (Y)", Language::Quenya => "Winya (Y)" }).fill(if c_mode == CompareMode::AbsDiff { active_color } else { Color32::TRANSPARENT })).clicked() {
            app.view_mut().mode = CompareMode::AbsDiff;
        }
        if ui.add(egui::Button::new(match app.view().lang { Language::Es => "Mapa Calor (Y)", Language::En => "Heatmap (Y)", Language::Quenya => "Úrë (Y)" }).fill(if c_mode == CompareMode::Heatmap { active_color } else { Color32::TRANSPARENT })).clicked() {
            app.view_mut().mode = CompareMode::Heatmap;
        }
        if ui.add(egui::Button::new(match app.view().lang { Language::Es => "Lado a Lado (L)", Language::En => "Side-by-Side (L)", Language::Quenya => "Ara (L)" }).fill(if c_mode == CompareMode::SideBySide { active_color } else { Color32::TRANSPARENT })).clicked() {
            app.view_mut().mode = CompareMode::SideBySide;
        }

        ui.separator();

        // ── Mode-specific sliders ───────────────────────────────────────
        match app.view().mode {
            CompareMode::SplitScreen => {
                ui.label(match app.view().lang { Language::Es => "Ajuste:", Language::En => "Split:", Language::Quenya => "Hya:" });
                let mut sp = app.view().split_pos;
                if ui.add(egui::Slider::new(&mut sp, 0.0f32..=1.0).step_by(0.01)).changed() {
                    app.view_mut().split_pos = sp;
                }
            }
            CompareMode::Heatmap => {
                ui.label(match app.view().lang { Language::Es => "Amplificador:", Language::En => "Amplifier:", Language::Quenya => "Púta:" });
                let mut amp = app.view().amplifier;
                if ui
                    .add(
                        egui::Slider::new(&mut amp, 1.0f32..=50.0)
                            .step_by(0.5)
                            .suffix("×"),
                    )
                    .changed()
                {
                    app.view_mut().amplifier = amp;
                }
            }
            CompareMode::AbsDiff => {
                ui.label(match app.view().lang { Language::Es => "Medición Diferencia:", Language::En => "Difference Measure:", Language::Quenya => "Winya Nótë:" });
                ui.horizontal(|ui| {
                    let mut d_mode = app.view().diff_mode;
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::LegacyAbs, "Legacy").changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::AbsLinear, "Linear").changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::AbsSqrt, "Sqrt").changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::SignedDiverging, "Signed").changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                });

                ui.label(match app.view().lang { Language::Es => "Amplificador:", Language::En => "Amplifier:", Language::Quenya => "Púta:" });
                let mut amp = app.view().amplifier;
                if ui.add(egui::Slider::new(&mut amp, 1.0f32..=50.0).step_by(0.5).suffix("×")).changed() {
                    app.view_mut().amplifier = amp;
                }
            }
            CompareMode::SideBySide => {
                ui.label(match app.view().lang { Language::Es => "Modo Lado a Lado (Diferencial)", Language::En => "Side-by-Side Mode (Diff)", Language::Quenya => "Ara (Winya)" });
                ui.horizontal_wrapped(|ui| {
                    let mut d_mode = app.view().diff_mode;
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::LegacyAbs, "Legacy").changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::AbsLinear, "Linear").changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::AbsSqrt, "Sqrt").changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::SignedDiverging, "Signed").changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                    if ui.radio_value(&mut d_mode, crate::types::DiffMode::None, match app.view().lang { Language::Es => "Sin Filtro", Language::En => "No Filter", Language::Quenya => "Munca U-winya" }).changed() {
                        app.view_mut().diff_mode = d_mode;
                    }
                });

                ui.label(match app.view().lang { Language::Es => "Amplificador Diferencial:", Language::En => "Differential Amplifier:", Language::Quenya => "Winya Púta:" });
                let mut amp = app.view().amplifier;
                if ui.add(egui::Slider::new(&mut amp, 1.0f32..=50.0).step_by(0.5).suffix("×")).changed() {
                    app.view_mut().amplifier = amp;
                }
            }
        }

        ui.separator();

        // ── Zoom reset ──────────────────────────────────────────────────
        let zoom = app.view().zoom;
        if zoom != 1.0 {
            let rs_text = match app.view().lang { Language::Es => "Reiniciar", Language::En => "Reset", Language::Quenya => "En-panya" };
            let rs_tip = match app.view().lang { Language::Es => "Doble clic en imagen para reiniciar", Language::En => "Double-click canvas to reset", Language::Quenya => "Ata-clickë mí cén en-panya" };
            if ui
                .button(format!("🔍 {:.1}× — {}", zoom, rs_text))
                .on_hover_text(rs_tip)
                .clicked()
            {
                app.view_mut().zoom  = 1.0;
                app.view_mut().pan_u = 0.0;
                app.view_mut().pan_v = 0.0;
            }
        }

        ui.separator();

        ui.label(match app.view().lang { Language::Es => "Fondo:", Language::En => "Canvas:", Language::Quenya => "Talan:" });
        let mut bg_col = app.view().canvas_bg_color;
        if ui.color_edit_button_rgb(&mut bg_col).changed() {
            app.view_mut().canvas_bg_color = bg_col;
        } else {
            ui.label(RichText::new("🔍 1.0×").weak());
        }
    });
}

fn short_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_owned())
}

pub fn show_audio_panel(ui: &mut Ui, app: &mut DiffPlayerApp) {
    let _is_es = app.view().lang == Language::Es;

    ui.vertical_centered(|ui| {
        ui.heading("Audio");
        ui.separator();
        
        ui.add_space(10.0);
        
        ui.label(RichText::new("A").color(Color32::from_rgb(100, 200, 120)).strong());
        
        let mut mute_a = app.view().mute_a;
        if ui.button(if mute_a { "🔇" } else { "🔊" }).clicked() {
            mute_a = !mute_a;
            app.view_mut().mute_a = mute_a;
        }
        
        ui.add_space(5.0);
        
        let mut vol_a = app.view().vol_a;
        if ui.add(egui::Slider::new(&mut vol_a, 0.0..=2.0).vertical().show_value(false)).changed() {
            app.view_mut().vol_a = vol_a;
        }
        
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);
        
        ui.label(RichText::new("B").color(Color32::from_rgb(100, 160, 240)).strong());
        
        let mut mute_b = app.view().mute_b;
        if ui.button(if mute_b { "🔇" } else { "🔊" }).clicked() {
            mute_b = !mute_b;
            app.view_mut().mute_b = mute_b;
        }
        
        ui.add_space(5.0);
        
        let mut vol_b = app.view().vol_b;
        if ui.add(egui::Slider::new(&mut vol_b, 0.0..=2.0).vertical().show_value(false)).changed() {
            app.view_mut().vol_b = vol_b;
        }
    });
}
