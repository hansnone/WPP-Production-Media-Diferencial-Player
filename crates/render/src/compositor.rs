//! Pipeline wgpu compartido entre egui (v1) y viewport Tauri (v2).

use bytemuck::{Pod, Zeroable};
use diffplayerqc_core::{CompareMode, DiffMode};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ShaderUniforms {
    pub split_pos: f32,
    pub mode: u32,
    pub diff_mode: u32,
    pub amplifier: f32,
    pub zoom: f32,
    pub pan_u: f32,
    pub pan_v: f32,
    pub scale_u: f32,
    pub scale_v: f32,
    pub bg_color: [f32; 3],
    pub split_horizontal: u32,
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
            split_horizontal: 0,
        }
    }
}

impl ShaderUniforms {
    pub fn desde_vista(
        modo: CompareMode,
        diff: DiffMode,
        split_pos: f32,
        amplifier: f32,
        zoom: f32,
        pan_u: f32,
        pan_v: f32,
        split_horizontal: bool,
        escala_u: f32,
        escala_v: f32,
    ) -> Self {
        Self {
            split_pos,
            mode: modo as u32,
            diff_mode: diff as u32,
            amplifier,
            zoom,
            pan_u,
            pan_v,
            scale_u: escala_u,
            scale_v: escala_v,
            bg_color: [0.0, 0.0, 0.0],
            split_horizontal: u32::from(split_horizontal),
        }
    }
}

/// Escala letterbox como en v1 (`sync_uniforms`).
pub fn calcular_escala_letterbox(
    canvas_w: f32,
    canvas_h: f32,
    vid_w: u32,
    vid_h: u32,
    modo: CompareMode,
) -> (f32, f32) {
    let mut cw = canvas_w;
    let ch = canvas_h;
    if modo == CompareMode::SideBySide {
        cw /= 2.0;
    }
    if cw <= 0.0 || ch <= 0.0 || vid_w == 0 || vid_h == 0 {
        return (1.0, 1.0);
    }
    let canvas_aspect = cw / ch;
    let video_aspect = vid_w as f32 / vid_h as f32;
    if canvas_aspect > video_aspect {
        (canvas_aspect / video_aspect, 1.0)
    } else {
        (1.0, video_aspect / canvas_aspect)
    }
}

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

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba_data: &[u8],
        width: u32,
        height: u32,
    ) {
        if self.width != width || self.height != height {
            *self = Self::new(device, width, height);
        }
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
                bytes_per_row: Some(width * 4),
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
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compare_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../shaders/compare.wgsl").into()),
        });

        let uniforms = ShaderUniforms::default();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

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

        let tex_a = VideoTexture::new(device, 1, 1);
        let tex_b = VideoTexture::new(device, 1, 1);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compare_bind_group_layout"),
            entries: &[
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
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
                buffers: &[],
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

    pub fn upload_uniforms(&self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.uniforms),
        );
    }

    pub fn dibujar_en_pass<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_pipeline(&self.pipeline);
        rp.set_bind_group(0, &self.bind_group, &[]);
        rp.draw(0..3, 0..1);
    }
}

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

/// Viewport wgpu acoplado a una ventana nativa (Tauri overlay).
pub struct ViewportGpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    pub renderer: VideoRenderer,
    ancho: u32,
    alto: u32,
}

impl ViewportGpu {
    pub fn nuevo(
        window: impl wgpu::WindowHandle + 'static,
        ancho: u32,
        alto: u32,
    ) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window)?;
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or_else(|| anyhow::anyhow!("sin adaptador wgpu"))?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("diffplayerqc_viewport"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ))?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: ancho.max(1),
            height: alto.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let renderer = VideoRenderer::new(&device, format);

        Ok(Self {
            device,
            queue,
            surface,
            config,
            renderer,
            ancho,
            alto,
        })
    }

    pub fn redimensionar(&mut self, ancho: u32, alto: u32) {
        if ancho == 0 || alto == 0 {
            return;
        }
        if self.ancho == ancho && self.alto == alto {
            return;
        }
        self.ancho = ancho;
        self.alto = alto;
        self.config.width = ancho;
        self.config.height = alto;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn presentar(&mut self) -> anyhow::Result<()> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("viewport_encoder"),
            });

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("viewport_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            self.renderer.dibujar_en_pass(&mut rp);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }
}
