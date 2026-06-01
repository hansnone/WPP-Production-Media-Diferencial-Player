//! Puente motor → hilo wgpu (sin bloquear main thread).

use std::sync::Arc;

use crate::hilo_render::{HiloRender, OrdenRender};

/// Encola frames RGBA hacia el hilo render.
pub struct PuenteViewport {
    hilo: Arc<HiloRender>,
}

impl PuenteViewport {
    pub fn nuevo(hilo: Arc<HiloRender>) -> Arc<Self> {
        Arc::new(Self { hilo })
    }

    /// Un solo upload + present por tick (A y/o B).
    pub fn subir_y_presentar(
        self: &Arc<Self>,
        a: Option<(Arc<Vec<u8>>, u32, u32)>,
        b: Option<(Arc<Vec<u8>>, u32, u32)>,
    ) {
        if a.is_none() && b.is_none() {
            return;
        }
        self.hilo
            .enviar(OrdenRender::SubirYPresentar { a, b });
    }
}
