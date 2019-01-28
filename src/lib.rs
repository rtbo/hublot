extern crate gfx_hal as hal;
extern crate gfx_backend_vulkan as gfx_back;
extern crate winit;

#[macro_use]
extern crate lazy_static;

pub mod color;
pub mod geom;
pub mod gfx;
pub mod paint;
pub mod render;
pub mod ui;

pub use color::Color;
pub use paint::Paint;
pub use ui::UserInterface;
