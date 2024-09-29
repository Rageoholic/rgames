use anyhow::Context;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop,
    window::WindowAttributes,
};

#[derive(Debug)]
enum App {
    Uninit,
    Init { win: winit::window::Window },
    Destroyed,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let App::Uninit = self {
            match event_loop.create_window(WindowAttributes::default()) {
                Ok(win) => {
                    *self = App::Init { win };
                }
                Err(e) => {
                    eprintln!("Unable to create window, Error: {}", e);
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match self {
            App::Uninit => {
                log::warn!(
                    "Received message {:?} on window {:?} but app was not initialized",
                    event,
                    window_id
                )
            }
            App::Init { ref win } => {
                if window_id == win.id() {
                    match event {
                        WindowEvent::CloseRequested => {
                            win.set_visible(false);
                            *self = App::Destroyed;
                            event_loop.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            log::trace!("Time to redraw");
                        }
                        _ => {}
                    }
                } else {
                    log::warn!(
                        "Recieved message {:?} for nonexistant window {:?}",
                        event,
                        window_id
                    )
                }
            }
            App::Destroyed => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let ev_loop = EventLoop::builder()
        .build()
        .context("Couldn't create event loop")?;

    ev_loop
        .run_app(&mut App::Uninit)
        .context("Could not run our application")?;

    Ok(())
}
