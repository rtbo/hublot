use crate::render;
use crate::{ui, UserInterface};
use winit;

pub fn run(mut event_loop: winit::EventsLoop, mut windows: Vec<(winit::Window, UserInterface)>) {
    let wins: Vec<_> = windows.iter().map(|w| &(*w).0).collect();
    let render_thread = render::Thread::new(wins);

    event_loop.run_forever(|event| {
        let mut frames = Vec::new();

        for w_ui in &mut windows {
            if w_ui.1.dirty(ui::Dirty::STYLE) {
                w_ui.1.style();
            }
            if w_ui.1.dirty(ui::Dirty::LAYOUT) {
                w_ui.1.layout();
            }
            if w_ui.1.dirty(ui::Dirty::FRAME) {
                frames.push(w_ui.1.frame(&w_ui.0));
            }
        }

        if frames.len() > 0 {
            render_thread.frames(frames);
        }

        match event {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested,
                ..
            } => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue,
        }
    });

    render_thread.stop();
}
