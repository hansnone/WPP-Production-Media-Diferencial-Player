//! Utilidades de temporización de repintado durante la reproducción.
//!
//! Separado del `impl DiffPlayerApp` para poder testear la matemática de `Duration`
//! sin arrancar egui. El reproductor acorta el intervalo entre frames cuando hay
//! audio activo para reducir underruns en rodio.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_clamped_to_max() {
        let d = next_frame_repaint_delay(25.0, 0.0, 8);
        assert!(d <= Duration::from_millis(8));
    }
}
