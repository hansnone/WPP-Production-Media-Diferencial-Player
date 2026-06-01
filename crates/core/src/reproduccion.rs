//! Reloj maestro de reproducción y utilidades de temporización (sin UI).

use std::time::{Duration, Instant};

/// Estado de reproducción compartido entre UI e IPC.
///
/// En play: `current_pts = playback_start_pts + elapsed` desde `playback_start_instant`.
#[derive(Debug, Clone, Default)]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current_pts: f64,
    pub duration_a: f64,
    pub duration_b: f64,
    pub playback_start_instant: Option<Instant>,
    pub playback_start_pts: f64,
}

impl PlaybackState {
    #[must_use]
    pub fn max_duration(&self) -> f64 {
        self.duration_a.max(self.duration_b)
    }

    pub fn start_playback(&mut self, now: Instant) {
        self.is_playing = true;
        self.playback_start_instant = Some(now);
        self.playback_start_pts = self.current_pts;
    }

    pub fn pause(&mut self, now: Instant) {
        if self.is_playing {
            self.current_pts = self.pts_at(now);
        }
        self.is_playing = false;
        self.playback_start_instant = None;
    }

    pub fn seek(&mut self, pts: f64, now: Instant) {
        let max_d = self.max_duration();
        self.current_pts = if max_d > 0.0 {
            pts.clamp(0.0, max_d)
        } else {
            pts.max(0.0)
        };
        if self.is_playing {
            self.playback_start_pts = self.current_pts;
            self.playback_start_instant = Some(now);
        }
    }

    #[must_use]
    pub fn pts_at(&self, now: Instant) -> f64 {
        if !self.is_playing {
            return self.current_pts;
        }
        let Some(start) = self.playback_start_instant else {
            return self.current_pts;
        };
        let elapsed = now.duration_since(start).as_secs_f64();
        let mut pts = self.playback_start_pts + elapsed;
        let max_d = self.max_duration();
        if max_d > 0.0 {
            pts = pts.clamp(0.0, max_d);
        }
        pts
    }

    /// Actualiza `current_pts`; devuelve `true` si se alcanzó el final del clip.
    pub fn tick_clock(&mut self, now: Instant) -> bool {
        if !self.is_playing {
            return false;
        }
        self.current_pts = self.pts_at(now);
        let max_d = self.max_duration();
        max_d > 0.0 && self.current_pts >= max_d
    }

    #[must_use]
    pub fn step_back_pts(&self, fps: f64) -> f64 {
        if fps <= 0.0 {
            return (self.current_pts - 1.0 / 25.0).max(0.0);
        }
        (self.current_pts - 1.0 / fps).max(0.0)
    }

    #[must_use]
    pub fn step_forward_pts(&self, fps: f64) -> f64 {
        if fps <= 0.0 {
            return self.current_pts + 1.0 / 25.0;
        }
        let next = self.current_pts + 1.0 / fps;
        let max_d = self.max_duration();
        if max_d > 0.0 {
            next.min(max_d)
        } else {
            next
        }
    }

    /// Instant wall en el que el reloj maestro alcanza `pts` (modelo VLC/mpv).
    #[must_use]
    pub fn instante_para_pts(&self, pts: f64) -> Option<Instant> {
        if !self.is_playing {
            return None;
        }
        let inicio = self.playback_start_instant?;
        let offset = (pts - self.playback_start_pts).max(0.0);
        Some(inicio + Duration::from_secs_f64(offset))
    }

    /// Bloquea hasta el instante de presentación del frame (sincronía A/V).
    pub fn dormir_hasta_pts(&self, pts: f64) {
        let Some(objetivo) = self.instante_para_pts(pts) else {
            return;
        };
        let ahora = Instant::now();
        if objetivo > ahora {
            std::thread::sleep(objetivo - ahora);
        }
    }

    /// Duración de un fotograma según fps del clip (p. ej. 40 ms @ 25 fps).
    #[must_use]
    pub fn duracion_frame(fps: f64) -> f64 {
        if fps <= 0.0 {
            1.0 / 25.0
        } else {
            1.0 / fps
        }
    }
}

/// Retardo hasta el siguiente repintado alineado al siguiente frame (v1: `next_frame_repaint_delay`).
#[must_use]
pub fn next_frame_repaint_delay(fps: f64, current_pts: f64, max_delay_ms: u64) -> Duration {
    if fps <= 0.0 {
        return Duration::from_millis(1);
    }
    let next_frame_pts = (current_pts * fps).ceil() / fps;
    let delay_secs = (next_frame_pts - current_pts).max(0.0);
    Duration::from_secs_f64(delay_secs).clamp(
        Duration::from_millis(1),
        Duration::from_millis(max_delay_ms),
    )
}

pub const REPINT_AUDIO_MAX_MS: u64 = 8;
pub const REPINT_IDLE_MAX_MS: u64 = 100;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_advances_while_playing() {
        let t0 = Instant::now();
        let mut p = PlaybackState::default();
        p.duration_a = 10.0;
        p.current_pts = 1.0;
        p.start_playback(t0);
        let t1 = t0 + Duration::from_millis(500);
        assert!((p.pts_at(t1) - 1.5).abs() < 0.02);
    }

    #[test]
    fn seek_clamps_to_duration() {
        let t = Instant::now();
        let mut p = PlaybackState {
            duration_a: 5.0,
            ..Default::default()
        };
        p.seek(99.0, t);
        assert_eq!(p.current_pts, 5.0);
    }

    #[test]
    fn instante_para_pts_durante_play() {
        let t0 = Instant::now();
        let mut p = PlaybackState::default();
        p.duration_a = 30.0;
        p.current_pts = 0.0;
        p.start_playback(t0);
        let t_frame = p.instante_para_pts(1.0).expect("instante");
        assert!(((t_frame - t0).as_secs_f64() - 1.0).abs() < 0.05);
    }
}
