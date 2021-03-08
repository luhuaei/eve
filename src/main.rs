use winit::dpi::LogicalSize;
use winit::ControlFlow;
use winit::Event;
use winit::EventsLoop;
use winit::KeyboardInput;
use winit::Window;
use winit::WindowBuilder;
use winit::WindowEvent;
use winit::WindowId;

struct Demo {
    events_loop: EventsLoop,
}

impl Demo {
    fn new(events_loop: EventsLoop) -> Demo {
        Demo { events_loop }
    }

    fn run(mut self) {
        println!("Hello Demo!");
        let window = self.build_window(&self.events_loop);
        window.show();
        self.events_loop.run_forever(|event| -> ControlFlow {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { .. },
                    ..
                }
                | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => return ControlFlow::Break,
                _ => return ControlFlow::Continue,
            }
        })
    }

    fn build_window(&self, events_loop: &EventsLoop) -> Window {
        let size = LogicalSize::new(460 as f64, 500 as f64);
        WindowBuilder::new()
            .with_dimensions(size)
            .with_title("demo")
            .build(events_loop)
            .unwrap()
    }
}

fn main() {
    let events_loop = EventsLoop::new();
    let d = Demo::new(events_loop);
    d.run();
    println!("End Demo!");
}
