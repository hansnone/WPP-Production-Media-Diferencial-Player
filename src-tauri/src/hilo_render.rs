//! Hilo dedicado wgpu: upload de texturas + present (vsync) fuera del main thread de Tauri.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{Receiver, Sender};
use diffplayerqc_render::ViewportGpu;
use tauri::{AppHandle, window::Window};

use crate::viewport::{emitir_estado_gpu_publico, VistaCompare};

/// Ventana overlay creada en main; wgpu la adopta en el hilo render (patrón habitual en macOS).
pub(crate) struct VentanaEnviable(Window);
unsafe impl Send for VentanaEnviable {}

/// Órdenes al hilo render (no bloquean el WebView ni el motor).
pub enum OrdenRender {
    /// Crea swapchain + pipeline compare en el hilo render.
    CrearGpu {
        ventana: VentanaEnviable,
        ancho: u32,
        alto: u32,
        vista: VistaCompare,
        vid_ancho: u32,
        vid_alto: u32,
    },
    Redimensionar { ancho: u32, alto: u32 },
    OcultarGpu,
    FrameA(Arc<Vec<u8>>, u32, u32),
    FrameB(Arc<Vec<u8>>, u32, u32),
    /// Sube A/B (los que cambien) y presenta una sola vez (compare fluido).
    SubirYPresentar {
        a: Option<(Arc<Vec<u8>>, u32, u32)>,
        b: Option<(Arc<Vec<u8>>, u32, u32)>,
    },
    EstablecerVista(VistaCompare),
    VidDimensiones { ancho: u32, alto: u32 },
    /// Present vsync (entre frames o tras upload).
    Presentar,
    Reproduciendo(bool),
}

pub struct HiloRender {
    tx: Sender<OrdenRender>,
    gpu_listo: Arc<AtomicBool>,
}

struct EstadoHiloRender {
    gpu: Option<ViewportGpu>,
    vista: VistaCompare,
    vid_ancho: u32,
    vid_alto: u32,
    pendiente_a: Option<(Arc<Vec<u8>>, u32, u32)>,
    pendiente_b: Option<(Arc<Vec<u8>>, u32, u32)>,
    reproduciendo: bool,
    app: AppHandle,
    gpu_listo: Arc<AtomicBool>,
}

impl HiloRender {
    pub fn iniciar(app: AppHandle) -> Arc<Self> {
        let (tx, rx) = crossbeam_channel::unbounded::<OrdenRender>();
        let gpu_listo = Arc::new(AtomicBool::new(false));
        let gpu_flag = Arc::clone(&gpu_listo);
        let app_hilo = app.clone();

        thread::Builder::new()
            .name("viewport-wgpu".into())
            .spawn(move || bucle_render(app_hilo, rx, gpu_flag))
            .expect("hilo viewport-wgpu");

        Arc::new(Self { tx, gpu_listo })
    }

    pub fn gpu_operativo(&self) -> bool {
        self.gpu_listo.load(Ordering::Relaxed)
    }

    pub fn enviar(&self, orden: OrdenRender) {
        if self.tx.send(orden).is_err() {
            log::warn!("hilo render: canal caído");
        }
    }

    pub fn presentar(&self) {
        self.enviar(OrdenRender::Presentar);
    }

    pub fn establecer_reproduciendo(&self, activo: bool) {
        self.enviar(OrdenRender::Reproduciendo(activo));
    }
}

/// Empaqueta la ventana overlay para init wgpu en el hilo render.
pub fn orden_crear_gpu(
    ventana: Window,
    ancho: u32,
    alto: u32,
    vista: VistaCompare,
    vid_ancho: u32,
    vid_alto: u32,
) -> OrdenRender {
    OrdenRender::CrearGpu {
        ventana: VentanaEnviable(ventana),
        ancho,
        alto,
        vista,
        vid_ancho,
        vid_alto,
    }
}

fn bucle_render(app: AppHandle, rx: Receiver<OrdenRender>, gpu_listo: Arc<AtomicBool>) {
    let mut estado = EstadoHiloRender {
        gpu: None,
        vista: VistaCompare::default(),
        vid_ancho: 0,
        vid_alto: 0,
        pendiente_a: None,
        pendiente_b: None,
        reproduciendo: false,
        app,
        gpu_listo,
    };

    // Present solo al recibir frame o cambio de vista (no vsync vacío cada 16 ms).
    while let Ok(orden) = rx.recv() {
        estado.procesar_orden(orden);
    }
}

impl EstadoHiloRender {
    fn procesar_orden(&mut self, orden: OrdenRender) {
        match orden {
            OrdenRender::CrearGpu {
                ventana,
                ancho,
                alto,
                vista,
                vid_ancho,
                vid_alto,
            } => {
                self.vista = vista;
                self.vid_ancho = vid_ancho;
                self.vid_alto = vid_alto;
                match ViewportGpu::nuevo(ventana.0, ancho, alto) {
                    Ok(gpu) => {
                        log::info!("hilo render: GPU compare {ancho}x{alto}");
                        self.gpu = Some(gpu);
                        self.gpu_listo.store(true, Ordering::Relaxed);
                        emitir_estado_gpu_publico(&self.app, true);
                        self.presentar_completo();
                    }
                    Err(e) => {
                        log::error!("hilo render: init GPU {e:#}");
                        self.gpu = None;
                        self.gpu_listo.store(false, Ordering::Relaxed);
                        emitir_estado_gpu_publico(&self.app, false);
                    }
                }
            }
            OrdenRender::Redimensionar { ancho, alto } => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.redimensionar(ancho, alto);
                    self.presentar_completo();
                }
            }
            OrdenRender::OcultarGpu => {
                self.gpu = None;
                self.gpu_listo.store(false, Ordering::Relaxed);
                emitir_estado_gpu_publico(&self.app, false);
            }
            OrdenRender::FrameA(rgba, w, h) => {
                self.pendiente_a = Some((rgba, w, h));
                self.vid_ancho = self.vid_ancho.max(w);
                self.vid_alto = self.vid_alto.max(h);
                if self.gpu.is_some() {
                    self.presentar_completo();
                }
            }
            OrdenRender::FrameB(rgba, w, h) => {
                self.pendiente_b = Some((rgba, w, h));
                self.vid_ancho = self.vid_ancho.max(w);
                self.vid_alto = self.vid_alto.max(h);
                if self.gpu.is_some() {
                    self.presentar_completo();
                }
            }
            OrdenRender::SubirYPresentar { a, b } => {
                if let Some((rgba, w, h)) = a {
                    self.pendiente_a = Some((rgba, w, h));
                    self.vid_ancho = self.vid_ancho.max(w);
                    self.vid_alto = self.vid_alto.max(h);
                }
                if let Some((rgba, w, h)) = b {
                    self.pendiente_b = Some((rgba, w, h));
                    self.vid_ancho = self.vid_ancho.max(w);
                    self.vid_alto = self.vid_alto.max(h);
                }
                if self.gpu.is_some() {
                    self.presentar_completo();
                }
            }
            OrdenRender::EstablecerVista(vista) => {
                self.vista = vista;
                self.presentar_completo();
            }
            OrdenRender::VidDimensiones { ancho, alto } => {
                if ancho >= 2 && alto >= 2 {
                    self.vid_ancho = ancho;
                    self.vid_alto = alto;
                }
            }
            OrdenRender::Presentar => self.presentar_completo(),
            OrdenRender::Reproduciendo(activo) => self.reproduciendo = activo,
        }
    }

    fn presentar_completo(&mut self) {
        let Some(gpu) = &mut self.gpu else {
            return;
        };

        if let Some((rgba, w, h)) = self.pendiente_a.take() {
            gpu.subir_textura_a(rgba.as_slice(), w, h);
        }
        if let Some((rgba, w, h)) = self.pendiente_b.take() {
            gpu.subir_textura_b(rgba.as_slice(), w, h);
        }

        gpu.actualizar_uniformes_vista(
            self.vista.modo,
            self.vista.diff_mode,
            self.vista.split_pos,
            self.vista.amplifier,
            self.vista.zoom,
            self.vista.pan_u,
            self.vista.pan_v,
            self.vista.split_horizontal,
            self.vid_ancho.max(2),
            self.vid_alto.max(2),
        );
        gpu.avanzar_colas();
        if let Err(e) = gpu.presentar() {
            log::warn!("hilo render presentar: {e:#}");
        }
    }
}
