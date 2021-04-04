use pathfinder_canvas::CanvasFontContext;
use pathfinder_canvas::CanvasRenderingContext2D;
use pathfinder_canvas::Path2D;
use pathfinder_canvas::RectF;
use pathfinder_canvas::Vector2F;
use pathfinder_canvas::Vector2I;
use pathfinder_canvas::{Canvas, ColorF};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::{
    concurrent::{rayon::RayonExecutor, scene_proxy::SceneProxy},
    gpu::{
        options::{DestFramebuffer, RendererMode, RendererOptions},
        renderer::Renderer,
    },
    options::BuildOptions,
};
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use surfman::{Connection, ContextAttributeFlags, ContextAttributes};
use surfman::{Context, Device, Surface};
use surfman::{GLVersion as SurfmanGLVersion, SurfaceType};
use winit::dpi::LogicalSize;
use winit::ControlFlow;
use winit::Event;
use winit::EventsLoop;
use winit::Window;
use winit::WindowBuilder;
use winit::WindowEvent;

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

        let connection = self.build_connection(&window);
        let mut device = self.build_device(&connection);
        let mut context = self.build_context(&mut device);

        let surface = self.build_surface(&connection, &window, &mut device, &context);
        self.load_gl(&device, &mut context, surface);

        let pathfinder_device = self.build_pathfinder_device(&device, &context);

        let mut render = self.build_render(&window, pathfinder_device);

        let mut is_first_render = true;
        self.events_loop.run_forever(|event| -> ControlFlow {
            let mut _should_render = is_first_render;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { .. },
                    ..
                }
                | Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    device.destroy_context(&mut context).unwrap();
                    return ControlFlow::Break;
                }
                Event::WindowEvent {
                    event: WindowEvent::Refresh,
                    ..
                } => {
                    _should_render = true;
                }
                _ => return ControlFlow::Continue,
            }

            if _should_render {
                let canvas = Demo::build_canvas();
                Demo::build_scene(canvas, &mut render);
                Demo::show(&device, &mut context);
            }

            is_first_render = false;
            ControlFlow::Continue
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

    fn build_connection(&self, window: &Window) -> Connection {
        Connection::from_winit_window(window).unwrap()
    }

    fn build_device(&self, connection: &Connection) -> Device {
        let adapter = connection.create_low_power_adapter().unwrap();
        connection.create_device(&adapter).unwrap()
    }

    fn build_context(&self, device: &mut Device) -> Context {
        let context_descriptor = device
            .create_context_descriptor(&self.context_attributes())
            .unwrap();

        device.create_context(&context_descriptor, None).unwrap()
    }

    fn context_attributes(&self) -> ContextAttributes {
        ContextAttributes {
            version: SurfmanGLVersion::new(3, 0),
            flags: ContextAttributeFlags::ALPHA,
        }
    }

    fn build_surface(
        &self,
        connection: &Connection,
        window: &Window,
        device: &mut Device,
        context: &Context,
    ) -> Surface {
        let native_widget = connection
            .create_native_widget_from_winit_window(&window)
            .unwrap();
        let surface_type = SurfaceType::Widget { native_widget };
        device
            .create_surface(&context, surfman::SurfaceAccess::GPUOnly, surface_type)
            .unwrap()
    }

    fn load_gl(&self, device: &Device, context: &mut Context, surface: Surface) {
        device.bind_surface_to_context(context, surface).unwrap();
        device.make_context_current(context).unwrap();
        gl::load_with(|symbol_name| device.get_proc_address(&context, symbol_name));
    }

    fn build_pathfinder_device(&self, device: &Device, context: &Context) -> GLDevice {
        let default_framebuffer = device
            .context_surface_info(context)
            .unwrap()
            .unwrap()
            .framebuffer_object;
        GLDevice::new(GLVersion::GL3, default_framebuffer)
    }

    fn build_render(&self, window: &Window, pdevice: GLDevice) -> Renderer<GLDevice> {
        let monitor_id = window.get_current_monitor();
        let physical_size = monitor_id.get_dimensions();
        let framebuffer_size =
            Vector2I::new(physical_size.width as i32, physical_size.height as i32);

        let mode = RendererMode::default_for_device(&pdevice);
        let options = RendererOptions {
            dest: DestFramebuffer::full_window(framebuffer_size),
            background_color: Some(ColorF::white()),
            show_debug_ui: true,
        };
        let resource_loader = EmbeddedResourceLoader::new();
        Renderer::new(pdevice, &resource_loader, mode, options)
    }

    fn build_canvas() -> CanvasRenderingContext2D {
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

    fn build_scene(canvas: CanvasRenderingContext2D, renderer: &mut Renderer<GLDevice>) {
        let mut scene = SceneProxy::from_scene(
            canvas.into_canvas().into_scene(),
            renderer.mode().level,
            RayonExecutor,
        );
        scene.build_and_render(renderer, BuildOptions::default());
    }

    fn show(device: &Device, context: &mut Context) {
        let mut surface = device
            .unbind_surface_from_context(context)
            .unwrap()
            .unwrap();
        device.present_surface(context, &mut surface).unwrap();
        device.bind_surface_to_context(context, surface).unwrap();
    }
}

fn main() {
    let events_loop = EventsLoop::new();
    let d = Demo::new(events_loop);
    d.run();
    println!("End Demo!");
}
