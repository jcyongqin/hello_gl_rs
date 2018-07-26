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

use std::*;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::BufRead;
use std::convert::From;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi;
use gl::types::*;


fn load_file(path: &str) -> Result<ffi::CString, String> {
    let mut file = File::open(path).unwrap();
    // allocate buffer of the same size as file
    let mut buffer: Vec<u8> = Vec::with_capacity(
        file.metadata().unwrap().len() as usize + 1
    );
    file.read_to_end(&mut buffer).unwrap();
    // check for nul byte
    if buffer.iter().find(|i| **i == 0).is_some() {
        return Err(String::from("FileContainsNil"));
    }
    Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
}

fn start() {
    let vert_shader = Shader::from_source(
        load_file("./asset/triangle.vert").unwrap().as_c_str(),
        gl::VERTEX_SHADER,
    ).unwrap();
    let frag_shader = Shader::from_source(
        load_file("./asset/triangle.frag").unwrap().as_c_str(),
        gl::FRAGMENT_SHADER,
    ).unwrap();
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
//        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
//        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let vert_loc: gl::types::GLint = gl::GetAttribLocation(shader_program.id(), CString::new("a_vertex").unwrap().as_ptr());

        gl::EnableVertexAttribArray(vert_loc as GLuint); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            vert_loc as GLuint, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(), // offset of the first component
        );
    }
    shader_program.set_used();
}

fn render() {
    unsafe {
        gl::DrawArrays(
            gl::TRIANGLES, // mode
            0, // starting index in the enabled arrays
            3, // number of indices to be rendered
        );
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
        render();
        window.gl_swap_window();
    }
}
