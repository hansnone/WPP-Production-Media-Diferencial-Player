use super::DiffPlayerApp;

impl DiffPlayerApp {
    pub(super) fn show_main_layout(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        is_first_frame: bool,
    ) {
        self.show_hud_panels(ctx, is_first_frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            super::canvas::show_canvas(ui, self, frame);
        });
    }

    fn show_hud_panels(&mut self, ctx: &egui::Context, is_first_frame: bool) {
        if !self.view.show_hud || is_first_frame {
            return;
        }
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            crate::ui::controls::show_menu_bar(ui, self);
        });
        if self.view.show_left_panel {
            egui::SidePanel::left("info_panel")
                .resizable(true)
                .default_width(260.0)
                .min_width(200.0)
                .max_width(340.0)
                .show(ctx, |ui| {
                    crate::ui::info_panel::show(ui, self);
                });
        }
        if self.view.show_right_panel {
            egui::SidePanel::right("audio_panel")
                .resizable(true)
                .default_width(110.0)
                .min_width(90.0)
                .max_width(220.0)
                .show(ctx, |ui| {
                    crate::ui::controls::show_audio_panel(ui, self);
                });
        }
        egui::TopBottomPanel::bottom("timeline")
            .exact_height(crate::ui::timeline::TIMELINE_PANEL_HEIGHT)
            .show(ctx, |ui| {
                crate::ui::timeline::show(ui, self);
            });
    }
}
