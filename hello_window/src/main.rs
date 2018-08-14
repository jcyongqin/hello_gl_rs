#[macro_use]
extern crate glium;

use glium::glutin::GlContext;

fn main() {
    use glium::{glutin, Surface};
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();


    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        // poll event
        events_loop.poll_events(|evt| {
            match evt {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = display.gl_window().get_hidpi_factor();
                        display.gl_window().resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => ()
                },
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::Key(input) => {
                        if input.virtual_keycode.unwrap() == glutin::VirtualKeyCode::Escape {
                            closed = true;
                        }
                    }
                    _ => ()
                },
                _ => ()
            }
        });
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

