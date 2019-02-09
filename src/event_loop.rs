use crate::render;
use crate::{ui, UserInterface};
use std::rc::Rc;
use winit;

pub fn run(
    mut event_loop: winit::EventsLoop,
    mut windows: Vec<(winit::Window, Rc<UserInterface>)>,
) {
    let wins: Vec<_> = windows.iter().map(|w| &(*w).0).collect();
    let render_thread = render::Thread::new(wins);

    event_loop.run_forever(|event| {
        let mut frames = Vec::new();

        for w_ui in &windows {
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
            winit::Event::WindowEvent { window_id, event } => {
                let idx = windows.iter().position(|w_ui| w_ui.0.id() == window_id);
                if let Some(idx) = idx {
                    let cf = windows[idx].1.handle_event(event);
                    match cf {
                        winit::ControlFlow::Break => {
                            let _ = windows.remove(idx);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        if windows.len() > 0 {
            winit::ControlFlow::Continue
        } else {
            winit::ControlFlow::Break
        }
    });

    render_thread.stop();
}
