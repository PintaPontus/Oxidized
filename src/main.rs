use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new();
    builder.build(&event_loop);
}
