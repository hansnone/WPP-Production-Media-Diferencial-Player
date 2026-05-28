//! Workspaces intercambiables de la shell v2 (Compare, Audio, etc.).

use serde::{Deserialize, Serialize};

/// Workspace activo en la aplicación v2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceLayout {
    #[default]
    Compare,
    Inspect,
    Audio,
    Report,
    Export,
}

impl WorkspaceLayout {
    /// Identificador estable para persistencia y telemetría.
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Self::Compare => "compare",
            Self::Inspect => "inspect",
            Self::Audio => "audio",
            Self::Report => "report",
            Self::Export => "export",
        }
    }

    /// Avanza al siguiente workspace en orden de pestaña.
    #[must_use]
    pub fn siguiente(self) -> Self {
        match self {
            Self::Compare => Self::Inspect,
            Self::Inspect => Self::Audio,
            Self::Audio => Self::Report,
            Self::Report => Self::Export,
            Self::Export => Self::Compare,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ciclo_workspaces() {
        let mut w = WorkspaceLayout::Compare;
        for _ in 0..5 {
            w = w.siguiente();
        }
        assert_eq!(w, WorkspaceLayout::Compare);
    }

    #[test]
    fn serde_roundtrip() {
        let json = serde_json::to_string(&WorkspaceLayout::Audio).unwrap();
        assert_eq!(
            serde_json::from_str::<WorkspaceLayout>(&json).unwrap(),
            WorkspaceLayout::Audio
        );
    }
}
