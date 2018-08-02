pub mod texture;
pub mod shader;
pub mod program;

pub use libgl::{GL, RcGl, GL::types};
pub use self::shader::Shader;
pub use self::program::Program;

use std::mem;
use std::ffi::CString;
use std::ptr;
use ::resources;

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name: String, inner: resources::Error },
    CanNotDetermineShaderTypeForResource { name: String },
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    unsafe { CString::from_vec_unchecked(vec![0u8; len + 1]) }
}

pub struct Buffer {
    gl: RcGl,
    id: types::GLuint,
    kind: types::GLenum,
}

impl Buffer {
    pub fn gen(gl: RcGl, kind: types::GLenum) -> Self {
        let mut id: types::GLuint = 0;
        unsafe { gl.GenBuffers(1, &mut id); }
        Buffer {
            gl,
            id,
            kind,
        }
    }
    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(self.kind, self.id) }
    }
    pub fn data<T>(&self, data: &[T], usage: types::GLenum) {
        unsafe {
            self.gl.BufferData(
                self.kind,
                (data.len() * mem::size_of::<T>()) as types::GLsizeiptr,
                data.as_ptr() as *const types::GLvoid,
                usage,
            );
        }
    }
}