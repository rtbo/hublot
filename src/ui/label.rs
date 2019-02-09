use crate::color;
use crate::geom::FRect;
use crate::render::frame;
use crate::ui::view::{self, HasRect, MeasureSpec, View};
use crate::{Color, Paint};

/// A view that can display text or image
#[derive(Debug)]
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

impl view::Measure for Label {
    fn measure(&mut self, _specs: [MeasureSpec; 2]) {}
}

impl view::Layout for Label {
    fn layout(&mut self, _rect: FRect) {}
}

impl view::FrameRender for Label {
    fn frame_render(&self) -> Option<frame::Node> {
        Some(frame::Node::Rect(frame::RectNode {
            rect: self.rect(),
            paint: Paint::Solid(self.color),
            radius: 0f32,
            border: None,
        }))
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
