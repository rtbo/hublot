use crate::geom::IRect;
use crate::render;
use crate::Color;
use std::rc::Rc;
use winit::Window;

pub mod layout;
pub mod node;
pub mod view;

pub use layout::Layout;
pub use node::Node;
pub use view::View;

pub struct UserInterface {
    clear_color: Option<Color>,
    dirty: Dirty,
}

impl UserInterface {
    pub fn new() -> Rc<UserInterface> {
        Rc::new(UserInterface {
            clear_color: None,
            dirty: Dirty::empty(),
        })
    }

    pub fn new_with_color(color: Color) -> Rc<UserInterface> {
        Rc::new(UserInterface {
            clear_color: Some(color),
            dirty: Dirty::empty(),
        })
    }

    pub fn frame(&self, win: &Window) -> render::Frame {
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
}

bitflags! {
    pub struct Dirty : u32 {
        const LAYOUT = 0x0001;
        const FRAME  = 0x0002;
    }
}
