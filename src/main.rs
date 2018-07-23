extern crate gl;
extern crate glutin;

use glutin::*;
use std::result::Result;
use glutin::GlProfile;
use glutin::dpi::*;
use glutin::GlContext;

struct RenderState {}

fn start() {}

fn render() {}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();// 消息循环
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_resizable(false)
        .with_dimensions(LogicalSize::new(1024.0, 768.0));// 创建窗口
    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(GlProfile::Core)
        .with_vsync(true);// GL上下文
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    make_current(&gl_window).unwrap();
    load_gl(&gl_window);

    unsafe {
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        gl_window.swap_buffers().unwrap();
    }
}

fn make_current(gl_window: &glutin::GlWindow) -> Result<(), glutin::ContextError> {
    unsafe {
        gl_window.make_current() // 获得GL上下文
    }
}

fn load_gl(gl_window: &glutin::GlWindow) {
    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);// 加载GL函数
    }
}
