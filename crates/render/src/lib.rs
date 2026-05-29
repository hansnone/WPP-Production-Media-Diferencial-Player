//! Compositor wgpu para comparación A/B (`compare.wgsl`).

mod compositor;

pub use compositor::{
    calcular_escala_letterbox, ShaderUniforms, VideoRenderer, VideoTexture, ViewportGpu,
};
