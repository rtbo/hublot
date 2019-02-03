extern crate hublot;
extern crate winit;

use hublot::event_loop;
use hublot::{color, Color, UserInterface};

fn main() {
    let events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_dimensions(winit::dpi::LogicalSize::new(640 as _, 480 as _))
        .build(&events_loop)
        .unwrap();

    let ui = UserInterface::new_with_color(Color::from(color::CssName::CadetBlue));

    event_loop::run(events_loop, vec![(window, ui)]);
}
