use std::{num::NonZeroU32, sync::Arc};

use winit::{dpi::PhysicalSize, window::Window};

use crate::application::Application;

pub struct WindowState {
    pub window: Arc<Window>,
}

impl WindowState {
    pub fn new(_app: &Application, window: Window) -> Self {
        let window = Arc::new(window);

        Self { window }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let (width, height) = match (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
            (Some(width), Some(height)) => (width, height),
            _ => return,
        };

        tracing::info!("Should have resized to {width} {height}");
        self.window.request_redraw();
    }

    pub fn draw(&self) -> Result<(), anyhow::Error> {
        self.window.request_redraw();
        Ok(())
    }
}
