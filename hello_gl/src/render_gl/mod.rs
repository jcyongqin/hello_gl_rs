pub use libgl::constants as GL;
pub use libgl::types;

pub use sdl2::video::{Window, GLContext};

use std::rc::Rc;
use std::ops::Deref;
use std::fmt::Debug;
use std::fmt;
use std::mem;
use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;
use ::resources;
use self::program::Program;
use self::shader::Shader;
use self::buffer::Buffer;
use self::texture::Texture;

pub mod texture;
pub mod shader;
pub mod program;
pub mod buffer;


struct _Context {
    gl: GL::Gl,
    context: GLContext,
    window: Window,
}

#[derive(Clone)]
pub struct Context {
    inner: Rc<_Context>,
}

impl Context {
    pub fn load_gl<F>(context: GLContext, window: Window, mut load_fn: F)
                      -> Self where F: FnMut(&'static str) -> *const types::GLvoid {
        let gl = GL::Gl::load_with(load_fn);
        let version = unsafe {
            let data = CStr::from_ptr(gl.GetString(GL::VERSION) as *const _)
                .to_bytes().to_vec();
            String::from_utf8(data).unwrap()
        };
        println!("OpenGL version {}", version);
        Context {
            inner: Rc::new(
                _Context {
                    gl,
                    context,
                    window,
                }
            )
        }
    }
    pub fn swap_window(&self) {
        self.inner.window.gl_swap_window()
    }

    pub fn program(&self, vertex_shader: &CString, fragment_shader: &CString)
                   -> Result<Program, String> {
        let vert_shader = Shader::new(self.clone())
            .comp_vert_source(vertex_shader).unwrap();
        let frag_shader = Shader::new(self.clone())
            .comp_frag_source(fragment_shader).unwrap();
        Program::new(self.clone()).link_shaders(&[vert_shader, frag_shader])
    }

    pub fn buffer<T>(&self, data: &[T], target: Option<types::GLenum>, usage: Option<types::GLenum>)
                     -> Buffer {
        let target = target.unwrap_or(GL::ARRAY_BUFFER);
        let usage = usage.unwrap_or(GL::STATIC_DRAW);

        let mut buffer = Buffer::new(self.clone());
        buffer.data::<T>(data, target, usage).unwrap()
    }
}


impl Deref for Context {
    type Target = GL::Gl;

    fn deref(&self) -> &GL::Gl {
        &self.inner.gl
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{Context}}")
    }
}

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name: String, inner: resources::Error },
    CanNotDetermineShaderTypeForResource { name: String },
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
}

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    unsafe { CString::from_vec_unchecked(vec![0u8; len + 1]) }
}

