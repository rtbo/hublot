use crate::geom::{FRect, FSize, IRect, Size};
use crate::render;
use crate::Color;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use winit::Window;

pub mod label;
pub mod layout;
pub mod node;
pub mod view;

pub use self::label::Label;
pub use self::layout::LinearLayout;
pub use self::node::Node;
pub use self::view::View;

#[derive(Debug)]
pub struct UserInterface {
    root: RefCell<Option<Rc<Node>>>,
    size: Cell<FSize>,
    clear_color: Cell<Option<Color>>,
    dirty: Cell<Dirty>,
}

impl UserInterface {
    pub fn new() -> Rc<UserInterface> {
        Rc::new(UserInterface {
            root: RefCell::new(None),
            size: Cell::new(Size(0f32, 0f32)),
            clear_color: Cell::new(None),
            dirty: Cell::new(Dirty::all()),
        })
    }

    pub fn new_with_color(color: Color) -> Rc<UserInterface> {
        let ui = Self::new();
        ui.clear_color.set(Some(color));
        ui
    }

    pub fn set_root(&self, root: Option<Rc<Node>>) {
        *self.root.borrow_mut() = root;
        self.add_dirty(Dirty::LAYOUT | Dirty::STYLE | Dirty::FRAME);
    }

    /// Get the size of the user interface
    pub fn size(&self) -> FSize {
        self.size.get()
    }

    /// Checks whether all given dirty flags are set
    pub fn dirty(&self, flags: Dirty) -> bool {
        self.dirty.get().contains(flags)
    }

    /// Handle a window event
    pub fn handle_event(&self, ev: winit::WindowEvent) -> winit::ControlFlow {
        match ev {
            winit::WindowEvent::Resized(size) => {
                self.size.set(From::from(size));
                self.add_dirty(Dirty::LAYOUT | Dirty::FRAME);
                winit::ControlFlow::Continue
            }
            winit::WindowEvent::CloseRequested => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue,
        }
    }

    pub fn layout(&self) {
        if let Some(root) = self.root.borrow().as_ref() {
            let size = self.size();
            let specs = [
                view::MeasureSpec::AtMost(size.width()),
                view::MeasureSpec::AtMost(size.height()),
            ];
            let mut root = root.view_mut();
            root.measure(specs);
            root.layout(FRect::new_s(0f32, 0f32, self.size()));
        }
    }

    pub fn style(&self) {}

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
            self.clear_color.get(),
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
