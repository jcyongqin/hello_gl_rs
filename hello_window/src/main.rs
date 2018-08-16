extern crate hello_window;
#[macro_use]
extern crate glium;

use hello_window::*;
use hello_window::world::*;

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();


    // Only the second entity will get a position update,
    // because the first one does not have a velocity.
    world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();
    world.add_resource(DeltaTime(0.05)); // Let's use some start value

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .build();

    // loop0
    dispatcher.dispatch(&mut world.res);
    let mut delta = world.write_resource::<DeltaTime>();
    *delta = DeltaTime(0.04);
    // loop1
    dispatcher.dispatch(&mut world.res);
    world.maintain();
}

fn _main() {
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

