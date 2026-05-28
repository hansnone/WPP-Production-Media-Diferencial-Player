//! Persistencia de paneles por workspace (JSON, sin egui).

use serde::{Deserialize, Serialize};

use crate::workspace::WorkspaceLayout;

/// Disposición de paneles lateral para un workspace.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisposicionPaneles {
    pub panel_izquierdo_visible: bool,
    pub panel_derecho_visible: bool,
    pub ancho_panel_izquierdo_px: u32,
    pub ancho_panel_derecho_px: u32,
}

impl Default for DisposicionPaneles {
    fn default() -> Self {
        Self {
            panel_izquierdo_visible: true,
            panel_derecho_visible: true,
            ancho_panel_izquierdo_px: 280,
            ancho_panel_derecho_px: 320,
        }
    }
}

/// Mapa workspace → disposición guardada en disco.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LayoutPersistido {
    pub layouts: std::collections::HashMap<String, DisposicionPaneles>,
    pub ultimo_workspace: WorkspaceLayout,
}

impl LayoutPersistido {
    #[must_use]
    pub fn vacio() -> Self {
        Self::default()
    }

    pub fn guardar_workspace(&mut self, workspace: WorkspaceLayout, disp: DisposicionPaneles) {
        self.ultimo_workspace = workspace;
        self.layouts.insert(workspace.id().to_string(), disp);
    }

    #[must_use]
    pub fn obtener(&self, workspace: WorkspaceLayout) -> DisposicionPaneles {
        self.layouts
            .get(workspace.id())
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_json() {
        let mut lp = LayoutPersistido::vacio();
        lp.guardar_workspace(
            WorkspaceLayout::Compare,
            DisposicionPaneles {
                panel_izquierdo_visible: false,
                ..Default::default()
            },
        );
        let json = serde_json::to_string(&lp).unwrap();
        let loaded: LayoutPersistido = serde_json::from_str(&json).unwrap();
        assert!(!loaded.obtener(WorkspaceLayout::Compare).panel_izquierdo_visible);
        assert_eq!(loaded.ultimo_workspace, WorkspaceLayout::Compare);
    }
}
