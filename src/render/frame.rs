use crate::geom::{FRect, IRect};
use crate::{Color, Paint};
use winit::WindowId;

pub struct Frame {
    pub window: WindowId,
    pub viewport: IRect,
    pub clear_color: Option<Color>,
    pub root: Option<Node>,
}

impl Frame {
    pub fn new(
        window: WindowId,
        viewport: IRect,
        clear_color: Option<Color>,
        root: Option<Node>,
    ) -> Frame {
        Frame {
            window,
            viewport,
            clear_color,
            root,
        }
    }
}

pub struct RectNode {
    pub rect: FRect,
    pub paint: Paint,
    pub radius: f32,
    pub border: Option<(Color, f32)>,
}

pub enum Node {
    Group(Vec<Node>),
    Transform(Box<Node>, [f32; 16]),
    Rect(RectNode),
}
