use super::Layout;
use crate::render::frame;
use crate::{Color, Paint};

pub trait View {
    fn layout(&self) -> &Layout;
    fn layout_mut(&mut self) -> &mut Layout;
    fn frame(&self) -> frame::Node;
}

/// hypothetical square view
pub struct SquareView {
    color: Color,
    layout: Layout,
}

impl View for SquareView {
    fn layout(&self) -> &Layout {
        &self.layout
    }
    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
    fn frame(&self) -> frame::Node {
        frame::Node::Rect {
            rect: self.layout().rect,
            paint: Paint::Solid(self.color),
            radius: 0f32,
            border: None,
        }
    }
}
