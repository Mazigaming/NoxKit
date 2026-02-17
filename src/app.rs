use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use crate::view::View;
use crate::layout::LayoutContext;
use crate::render::RenderContext;
use std::sync::Arc;

pub struct App {
    view: Box<dyn View>,
    state: AppState,
}

enum AppState {
    Idle,
    Running {
        window: Arc<Window>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: wgpu::Surface<'static>,
    },
}

impl App {
    pub fn new(view: Box<dyn View>) -> Self {
        Self {
            view,
            state: AppState::Idle,
        }
    }

    pub fn run(mut self) {
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
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )).expect("Failed to create device");

            let config = surface.get_default_config(&adapter, window.inner_size().width, window.inner_size().height).unwrap();
            surface.configure(&device, &config);

            self.state = AppState::Running {
                window,
                device,
                queue,
                surface,
            };
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                if let AppState::Running { device, queue, surface, .. } = &self.state {
                    let mut _layout_ctx = LayoutContext::new();
                    let mut _render_ctx = RenderContext::new(device.clone(), queue.clone());
                    
                    self.view.layout(&mut _layout_ctx);
                    
                    let frame = surface.get_current_texture().unwrap();
                    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    
                    {
                        let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });
                        
                        // self.view.render(&mut _render_ctx); // Need to pass rpass or similar
                    }

                    queue.submit(Some(encoder.finish()));
                    frame.present();
                }
            }
            _ => (),
        }
    }
}
