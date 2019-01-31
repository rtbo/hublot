use crate::geom::{FRect, FSize, Transform};
use crate::render::frame;

// use std::iter;
use std::slice;

/// The View trait is intended for trait objects
pub trait View {
    fn measure(&mut self);
    fn layout(&mut self);
    fn frame(&self) -> Option<frame::Node>;
}

pub trait Parent<'a> {
    type Children: IntoIterator<Item = &'a dyn View>;
    type ChildrenMut: IntoIterator<Item = &'a mut dyn View>;

    fn children(&'a self) -> Self::Children;
    fn children_mut(&'a mut self) -> Self::ChildrenMut;
}

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
    _measurement: FSize,
    _rect: FRect,
    _transform: Transform,
}

impl Default for Common {
    fn default() -> Common {
        Common {
            _measurement: FSize::new(0f32, 0f32),
            _rect: FRect::new(0f32, 0f32, 0f32, 0f32),
            _transform: Transform::identity(),
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
