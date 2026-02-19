use bytemuck::{Pod, Zeroable};
use glyphon::{
    FontSystem, SwashCache, TextRenderer, TextAtlas, Cache, Viewport,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub rect_pos: [f32; 2],
    pub rect_size: [f32; 2],
    pub corner_radius: f32,
    pub shape_type: f32, // 0: rect, 1: rounded rect, 2: circle
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 40,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 44,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

pub struct RenderQueue {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            vertices: Vec::with_capacity(1024),
            indices: Vec::with_capacity(1536),
        }
    }

    pub fn push_rect(&mut self, geometry: crate::view::Geometry, color: [f32; 4]) {
        self.push_raw(geometry, color, 0.0, 0.0);
    }

    pub fn push_rounded_rect(&mut self, geometry: crate::view::Geometry, color: [f32; 4], radius: f32) {
        self.push_raw(geometry, color, radius, 1.0);
    }

    pub fn push_circle(&mut self, geometry: crate::view::Geometry, color: [f32; 4]) {
        self.push_raw(geometry, color, 0.0, 2.0);
    }

    fn push_raw(&mut self, geometry: crate::view::Geometry, color: [f32; 4], radius: f32, shape: f32) {
        let x = geometry.x;
        let y = geometry.y;
        let w = geometry.width;
        let h = geometry.height;
        let start_index = self.vertices.len() as u16;

        let rect_pos = [x, y];
        let rect_size = [w, h];

        self.vertices.extend_from_slice(&[
            Vertex { position: [x, y], color, rect_pos, rect_size, corner_radius: radius, shape_type: shape },
            Vertex { position: [x + w, y], color, rect_pos, rect_size, corner_radius: radius, shape_type: shape },
            Vertex { position: [x, y + h], color, rect_pos, rect_size, corner_radius: radius, shape_type: shape },
            Vertex { position: [x + w, y + h], color, rect_pos, rect_size, corner_radius: radius, shape_type: shape },
        ]);

        self.indices.extend_from_slice(&[
            start_index, start_index + 1, start_index + 2,
            start_index + 2, start_index + 1, start_index + 3,
        ]);
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}

pub struct RenderContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub render_queue: RenderQueue,
    
    // Text rendering
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub text_atlas: TextAtlas,
    pub text_renderer: TextRenderer,
    pub viewport: Viewport,
    pub debug_buffer: glyphon::Buffer,
    pub debug: bool,
}

impl RenderContext {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/view.wgsl"));

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Pre-allocate buffers for batching (large enough for most UIs)
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Batch Vertex Buffer"),
            size: (std::mem::size_of::<Vertex>() * 16384) as u64, // 16384 vertices
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Batch Index Buffer"),
            size: (std::mem::size_of::<u16>() * 24576) as u64, // 24576 indices (4096 quads)
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Initialize glyphon
        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let mut text_atlas = TextAtlas::new(&device, &queue, &cache, surface_config.format);
        let text_renderer = TextRenderer::new(
            &mut text_atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );
        let viewport = Viewport::new(&device, &cache);
        let debug_buffer = glyphon::Buffer::new(&mut font_system, glyphon::Metrics::new(14.0, 20.0));

        Self {
            device,
            queue,
            pipeline,
            bind_group,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
            render_queue: RenderQueue::new(),
            font_system,
            swash_cache,
            text_atlas,
            text_renderer,
            viewport,
            debug_buffer,
            debug: true,
        }
    }
}
