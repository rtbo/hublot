
use crate::geom::{FRect, FSize};
use crate::render::frame;
use crate::ui::View;
use crate::ui::view::{self, Measured, MeasureSpec};

pub struct LinearLayout {
    common: view::Common,
    views: Vec<Box<dyn View>>,
}

impl View for LinearLayout {}

impl Measured for LinearLayout {
    fn measure(&self, _specs: [MeasureSpec; 2]) -> FSize {
        FSize::new(0f32, 0f32)
    }
}

impl view::LaidOut for LinearLayout {
    fn layout(&mut self, _rect: FRect) {}
}

impl view::FrameRendered for LinearLayout {
    fn frame(&self) -> Option<frame::Node> {
        None
    }
}

impl view::SliceParent for LinearLayout {
    fn children_slice(&self) -> &[Box<dyn View>] {
        &self.views
    }
    fn children_slice_mut(&mut self) -> &mut [Box<dyn View>] {
        &mut self.views
    }
}

impl view::Base for LinearLayout {
    type Style = ();
    type State = ();

    fn common(&self) -> &view::Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut view::Common {
        &mut self.common
    }
}
