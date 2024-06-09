use winit::event_loop::{ControlFlow, EventLoop};

use bootstrap_renderer::application::Application;
use bootstrap_renderer::telemetry::{get_subscriber, init_subscriber};

fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("bootstrap-renderer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = Application::new(&event_loop);
    event_loop.run_app(&mut app).map_err(Into::into)
}
