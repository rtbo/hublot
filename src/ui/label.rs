
use crate::Color;
use crate::geom::{FRect, FSize};
use crate::render::frame;
use crate::ui::{view, View};

pub struct Label {
    common: view::Common,
    _color: Color,
}

impl view::View for Label {}

impl view::Leaf for Label {}

impl view::Measured for Label {
    fn measure(&self, _specs: [view::MeasureSpec; 2]) -> FSize {
        FSize::new(0f32, 0f32)
    }
}

impl view::LaidOut for Label {
    fn layout(&mut self, _rect: FRect) {}
}

impl view::FrameRendered for Label {
    fn frame(&self) -> Option<frame::Node> {
        None
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
