use taffy::prelude::*;

pub struct LayoutContext {
    pub taffy: TaffyTree<()>,
}

impl Default for LayoutContext {
    fn default() -> Self {
        Self {
            taffy: TaffyTree::new(),
        }
    }
}

impl LayoutContext {
    pub fn new() -> Self {
        Self::default()
    }
}
