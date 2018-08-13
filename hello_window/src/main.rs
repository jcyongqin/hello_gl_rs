extern crate libgl;
extern crate glutin;

use glutin::dpi::*;
use glutin::GlContext;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    gl_window.set_cursor(glutin::MouseCursor::Default);


    unsafe {
        gl_window.make_current().unwrap();
    }

    let gl =
        libgl::Gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    unsafe {
        gl.ClearColor(0.0, 1.0, 0.0, 1.0);
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
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::Key(input) => {
                        if input.virtual_keycode.unwrap() == glutin::VirtualKeyCode::Escape {
                            running = false;
                        }
                    }
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl.Clear(libgl::constants::COLOR_BUFFER_BIT);
        }

        gl_window.swap_buffers().unwrap();
    }
}