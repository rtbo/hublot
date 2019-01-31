
use crate::Color;
use crate::render::frame;
use crate::ui::{view, View};

pub struct Label {
    common: view::Common,
    _color: Color,
}

impl View for Label {
    fn measure(&mut self) {}
    fn layout(&mut self) {}
    fn frame(&self) -> Option<frame::Node> {
        None
    }
}

impl view::Leaf for Label {}

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
