use std::collections::HashMap;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use crate::window::WindowState;

pub struct Application {
    pub windows: HashMap<WindowId, WindowState>,
}

impl Application {
    pub fn new<T>(_event_loop: &EventLoop<T>) -> Self {
        Self {
            windows: Default::default(),
        }
    }

    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        _tab_id: Option<String>,
    ) -> Result<WindowId, anyhow::Error> {
        let window_attributes = Window::default_attributes().with_title("New window");
        let window = event_loop
            .create_window(window_attributes)
            .expect("Unable to create window");
        let window_state = WindowState::new(self, window);
        let window_id = window_state.window.id();
        tracing::info!("Created new window with id={window_id:?}");
        self.windows.insert(window_id, window_state);
        Ok(window_id)
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop, None)
            .expect("Failed to create initial window");
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = match self.windows.get_mut(&window_id) {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::Resized(size) => {
                window.resize(size);
            }
            WindowEvent::CloseRequested => {
                self.windows.remove(&window_id);
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = window.draw() {
                    tracing::error!("Error drawing window: {err}");
                }
            }
            _ => (),
        }
    }
}
