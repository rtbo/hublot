use crate::geom::IRect;
use crate::Color;
use winit::WindowId;

pub struct Frame {
    pub window: WindowId,
    pub viewport: IRect,
    pub clear_color: Option<Color>,
}

impl Frame {
    pub fn new (window: WindowId, viewport: IRect, clear_color: Option<Color>) -> Frame {
        Frame { window, viewport, clear_color }
    }
}