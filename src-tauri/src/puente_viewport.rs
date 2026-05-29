//! Puente motor → GPU: encola el último frame sin bloquear el hilo del motor ni saturar el main.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tauri::AppHandle;

use crate::viewport::{enviar_en_main, EstadoViewport};

/// Intervalo mínimo entre `presentar` (≈30 fps) para no ahogar el hilo principal.
const INTERVALO_PRESENTAR: Duration = Duration::from_millis(33);

/// Frames pendientes de subir a wgpu (solo el más reciente por canal).
pub struct PuenteViewport {
    pendiente_a: Mutex<Option<(Arc<Vec<u8>>, u32, u32)>>,
    pendiente_b: Mutex<Option<(Arc<Vec<u8>>, u32, u32)>>,
    repintado_programado: AtomicBool,
    ultimo_presentar: Mutex<Instant>,
    viewport: Arc<Mutex<EstadoViewport>>,
    app: AppHandle,
}

impl PuenteViewport {
    pub fn nuevo(app: AppHandle, viewport: Arc<Mutex<EstadoViewport>>) -> Arc<Self> {
        Arc::new(Self {
            pendiente_a: Mutex::new(None),
            pendiente_b: Mutex::new(None),
            repintado_programado: AtomicBool::new(false),
            ultimo_presentar: Mutex::new(Instant::now()),
            viewport,
            app,
        })
    }

    pub fn encolar_a(self: &Arc<Self>, rgba: Arc<Vec<u8>>, ancho: u32, alto: u32) {
        *self.pendiente_a.lock().expect("pendiente_a") = Some((rgba, ancho, alto));
        self.programar_repintado();
    }

    pub fn encolar_b(self: &Arc<Self>, rgba: Arc<Vec<u8>>, ancho: u32, alto: u32) {
        *self.pendiente_b.lock().expect("pendiente_b") = Some((rgba, ancho, alto));
        self.programar_repintado();
    }

    fn programar_repintado(self: &Arc<Self>) {
        if self
            .repintado_programado
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_err()
        {
            return;
        }
        let puente = Arc::clone(self);
        let app = self.app.clone();
        enviar_en_main(&app, move || puente.vaciar_y_presentar());
    }

    fn vaciar_y_presentar(self: &Arc<Self>) {
        self.repintado_programado.store(false, Ordering::Release);
        let Ok(mut vp) = self.viewport.try_lock() else {
            Self::programar_repintado(self);
            return;
        };

        if let Some((rgba, w, h)) = self.pendiente_a.lock().expect("pendiente_a").take() {
            vp.subir_frame_a(rgba.as_slice(), w, h);
        }
        if let Some((rgba, w, h)) = self.pendiente_b.lock().expect("pendiente_b").take() {
            vp.subir_frame_b(rgba.as_slice(), w, h);
        }

        let mut ultimo = self.ultimo_presentar.lock().expect("ultimo_presentar");
        if ultimo.elapsed() >= INTERVALO_PRESENTAR {
            *ultimo = Instant::now();
            vp.presentar();
        }
    }
}
