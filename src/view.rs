use crate::layout::LayoutContext;
use crate::render::RenderContext;

#[derive(Debug, Clone, Copy, Default)]
pub struct Geometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Geometry {
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    MouseClick { x: f32, y: f32 },
    MouseMove { x: f32, y: f32 },
    MouseDown { x: f32, y: f32 },
    MouseUp { x: f32, y: f32 },
}

pub trait View {
    fn layout(&mut self, ctx: &mut LayoutContext) -> taffy::prelude::NodeId;
    fn prepare(&mut self, _ctx: &mut RenderContext, _layout_ctx: &LayoutContext, _geometry: Geometry) {}
    fn collect_text_areas<'a>(&'a self, _layout_ctx: &LayoutContext, _geometry: Geometry, _areas: &mut Vec<glyphon::TextArea<'a>>) {}
    fn render<'rp>(&'rp self, ctx: &'rp RenderContext, render_pass: &mut wgpu::RenderPass<'rp>, geometry: Geometry);
    fn handle_event(&mut self, event: &Event, layout_ctx: &LayoutContext, geometry: Geometry);

    // Lifecycle hooks
    fn on_init(&mut self) {}
    fn on_mount(&mut self) {}
    fn on_update(&mut self) {}
    fn on_unmount(&mut self) {}
}
