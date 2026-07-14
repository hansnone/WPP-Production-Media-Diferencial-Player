use crate::app::DiffPlayerApp;
use egui::{Color32, FontId, Pos2, Rect, Rounding, Stroke, Vec2};

// ─── Peak Hold State for True Peak ──────────────────────────────────────────
use std::sync::Mutex;

#[derive(Clone, Copy)]
struct TpState {
    peak: f32, // Peak hold for True Peak marker
    age: f32,
    clip: bool,
    ppm_level: f32, // Fast PPM level for the left bar
}

impl Default for TpState {
    fn default() -> Self {
        Self {
            peak: -60.0,
            age: 0.0,
            clip: false,
            ppm_level: 0.0,
        }
    }
}

impl TpState {
    fn update(&mut self, tp: f32, dt: f32) {
        const HOLD_SEC: f32 = 2.0;
        const FALL_RATE: f32 = 20.0; // dB/s

        // tp is in linear amplitude! Convert to dB
        let tp_db = if tp <= 0.00001 {
            -60.0
        } else {
            20.0 * tp.log10()
        };

        // 1) Peak hold update
        if tp_db > self.peak {
            self.peak = tp_db;
            self.age = 0.0;
        } else {
            self.age += dt;
            if self.age > HOLD_SEC {
                self.peak -= FALL_RATE * dt;
            }
        }
        self.peak = self.peak.max(-60.0);

        // 2) PPM Ballistics for the fast bar (Linear Domain)
        const TAU_ATTACK: f32 = 0.006; // Fast attack ~10ms
        const TAU_RELEASE: f32 = 1.0; // Slow release ~8.6 dB/s
        let tau = if tp > self.ppm_level {
            TAU_ATTACK
        } else {
            TAU_RELEASE
        };
        let alpha = (-dt / tau).exp();
        self.ppm_level = tp + (self.ppm_level - tp) * alpha;

        // Clip detection
        if tp_db >= -1.0 {
            self.clip = true;
        }
    }
}

static TP_STATE: Mutex<[[TpState; 2]; 2]> = Mutex::new(
    [[TpState {
        peak: -60.0,
        age: 0.0,
        clip: false,
        ppm_level: 0.0,
    }; 2]; 2],
);

pub fn reset_meter_state(ch_idx: usize) {
    if let Ok(mut state) = TP_STATE.lock() {
        if ch_idx < 2 {
            state[ch_idx][0] = TpState::default();
            state[ch_idx][1] = TpState::default();
        }
    }
}

// ─── Window entry point ──────────────────────────────────────────────────────
pub fn show_vu_meter_window(ctx: &egui::Context, app: &mut DiffPlayerApp) {
    if !app.view().show_vu_meter {
        return;
    }

    let dt = ctx.input(|i| i.stable_dt).min(0.1_f32);
    ctx.request_repaint();

    let (ch_idx, ch_label, loudness) = if !app.view().mute_a {
        (0usize, "CHANNEL A - HYBRID METER", app.view().loudness_a)
    } else if !app.view().mute_b {
        (1usize, "CHANNEL B - HYBRID METER", app.view().loudness_b)
    } else {
        (0, "— MUTED —", Default::default())
    };

    let (tp_l, tp_r) = {
        let mut state = TP_STATE.lock().unwrap();
        // Since we want the fast bar to represent the overall peak, we can just use the max of L/R
        // or we could show L/R in separate bars. But we only have two bars: Left=PPM, Right=Momentary LUFS.
        // We will just process both channels, and use the max for the PPM bar, or mix them.
        state[ch_idx][0].update(loudness.true_peak[0] as f32, dt);
        state[ch_idx][1].update(loudness.true_peak[1] as f32, dt);
        (state[ch_idx][0], state[ch_idx][1])
    };

    let mut open = true;
    egui::Window::new("Loudness & Peak Meter")
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .default_pos(Pos2::new(
            ctx.screen_rect().width() - 320.0,
            ctx.screen_rect().height() - 600.0,
        ))
        .frame(
            egui::Frame::none()
                .fill(Color32::from_rgb(12, 12, 16))
                .inner_margin(egui::Margin::same(12.0))
                .stroke(Stroke::new(1.5, Color32::from_rgb(50, 50, 68)))
                .rounding(Rounding::same(6.0)),
        )
        .show(ctx, |ui| {
            draw_ebu_panel(ui, ch_idx, ch_label, &loudness, tp_l, tp_r);
        });

    if !open {
        app.view_mut().show_vu_meter = false;
    }
}

// ─── Drawing ─────────────────────────────────────────────────────────────────
fn lufs_to_t(lufs: f32) -> f32 {
    const MIN: f32 = -54.0;
    const MAX: f32 = 9.0;
    ((lufs - MIN) / (MAX - MIN)).clamp(0.0, 1.0)
}

fn lufs_color(lufs: f32, lit: bool) -> Color32 {
    let (r, g, b) = if lufs >= -14.0 {
        (255, 40, 40)
    } else if lufs >= -20.0 {
        (255, 180, 40)
    } else if lufs >= -26.0 {
        (40, 220, 80)
    } else {
        (30, 140, 180)
    };
    if lit {
        Color32::from_rgb(r, g, b)
    } else {
        Color32::from_rgb(
            (r as f32 * 0.08) as u8,
            (g as f32 * 0.08) as u8,
            (b as f32 * 0.08) as u8,
        )
    }
}

fn ppm_color(db: f32, lit: bool) -> Color32 {
    let (r, g, b) = if db >= -1.0 {
        (255, 40, 40)
    } else if db >= -9.0 {
        (255, 180, 40)
    } else {
        (40, 220, 80)
    };
    if lit {
        Color32::from_rgb(r, g, b)
    } else {
        Color32::from_rgb(
            (r as f32 * 0.08) as u8,
            (g as f32 * 0.08) as u8,
            (b as f32 * 0.08) as u8,
        )
    }
}

fn draw_ebu_panel(
    ui: &mut egui::Ui,
    ch_idx: usize,
    ch_label: &str,
    loudness: &crate::types::LoudnessResult,
    tp_l: TpState,
    tp_r: TpState,
) {
    const NUM_LEDS: usize = 48;
    const LED_W: f32 = 64.0;
    const LED_H: f32 = 8.0;
    const GAP: f32 = 2.0;
    const GUTTER: f32 = 16.0;
    const SCALE_W: f32 = 40.0;
    const OUTER_PAD: f32 = 20.0;

    // The scale applies to both bars conceptually but physically they are different units
    const MARKS: &[f32] = &[0.0, -9.0, -14.0, -18.0, -23.0, -30.0, -40.0, -54.0];

    let col_h = (LED_H + GAP) * NUM_LEDS as f32;
    let header_h = 36.0;
    let footer_h = 96.0;
    let total_h = header_h + col_h + footer_h;
    let total_w = OUTER_PAD + LED_W + GUTTER + SCALE_W + GUTTER + LED_W + OUTER_PAD;

    // ── Header ────────────────────────────────────────────────────────────────
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new("⬛  HYBRID METER  ⬛")
                .font(FontId::monospace(11.0))
                .color(Color32::from_rgb(160, 160, 190))
                .strong(),
        );
        ui.label(
            egui::RichText::new(ch_label)
                .font(FontId::monospace(10.0))
                .color(Color32::from_rgb(80, 190, 255)),
        );
        ui.add_space(4.0);
    });

    let (resp, painter) = ui.allocate_painter(Vec2::new(total_w, total_h), egui::Sense::hover());
    let origin = resp.rect.min;

    let col_l_x = origin.x + OUTER_PAD;
    let scale_x = col_l_x + LED_W + GUTTER;
    let col_r_x = scale_x + SCALE_W + GUTTER;
    let leds_top = origin.y + header_h;

    let db_to_y = |db: f32| -> f32 {
        let t = 1.0 - lufs_to_t(db);
        leds_top + t * col_h
    };

    let bg = Rect::from_min_size(Pos2::new(origin.x, leds_top), Vec2::new(total_w, col_h));
    painter.rect_filled(bg, Rounding::same(4.0), Color32::from_rgb(6, 6, 10));
    painter.rect_stroke(
        bg,
        Rounding::same(4.0),
        Stroke::new(1.0, Color32::from_rgb(35, 35, 50)),
    );

    // Left Column: PPM (True Peak Fast)
    let ppm_val_l_db = if tp_l.ppm_level <= 0.00001 {
        -60.0
    } else {
        20.0 * tp_l.ppm_level.log10()
    };
    let ppm_val_r_db = if tp_r.ppm_level <= 0.00001 {
        -60.0
    } else {
        20.0 * tp_r.ppm_level.log10()
    };

    // Right Column: Momentary LUFS
    let m_lufs = loudness.momentary as f32;

    for i in 0..NUM_LEDS {
        let t = i as f32 / (NUM_LEDS - 1) as f32;
        let led_val = egui::lerp(-54.0f32..=9.0, 1.0 - t);
        let y = leds_top + t * col_h;

        let rect_l_l =
            Rect::from_min_size(Pos2::new(col_l_x, y), Vec2::new(LED_W / 2.0 - 1.0, LED_H));
        let rect_l_r = Rect::from_min_size(
            Pos2::new(col_l_x + LED_W / 2.0 + 1.0, y),
            Vec2::new(LED_W / 2.0 - 1.0, LED_H),
        );
        let rect_r = Rect::from_min_size(Pos2::new(col_r_x, y), Vec2::new(LED_W, LED_H));

        // Draw PPM L/R on left, LUFS on right
        painter.rect_filled(
            rect_l_l,
            Rounding::same(1.0),
            ppm_color(led_val, ppm_val_l_db >= led_val),
        );
        painter.rect_filled(
            rect_l_r,
            Rounding::same(1.0),
            ppm_color(led_val, ppm_val_r_db >= led_val),
        );
        painter.rect_filled(
            rect_r,
            Rounding::same(1.5),
            lufs_color(led_val, m_lufs >= led_val),
        );
    }

    // Target reference line at -23 (Target LUFS)
    let ref_y = db_to_y(-23.0) + LED_H / 2.0;
    painter.line_segment(
        [
            Pos2::new(col_r_x - 4.0, ref_y),
            Pos2::new(col_r_x + LED_W + 4.0, ref_y),
        ],
        Stroke::new(2.0, Color32::from_rgb(0, 255, 255)),
    );

    // Peak limit line at -1 dBTP (Left column)
    let tp_ref_y = db_to_y(-1.0) + LED_H / 2.0;
    painter.line_segment(
        [
            Pos2::new(col_l_x - 4.0, tp_ref_y),
            Pos2::new(col_l_x + LED_W + 4.0, tp_ref_y),
        ],
        Stroke::new(2.0, Color32::from_rgb(255, 50, 50)),
    );

    // ── True Peak markers ────────────────
    let draw_tp_marker = |x: f32, w: f32, peak_db: f32| {
        if peak_db <= -53.5 {
            return;
        }
        let py = db_to_y(peak_db);
        let mrect = Rect::from_min_size(Pos2::new(x, py), Vec2::new(w, 3.0));
        let mut color = Color32::from_rgb(255, 100, 100);
        if peak_db >= -1.0 {
            color = Color32::from_rgb(255, 0, 0);
        }
        painter.rect_filled(mrect, Rounding::same(0.0), color);
        painter.rect_stroke(mrect, Rounding::same(0.0), Stroke::new(1.0, Color32::WHITE));
    };
    draw_tp_marker(col_l_x, LED_W / 2.0 - 1.0, tp_l.peak);
    draw_tp_marker(col_l_x + LED_W / 2.0 + 1.0, LED_W / 2.0 - 1.0, tp_r.peak);

    // ── Centre Scale ──────────────────────────────────────────────────────────
    for &db in MARKS {
        let y = db_to_y(db) + LED_H / 2.0;
        painter.line_segment(
            [Pos2::new(scale_x, y), Pos2::new(scale_x + 5.0, y)],
            Stroke::new(1.0, Color32::from_rgb(70, 70, 90)),
        );
        painter.line_segment(
            [
                Pos2::new(scale_x + SCALE_W - 5.0, y),
                Pos2::new(scale_x + SCALE_W, y),
            ],
            Stroke::new(1.0, Color32::from_rgb(70, 70, 90)),
        );
        let label = format!("{:3.0}", db);
        painter.text(
            Pos2::new(scale_x + SCALE_W / 2.0, y),
            egui::Align2::CENTER_CENTER,
            label,
            FontId::monospace(9.5),
            if db == -23.0 {
                Color32::from_rgb(0, 255, 255)
            } else {
                Color32::from_rgb(150, 150, 170)
            },
        );
    }

    // ── CLIP indicators ──────────────────────────────────────────────────────
    let clip_y = origin.y + 2.0;
    let clip_h = 14.0;

    // Left clip (True Peak)
    let is_clip = tp_l.clip || tp_r.clip;
    let cr_l = Rect::from_min_size(Pos2::new(col_l_x, clip_y), Vec2::new(LED_W, clip_h));

    // Make clip indicator clickable
    let clip_resp = ui.interact(
        cr_l,
        ui.id().with(format!("clip_{}", ch_idx)),
        egui::Sense::click(),
    );
    if clip_resp.clicked() {
        if let Ok(mut state) = TP_STATE.lock() {
            state[ch_idx][0].clip = false;
            state[ch_idx][1].clip = false;
        }
    }

    let (fill, text_col) = if is_clip {
        (Color32::from_rgb(220, 20, 20), Color32::WHITE)
    } else {
        (Color32::from_rgb(25, 10, 10), Color32::from_rgb(70, 30, 30))
    };
    if clip_resp.hovered() {
        painter.rect_filled(cr_l, Rounding::same(2.0), Color32::from_rgb(100, 30, 30));
    } else {
        painter.rect_filled(cr_l, Rounding::same(2.0), fill);
    }
    painter.rect_stroke(
        cr_l,
        Rounding::same(2.0),
        Stroke::new(0.5, Color32::from_rgb(80, 30, 30)),
    );
    painter.text(
        cr_l.center(),
        egui::Align2::CENTER_CENTER,
        "OVER",
        FontId::monospace(7.5),
        text_col,
    );

    // ── Headers ───────────────────────────────────────────────────────────────
    let header_label_y = origin.y + 18.0;
    painter.text(
        Pos2::new(col_l_x + LED_W / 2.0, header_label_y),
        egui::Align2::CENTER_CENTER,
        "PPM",
        FontId::monospace(12.0),
        Color32::from_rgb(255, 120, 120),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W / 2.0, header_label_y),
        egui::Align2::CENTER_CENTER,
        "LUFS",
        FontId::monospace(12.0),
        Color32::from_rgb(120, 190, 255),
    );

    // ── Footer: numeric readouts ──────────────────────────────────────────────
    let mut footer_y = leds_top + col_h + 8.0;

    let fmt_val = |v: f32| -> String {
        if !v.is_finite() || v <= -119.5 {
            " -∞  ".into()
        } else {
            format!("{:5.1}", v)
        }
    };

    let max_ppm = ppm_val_l_db.max(ppm_val_r_db);
    painter.text(
        Pos2::new(col_l_x + LED_W / 2.0, footer_y),
        egui::Align2::CENTER_TOP,
        fmt_val(max_ppm),
        FontId::monospace(12.0),
        ppm_color(max_ppm, true),
    );
    painter.text(
        Pos2::new(scale_x + SCALE_W / 2.0, footer_y),
        egui::Align2::CENTER_TOP,
        "dB/LU",
        FontId::monospace(9.0),
        Color32::from_rgb(70, 70, 90),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W / 2.0, footer_y),
        egui::Align2::CENTER_TOP,
        fmt_val(m_lufs),
        FontId::monospace(12.0),
        lufs_color(m_lufs, true),
    );

    footer_y += 20.0;
    // Short-term & Integrated
    let s_lufs = loudness.short_term as f32;
    let i_lufs = loudness.integrated as f32;
    painter.text(
        Pos2::new(col_l_x, footer_y),
        egui::Align2::LEFT_TOP,
        "Short-term",
        FontId::monospace(11.0),
        Color32::from_rgb(150, 150, 170),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W, footer_y),
        egui::Align2::RIGHT_TOP,
        format!("{} LUFS", fmt_val(s_lufs)),
        FontId::monospace(12.0),
        if (s_lufs + 23.0).abs() <= 2.0 {
            Color32::from_rgb(50, 255, 100)
        } else {
            Color32::from_rgb(255, 200, 50)
        },
    );

    footer_y += 16.0;
    painter.text(
        Pos2::new(col_l_x, footer_y),
        egui::Align2::LEFT_TOP,
        "Integrated",
        FontId::monospace(11.0),
        Color32::from_rgb(150, 150, 170),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W, footer_y),
        egui::Align2::RIGHT_TOP,
        format!("{} LUFS", fmt_val(i_lufs)),
        FontId::monospace(12.0),
        if (i_lufs + 23.0).abs() <= 1.0 {
            Color32::from_rgb(50, 255, 100)
        } else {
            Color32::from_rgb(255, 200, 50)
        },
    );

    footer_y += 16.0;
    // True Peak Max
    let max_tp = tp_l.peak.max(tp_r.peak);
    painter.text(
        Pos2::new(col_l_x, footer_y),
        egui::Align2::LEFT_TOP,
        "True Peak",
        FontId::monospace(11.0),
        Color32::from_rgb(150, 150, 170),
    );
    painter.text(
        Pos2::new(col_r_x + LED_W, footer_y),
        egui::Align2::RIGHT_TOP,
        format!("{} dBTP", fmt_val(max_tp)),
        FontId::monospace(12.0),
        if max_tp >= -1.0 {
            Color32::from_rgb(255, 50, 50)
        } else {
            Color32::from_rgb(50, 200, 255)
        },
    );
}
