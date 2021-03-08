use pathfinder_canvas::Canvas;
use pathfinder_canvas::CanvasFontContext;
use pathfinder_canvas::CanvasRenderingContext2D;
use pathfinder_canvas::Path2D;
use pathfinder_canvas::RectF;
use pathfinder_canvas::Vector2F;
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

    fn draw(&self) -> CanvasRenderingContext2D {
        let mut canvas = Canvas::new(Vector2F::new(460.0, 500.0))
            .get_context_2d(CanvasFontContext::from_system_source());
        canvas.set_line_width(10.0);
        canvas.stroke_rect(RectF::new(
            Vector2F::new(75.0, 140.0),
            Vector2F::new(150.0, 110.0),
        ));
        canvas.fill_rect(RectF::new(
            Vector2F::new(130.0, 190.0),
            Vector2F::new(40.0, 60.0),
        ));

        let mut path = Path2D::new();
        path.move_to(Vector2F::new(50.0, 140.0));
        path.line_to(Vector2F::new(150.0, 60.0));
        path.line_to(Vector2F::new(250.0, 140.0));
        path.close_path();

        canvas.stroke_path(path);

        canvas
    }
}

fn main() {
    let events_loop = EventsLoop::new();
    let d = Demo::new(events_loop);
    d.run();
    println!("End Demo!");
}
