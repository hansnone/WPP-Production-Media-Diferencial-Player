//! Compositor wgpu para comparación A/B (`compare.wgsl`).

mod compositor;

pub use compositor::{
    calcular_escala_letterbox, calcular_escala_region, calcular_escalas_compare,
    generar_patron_prueba, ShaderUniforms, VideoRenderer, VideoTexture, ViewportBlitRenderer,
    ViewportGpu,
};
