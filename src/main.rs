#![windows_subsystem = "windows"]

use winit::{event::{Event, WindowEvent}, event_loop::EventLoop, window::*};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Oxidized");
    print!("Avaiable monitors:");
    for monitor in window.available_monitors() {
        print!(" {};", monitor.name().unwrap());
    }
    println!();
    println!("Current monitor: {}", window.current_monitor().unwrap().name().unwrap());
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        control_flow.set_poll();

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            },
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.

                let size = window.inner_size();
                let position = window.inner_position().unwrap();
                window.set_title(&format!("Oxidized: {} X {} - {} X {}", size.width, size.height,
                                          position.x, position.y));
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
                // println!("RedrawRequested");
            },
            _ => ()
        }
    });
}
