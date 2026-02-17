use crate::view::View;
use crate::layout::LayoutContext;
use crate::render::RenderContext;
use taffy::prelude::*;

pub struct Column {
    pub children: Vec<Box<dyn View>>,
}

impl Column {
    pub fn new(children: Vec<Box<dyn View>>) -> Self {
        Self { children }
    }
}

impl View for Column {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        let child_nodes: Vec<NodeId> = self.children.iter_mut()
            .map(|child| child.layout(ctx))
            .collect();
        
        ctx.taffy.new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            &child_nodes,
        ).unwrap()
    }

    fn render(&self, ctx: &mut RenderContext) {
        for child in &self.children {
            child.render(ctx);
        }
    }
}

pub struct Text {
    pub text: String,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl View for Text {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        ctx.taffy.new_leaf(Style::default()).unwrap()
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // Render text
    }
}

pub struct Button {
    pub text: String,
    pub on_click: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(text: impl Into<String>, on_click: impl FnMut() + 'static) -> Self {
        Self {
            text: text.into(),
            on_click: Box::new(on_click),
        }
    }
}

impl View for Button {
    fn layout(&mut self, ctx: &mut LayoutContext) -> NodeId {
        ctx.taffy.new_leaf(Style::default()).unwrap()
    }

    fn render(&self, _ctx: &mut RenderContext) {
        // Render button
    }
}

#[allow(non_snake_case)]
pub fn Text(text: impl Into<String>) -> Text {
    Text::new(text)
}

#[allow(non_snake_case)]
pub fn Button(text: impl Into<String>, on_click: impl FnMut() + 'static) -> Button {
    Button::new(text, on_click)
}
