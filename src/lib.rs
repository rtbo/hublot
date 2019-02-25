extern crate gfx_backend_vulkan as gfx_back;
extern crate gfx_hal as hal;
extern crate winit;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod color;
pub mod event_loop;
pub mod geom;
pub mod gfx;
pub mod paint;
pub mod render;
pub mod transform;
pub mod ui;

pub use self::color::Color;
pub use self::paint::Paint;
pub use self::transform::Transform;
pub use self::ui::UserInterface;
