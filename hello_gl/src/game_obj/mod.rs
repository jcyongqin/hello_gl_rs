use Behavior;
use Context;
use render_gl;
use render_gl::{GL, types, program::Program, shader::Shader, buffer::Buffer};
use vm;
use image::{open, DynamicImage, RgbImage, GenericImage};
use std::ffi::CString;
use std::path::Path;
use std;
use std::ffi;
use std::fs::File;
use std::io::{Read, BufRead};
use std::time;
use std::thread;

pub type Mat4 = vm::Matrix4<f32>;


pub struct GameObject {
    shader_prog: Program,
    now: time::Instant,
}

impl Behavior for GameObject {
    fn start(ctx: Context) -> GameObject {
        unsafe {
            ctx.Enable(GL::DEPTH_TEST);
            ctx.Enable(GL::BLEND);
            ctx.BlendFunc(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
            ctx.ClearColor(0.2, 0.2, 0.6, 1.0);
        }

        let vert_src = load_file("./asset/triangle.vert").unwrap();
        let frag_src = load_file("./asset/triangle.frag").unwrap();
        let shader_program = ctx.program(&vert_src, &frag_src).unwrap();


        let vertices: Vec<[f32; 32]> = vec![
            // positions          // colors      // UV
            [
                -0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
                -0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, ],
            [
                -0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 1.0,
                -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, ],
            [
                -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0,
                -0.5, 0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0,
                -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0,
                -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, ],
            [
                0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 1.0, 0.0,
                0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 1.0,
                0.5, -0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0,
                0.5, -0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 0.0, ],
            [
                -0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 1.0,
                0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0,
                0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0,
                -0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, ],
            [
                -0.5, 0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
                0.5, 0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
                0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
                -0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 0.0, ],
        ];

        let vbo = ctx.buffer(&vertices, Option::from(GL::ARRAY_BUFFER), Option::from(GL::STATIC_DRAW));

        let indices: Vec<u32> = vec![
            // 注意索引从0开始!
            0, 1, 3, 1, 2, 3,   // 第一面
            4, 5, 7, 5, 6, 7,   // 第二面
            8, 9, 11, 9, 10, 11,
            12, 13, 15, 13, 14, 15,
            16, 17, 19, 17, 18, 19,
            20, 21, 23, 21, 22, 23,
        ];
        let mut ebo = ctx.buffer(&indices, GL::ELEMENT_ARRAY_BUFFER.into(), GL::STATIC_DRAW.into());
        //test
        unsafe {
            let mut attr_sum = 0;
            let mut len = 0;
            ctx.GetProgramiv(shader_program.id(), GL::ACTIVE_ATTRIBUTES, &mut attr_sum);
            ctx.GetProgramiv(shader_program.id(), GL::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut len);
            let mut stri = render_gl::create_whitespace_cstring_with_len(len as usize);
            let mut name_len = 0;
            let mut array_len = 0;
            let mut types = 0;
            for i in 0..attr_sum {
                ctx.GetActiveAttrib(shader_program.id(),
                                    i as u32,
                                    len,
                                    &mut name_len,
                                    &mut array_len,
                                    &mut types,
                                    stri.as_ptr() as *mut types::GLchar,
                );
                println!("{:?}", &stri);
            }
        }

        unsafe {
            let vert_loc: types::GLint = ctx.GetAttribLocation(
                shader_program.id(), CString::new("a_vertex").unwrap().as_ptr());

            let color_loc: types::GLint = ctx.GetAttribLocation(
                shader_program.id(), CString::new("a_color").unwrap().as_ptr());

            let texcoord_loc: types::GLint = ctx.GetAttribLocation(
                shader_program.id(), CString::new("a_texcoord").unwrap().as_ptr());

            ctx.EnableVertexAttribArray(vert_loc as types::GLuint); // this is "layout (location = 0)" in vertex shader
            ctx.VertexAttribPointer(
                vert_loc as types::GLuint, // index of the generic vertex attribute ("layout (location = 0)")
                3, // the number of components per generic vertex attribute
                GL::FLOAT, // data type
                GL::FALSE, // normalized (int-to-float conversion)
                (8 * std::mem::size_of::<f32>()) as types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(), // offset of the first component
            );
            print_gl_err(ctx.clone(), "ss col ");

            ctx.EnableVertexAttribArray(color_loc as types::GLuint); // this is "layout (location = 0)" in vertex shader
            print_gl_err(ctx.clone(), "start col ");

            ctx.VertexAttribPointer(
                color_loc as types::GLuint, // index of the generic vertex attribute ("layout (location = 0)")
                3, // the number of components per generic vertex attribute
                GL::FLOAT, // data type
                GL::FALSE, // normalized (int-to-float conversion)
                (8 * std::mem::size_of::<f32>()) as types::GLint, // stride (byte offset between consecutive attributes)
                (3 * std::mem::size_of::<f32>()) as *const types::GLvoid, // offset of the first component
            );
            print_gl_err(ctx.clone(), "start img ");
            ctx.EnableVertexAttribArray(texcoord_loc as types::GLuint); // this is "layout (location = 0)" in vertex shader
            ctx.VertexAttribPointer(
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
            ctx.GenTextures(1, &mut texture1);
            ctx.BindTexture(GL::TEXTURE_2D, texture1);
            // set the texture wrapping parameters
            ctx.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as types::GLint);    // set texture wrapping to GL_REPEAT (default wrapping method)
            ctx.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as types::GLint);
            // set texture filtering parameters
            ctx.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as types::GLint);
            ctx.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as types::GLint);
            // load image, create texture and generate mipmaps

            // The FileSystem::getPath(...) is part of the GitHub repository so we can find files on any IDE/platform; replace it with your own image path.
            let img = open(Path::new("./asset/awesomeface.png")).unwrap().flipv();//.to_rgba();

            let img = img.to_rgba();
            print_gl_err(ctx.clone(), "start MVP ");

            ctx.TexImage2D(GL::TEXTURE_2D,
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


        return GameObject {
            shader_prog: shader_program,
            now: time::Instant::now(),
        };
    }

    fn update(&mut self, ctx: Context) {
        thread::sleep(time::Duration::from_millis(5));

        let mut angle = self.now.elapsed().subsec_nanos() as f32 / 4000000.0 % 720.0;

        //println!("{:?}", &angle);
        let trans = Mat4::from_translation(vm::vec3(0.0, 0.0, 0.0));
        let rotate_x = Mat4::from_angle_x(vm::Deg(45.0));
        let rotate_y = Mat4::from_angle_y(vm::Deg(angle));
        let rotate_z = Mat4::from_angle_z(vm::Deg(angle));
        let scale = Mat4::from_scale(0.5);
        let MVP = trans * rotate_x * rotate_y * rotate_z * scale;

        self.shader_prog.set_used();
        unsafe {
            let a: [[f32; 4]; 4] = MVP.into();


            let mvp_matrix_loc = ctx.GetUniformLocation(self.shader_prog.id(), CString::new("mvp_matrix").unwrap().as_ptr());
            print_gl_err(ctx.clone(), "end MVP");
            ctx.UniformMatrix4fv(mvp_matrix_loc, 1, GL::FALSE,
                                 a.as_ptr() as *const types::GLfloat);
        }
    }

    fn render(&mut self, ctx: Context) {
        unsafe {
            ctx.Clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
//        gl.PolygonMode(GL::FRONT_AND_BACK,GL::LINE);
            self.shader_prog.set_used();
            ctx.DrawElements(
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

fn print_gl_err(gl: Context, tag: &str) {
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