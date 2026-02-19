use crate::view::{View, Geometry, Event};
use crate::layout::LayoutContext;
use crate::render::RenderContext;
use taffy::prelude::*;

pub struct Column {
    pub children: Vec<Box<dyn View>>,
    node_id: Option<NodeId>,
}

impl Column {
    pub fn new(children: Vec<Box<dyn View>>) -> Self {
        Self { children, node_id: None }
    }
}

fn render_outline_helper(ctx: &mut RenderContext, geometry: Geometry, color: [f32; 4]) {
    let thickness = 1.0;
    let x = geometry.x;
    let y = geometry.y;
    let w = geometry.width;
    let h = geometry.height;

    ctx.render_queue.push_rect(Geometry { x, y, width: w, height: thickness }, color);
    ctx.render_queue.push_rect(Geometry { x, y: y + h - thickness, width: w, height: thickness }, color);
    ctx.render_queue.push_rect(Geometry { x, y, width: thickness, height: h }, color);
    ctx.render_queue.push_rect(Geometry { x: x + w - thickness, y, width: thickness, height: h }, color);
}

impl View for Column {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        let child_nodes: Vec<NodeId> = self.children.iter_mut()
            .map(|child| child.layout(ctx))
            .collect();
        
        let node = ctx.taffy.new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: Some(AlignItems::Center), // Material-like centering
                justify_content: Some(JustifyContent::Start),
                size: Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Percent(1.0),
                },
                padding: taffy::prelude::Rect {
                    left: length(16.0),
                    right: length(16.0),
                    top: length(24.0),
                    bottom: length(24.0),
                },
                gap: Size {
                    width: length(0.0),
                    height: length(16.0), // More breathing room
                },
                ..Default::default()
            },
            &child_nodes,
        ).unwrap();
        self.node_id = Some(node);
        node
    }

    fn prepare(&mut self, ctx: &mut RenderContext, layout_ctx: &LayoutContext, geometry: Geometry) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };

        for child in self.children.iter_mut() {
            child.prepare(ctx, layout_ctx, my_geo);
        }
        
        if ctx.debug {
            render_outline_helper(ctx, my_geo, [1.0, 0.0, 0.0, 1.0]);
        }
    }

    fn collect_text_areas<'a>(&'a self, layout_ctx: &LayoutContext, geometry: Geometry, areas: &mut Vec<glyphon::TextArea<'a>>) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };

        for child in self.children.iter() {
            child.collect_text_areas(layout_ctx, my_geo, areas);
        }
    }

    fn render<'rp>(&'rp self, ctx: &'rp RenderContext, render_pass: &mut wgpu::RenderPass<'rp>, geometry: Geometry) {
        for child in self.children.iter() {
            child.render(ctx, render_pass, geometry);
        }
    }

    fn handle_event(&mut self, event: &Event, layout_ctx: &LayoutContext, geometry: Geometry) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };

        for child in self.children.iter_mut() {
            child.handle_event(event, layout_ctx, my_geo);
        }
    }

    fn on_init(&mut self) {
        for child in &mut self.children {
            child.on_init();
        }
    }

    fn on_mount(&mut self) {
        for child in &mut self.children {
            child.on_mount();
        }
    }

    fn on_update(&mut self) {
        for child in &mut self.children {
            child.on_update();
        }
    }

    fn on_unmount(&mut self) {
        for child in &mut self.children {
            child.on_unmount();
        }
    }
}

pub struct Text {
    pub text: String,
    pub font_size: f32,
    buffer: Option<glyphon::Buffer>,
    node_id: Option<NodeId>,
    last_text: Option<String>,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self { 
            text: text.into(),
            font_size: 16.0,
            buffer: None,
            node_id: None,
            last_text: None,
        }
    }
}

impl View for Text {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        let node = ctx.taffy.new_leaf(Style::default()).unwrap();
        self.node_id = Some(node);
        node
    }

    fn prepare(&mut self, ctx: &mut RenderContext, layout_ctx: &LayoutContext, geometry: Geometry) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };

        if self.buffer.is_none() {
            // Material/Android standard: 16dp text, 24dp line height
            self.buffer = Some(glyphon::Buffer::new(&mut ctx.font_system, glyphon::Metrics::new(self.font_size, self.font_size * 1.5)));
        }
        
        let buffer = self.buffer.as_mut().unwrap();
        
        if self.last_text.as_ref() != Some(&self.text) {
            buffer.set_text(&mut ctx.font_system, &self.text, &glyphon::Attrs::new().family(glyphon::Family::SansSerif), glyphon::Shaping::Advanced);
            buffer.set_size(&mut ctx.font_system, Some(my_geo.width), Some(my_geo.height));
            buffer.shape_until_scroll(&mut ctx.font_system, false);
            self.last_text = Some(self.text.clone());
        }

        if ctx.debug {
            render_outline_helper(ctx, my_geo, [0.0, 1.0, 0.0, 1.0]);
        }
    }

    fn collect_text_areas<'a>(&'a self, layout_ctx: &LayoutContext, geometry: Geometry, areas: &mut Vec<glyphon::TextArea<'a>>) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };

        if let Some(buffer) = &self.buffer {
            areas.push(glyphon::TextArea {
                buffer,
                left: my_geo.x,
                top: my_geo.y,
                scale: 1.0,
                bounds: glyphon::TextBounds {
                    left: my_geo.x as i32,
                    top: my_geo.y as i32,
                    right: (my_geo.x + my_geo.width) as i32,
                    bottom: (my_geo.y + my_geo.height) as i32,
                },
                default_color: glyphon::Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            });
        }
    }

    fn render<'rp>(&'rp self, _ctx: &'rp RenderContext, _render_pass: &mut wgpu::RenderPass<'rp>, _geometry: Geometry) {
    }

    fn handle_event(&mut self, _event: &Event, _layout_ctx: &LayoutContext, _geometry: Geometry) {
    }
}

pub struct Button {
    pub text: String,
    pub on_click: Box<dyn FnMut()>,
    text_view: Text,
    node_id: Option<NodeId>,
    hovered: bool,
    pressed: bool,
}

impl Button {
    pub fn new(text: impl Into<String>, on_click: impl FnMut() + 'static) -> Self {
        let t = text.into();
        let mut text_view = Text::new(t.clone());
        text_view.font_size = 14.0; // Material buttons often use slightly smaller text
        Self {
            text: t,
            on_click: Box::new(on_click),
            text_view,
            node_id: None,
            hovered: false,
            pressed: false,
        }
    }
}

impl View for Button {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        let text_node = self.text_view.layout(ctx);
        let node = ctx.taffy.new_with_children(
            Style {
                padding: taffy::prelude::Rect {
                    left: length(24.0),
                    right: length(24.0),
                    top: length(10.0),
                    bottom: length(10.0),
                },
                justify_content: Some(JustifyContent::Center),
                align_items: Some(AlignItems::Center),
                ..Default::default()
            },
            &[text_node],
        ).unwrap();
        self.node_id = Some(node);
        node
    }

    fn prepare(&mut self, ctx: &mut RenderContext, layout_ctx: &LayoutContext, geometry: Geometry) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };

        // Modern Material Design colors (Primary/Indigo)
        let mut color = [0.247, 0.317, 0.709, 1.0]; 
        if self.pressed {
            color = [0.188, 0.247, 0.623, 1.0];
        } else if self.hovered {
            color = [0.301, 0.380, 0.780, 1.0];
        }

        ctx.render_queue.push_rounded_rect(my_geo, color, 8.0); // Rounded corners
        self.text_view.prepare(ctx, layout_ctx, my_geo); // Note: using my_geo as parent
        
        if ctx.debug {
            render_outline_helper(ctx, my_geo, [1.0, 1.0, 0.0, 1.0]);
        }
    }

    fn collect_text_areas<'a>(&'a self, layout_ctx: &LayoutContext, geometry: Geometry, areas: &mut Vec<glyphon::TextArea<'a>>) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };
        self.text_view.collect_text_areas(layout_ctx, my_geo, areas);
    }

    fn render<'rp>(&'rp self, ctx: &'rp RenderContext, render_pass: &mut wgpu::RenderPass<'rp>, geometry: Geometry) {
        self.text_view.render(ctx, render_pass, geometry);
    }

    fn handle_event(&mut self, event: &Event, layout_ctx: &LayoutContext, geometry: Geometry) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };

        match event {
            Event::MouseClick { x, y } => {
                if my_geo.contains(*x, *y) {
                    (self.on_click)();
                }
            }
            Event::MouseMove { x, y } => {
                self.hovered = my_geo.contains(*x, *y);
            }
            Event::MouseDown { x, y } => {
                if my_geo.contains(*x, *y) {
                    self.pressed = true;
                }
            }
            Event::MouseUp { .. } => {
                self.pressed = false;
            }
        }
    }

    fn on_init(&mut self) { self.text_view.on_init(); }
    fn on_mount(&mut self) { self.text_view.on_mount(); }
    fn on_update(&mut self) { self.text_view.on_update(); }
    fn on_unmount(&mut self) { self.text_view.on_unmount(); }
}

pub struct Rect {
    pub color: [f32; 4],
    node_id: Option<NodeId>,
}

impl Rect {
    pub fn new(color: [f32; 4]) -> Self { Self { color, node_id: None } }
}

impl View for Rect {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        let node = ctx.taffy.new_leaf(Style {
            size: Size { width: length(100.0), height: length(100.0) },
            ..Default::default()
        }).unwrap();
        self.node_id = Some(node);
        node
    }

    fn prepare(&mut self, ctx: &mut RenderContext, layout_ctx: &LayoutContext, geometry: Geometry) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };
        ctx.render_queue.push_rect(my_geo, self.color);
    }

    fn render<'rp>(&'rp self, _: &'rp RenderContext, _: &mut wgpu::RenderPass<'rp>, _: Geometry) {}
    fn handle_event(&mut self, _: &Event, _: &LayoutContext, _: Geometry) {}
}

pub struct Circle {
    pub color: [f32; 4],
    node_id: Option<NodeId>,
}

impl Circle {
    pub fn new(color: [f32; 4]) -> Self { Self { color, node_id: None } }
}

impl View for Circle {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        let node = ctx.taffy.new_leaf(Style {
            size: Size { width: length(50.0), height: length(50.0) },
            ..Default::default()
        }).unwrap();
        self.node_id = Some(node);
        node
    }

    fn prepare(&mut self, ctx: &mut RenderContext, layout_ctx: &LayoutContext, geometry: Geometry) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };
        ctx.render_queue.push_circle(my_geo, self.color);
    }

    fn render<'rp>(&'rp self, _: &'rp RenderContext, _: &mut wgpu::RenderPass<'rp>, _: Geometry) {}
    fn handle_event(&mut self, _: &Event, _: &LayoutContext, _: Geometry) {}
}

pub struct RoundedRect {
    pub color: [f32; 4],
    pub radius: f32,
    node_id: Option<NodeId>,
}

impl RoundedRect {
    pub fn new(color: [f32; 4], radius: f32) -> Self { Self { color, radius, node_id: None } }
}

impl View for RoundedRect {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        let node = ctx.taffy.new_leaf(Style {
            size: Size { width: length(100.0), height: length(50.0) },
            ..Default::default()
        }).unwrap();
        self.node_id = Some(node);
        node
    }

    fn prepare(&mut self, ctx: &mut RenderContext, layout_ctx: &LayoutContext, geometry: Geometry) {
        let node_layout = layout_ctx.taffy.layout(self.node_id.unwrap()).unwrap();
        let my_geo = Geometry {
            x: geometry.x + node_layout.location.x,
            y: geometry.y + node_layout.location.y,
            width: node_layout.size.width,
            height: node_layout.size.height,
        };
        ctx.render_queue.push_rounded_rect(my_geo, self.color, self.radius);
    }

    fn render<'rp>(&'rp self, _: &'rp RenderContext, _: &mut wgpu::RenderPass<'rp>, _: Geometry) {}
    fn handle_event(&mut self, _: &Event, _: &LayoutContext, _: Geometry) {}
}

#[allow(non_snake_case)] pub fn Text(text: impl Into<String>) -> Text { Text::new(text) }
#[allow(non_snake_case)] pub fn Button(text: impl Into<String>, on_click: impl FnMut() + 'static) -> Button { Button::new(text, on_click) }
#[allow(non_snake_case)] pub fn Rect(color: [f32; 4]) -> Rect { Rect::new(color) }
#[allow(non_snake_case)] pub fn Circle(color: [f32; 4]) -> Circle { Circle::new(color) }
#[allow(non_snake_case)] pub fn RoundedRect(color: [f32; 4], radius: f32) -> RoundedRect { RoundedRect::new(color, radius) }
