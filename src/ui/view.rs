use crate::geom::{FPoint, FRect, FSize, Transform};
use crate::render::frame;

// use std::iter;
use std::slice;

/// The View trait represent a single or composed view in a view tree.
/// The View trait is object safe.
pub trait View : Measured + LaidOut + FrameRendered + HasRect {
}

/// Specify how a View should measure itself
#[derive(Clone, Copy, Debug)]
pub enum MeasureSpec {
    Unspecified,
    AtMost(f32),
    Exactly(f32),
}

/// Trait for objects that can be measured
pub trait Measured {
    /// Measure the object according to spec and return the size
    fn measure(&self, spec: [MeasureSpec; 2]) -> FSize;
}

/// Trait for objects that can layout themselves
pub trait LaidOut {
    /// Lay the object out in the given rect
    fn layout(&mut self, rect: FRect);
}

/// Trait for objects that can render themselves in a framegraph node
pub trait FrameRendered {
    /// Render the object in a frame graph node
    fn frame(&self) -> Option<frame::Node>;
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


pub trait Parent<'a> {
    type Children: IntoIterator<Item = &'a dyn View>;
    type ChildrenMut: IntoIterator<Item = &'a mut dyn View>;

    fn children(&'a self) -> Self::Children;
    fn children_mut(&'a mut self) -> Self::ChildrenMut;
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

pub trait SliceParent {
    fn children_slice(&self) -> &[Box<dyn View>];
    fn children_slice_mut(&mut self) -> &mut [Box<dyn View>];
}

pub struct ChildrenIter<'a> {
    iter: slice::Iter<'a, Box<dyn View>>,
}

impl<'a> Iterator for ChildrenIter<'a> {
    type Item = &'a dyn View;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|boxed| &**boxed)
    }
}

pub struct ChildrenIterMut<'a> {
    iter: slice::IterMut<'a, Box<dyn View>>,
}

impl<'a> Iterator for ChildrenIterMut<'a> {
    type Item = &'a mut dyn View;
    fn next(&mut self) -> Option<Self::Item> {
        //self.iter.next().map(|boxed| &mut **boxed)
        match self.iter.next() {
            None => None,
            Some(boxed) => {
                Some(&mut **boxed)
            }
        }
    }
}

impl<'a, T: SliceParent> Parent<'a> for T
{
    type Children = ChildrenIter<'a>;
    type ChildrenMut = ChildrenIterMut<'a>;

    fn children(&'a self) -> Self::Children {
        ChildrenIter {
            iter: self.children_slice().iter()
        }
    }
    fn children_mut(&'a mut self) -> Self::ChildrenMut {
        ChildrenIterMut {
            iter: self.children_slice_mut().iter_mut()
        }
    }
}

pub trait Base: View {
    type State;
    type Style;

    fn common(&self) -> &Common;
    fn common_mut(&mut self) -> &mut Common;
}

pub struct Common {
    pub rect: FRect,
    pub transform: Transform,
}

impl<T: Base> HasRect for T {
    fn rect(&self) -> FRect {
        self.common().rect
    }
}

impl Default for Common {
    fn default() -> Common {
        Common {
            rect: FRect::new(0f32, 0f32, 0f32, 0f32),
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
