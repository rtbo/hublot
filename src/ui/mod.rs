use crate::geom::IRect;
use crate::render;
use crate::Color;
use std::rc::Rc;
use winit::Window;

pub struct UserInterface {
    clear_color: Option<Color>,
}

impl UserInterface {
    pub fn new() -> Rc<UserInterface> {
        Rc::new(UserInterface { clear_color: None })
    }

    pub fn new_with_color(color: Color) -> Rc<UserInterface> {
        Rc::new(UserInterface {
            clear_color: Some(color),
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
