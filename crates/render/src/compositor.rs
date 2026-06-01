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
    /// Letterbox canal B (cortina / lado derecho o inferior).
    pub scale_u_b: f32,
    pub scale_v_b: f32,
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
            scale_u_b: 1.0,
            scale_v_b: 1.0,
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
        escala_u_a: f32,
        escala_v_a: f32,
        escala_u_b: f32,
        escala_v_b: f32,
    ) -> Self {
        Self {
            split_pos,
            mode: modo as u32,
            diff_mode: diff as u32,
            amplifier,
            zoom: zoom.max(0.001),
            pan_u,
            pan_v,
            scale_u: escala_u_a,
            scale_v: escala_v_a,
            scale_u_b: escala_u_b,
            scale_v_b: escala_v_b,
            split_horizontal: u32::from(split_horizontal),
        }
    }
}

/// Letterbox para un rectángulo concreto (p. ej. mitad izquierda de la cortina).
pub fn calcular_escala_region(
    ancho_region: f32,
    alto_region: f32,
    vid_w: u32,
    vid_h: u32,
) -> (f32, f32) {
    if ancho_region <= 0.0 || alto_region <= 0.0 || vid_w < 2 || vid_h < 2 {
        return (1.0, 1.0);
    }
    let canvas_aspect = ancho_region / alto_region;
    let video_aspect = vid_w as f32 / vid_h as f32;
    if canvas_aspect > video_aspect {
        (canvas_aspect / video_aspect, 1.0)
    } else {
        (1.0, video_aspect / canvas_aspect)
    }
}

/// Escalas A y B según modo. Cortina = mismo encuadre; SideBySide = mitad pantalla.
pub fn calcular_escalas_compare(
    canvas_w: f32,
    canvas_h: f32,
    vid_w: u32,
    vid_h: u32,
    modo: CompareMode,
    _split_pos: f32,
    _split_horizontal: bool,
) -> (f32, f32, f32, f32) {
    match modo {
        CompareMode::SplitScreen | CompareMode::AbsDiff | CompareMode::Heatmap => {
            // Cortina: un solo encuadre; A y B comparten escala (comparación “mismo plano”).
            let (u, v) = calcular_escala_region(canvas_w, canvas_h, vid_w, vid_h);
            (u, v, u, v)
        }
        CompareMode::SideBySide => {
            let (u, v) = calcular_escala_region(canvas_w / 2.0, canvas_h, vid_w, vid_h);
            (u, v, u, v)
        }
    }
}

/// Escala letterbox como en v1 (`sync_uniforms`) — un solo panel a pantalla completa.
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
    calcular_escala_region(cw, ch, vid_w, vid_h)
}

pub struct VideoTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub width: u32,
    pub height: u32,
}

impl VideoTexture {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        Self::new_format(device, width, height, wgpu::TextureFormat::Rgba8UnormSrgb)
    }

    pub fn new_format(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
    ) -> Self {
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
            format,
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
            let format = self.texture.format();
            *self = Self::new_format(device, width, height, format);
        }

        let bytes_por_fila = width * 4;
        let alineacion = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let bytes_por_fila_alineados =
            bytes_por_fila.div_ceil(alineacion) * alineacion;

        let datos_subida: std::borrow::Cow<'_, [u8]> = if bytes_por_fila_alineados == bytes_por_fila {
            std::borrow::Cow::Borrowed(rgba_data)
        } else {
            let esperado = (width as usize) * (height as usize) * 4;
            if rgba_data.len() < esperado {
                log::warn!(
                    "VideoTexture: buffer RGBA demasiado pequeño ({} < {esperado})",
                    rgba_data.len()
                );
                return;
            }
            let mut con_padding = vec![0u8; (bytes_por_fila_alineados * height) as usize];
            for fila in 0..height as usize {
                let origen = fila * bytes_por_fila as usize;
                let destino = fila * bytes_por_fila_alineados as usize;
                con_padding[destino..destino + bytes_por_fila as usize]
                    .copy_from_slice(&rgba_data[origen..origen + bytes_por_fila as usize]);
            }
            std::borrow::Cow::Owned(con_padding)
        };

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &datos_subida,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(bytes_por_fila_alineados),
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

        let tex_a = VideoTexture::new_format(device, 1, 1, wgpu::TextureFormat::Rgba8Unorm);
        let tex_b = VideoTexture::new_format(device, 1, 1, wgpu::TextureFormat::Rgba8Unorm);

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
        let prev_w = self.tex_a.width;
        let prev_h = self.tex_a.height;
        self.tex_a.update(device, queue, rgba, width, height);
        if self.tex_a.width != prev_w || self.tex_a.height != prev_h {
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
        let prev_w = self.tex_b.width;
        let prev_h = self.tex_b.height;
        self.tex_b.update(device, queue, rgba, width, height);
        if self.tex_b.width != prev_w || self.tex_b.height != prev_h {
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

/// Genera un gradiente RGBA de prueba (diagnóstico cuando no hay frames del decoder).
pub fn generar_patron_prueba(ancho: u32, alto: u32) -> Vec<u8> {
    let mut datos = vec![0u8; (ancho as usize) * (alto as usize) * 4];
    for y in 0..alto {
        for x in 0..ancho {
            let i = ((y * ancho + x) * 4) as usize;
            datos[i] = (x.saturating_mul(255) / ancho.max(1)) as u8;
            datos[i + 1] = (y.saturating_mul(255) / alto.max(1)) as u8;
            datos[i + 2] = 180;
            datos[i + 3] = 255;
        }
    }
    datos
}

/// Pipeline mínimo A→pantalla (sin letterbox; evita pantalla negra por `border=0`).
pub struct ViewportBlitRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    pub tex_a: VideoTexture,
    pub tex_b: VideoTexture,
    bind_group: wgpu::BindGroup,
}

impl ViewportBlitRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viewport_blit_shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../../shaders/viewport_blit.wgsl").into(),
            ),
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("viewport_blit_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let tex_a = VideoTexture::new_format(device, 1, 1, wgpu::TextureFormat::Rgba8Unorm);
        let tex_b = VideoTexture::new_format(device, 1, 1, wgpu::TextureFormat::Rgba8Unorm);
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("viewport_blit_layout"),
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
            ],
        });
        let bind_group = Self::crear_bind_group(device, &bind_group_layout, &tex_a, &tex_b, &sampler);
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viewport_blit_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viewport_blit_pipeline"),
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
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        Self {
            pipeline,
            bind_group_layout,
            sampler,
            tex_a,
            tex_b,
            bind_group,
        }
    }

    fn crear_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        tex_a: &VideoTexture,
        tex_b: &VideoTexture,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport_blit_bind_group"),
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
            ],
        })
    }

    pub fn update_texture_a(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) {
        let prev_w = self.tex_a.width;
        let prev_h = self.tex_a.height;
        self.tex_a.update(device, queue, rgba, width, height);
        if self.tex_a.width != prev_w || self.tex_a.height != prev_h {
            self.bind_group = Self::crear_bind_group(
                device,
                &self.bind_group_layout,
                &self.tex_a,
                &self.tex_b,
                &self.sampler,
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
        let prev_w = self.tex_b.width;
        let prev_h = self.tex_b.height;
        self.tex_b.update(device, queue, rgba, width, height);
        if self.tex_b.width != prev_w || self.tex_b.height != prev_h {
            self.bind_group = Self::crear_bind_group(
                device,
                &self.bind_group_layout,
                &self.tex_a,
                &self.tex_b,
                &self.sampler,
            );
        }
    }

    pub fn dibujar_en_pass<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>) {
        rp.set_pipeline(&self.pipeline);
        rp.set_bind_group(0, &self.bind_group, &[]);
        rp.draw(0..3, 0..1);
    }
}

/// Viewport wgpu acoplado a una ventana nativa (Tauri overlay) con `compare.wgsl`.
pub struct ViewportGpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    pub     compare: VideoRenderer,
    ancho: u32,
    alto: u32,
    /// Primera presentación tras crear/redimensionar: clear en lugar de load.
    limpiar_siguiente: bool,
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

        // Ventana opaca: en macOS `Premultiplied` + NSWindow transparente deja el swapchain invisible.
        let alpha_mode = caps
            .alpha_modes
            .iter()
            .copied()
            .find(|m| *m == wgpu::CompositeAlphaMode::Opaque)
            .or_else(|| {
                caps.alpha_modes
                    .iter()
                    .copied()
                    .find(|m| *m == wgpu::CompositeAlphaMode::PostMultiplied)
            })
            .unwrap_or(caps.alpha_modes[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: ancho.max(1),
            height: alto.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let compare = VideoRenderer::new(&device, format);

        Ok(Self {
            device,
            queue,
            surface,
            config,
            compare,
            ancho,
            alto,
            limpiar_siguiente: true,
        })
    }

    /// Uniforms del shader (modo cortina, diff, letterbox).
    pub fn actualizar_uniformes_vista(
        &mut self,
        modo: CompareMode,
        diff: DiffMode,
        split_pos: f32,
        amplifier: f32,
        zoom: f32,
        pan_u: f32,
        pan_v: f32,
        split_horizontal: bool,
        vid_ancho: u32,
        vid_alto: u32,
    ) {
        let (ua, va, ub, vb) = calcular_escalas_compare(
            self.ancho as f32,
            self.alto as f32,
            vid_ancho,
            vid_alto,
            modo,
            split_pos,
            split_horizontal,
        );
        self.compare.uniforms = ShaderUniforms::desde_vista(
            modo,
            diff,
            split_pos,
            amplifier,
            zoom,
            pan_u,
            pan_v,
            split_horizontal,
            ua,
            va,
            ub,
            vb,
        );
        self.compare.upload_uniforms(&self.queue);
    }

    pub fn subir_textura_a(&mut self, rgba: &[u8], width: u32, height: u32) {
        self.compare
            .update_texture_a(&self.device, &self.queue, rgba, width, height);
    }

    pub fn subir_textura_b(&mut self, rgba: &[u8], width: u32, height: u32) {
        self.compare
            .update_texture_b(&self.device, &self.queue, rgba, width, height);
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
        self.limpiar_siguiente = true;
    }

    pub fn presentar(&mut self) -> anyhow::Result<()> {
        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                self.surface.get_current_texture()?
            }
            Err(e) => return Err(e.into()),
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("viewport_encoder"),
            });

        {
            let load_op = if self.limpiar_siguiente {
                self.limpiar_siguiente = false;
                wgpu::LoadOp::Clear(wgpu::Color::BLACK)
            } else {
                wgpu::LoadOp::Load
            };
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("viewport_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: load_op,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            self.compare.dibujar_en_pass(&mut rp);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    /// Avanza uploads pendientes antes del render pass.
    pub fn avanzar_colas(&self) {
        self.device.poll(wgpu::Maintain::Poll);
    }
}
