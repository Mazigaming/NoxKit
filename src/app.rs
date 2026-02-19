use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use winit::event::{WindowEvent, ElementState, MouseButton};
use crate::view::{View, Geometry, Event};
use crate::layout::LayoutContext;
use crate::render::RenderContext;
use std::sync::Arc;
use glam::Mat4;

pub struct App {
    view: Box<dyn View>,
    state: AppState,
    dirty: bool,
    last_frame: std::time::Instant,
    fps: f32,
}

enum AppState {
    Idle,
    Running {
        window: Arc<Window>,
        surface: wgpu::Surface<'static>,
        adapter: wgpu::Adapter,
        render_ctx: RenderContext,
        cursor_pos: (f32, f32),
        layout_ctx: LayoutContext,
        root_node: Option<taffy::prelude::NodeId>,
    },
}

impl App {
    pub fn new(view: Box<dyn View>) -> Self {
        Self {
            view,
            state: AppState::Idle,
            dirty: true,
            last_frame: std::time::Instant::now(),
            fps: 0.0,
        }
    }

    fn update_layout(view: &mut Box<dyn View>, layout_ctx: &mut LayoutContext, size: winit::dpi::PhysicalSize<u32>) -> taffy::prelude::NodeId {
        let root_node = view.layout(layout_ctx);
        layout_ctx.taffy.compute_layout(
            root_node,
            taffy::prelude::Size {
                width: taffy::prelude::AvailableSpace::Definite(size.width as f32),
                height: taffy::prelude::AvailableSpace::Definite(size.height as f32),
            },
        ).unwrap();
        root_node
    }

    pub fn run(mut self) {
        tracing_subscriber::fmt::init();
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut self).unwrap();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let AppState::Idle = self.state {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
            
            let instance = wgpu::Instance::default();
            let surface = instance.create_surface(window.clone()).unwrap();
            
            let adapter = pollster::block_on(instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                }
            )).expect("Failed to find an appropriate adapter");

            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    ..Default::default()
                }
            )).expect("Failed to create device");

            let config = surface.get_default_config(&adapter, window.inner_size().width, window.inner_size().height).unwrap();
            surface.configure(&device, &config);

            let render_ctx = RenderContext::new(device, queue, &config);

            self.view.on_init();
            self.view.on_mount();

            let mut layout_ctx = LayoutContext::new();
            let root_node = Self::update_layout(&mut self.view, &mut layout_ctx, window.inner_size());

            self.state = AppState::Running {
                window,
                surface,
                adapter,
                render_ctx,
                cursor_pos: (0.0, 0.0),
                layout_ctx,
                root_node: Some(root_node),
            };
            self.dirty = true;
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let AppState::Running { surface, adapter, render_ctx, layout_ctx, root_node, .. } = &mut self.state {
                    if size.width > 0 && size.height > 0 {
                        let config = surface.get_default_config(adapter, size.width, size.height).unwrap();
                        surface.configure(&render_ctx.device, &config);
                        *root_node = Some(Self::update_layout(&mut self.view, layout_ctx, size));
                        self.dirty = true;
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let AppState::Running { window, cursor_pos, layout_ctx, .. } = &mut self.state {
                    *cursor_pos = (position.x as f32, position.y as f32);
                    let size = window.inner_size();
                    let root_geometry = Geometry {
                        x: 0.0,
                        y: 0.0,
                        width: size.width as f32,
                        height: size.height as f32,
                    };
                    
                    let ev = Event::MouseMove { x: cursor_pos.0, y: cursor_pos.1 };
                    self.view.handle_event(&ev, &layout_ctx, root_geometry);
                    
                    self.dirty = true;
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                if let AppState::Running { window, cursor_pos, layout_ctx, .. } = &mut self.state {
                    let size = window.inner_size();
                    let root_geometry = Geometry {
                        x: 0.0,
                        y: 0.0,
                        width: size.width as f32,
                        height: size.height as f32,
                    };

                    let ev = if let ElementState::Pressed = state {
                        Event::MouseDown { x: cursor_pos.0, y: cursor_pos.1 }
                    } else {
                        Event::MouseUp { x: cursor_pos.0, y: cursor_pos.1 }
                    };
                    self.view.handle_event(&ev, &layout_ctx, root_geometry);
                    
                    if let ElementState::Pressed = state {
                        let ev_click = Event::MouseClick { x: cursor_pos.0, y: cursor_pos.1 };
                        self.view.handle_event(&ev_click, &layout_ctx, root_geometry);
                    }
                    
                    self.dirty = true;
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if !self.dirty { return; }
                if let AppState::Running { window, surface, render_ctx, layout_ctx, .. } = &mut self.state {
                    let size = window.inner_size();
                    let root_geometry = Geometry {
                        x: 0.0,
                        y: 0.0,
                        width: size.width as f32,
                        height: size.height as f32,
                    };

                    // 1. Lifecycle Update
                    self.view.on_update();

                    // FPS calculation
                    let now = std::time::Instant::now();
                    let dt = now.duration_since(self.last_frame).as_secs_f32();
                    self.last_frame = now;
                    if dt > 0.0 {
                        self.fps = 0.9 * self.fps + 0.1 * (1.0 / dt);
                    }

                    // 2. Clear render queue
                    render_ctx.render_queue.clear();

                    // 3. Prepare (Collect primitives and text)
                    self.view.prepare(render_ctx, &layout_ctx, root_geometry);

                    let mut text_areas = Vec::new();
                    self.view.collect_text_areas(&layout_ctx, root_geometry, &mut text_areas);

                    // Add FPS debug text
                    if render_ctx.debug {
                        let fps_text = format!("FPS: {:.1}", self.fps);
                        render_ctx.debug_buffer.set_text(&mut render_ctx.font_system, &fps_text, &glyphon::Attrs::new().family(glyphon::Family::Monospace).color(glyphon::Color::rgb(0, 255, 0)), glyphon::Shaping::Advanced);
                        render_ctx.debug_buffer.set_size(&mut render_ctx.font_system, Some(100.0), Some(20.0));
                        render_ctx.debug_buffer.shape_until_scroll(&mut render_ctx.font_system, false);

                        text_areas.push(glyphon::TextArea {
                            buffer: &render_ctx.debug_buffer,
                            left: 10.0,
                            top: 10.0,
                            scale: 1.0,
                            bounds: glyphon::TextBounds {
                                left: 0,
                                top: 0,
                                right: size.width as i32,
                                bottom: size.height as i32,
                            },
                            default_color: glyphon::Color::rgb(255, 255, 255),
                            custom_glyphs: &[],
                        });
                    }

                    // 4. Update viewport and prepare text renderer
                    render_ctx.viewport.update(&render_ctx.queue, glyphon::Resolution {
                        width: size.width,
                        height: size.height,
                    });
                    render_ctx.text_renderer.prepare(
                        &render_ctx.device,
                        &render_ctx.queue,
                        &mut render_ctx.font_system,
                        &mut render_ctx.text_atlas,
                        &render_ctx.viewport,
                        text_areas,
                        &mut render_ctx.swash_cache,
                    ).unwrap();

                    // 5. Render
                    let projection = Mat4::orthographic_lh(0.0, size.width as f32, size.height as f32, 0.0, -1.0, 1.0);
                    render_ctx.queue.write_buffer(&render_ctx.uniform_buffer, 0, bytemuck::cast_slice(&projection.to_cols_array_2d()));

                    let frame = surface.get_current_texture().unwrap();
                    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = render_ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    
                    {
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.01, // Near-black for modern look
                                        g: 0.01,
                                        b: 0.02,
                                        a: 1.0,
                                    }),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });
                        
                        rpass.set_pipeline(&render_ctx.pipeline);
                        rpass.set_bind_group(0, &render_ctx.bind_group, &[]);
                        
                        // Render batched primitives from queue
                        if !render_ctx.render_queue.vertices.is_empty() {
                            let v_len = render_ctx.render_queue.vertices.len();
                            let i_len = render_ctx.render_queue.indices.len();
                            
                            // Safety check to avoid write_buffer overflow
                            let v_data = &render_ctx.render_queue.vertices[..v_len.min(16384)];
                            let i_data = &render_ctx.render_queue.indices[..i_len.min(24576)];

                            render_ctx.queue.write_buffer(&render_ctx.vertex_buffer, 0, bytemuck::cast_slice(v_data));
                            render_ctx.queue.write_buffer(&render_ctx.index_buffer, 0, bytemuck::cast_slice(i_data));
                            
                            rpass.set_vertex_buffer(0, render_ctx.vertex_buffer.slice(..));
                            rpass.set_index_buffer(render_ctx.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                            rpass.draw_indexed(0..i_data.len() as u32, 0, 0..1);
                        }

                        // Render widgets (for nested renders if any, though most now use queue)
                        self.view.render(render_ctx, &mut rpass, root_geometry);

                        // Render text
                        render_ctx.text_renderer.render(&render_ctx.text_atlas, &render_ctx.viewport, &mut rpass).unwrap();
                    }

                    render_ctx.queue.submit(Some(encoder.finish()));
                    frame.present();
                    self.dirty = false;
                }
            }
            _ => (),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.view.on_unmount();
    }
}
