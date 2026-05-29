//! Integración egui/wgpu: re-export del compositor y callback de pintado.

pub use diffplayerqc_render::{ShaderUniforms, VideoRenderer};

use egui_wgpu::wgpu;

pub struct RenderCallback {
    pub renderer: std::sync::Arc<parking_lot::Mutex<VideoRenderer>>,
}

impl egui_wgpu::CallbackTrait for RenderCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        _callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let rend = self.renderer.lock();
        rend.upload_uniforms(queue);
        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        _callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        // SAFETY: el guard del Mutex vive todo el callback `paint`; el pass egui comparte ese scope.
        let rend = self.renderer.lock();
        unsafe {
            let rend_estatico: &'a VideoRenderer = std::mem::transmute(&*rend);
            rend_estatico.dibujar_en_pass(render_pass);
        }
    }
}
