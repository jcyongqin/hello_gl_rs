extern crate hello_gl;
extern crate sdl2;
extern crate cgmath as vm;

use hello_gl::*;
use hello_gl::render_gl::{
    GL,
    Shader,
    Program,
    Buffer,
    RcGl,
    types,
};

use vm::prelude::*;

type Mat4 = vm::Matrix4<f32>;

use std::*;

use std::ffi::{CStr, CString};
use std::convert::From;
use std::fs::File;
use std::io::{Read, BufRead};
use std::path::Path;

fn start(gl: RcGl) -> Program {
    unsafe {
        gl.Enable(GL::DEPTH_TEST);
        gl.Enable(GL::BLEND);
        gl.BlendFunc(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
        gl.ClearColor(0.2, 0.2, 0.6, 1.0);
    }

    let vert_shader = Shader::new(gl.clone())
        .comp_vert_source(&load_file("./asset/triangle.vert").unwrap())
        .unwrap().end();

    let frag_shader = Shader::new(gl.clone())
        .comp_frag_source(&load_file("./asset/triangle.frag").unwrap())
        .unwrap().end();
    let shader_program = Program::new(gl.clone())
        .link_shaders(&[vert_shader, frag_shader])
        .unwrap();

    let vertices: Vec<f32> = vec![
        // positions        // colors       // UV
        -0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0,
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
        0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
        -0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
        -0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 1.0,
        -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0,
        -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0,
        -0.5, 0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0,
        -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0,
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
        0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 1.0, 0.0,
        0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 1.0,
        0.5, -0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0,
        0.5, -0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 1.0,
        0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0,
        0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0,
        -0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0,
        -0.5, 0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
        0.5, 0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
        -0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 0.0,
    ];
    let vbo = Buffer::gen(gl.clone(), GL::ARRAY_BUFFER);
    vbo.bind();
    vbo.data(&vertices, GL::STATIC_DRAW);

//    let mut vbo: types::GLuint = 0;
//    unsafe {
//        gl.GenBuffers(1, &mut vbo);
//        gl.BindBuffer(GL::ARRAY_BUFFER, vbo);
//        gl.BufferData(
//            GL::ARRAY_BUFFER, // target
//            (vertices.len() * std::mem::size_of::<f32>()) as types::GLsizeiptr, // size of data in bytes
//            vertices.as_ptr() as *const types::GLvoid, // pointer to data
//            GL::STATIC_DRAW, // usage
//        );
//    }


    let indices: Vec<u32> = vec![
        // 注意索引从0开始!
        0, 1, 3, 1, 2, 3,   // 第一面
        4, 5, 7, 5, 6, 7,   // 第二面
        8, 9, 11, 9, 10, 11,
        12, 13, 15, 13, 14, 15,
        16, 17, 19, 17, 18, 19,
        20, 21, 23, 21, 22, 23,
    ];
    let mut ebo: types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut ebo);
        gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, ebo);
        gl.BufferData(
            GL::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as types::GLsizeiptr,
            indices.as_ptr() as *const types::GLvoid,
            GL::STATIC_DRAW,
        );
    }

    unsafe {
        let vert_loc: types::GLint = gl.GetAttribLocation(
            shader_program.id(), CString::new("a_vertex").unwrap().as_ptr());

        let color_loc: types::GLint = gl.GetAttribLocation(
            shader_program.id(), CString::new("a_color").unwrap().as_ptr());

        let texcoord_loc: types::GLint = gl.GetAttribLocation(
            shader_program.id(), CString::new("a_texcoord").unwrap().as_ptr());

        gl.EnableVertexAttribArray(vert_loc as types::GLuint); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            vert_loc as types::GLuint, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            GL::FLOAT, // data type
            GL::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(), // offset of the first component
        );
        print_gl_err(gl.clone(), "ss col ");

        gl.EnableVertexAttribArray(color_loc as types::GLuint); // this is "layout (location = 0)" in vertex shader
        print_gl_err(gl.clone(), "start col ");

        gl.VertexAttribPointer(
            color_loc as types::GLuint, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            GL::FLOAT, // data type
            GL::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const types::GLvoid, // offset of the first component
        );
        print_gl_err(gl.clone(), "start img ");
        gl.EnableVertexAttribArray(texcoord_loc as types::GLuint); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            texcoord_loc as types::GLuint, // index of the generic vertex attribute ("layout (location = 0)")
            2, // the number of components per generic vertex attribute
            GL::FLOAT, // data type
            GL::FALSE, // normalized (int-to-float conversion)
            (8 * std::mem::size_of::<f32>()) as types::GLint, // stride (byte offset between consecutive attributes)
            (6 * std::mem::size_of::<f32>()) as *const types::GLvoid, // offset of the first component
        );
    }
    unsafe {

        // load and create a texture
        // -------------------------
        let mut texture1: types::GLuint = 0;
        // texture 1
        // ---------
        gl.GenTextures(1, &mut texture1);
        gl.BindTexture(GL::TEXTURE_2D, texture1);
        // set the texture wrapping parameters
        gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as types::GLint);    // set texture wrapping to GL_REPEAT (default wrapping method)
        gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as types::GLint);
        // set texture filtering parameters
        gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as types::GLint);
        gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as types::GLint);
        // load image, create texture and generate mipmaps

        // The FileSystem::getPath(...) is part of the GitHub repository so we can find files on any IDE/platform; replace it with your own image path.
        let img = open(Path::new("./asset/awesomeface.png")).unwrap().flipv();//.to_rgba();

        let img = img.to_rgba();
        print_gl_err(gl.clone(), "start MVP ");

        gl.TexImage2D(GL::TEXTURE_2D,
                      0,
                      GL::RGBA as types::GLint,
                      img.width() as types::GLint,
                      img.height() as types::GLint,
                      0,
                      GL::RGBA,
                      GL::UNSIGNED_BYTE,
                      img.as_ptr() as *const types::GLvoid,
        );
    }


    return shader_program;
}


fn update(gl: RcGl, shader: &Program, now: time::Instant) {
    thread::sleep(time::Duration::from_millis(5));

    let mut angle = now.elapsed().subsec_nanos() as f32 /4000000.0 % 720.0;

    println!("{:?}", &angle);
    let trans = Mat4::from_translation(vm::vec3(0.0, 0.0, 0.0));
    let rotate_x = Mat4::from_angle_x(vm::Deg(45.0));
    let rotate_y = Mat4::from_angle_y(vm::Deg(angle));
    let rotate_z = Mat4::from_angle_z(vm::Deg(angle));
    let scale = Mat4::from_scale(0.5);
    let MVP = trans * rotate_x * rotate_y * rotate_z * scale;

    shader.set_used();
    unsafe {
        let a: [[f32; 4]; 4] = MVP.into();


        let mvp_matrix_loc = gl.GetUniformLocation(shader.id(), CString::new("mvp_matrix").unwrap().as_ptr());
        print_gl_err(gl.clone(), "end MVP");
        gl.UniformMatrix4fv(mvp_matrix_loc, 1, GL::FALSE,
                            a.as_ptr() as *const types::GLfloat);
    }
}

fn render(gl: RcGl, shader: &Program) {
    unsafe {
        gl.Clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
//        gl.PolygonMode(GL::FRONT_AND_BACK,GL::LINE);
        shader.set_used();
        gl.DrawElements(
            GL::TRIANGLES,
            36,
            GL::UNSIGNED_INT,
            0 as *const types::GLvoid,
        );

//        gl.DrawArrays(
//            GL::TRIANGLES, // mode
//            0, // starting index in the enabled arrays
//            6, // number of indices to be rendered
//        );
    }
}

fn load_opengl(window: &Window) -> (GLContext, RcGl) {
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    let video = window.subsystem();
    let gl = render_gl::RcGl::load_with(move |s| {
        video.gl_get_proc_address(s) as *const std::os::raw::c_void
    });
    return (gl_context, gl);
}

fn main() {
    let (video, mut event_pump) = init_sys().unwrap();
    let window = create_window(&video, "Game", 900, 700).unwrap();
    let (gl_context, gl) = load_opengl(&window);

//    let gl_context = window.gl_create_context().unwrap();
//    window.gl_make_current(&gl_context).unwrap();
//
//   let gl= render_gl::RcGl::load_with(|s| {
//        v.gl_get_proc_address(s) as *const std::os::raw::c_void
//    }).clone();

    let now = time::Instant::now();
    let shader = start(gl.clone());
    print_gl_err(gl.clone(), "end start");
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } =>
                    break 'main,
                sdl2::event::Event::KeyDown { scancode, .. } =>
                    if scancode.unwrap() == sdl2::keyboard::Scancode::Escape { break 'main; }
                sdl2::event::Event::Window { win_event, .. } => unsafe {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(w, h) => {
                            gl.Viewport(0, 0, w, h);
                            //println!("{},{}", w, h)
                        }
                        _ => ()
                    }
                }
                _ => {}
            }
        }
        update(gl.clone(), &shader, now);
        render(gl.clone(), &shader);
        window.gl_swap_window();
    }
}

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

fn print_gl_err(gl: RcGl, tag: &str) {
    unsafe {
        match gl.GetError() {
            0 => return,
            0x0500 => print!("INVALID_ENUM: 0x0500"),
            0x0501 => print!("INVALID_VALUE: 0x0501"),
            0x0502 => print!("INVALID_OPERATION: 0x0502;"),
            err @ _ => print!("UNKNOW ERR:0x{:x}", err),
        };
        println!("{}", tag);
    }
}
