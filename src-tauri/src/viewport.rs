//! Ventana overlay wgpu alineada al `#canvas-slot` (geometría en main; GPU en hilo render).

use std::sync::{Arc, Mutex};

use diffplayerqc_core::CompareMode;
use serde::{Deserialize, Serialize};
use tauri::{
    utils::config::Color, AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Position,
    Size, WebviewWindow,
    window::{Window, WindowBuilder},
};

use crate::hilo_render::{orden_crear_gpu, HiloRender, OrdenRender};

/// Estado de la overlay GPU (evento `viewport-gpu` hacia el frontend).
#[derive(Clone, Serialize)]
pub struct ViewportGpuEstado {
    pub listo: bool,
}

/// Emite `viewport-gpu` deduplicado (también desde el hilo render).
pub(crate) fn emitir_estado_gpu_publico(app: &AppHandle, listo: bool) {
    emitir_estado_gpu(app, listo);
}

fn emitir_estado_gpu(app: &AppHandle, listo: bool) {
    // Evita parpadeo canvas↔GPU por eventos repetidos en cada resize.
    use std::sync::Mutex;
    static ULTIMO: Mutex<Option<bool>> = Mutex::new(None);
    let mut guard = ULTIMO.lock().expect("ultimo gpu");
    if guard.map(|v| v == listo).unwrap_or(false) {
        return;
    }
    *guard = Some(listo);
    let payload = ViewportGpuEstado { listo };
    if let Err(e) = app.emit("viewport-gpu", payload) {
        log::warn!("viewport-gpu emit: {e}");
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RectViewport {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    pub fisico: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VistaCompare {
    pub modo: CompareMode,
    pub diff_mode: diffplayerqc_core::DiffMode,
    pub split_pos: f32,
    pub amplifier: f32,
    pub zoom: f32,
    pub pan_u: f32,
    pub pan_v: f32,
    pub split_horizontal: bool,
}

impl Default for VistaCompare {
    fn default() -> Self {
        Self {
            modo: CompareMode::SplitScreen,
            diff_mode: diffplayerqc_core::DiffMode::AbsLinear,
            split_pos: 0.5,
            amplifier: 5.0,
            zoom: 1.0,
            pan_u: 0.0,
            pan_v: 0.0,
            split_horizontal: false,
        }
    }
}

/// Metadatos de overlay (posición en main; wgpu vive en [`HiloRender`]).
pub struct EstadoViewport {
    vista: VistaCompare,
    ancho_logico: u32,
    alto_logico: u32,
    ancho_fisico: u32,
    alto_fisico: u32,
    vid_ancho: u32,
    vid_alto: u32,
}

impl EstadoViewport {
    pub fn nuevo() -> Self {
        Self {
            vista: VistaCompare::default(),
            ancho_logico: 1,
            alto_logico: 1,
            ancho_fisico: 1,
            alto_fisico: 1,
            vid_ancho: 0,
            vid_alto: 0,
        }
    }

    fn asegurar_overlay(app: &AppHandle) -> tauri::Result<Window> {
        if let Some(w) = app.get_window("viewport") {
            return Ok(w);
        }

        let overlay = WindowBuilder::new(app, "viewport")
            .title("")
            .decorations(false)
            .transparent(false)
            .skip_taskbar(true)
            .visible(false)
            .background_color(Color(0, 0, 0, 255))
            .always_on_top(true)
            .build()?;

        Ok(overlay)
    }

    fn aplicar_geometria(
        overlay: &Window,
        main: &WebviewWindow,
        rect: &RectViewport,
    ) -> tauri::Result<(u32, u32, f64)> {
        let escala = main.scale_factor().unwrap_or(1.0);

        let (x, y, w_fis, h_fis) = if rect.fisico {
            (
                rect.x.round() as i32,
                rect.y.round() as i32,
                rect.width.round().max(1.0) as u32,
                rect.height.round().max(1.0) as u32,
            )
        } else {
            let origen = main.inner_position().unwrap_or_default();
            (
                origen.x + (rect.x * escala).round() as i32,
                origen.y + (rect.y * escala).round() as i32,
                (rect.width * escala).round().max(1.0) as u32,
                (rect.height * escala).round().max(1.0) as u32,
            )
        };

        overlay.set_position(Position::Physical(PhysicalPosition::new(x, y)))?;
        overlay.set_size(Size::Physical(PhysicalSize::new(w_fis, h_fis)))?;
        Ok((w_fis, h_fis, escala))
    }

    /// Alinea la ventana overlay en main y delega init/resize/present al hilo wgpu.
    pub fn sincronizar_recto(
        &mut self,
        app: &AppHandle,
        hilo: &HiloRender,
        rect: RectViewport,
    ) -> tauri::Result<()> {
        let main = app
            .get_webview_window("main")
            .ok_or_else(|| tauri::Error::Anyhow(anyhow::anyhow!("ventana main no encontrada")))?;

        let overlay = Self::asegurar_overlay(app)?;

        if rect.width < 8.0 || rect.height < 8.0 {
            let _ = overlay.hide();
            emitir_estado_gpu(app, false);
            return Ok(());
        }

        let (w_fis, h_fis, escala) = Self::aplicar_geometria(&overlay, &main, &rect)?;
        self.ancho_logico = (w_fis as f64 / escala).round().max(1.0) as u32;
        self.alto_logico = (h_fis as f64 / escala).round().max(1.0) as u32;

        let _ = overlay.set_always_on_top(true);
        let _ = overlay.set_ignore_cursor_events(true);
        overlay.show()?;
        #[cfg(target_os = "macos")]
        ordenar_overlay_encima(&overlay);

        if hilo.gpu_operativo() {
            hilo.enviar(OrdenRender::Redimensionar {
                ancho: w_fis,
                alto: h_fis,
            });
        } else {
            hilo.enviar(orden_crear_gpu(
                overlay,
                w_fis,
                h_fis,
                self.vista.clone(),
                self.vid_ancho,
                self.vid_alto,
            ));
        }

        self.ancho_fisico = w_fis;
        self.alto_fisico = h_fis;
        Ok(())
    }

    pub fn establecer_vista(&mut self, vista: VistaCompare, hilo: &HiloRender) {
        self.vista = vista.clone();
        hilo.enviar(OrdenRender::EstablecerVista(vista));
    }

    pub fn establecer_dimensiones_video(&mut self, ancho: u32, alto: u32, hilo: &HiloRender) {
        if ancho >= 2 && alto >= 2 {
            self.vid_ancho = ancho;
            self.vid_alto = alto;
            hilo.enviar(OrdenRender::VidDimensiones { ancho, alto });
        }
    }
}

pub type ViewportCompartido = Arc<Mutex<EstadoViewport>>;

#[cfg(target_os = "macos")]
fn ordenar_overlay_encima(ventana: &Window) {
    use cocoa::appkit::{NSMainMenuWindowLevel, NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::id;

    let Ok(ptr) = ventana.ns_window() else {
        return;
    };
    let ns_win = ptr as id;
    unsafe {
        ns_win.setLevel_((NSMainMenuWindowLevel + 1) as i64);
        let comportamiento = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary;
        ns_win.setCollectionBehavior_(comportamiento);
        ns_win.orderFrontRegardless();
    }
}

#[cfg(not(target_os = "macos"))]
fn ordenar_overlay_encima(_ventana: &Window) {}

pub fn ocultar_overlay(app: &AppHandle) -> tauri::Result<()> {
    if let Some(w) = app.get_window("viewport") {
        w.hide()?;
    }
    if let Some(w) = app.get_webview_window("viewport") {
        w.hide()?;
    }
    emitir_estado_gpu(app, false);
    Ok(())
}

pub fn enviar_en_main<F>(app: &AppHandle, f: F)
where
    F: FnOnce() + Send + 'static,
{
    let app = app.clone();
    let _ = app.run_on_main_thread(move || f());
}
