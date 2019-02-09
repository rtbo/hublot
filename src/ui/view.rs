use crate::geom::{FMargins, FPoint, FRect, FSize, Margins, Size};
use crate::render::frame;
use crate::ui::Node;
use crate::Transform;

use downcast_rs::Downcast;

use std::fmt::Debug;
// use std::iter;
use std::rc::{Rc, Weak};
//use std::slice;

/// The View trait represent a single or composed view in a view tree.
/// The View trait is object safe.
pub trait View:
    Debug + Downcast + NodeOwned + Measure + Layout + FrameRender + HasRect + HasPadding + HasMargins
{
}

impl_downcast!(View);

/// Specify how a View should measure itself
#[derive(Clone, Copy, Debug)]
pub enum MeasureSpec {
    Unspecified,
    AtMost(f32),
    Exactly(f32),
}

/// Trait for being owned by a Node
pub trait NodeOwned {
    /// Get the node owning self
    fn node(&self) -> Rc<Node>;
}

/// Trait for object that store measurement
pub trait Measurement {
    /// Get the stored measurement
    fn measurement(&self) -> FSize;
}

/// Trait for objects that can be measured
pub trait Measure: Measurement {
    /// Measure the object according to spec and store the measurement
    fn measure(&mut self, spec: [MeasureSpec; 2]);
}

/// Trait for objects that can layout themselves
pub trait Layout {
    /// Lay the object out in the given rect
    fn layout(&mut self, rect: FRect);
}

/// Trait for objects that can render themselves in a framegraph node
pub trait FrameRender {
    /// Render the object in a frame graph node
    fn frame_render(&self) -> Option<frame::Node>;
}

/// Object that has an assigned Rect within its parent
pub trait HasRect {
    /// the rect within its parent
    fn rect(&self) -> FRect;
}

/// Object that has an assigned position within its parent
pub trait HasPosition {
    /// the position within its parent
    fn position(&self) -> FPoint;
}

/// Object that has an assigned size
pub trait HasSize {
    /// the size of the view
    fn size(&self) -> FSize;
}

/// View that has margins
pub trait HasMargins {
    fn margins(&self) -> FMargins; // left, top, right, bottom
}

/// View that has padding
pub trait HasPadding {
    fn padding(&self) -> FMargins; // left, top, right, bottom
}

impl<T: HasRect> HasPosition for T {
    fn position(&self) -> FPoint {
        self.rect().point()
    }
}

impl<T: HasRect> HasSize for T {
    fn size(&self) -> FSize {
        self.rect().size()
    }
}

/// Marker to indicate that Children should be implemented
pub trait HasChildren {}

/// A View with children
pub trait Children {
    type Children: IntoIterator<Item = Rc<Node>>;

    fn children(&self) -> Self::Children;
}

/// A View without children
pub trait Leaf {}

// impl<'a, T: Leaf> Parent<'a> for T {
//     type Children = iter::Empty<&'a dyn View>;
//     type ChildrenMut = iter::Empty<&'a mut dyn View>;

//     fn children(&'a self) -> Self::Children {
//         iter::empty()
//     }
//     fn children_mut(&'a mut self) -> Self::ChildrenMut {
//         iter::empty()
//     }
// }

// pub trait SliceParent {
//     fn children_slice(&self) -> &[Box<dyn View>];
//     fn children_slice_mut(&mut self) -> &mut [Box<dyn View>];
// }

// pub struct ChildrenIter<'a> {
//     iter: slice::Iter<'a, Box<dyn View>>,
// }

// impl<'a> Iterator for ChildrenIter<'a> {
//     type Item = &'a dyn View;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().map(|boxed| &**boxed)
//     }
// }

// pub struct ChildrenIterMut<'a> {
//     iter: slice::IterMut<'a, Box<dyn View>>,
// }

// impl<'a> Iterator for ChildrenIterMut<'a> {
//     type Item = &'a mut dyn View;
//     fn next(&mut self) -> Option<Self::Item> {
//         //self.iter.next().map(|boxed| &mut **boxed)
//         match self.iter.next() {
//             None => None,
//             Some(boxed) => Some(&mut **boxed),
//         }
//     }
// }

// impl<'a, T: SliceParent> Parent<'a> for T {
//     type Children = ChildrenIter<'a>;
//     type ChildrenMut = ChildrenIterMut<'a>;

//     fn children(&'a self) -> Self::Children {
//         ChildrenIter {
//             iter: self.children_slice().iter(),
//         }
//     }
//     fn children_mut(&'a mut self) -> Self::ChildrenMut {
//         ChildrenIterMut {
//             iter: self.children_slice_mut().iter_mut(),
//         }
//     }
// }

pub trait Base: View {
    type State;
    type Style;

    fn common(&self) -> &Common;
    fn common_mut(&mut self) -> &mut Common;

    fn set_measurement(&mut self, size: FSize) {
        self.common_mut().measurement = size;
    }
}

#[derive(Debug)]
pub struct Common {
    pub node: Weak<Node>,
    pub measurement: FSize,
    pub rect: FRect,
    pub padding: FMargins,
    pub margins: FMargins,
    pub transform: Transform,
}

impl<T: Base> NodeOwned for T {
    fn node(&self) -> Rc<Node> {
        self.common().node.upgrade().unwrap()
    }
}

impl<T: Base> Measurement for T {
    fn measurement(&self) -> FSize {
        self.common().measurement
    }
}

impl<T: Base> HasRect for T {
    fn rect(&self) -> FRect {
        self.common().rect
    }
}

impl<T: Base> HasPadding for T {
    fn padding(&self) -> FMargins {
        self.common().padding
    }
}

impl<T: Base> HasMargins for T {
    fn margins(&self) -> FMargins {
        self.common().margins
    }
}

pub struct ChildrenIter {
    sibling: Option<Rc<Node>>,
}

impl Iterator for ChildrenIter {
    type Item = Rc<Node>;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.sibling {
            None => None,
            Some(node) => {
                self.sibling = node.next_sibling();
                self.sibling.clone()
            }
        }
    }
}

impl<T: Base + HasChildren> Children for T {
    type Children = ChildrenIter;

    fn children(&self) -> ChildrenIter {
        ChildrenIter {
            sibling: self.node().first_child(),
        }
    }
}

impl Default for Common {
    fn default() -> Common {
        Common {
            node: Weak::default(),
            measurement: Size(0f32, 0f32),
            rect: FRect::new(0f32, 0f32, 0f32, 0f32),
            padding: Margins(0f32, 0f32, 0f32, 0f32),
            margins: Margins(0f32, 0f32, 0f32, 0f32),
            transform: Transform::identity(),
        }
    }
}

bitflags! {
    pub struct Dirty : u32 {
        const LAYOUT    = 1;
        const STYLE     = 2;
        const FRAME     = 4;
        const TRANSFORM = 8;
    }
}
