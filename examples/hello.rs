extern crate hublot;
extern crate winit;

use hublot::event_loop;
use hublot::{color, Color, UserInterface};
use hublot::ui::{LinearLayout, Label};

fn main() {
    let events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_dimensions(winit::dpi::LogicalSize::new(640 as _, 480 as _))
        .build(&events_loop)
        .unwrap();

    let mut ui = UserInterface::new_with_color(Color::from(color::CssName::CadetBlue));

    let mut layout = LinearLayout::new_vertical();
    layout.set_spacing(6f32);
    let lbl1 = Label::new(From::from(color::CssName::Chocolate));
    layout.add_view(Box::new(lbl1));
    let lbl2 = Label::new(From::from(color::CssName::Coral));
    layout.add_view(Box::new(lbl2));

    ui.set_root(Some(Box::new(layout)));

    event_loop::run(events_loop, vec![(window, ui)]);
}
