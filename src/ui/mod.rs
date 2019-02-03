use crate::geom::{FSize, FRect, IRect, Size};
use crate::render;
use crate::Color;
use std::cell::Cell;
use winit::Window;

pub mod label;
pub mod layout;
pub mod view;

pub use label::Label;
pub use layout::LinearLayout;
pub use view::View;

pub struct UserInterface {
    root: Option<Box<dyn View>>,
    size: FSize,
    clear_color: Option<Color>,
    dirty: Cell<Dirty>,
}

impl UserInterface {
    pub fn new() -> UserInterface {
        UserInterface {
            root: None,
            size: Size(0f32, 0f32),
            clear_color: None,
            dirty: Cell::new(Dirty::all()),
        }
    }

    pub fn new_with_color(color: Color) -> UserInterface {
        UserInterface {
            root: None,
            size: Size(0f32, 0f32),
            clear_color: Some(color),
            dirty: Cell::new(Dirty::all()),
        }
    }

    pub fn set_root(&mut self, root: Option<Box<dyn View>>) {
        self.root = root;
        self.add_dirty(Dirty::LAYOUT | Dirty::STYLE | Dirty::FRAME);
    }

    /// Checks whether all given dirty flags are set
    pub fn dirty(&self, flags: Dirty) -> bool {
        self.dirty.get().contains(flags)
    }

    /// Handle a window event
    pub fn handle_event(&mut self, ev: winit::WindowEvent) -> winit::ControlFlow {
        match ev {
            winit::WindowEvent::Resized(size) => {
                self.size = From::from(size);
                self.add_dirty(Dirty::LAYOUT | Dirty::FRAME);
                winit::ControlFlow::Continue
            }
            winit::WindowEvent::CloseRequested => {
                winit::ControlFlow::Break
            }
            _ => {
                winit::ControlFlow::Continue
            }
        }
    }

    pub fn layout(&mut self) {
        if let Some(root) = &mut self.root {
            let specs = [
                view::MeasureSpec::AtMost(self.size.width()),
                view::MeasureSpec::AtMost(self.size.height()),
            ];
            root.measure(specs);
            root.layout(FRect::new_s(0f32, 0f32, self.size));
        }
    }

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

    fn add_dirty(&self, flags: Dirty) {
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
