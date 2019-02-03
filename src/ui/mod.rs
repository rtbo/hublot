use crate::geom::IRect;
use crate::render;
use crate::Color;
use std::cell::Cell;
use winit::Window;

pub mod label;
pub mod layout;
pub mod view;

pub use label::Label;
pub use view::View;

pub struct UserInterface {
    clear_color: Option<Color>,
    dirty: Cell<Dirty>,
}

impl UserInterface {
    pub fn new() -> UserInterface {
        UserInterface {
            clear_color: None,
            dirty: Cell::new(Dirty::all()),
        }
    }

    pub fn new_with_color(color: Color) -> UserInterface {
        UserInterface {
            clear_color: Some(color),
            dirty: Cell::new(Dirty::all()),
        }
    }

    /// Checks whether all given dirty flags are set
    pub fn dirty(&self, flags: Dirty) -> bool {
        self.dirty.get().contains(flags)
    }

    pub fn layout(&mut self) {}

    pub fn style(&mut self) {}

    pub fn frame(&self, win: &Window) -> render::Frame {
        self.remove_dirty(Dirty::FRAME);
        let size: (u32, u32) = win
            .get_inner_size()
            .map(|s| s.to_physical(win.get_hidpi_factor()))
            .unwrap()
            .into();
        render::Frame::new(
            win.id(),
            IRect::new(0, 0, size.0 as _, size.1 as _),
            self.clear_color,
            None,
        )
    }

    fn _add_dirty(&self, flags: Dirty) {
        let mut dirty = self.dirty.get();
        dirty.insert(flags);
        self.dirty.set(dirty);
    }

    fn remove_dirty(&self, flags: Dirty) {
        let mut dirty = self.dirty.get();
        dirty.remove(flags);
        self.dirty.set(dirty);
    }
}

bitflags! {
    pub struct Dirty : u32 {
        const LAYOUT = 1;
        const STYLE  = 2;
        const FRAME  = 4;
    }
}
