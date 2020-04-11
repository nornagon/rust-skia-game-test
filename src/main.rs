//use skia_safe::{Color, Data, EncodedImageFormat, Paint, PaintStyle, Path, Surface};
use skia_safe::gpu::{Context, BackendRenderTarget, SurfaceOrigin};
use skia_safe::gpu::gl::{FramebufferInfo, Format};
use skia_safe::{ColorType, Surface, Color};
use std::convert::TryInto;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile};

extern crate gl;
use gl::types::*;



fn main() {

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let cb = ContextBuilder::new()
        .with_depth_buffer(0)
        .with_stencil_buffer(8)
        .with_pixel_format(24, 8)
        .with_double_buffer(Some(true))
        .with_gl_profile(GlProfile::Core)
        //.with_srgb(false)
        ;
    let windowed_context =
        cb.build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    let pixel_format = windowed_context.get_pixel_format();

    println!("Pixel format of the window's GL context: {:?}", pixel_format);

    gl::load_with(|s| windowed_context.get_proc_address(&s) as *const _);

    let mut gr_context = Context::new_gl(None).unwrap();

    let mut fboid: GLint = 0;
    unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

    let fb_info = FramebufferInfo {
        fboid: fboid.try_into().unwrap(),
        format: 0x8058 /* GR_GL_RGBA8, see https://github.com/rust-skia/rust-skia/issues/311 */,
    };

    let size = windowed_context.window().inner_size();
    let backend_render_target = BackendRenderTarget::new_gl(
        (size.width.try_into().unwrap(), size.height.try_into().unwrap()),
        pixel_format.multisampling.map(|s| s.try_into().unwrap()),
        pixel_format.stencil_bits.try_into().unwrap(),
        fb_info
    );
    let mut surface = Surface::from_backend_render_target(
        &mut gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None
    ).unwrap();
    surface.canvas().clear(Color::WHITE);

    el.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size)
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                //gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                surface.canvas().flush();
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });

    /*
    let width = 200;
    let height = 200;
    let mut surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
    let path = Path::new();
    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.set_anti_alias(true);
    paint.set_stroke_width(1.0);
    surface.canvas().clear(Color::WHITE);
    let image = surface.image_snapshot();
    let data = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    let mut file = File::create("test.png").unwrap();
    let bytes = data.as_bytes();
    file.write_all(bytes).unwrap();
    */
}