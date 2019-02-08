extern crate hublot;
extern crate winit;

use hublot::event_loop;
use hublot::{color, Color, UserInterface};
use hublot::ui;

fn main() {
    let events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_dimensions(winit::dpi::LogicalSize::new(640 as _, 480 as _))
        .build(&events_loop)
        .unwrap();

    let ui = UserInterface::new_with_color(Color::from(color::CssName::CadetBlue));

    let mut layout = ui::LinearLayout::new_vertical();
    layout.set_spacing(6f32);
    let lbl1 = ui::Label::new(From::from(color::CssName::Chocolate));
    let lbl2 = ui::Label::new(From::from(color::CssName::Coral));

    let layout = ui::Node::new(layout, ui.clone(), None);
    let lbl1 = ui::Node::new(lbl1, ui.clone(), None);
    let lbl2 = ui::Node::new(lbl2, ui.clone(), None);
    layout.add_child(&lbl1, None);
    layout.add_child(&lbl2, None);

    ui.set_root(Some(layout));

    event_loop::run(events_loop, vec![(window, ui)]);
}
