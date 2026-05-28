//! Estado agregado del reproductor (dominio v2 / extracción desde v1).

use serde::{Deserialize, Serialize};

use crate::modos::{ciclar_modo_comparacion, normalizar_modo_diferencia, CompareMode, DiffMode};
use crate::reproduccion::PlaybackState;
use crate::workspace::WorkspaceLayout;

/// Estado de dominio del jugador sin dependencias de UI ni GPU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub modo_comparacion: CompareMode,
    pub modo_diferencia: DiffMode,
    pub workspace: WorkspaceLayout,
    pub reproduccion: PlaybackStateSer,
    pub split_pos: f32,
    pub zoom: f32,
    pub pan_u: f32,
    pub pan_v: f32,
    pub amplifier: f32,
    pub split_horizontal: bool,
}

/// Versión serializable del reloj (sin `Instant`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaybackStateSer {
    pub is_playing: bool,
    pub current_pts: f64,
    pub duration_a: f64,
    pub duration_b: f64,
    pub playback_start_pts: f64,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            modo_comparacion: CompareMode::default(),
            modo_diferencia: DiffMode::default(),
            workspace: WorkspaceLayout::default(),
            reproduccion: PlaybackStateSer::default(),
            split_pos: 0.5,
            zoom: 1.0,
            pan_u: 0.0,
            pan_v: 0.0,
            amplifier: 5.0,
            split_horizontal: false,
        }
    }
}

impl PlayerState {
    /// Sincroniza desde el `PlaybackState` en memoria de la v1.
    pub fn sincronizar_desde_reproduccion(&mut self, playback: &PlaybackState) {
        self.reproduccion.is_playing = playback.is_playing;
        self.reproduccion.current_pts = playback.current_pts;
        self.reproduccion.duration_a = playback.duration_a;
        self.reproduccion.duration_b = playback.duration_b;
        self.reproduccion.playback_start_pts = playback.playback_start_pts;
    }

    pub fn aplicar_a_reproduccion(&self, playback: &mut PlaybackState) {
        playback.is_playing = self.reproduccion.is_playing;
        playback.current_pts = self.reproduccion.current_pts;
        playback.duration_a = self.reproduccion.duration_a;
        playback.duration_b = self.reproduccion.duration_b;
        playback.playback_start_pts = self.reproduccion.playback_start_pts;
    }

    pub fn ciclar_modo(&mut self) {
        self.modo_comparacion = ciclar_modo_comparacion(self.modo_comparacion);
        self.modo_diferencia =
            normalizar_modo_diferencia(self.modo_comparacion, self.modo_diferencia);
    }

    pub fn fijar_split_extremo(&mut self, lado_b: bool) {
        self.modo_comparacion = CompareMode::SplitScreen;
        if lado_b {
            self.split_pos = if self.split_pos > 0.95 { 0.5 } else { 1.0 };
        } else {
            self.split_pos = if self.split_pos < 0.05 { 0.5 } else { 0.0 };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ciclar_normaliza_diff() {
        let mut p = PlayerState::default();
        p.modo_comparacion = CompareMode::AbsDiff;
        p.modo_diferencia = DiffMode::None;
        p.ciclar_modo();
        assert_ne!(p.modo_comparacion, CompareMode::AbsDiff);
        let validos = crate::modos::modos_diferencia_validos(p.modo_comparacion);
        if !validos.is_empty() {
            assert!(validos.contains(&p.modo_diferencia));
        }
    }

    #[test]
    fn serde_player_state() {
        let json = serde_json::to_string(&PlayerState::default()).unwrap();
        let _: PlayerState = serde_json::from_str(&json).unwrap();
    }
}
