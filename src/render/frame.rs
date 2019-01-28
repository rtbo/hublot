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

pub enum Node {
    Group(Vec<Node>),
    Transform(Box<Node>, [f32; 16]),
    Rect {
        rect: FRect,
        paint: Paint,
        radius: f32,
        border: Option<(Color, f32)>,
    },
}
