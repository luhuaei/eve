use euclid::default::Size2D;
use pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use pathfinder_color::ColorF;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use surfman::{
    Connection, ContextAttributeFlags, ContextAttributes, GLVersion as SurfmanGLVersion,
};
use surfman::{SurfaceAccess, SurfaceType};
use winit::dpi::LogicalSize;
use winit::{ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};

fn main() {
    let mut events_loop = EventsLoop::new();
    let window_size = Size2D::new(640, 480);
    let logical_size = LogicalSize::new(window_size.width as f64, window_size.height as f64);
    let window = WindowBuilder::new()
        .with_title("demo")
        .with_dimensions(logical_size)
        .build(&events_loop)
        .unwrap();

    window.show();

    let connection = Connection::from_winit_window(&window).unwrap();
    let native_widget = connection
        .create_native_widget_from_winit_window(&window)
        .unwrap();
    let adapter = connection.create_low_power_adapter().unwrap();
    let mut device = connection.create_device(&adapter).unwrap();

    let context_attributes = ContextAttributes {
        version: SurfmanGLVersion::new(3, 0),
        flags: ContextAttributeFlags::ALPHA,
    };

    let context_descriptor = device
        .create_context_descriptor(&context_attributes)
        .unwrap();
    let surface_type = SurfaceType::Widget { native_widget };
    let mut context = device.create_context(&context_descriptor, None).unwrap();
    let surface = device
        .create_surface(&context, SurfaceAccess::GPUOnly, surface_type)
        .unwrap();

    device
        .bind_surface_to_context(&mut context, surface)
        .unwrap();
    device.make_context_current(&context).unwrap();

    gl::load_with(|symbol_name| device.get_proc_address(&context, symbol_name));

    let hidpi_factor = window.get_current_monitor().get_hidpi_factor();
    let physical_size = logical_size.to_physical(hidpi_factor);
    let framebuffer_size = vec2i(physical_size.width as i32, physical_size.height as i32);

    let default_framebuffer = device
        .context_surface_info(&context)
        .unwrap()
        .unwrap()
        .framebuffer_object;

    let pathfinder_device = GLDevice::new(GLVersion::GL3, default_framebuffer);

    let mode = DestFramebuffer::full_window(framebuffer_size);
    let options = RendererOptions {
        background_color: Some(ColorF::white()),
    };
    let resource_loader = EmbeddedResourceLoader::new();
    let mut renderer = Renderer::new(pathfinder_device, &resource_loader, mode, options);

    let font_context = CanvasFontContext::from_system_source();
    let mut is_first_render = true;
    events_loop.run_forever(|event| {
        let mut should_render = is_first_render;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::WindowEvent {
                event: WindowEvent::KeyboardInput { .. },
                ..
            } => return ControlFlow::Break,
            Event::WindowEvent {
                event: WindowEvent::Refresh,
                ..
            } => should_render = true,
            _ => {}
        }
        if should_render {
            let mut canvas =
                Canvas::new(framebuffer_size.to_f32()).get_context_2d(font_context.clone());

            canvas.set_line_width(10.0);
            canvas.stroke_rect(RectF::new(vec2f(75.0, 140.0), vec2f(150.0, 110.0)));
            canvas.fill_rect(RectF::new(vec2f(130.0, 190.0), vec2f(40.0, 60.0)));

            let mut path = Path2D::new();
            path.move_to(vec2f(50.0, 140.0));
            path.line_to(vec2f(150.0, 60.0));
            path.line_to(vec2f(250.0, 140.0));
            path.close_path();
            canvas.stroke_path(path);

            let scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(), RayonExecutor);
            scene.build_and_render(&mut renderer, BuildOptions::default());

            let mut surface = device
                .unbind_surface_from_context(&mut context)
                .unwrap()
                .unwrap();
            device.present_surface(&mut context, &mut surface).unwrap();
            device
                .bind_surface_to_context(&mut context, surface)
                .unwrap();
        }
        is_first_render = false;
        ControlFlow::Continue
    });

    drop(device.destroy_context(&mut context));
}
