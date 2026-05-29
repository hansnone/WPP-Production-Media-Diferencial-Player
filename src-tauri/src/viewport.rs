//! Ventana overlay wgpu alineada con `#canvas-slot` del WebView.
//!
//! Usamos [`Window`] sin webview: el WKWebView tapaba la superficie wgpu (cuadro negro).

use std::sync::{Arc, Mutex};

use diffplayerqc_render::{calcular_escala_letterbox, ShaderUniforms, ViewportGpu};
use diffplayerqc_core::CompareMode;
use serde::Deserialize;
use tauri::{
    utils::config::Color, AppHandle, LogicalPosition, LogicalSize, Manager, Position, Size,
    window::{Window, WindowBuilder},
};

#[derive(Debug, Clone, Deserialize)]
pub struct RectViewport {
    /// Posición X en coords lógicas del webview (`getBoundingClientRect`).
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
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

pub struct EstadoViewport {
    gpu: Option<ViewportGpu>,
    vista: VistaCompare,
    ancho_logico: u32,
    alto_logico: u32,
}

impl EstadoViewport {
    pub fn nuevo() -> Self {
        Self {
            gpu: None,
            vista: VistaCompare::default(),
            ancho_logico: 1,
            alto_logico: 1,
        }
    }

    /// Ventana nativa hija de `main` (solo superficie wgpu, sin HTML).
    fn asegurar_overlay(app: &AppHandle) -> tauri::Result<Window> {
        if let Some(w) = app.get_window("viewport") {
            return Ok(w);
        }

        let parent = app.get_window("main").ok_or_else(|| {
            tauri::Error::Anyhow(anyhow::anyhow!("ventana principal no encontrada"))
        })?;

        let overlay = WindowBuilder::new(app, "viewport")
            .title("")
            .decorations(false)
            .transparent(true)
            .skip_taskbar(true)
            .visible(false)
            .background_color(Color(0, 0, 0, 0))
            .parent(&parent)?
            .build()?;

        // Clics pasan al webview (toolbar, timeline, etc.) salvo sobre el canvas.
        let _ = overlay.set_ignore_cursor_events(true);

        Ok(overlay)
    }

    pub fn sincronizar_recto(
        &mut self,
        app: &AppHandle,
        rect: RectViewport,
    ) -> tauri::Result<()> {
        let overlay = Self::asegurar_overlay(app)?;

        let w_log = rect.width.max(1.0);
        let h_log = rect.height.max(1.0);

        if w_log < 8.0 || h_log < 8.0 {
            let _ = overlay.hide();
            return Ok(());
        }

        self.ancho_logico = w_log as u32;
        self.alto_logico = h_log as u32;

        // Hijo de `main`: coords lógicas relativas al área cliente (como getBoundingClientRect).
        overlay.set_position(Position::Logical(LogicalPosition::new(rect.x, rect.y)))?;
        overlay.set_size(Size::Logical(LogicalSize::new(w_log, h_log)))?;

        let scale = app
            .get_webview_window("main")
            .map(|w| w.scale_factor().unwrap_or(1.0))
            .unwrap_or(1.0);
        let w_fis = (w_log * scale).round() as u32;
        let h_fis = (h_log * scale).round() as u32;

        if self.gpu.is_none() {
            let gpu = ViewportGpu::nuevo(overlay.clone(), w_fis.max(1), h_fis.max(1))
                .map_err(|e| tauri::Error::Anyhow(e.into()))?;
            self.gpu = Some(gpu);
        } else if let Some(gpu) = &mut self.gpu {
            gpu.redimensionar(w_fis.max(1), h_fis.max(1));
        }

        overlay.show()?;

        Ok(())
    }

    pub fn establecer_vista(&mut self, vista: VistaCompare) {
        self.vista = vista;
    }

    fn actualizar_uniforms(&mut self) {
        let Some(gpu) = &mut self.gpu else {
            return;
        };
        let vid_w = gpu.renderer.tex_a.width.max(gpu.renderer.tex_b.width);
        let vid_h = gpu.renderer.tex_a.height.max(gpu.renderer.tex_b.height);
        let (su, sv) = calcular_escala_letterbox(
            self.ancho_logico as f32,
            self.alto_logico as f32,
            vid_w,
            vid_h,
            self.vista.modo,
        );
        gpu.renderer.uniforms = ShaderUniforms::desde_vista(
            self.vista.modo,
            self.vista.diff_mode,
            self.vista.split_pos,
            self.vista.amplifier,
            self.vista.zoom,
            self.vista.pan_u,
            self.vista.pan_v,
            self.vista.split_horizontal,
            su,
            sv,
        );
        gpu.renderer.upload_uniforms(&gpu.queue);
    }

    pub fn subir_frame_a(&mut self, rgba: &[u8], w: u32, h: u32) {
        if let Some(gpu) = &mut self.gpu {
            gpu.renderer
                .update_texture_a(&gpu.device, &gpu.queue, rgba, w, h);
        }
    }

    pub fn subir_frame_b(&mut self, rgba: &[u8], w: u32, h: u32) {
        if let Some(gpu) = &mut self.gpu {
            gpu.renderer
                .update_texture_b(&gpu.device, &gpu.queue, rgba, w, h);
        }
    }

    pub fn presentar(&mut self) {
        self.actualizar_uniforms();
        if let Some(gpu) = &mut self.gpu {
            let _ = gpu.presentar();
        }
    }
}

pub type ViewportCompartido = Arc<Mutex<EstadoViewport>>;

/// Oculta la overlay al salir del workspace Compare.
pub fn ocultar_overlay(app: &AppHandle) -> tauri::Result<()> {
    if let Some(w) = app.get_window("viewport") {
        w.hide()?;
    }
    Ok(())
}

pub fn enviar_en_main<F>(app: &AppHandle, f: F)
where
    F: FnOnce() + Send + 'static,
{
    let app = app.clone();
    let _ = app.run_on_main_thread(move || f());
}
