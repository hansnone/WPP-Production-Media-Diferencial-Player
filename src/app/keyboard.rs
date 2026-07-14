use super::{DiffPlayerApp, PendingKeyAction, PendingTransportAction};

impl DiffPlayerApp {
    pub(super) fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        if ctx.wants_keyboard_input() {
            return;
        }
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.pending_transport_action = PendingTransportAction::Toggle;
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                self.pending_key_action = PendingKeyAction::StepFwd;
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.pending_key_action = PendingKeyAction::StepBck;
            }
            if i.key_pressed(egui::Key::Home) {
                self.pending_key_action = PendingKeyAction::Seek(0.0);
            }
            if i.key_pressed(egui::Key::Y) {
                self.pending_key_action = PendingKeyAction::CycleMode;
            }
            if i.key_pressed(egui::Key::L) {
                self.pending_key_action = PendingKeyAction::SideBySide;
            }
            if i.key_pressed(egui::Key::Num1) {
                self.pending_key_action = PendingKeyAction::SplitPos0;
            }
            if i.key_pressed(egui::Key::Num2) {
                self.pending_key_action = PendingKeyAction::SplitPos1;
            }
            if i.key_pressed(egui::Key::Num3) {
                self.pending_key_action = PendingKeyAction::ToggleHud;
            }
            if i.key_pressed(egui::Key::Num4) {
                self.pending_key_action = PendingKeyAction::Zoom(1.0);
            }
            if i.key_pressed(egui::Key::Num5) {
                self.pending_key_action = PendingKeyAction::Zoom(0.5);
            }
            if i.key_pressed(egui::Key::Num6) {
                self.pending_key_action = PendingKeyAction::Zoom(1.0);
            }
            if i.key_pressed(egui::Key::Num7) {
                self.pending_key_action = PendingKeyAction::Zoom(2.0);
            }
            if i.key_pressed(egui::Key::Num8) {
                self.pending_key_action = PendingKeyAction::Zoom(4.0);
            }
            if i.key_pressed(egui::Key::Num9) {
                self.pending_key_action = PendingKeyAction::Zoom(8.0);
            }
            if i.key_pressed(egui::Key::F) {
                log::trace!("Key 'F': xcap OS-native capture");
                let dir_for_thread = self.view.screenshot_dir.clone();

                std::thread::spawn(move || {
                    let mut success = false;
                    log::trace!("xcap: scanning OS windows");
                    if let Ok(windows) = xcap::Window::all() {
                        for window in windows {
                            if let Ok(title) = window.title() {
                                if title.contains("Production Media")
                                    || title.contains("Diferencial")
                                {
                                    log::trace!("xcap: window -> {}", title);
                                    if let Ok(img_buf) = window.capture_image() {
                                        if let Some(dir) = dir_for_thread.as_ref() {
                                            let timestamp =
                                                chrono::Local::now().format("%Y%m%d_%H%M%S");
                                            let filename = format!("WPP_QC_{timestamp}.png");
                                            let path = dir.join(filename);
                                            log::trace!("xcap: writing PNG to {:?}", path);

                                            if let Err(e) = img_buf.save(&path) {
                                                log::error!("xcap disk write error: {}", e);
                                            } else {
                                                log::trace!("xcap: screenshot saved");
                                                success = true;
                                            }
                                        }
                                    } else {
                                        log::error!("xcap failed to read window buffer");
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    if !success {
                        log::error!("xcap: target WPP window not found or capture failed");
                    }
                });
            }
            if i.key_pressed(egui::Key::R) {
                self.pending_key_action = PendingKeyAction::ResetZoomPan;
            }
            if i.key_pressed(egui::Key::S) {
                self.pending_key_action = PendingKeyAction::SwapVideos;
            }

            let now = i.time;
            let repeat_delay = 0.25;
            let repeat_interval = 0.05;

            if i.key_down(egui::Key::ArrowRight) {
                if i.key_pressed(egui::Key::ArrowRight)
                    || (now - self.last_step_time) > repeat_interval
                {
                    let delay_ok = (now - self.last_step_time) > repeat_delay;
                    if i.key_pressed(egui::Key::ArrowRight) || delay_ok {
                        self.pending_key_action = PendingKeyAction::StepFwd;
                        self.last_step_time = now;
                    }
                }
            } else if i.key_down(egui::Key::ArrowLeft) {
                if i.key_pressed(egui::Key::ArrowLeft)
                    || (now - self.last_step_time) > repeat_interval
                {
                    let delay_ok = (now - self.last_step_time) > repeat_delay;
                    if i.key_pressed(egui::Key::ArrowLeft) || delay_ok {
                        self.pending_key_action = PendingKeyAction::StepBck;
                        self.last_step_time = now;
                    }
                }
            } else {
                self.last_step_time = 0.0;
            }
        });
    }
}
