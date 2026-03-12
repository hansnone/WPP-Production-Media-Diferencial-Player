// renderer.rs — wgpu render pipeline for DiffPlayerQC

use bytemuck::{Pod, Zeroable};
use egui_wgpu::wgpu;
use wgpu::util::DeviceExt;
// No unnecessary crate imports here

// ---------------------------------------------------------------------------
// Uniform buffer layout (must match compare.wgsl)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ShaderUniforms {
    /// 0.0–1.0 curtain split position (Split-Screen mode)
    pub split_pos: f32,
    /// Compare mode: 0=SplitScreen, 1=AbsDiff, 2=Heatmap, 3=SideBySide
    pub mode: u32,
    /// Subtraction mode inside AbsDiff: 0=LegacyAbs, 1=AbsLinear, 2=AbsSqrt, 3=SignedDiverging
    pub diff_mode: u32,
    /// Error amplifier for heatmap mode (1.0–50.0)
    pub amplifier: f32,

    /// Current zoom level (>1.0 = zoomed in)
    pub zoom: f32,
    /// UV pan offsets
    pub pan_u: f32,
    pub pan_v: f32,
    /// Aspect ratio letterbox scales
    pub scale_u: f32,

    pub scale_v: f32,
    pub bg_color: [f32; 3],
}

impl Default for ShaderUniforms {
    fn default() -> Self {
        Self {
            split_pos: 0.5,
            mode: 0,
            diff_mode: 1,
            amplifier: 5.0,
            zoom: 1.0,
            pan_u: 0.0,
            pan_v: 0.0,
            scale_u: 1.0,
            scale_v: 1.0,
            bg_color: [0.0, 0.0, 0.0],
        }
    }
}

// ---------------------------------------------------------------------------
// VideoTexture — owns a wgpu texture and the view/sampler for one video
// ---------------------------------------------------------------------------

pub struct VideoTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub width: u32,
    pub height: u32,
}

impl VideoTexture {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("video_texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            texture,
            view,
            width,
            height,
        }
    }

    /// Upload new RGBA pixel data. Recreates the texture if dimensions changed.
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba_data: &[u8],
        width: u32,
        height: u32,
    ) {
        // Recreate texture if size changed
        if self.width != width || self.height != height {
            *self = Self::new(device, width, height);
        }

        let bytes_per_row = width * 4;
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }
}

// ---------------------------------------------------------------------------
// VideoRenderer — the egui_wgpu::CallbackTrait implementation
// ---------------------------------------------------------------------------

pub struct VideoRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
    pub sampler: wgpu::Sampler,
    pub tex_a: VideoTexture,
    pub tex_b: VideoTexture,
    pub bind_group: wgpu::BindGroup,
    pub uniforms: ShaderUniforms,
}

impl VideoRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        // Load WGSL shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compare_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/compare.wgsl").into()),
        });

        // Uniform buffer
        let uniforms = ShaderUniforms::default();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Sampler — linear filtering for sub-pixel zoom
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("video_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Placeholder 1×1 textures
        let tex_a = VideoTexture::new(device, 1, 1);
        let tex_b = VideoTexture::new(device, 1, 1);

        // Bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compare_bind_group_layout"),
            entries: &[
                // binding 0: texture A
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // binding 1: texture B
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // binding 2: sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // binding 3: uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = make_bind_group(
            device,
            &bind_group_layout,
            &tex_a,
            &tex_b,
            &sampler,
            &uniform_buffer,
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compare_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("compare_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[], // fullscreen triangle, no vertex buffer
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            pipeline,
            bind_group_layout,
            uniform_buffer,
            sampler,
            tex_a,
            tex_b,
            bind_group,
            uniforms,
        }
    }

    /// Upload new RGBA data for channel A.
    pub fn update_texture_a(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) {
        let size_changed = self.tex_a.width != width || self.tex_a.height != height;
        self.tex_a.update(device, queue, rgba, width, height);
        if size_changed {
            self.bind_group = make_bind_group(
                device,
                &self.bind_group_layout,
                &self.tex_a,
                &self.tex_b,
                &self.sampler,
                &self.uniform_buffer,
            );
        }
    }

    /// Upload new RGBA data for channel B.
    pub fn update_texture_b(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) {
        let size_changed = self.tex_b.width != width || self.tex_b.height != height;
        self.tex_b.update(device, queue, rgba, width, height);
        if size_changed {
            self.bind_group = make_bind_group(
                device,
                &self.bind_group_layout,
                &self.tex_a,
                &self.tex_b,
                &self.sampler,
                &self.uniform_buffer,
            );
        }
    }

    /// Write uniforms to GPU buffer.
    pub fn upload_uniforms(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&self.uniforms));
    }
}

// egui_wgpu Callback trait integration
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
        let rend = self.renderer.lock();

        // SAFETY: We are recording commands into the RenderPass which will be submitted immediately.
        // The VideoRenderer (and its pipeline/bind_group) is kept alive by the Arc in RenderCallback.
        unsafe {
            let rp: &mut wgpu::RenderPass<'a> = std::mem::transmute(render_pass);
            rp.set_pipeline(std::mem::transmute(&rend.pipeline));
            rp.set_bind_group(0, std::mem::transmute(&rend.bind_group), &[]);
            rp.draw(0..3, 0..1);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    tex_a: &VideoTexture,
    tex_b: &VideoTexture,
    sampler: &wgpu::Sampler,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("compare_bind_group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&tex_a.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&tex_b.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
    })
}
