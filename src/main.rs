use std::num::NonZeroU32;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use bootstrap_renderer::telemetry::{get_subscriber, init_subscriber};

fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("bootstrap-renderer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    tracing::info!("Starting app");
    let mut app = App::default();
    event_loop.run_app(&mut app).map_err(Into::into)
}

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = match &self.window {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::Resized(size) => {
                tracing::info!("Resizing window");
                let (width, height) =
                    match (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
                        (Some(width), Some(height)) => (width, height),
                        _ => return,
                    };

                tracing::info!("Should have resized to {width} {height}");
                window.request_redraw();
            }
            WindowEvent::CloseRequested => {
                tracing::info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
