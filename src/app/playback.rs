//! Utilidades de temporización de repintado durante la reproducción.
//!
//! Separado del `impl DiffPlayerApp` para poder testear la matemática de `Duration`
//! sin arrancar egui. El reproductor acorta el intervalo entre frames cuando hay
//! audio activo para reducir underruns en rodio.

use crate::types::VideoFrame;
use std::time::Duration;

/// Máximo tiempo entre repintados cuando hay sink de audio (ms).
pub const REPINT_AUDIO_MAX_MS: u64 = 8;
/// Máximo cuando no hay audio activo en ese camino (ms).
pub const REPINT_IDLE_MAX_MS: u64 = 100;

/// Calcula el retardo hasta el siguiente repintado alineado al siguiente frame de vídeo.
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

/// Pure function to select the best frame for the current clock given the tolerance.
/// It takes the currently cached frame (if any) and an iterator of incoming frames.
/// Returns `(best_frame_to_render, next_frame_to_cache)`.
pub fn select_best_frame<I>(
    mut current_candidate: Option<VideoFrame>,
    incoming_frames: I,
    current_pts: f64,
    pts_tolerance: f64,
) -> (Option<VideoFrame>, Option<VideoFrame>)
where
    I: Iterator<Item = VideoFrame>,
{
    let mut best_frame = current_candidate.take();

    if let Some(ref bf) = best_frame {
        if bf.pts > current_pts + pts_tolerance {
            // The cached next_frame is still in the future, keep waiting.
            return (None, best_frame);
        }
    }

    for frame in incoming_frames {
        if frame.pts <= current_pts + pts_tolerance {
            best_frame = Some(frame);
        } else {
            return (best_frame, Some(frame));
        }
    }

    (best_frame, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_clamped_to_max() {
        let d = next_frame_repaint_delay(25.0, 0.0, 8);
        assert!(d <= Duration::from_millis(8));
    }

    #[test]
    fn select_best_frame_future_cached() {
        let cached = Some(VideoFrame {
            pts: 100.0,
            width: 0,
            height: 0,
            rgba_data: std::sync::Arc::new([]),
        });
        let incoming = vec![];
        let (best, next) = select_best_frame(cached.clone(), incoming.into_iter(), 50.0, 10.0);
        // The cached frame is at 100 > 60 (current_pts 50 + tolerance 10). It should be kept as next.
        assert!(best.is_none());
        assert_eq!(next.unwrap().pts, 100.0);
    }

    #[test]
    fn select_best_frame_consumes_and_advances() {
        let cached = Some(VideoFrame {
            pts: 40.0,
            width: 0,
            height: 0,
            rgba_data: std::sync::Arc::new([]),
        });
        let incoming = vec![
            VideoFrame {
                pts: 50.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
            VideoFrame {
                pts: 60.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
            VideoFrame {
                pts: 70.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
        ];

        let (best, next) = select_best_frame(cached, incoming.into_iter(), 50.0, 10.0);
        // Best should be the one at 60 (50 + 10 tolerance), next should be 70.
        assert_eq!(best.unwrap().pts, 60.0);
        assert_eq!(next.unwrap().pts, 70.0);
    }

    #[test]
    fn select_best_frame_exhausts_incoming() {
        let cached = None;
        let incoming = vec![
            VideoFrame {
                pts: 10.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
            VideoFrame {
                pts: 20.0,
                width: 0,
                height: 0,
                rgba_data: std::sync::Arc::new([]),
            },
        ];

        let (best, next) = select_best_frame(cached, incoming.into_iter(), 50.0, 10.0);
        // It consumes all, best is the last one (20), none pending.
        assert_eq!(best.unwrap().pts, 20.0);
        assert!(next.is_none());
    }
}
