extern crate gfx_hal as hal;
extern crate gfx_backend_vulkan as gfx_back;
extern crate winit;

#[macro_use]
extern crate lazy_static;

pub mod color;
pub mod geom;
pub mod gfx;
pub mod render;

pub use color::Color;
