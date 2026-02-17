use crate::layout::LayoutContext;
use crate::render::RenderContext;

pub trait View {
    fn layout(&mut self, ctx: &mut LayoutContext) -> taffy::prelude::NodeId;
    fn render(&self, ctx: &mut RenderContext);
}
