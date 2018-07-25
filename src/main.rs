extern crate sdl2;
extern crate gl;
extern crate hello_gl_rs;

mod render_gl;

use render_gl::{
    Shader,
    Program,
    init_video,
    create_window,
};

use std::ffi::CStr;
use gl::types::*;


fn start() {
    let vert_shader = Shader::from_source(
        &CStr::from_bytes_with_nul(b"<source code here>\0").unwrap(),
        gl::VERTEX_SHADER,
    ).unwrap();
    let frag_shader = Shader::from_source(
        &CStr::from_bytes_with_nul(b"<>\0").unwrap(),
        gl::FRAGMENT_SHADER,
    ).unwrap();

    let program_id = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program_id, vert_shader.id());
        gl::AttachShader(program_id, frag_shader.id());
        gl::LinkProgram(program_id);
        gl::DetachShader(program_id, vert_shader.id());
        gl::DetachShader(program_id, frag_shader.id());
    }
}

fn main() {
    let (video, mut event_pump) = init_video().unwrap();
    let window = create_window(&video, "Game", 900, 700).unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
        gl::ClearColor(0.1, 0.1, 0.8, 1.0);
    }
    start();




    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } =>
                    break 'main,
                sdl2::event::Event::KeyDown { scancode: code, .. } =>
                    if code.unwrap() == sdl2::keyboard::Scancode::Escape { break 'main; }
                _ => {}
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
    }
}
