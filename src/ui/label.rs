
use crate::{color, Color};
use crate::Paint;
use crate::geom::{FRect, FSize};
use crate::render::frame;
use crate::ui::view::{self, HasRect, MeasureSpec, View};

/// A view that can display text or image
pub struct Label {
    common: view::Common,
    color: Color,
}

impl Label {
    pub fn new(color: Color) -> Label {
        Label {
            common: view::Common::default(),
            color,
        }
    }
}

impl Default for Label {
    fn default() -> Label {
        Label {
            common: view::Common::default(),
            color: Color::from(color::CssName::Black),
        }
    }
}

impl View for Label {}

impl view::Leaf for Label {}

impl view::Measured for Label {
    fn measure(&self, _specs: [MeasureSpec; 2]) -> FSize {
        FSize::new(100f32, 50f32)
    }
}

impl view::LaidOut for Label {
    fn layout(&mut self, _rect: FRect) {}
}

impl view::FrameRendered for Label {
    fn frame(&self) -> Option<frame::Node> {
        Some(frame::Node::Rect{
            rect: self.rect(),
            paint: Paint::Solid(self.color),
            radius: 0f32,
            border: None,
        })
    }
}

impl view::Base for Label {
    type State = ();
    type Style = ();

    fn common(&self) -> &view::Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut view::Common {
        &mut self.common
    }
}
