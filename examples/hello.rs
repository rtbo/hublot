extern crate hublot;
extern crate winit;

use hublot::render;
use hublot::{color, Color, UserInterface};

fn main() {
    let mut events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_dimensions(winit::dpi::LogicalSize::new(640 as _, 480 as _))
        .build(&events_loop)
        .unwrap();

    let ui = UserInterface::new_with_color(Color::from(color::CssName::CadetBlue));

    let render_thread = render::Thread::new(Some(&window));

    // spawn the render thread
    events_loop.run_forever(|event| {
        println!("received event: {:?}", event);

        render_thread.frame(ui.frame(&window));

        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => {
                winit::ControlFlow::Break
            }
            _ => winit::ControlFlow::Continue,
        }
    });

    render_thread.stop();
}
