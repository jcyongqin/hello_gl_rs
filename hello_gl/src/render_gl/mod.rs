pub use libgl::*;
use libgl as GL;

use std::ffi::{CString};
use std::ptr;
use ::resources;

#[derive(Debug)]
pub enum Error {
    ResourceLoad { name: String, inner: resources::Error },
    CanNotDetermineShaderTypeForResource { name: String },
    CompileError { name: String, message: String },
    LinkError { name: String, message: String },
}

#[derive(Debug)]
pub struct Shader {
    gl: RcGl,
    id: types::GLuint,
    kind: types::GLenum,
}


impl Shader {
    pub fn new(gl: RcGl) -> Shader {
        Shader { gl, id: 0, kind: 0 }
    }
    pub fn end(self) -> Shader { self }
    pub fn comp_vert_source(mut self, source: &CString) -> Result<Shader, String> {
        match self.compile_source(source, GL::VERTEX_SHADER) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }

    pub fn comp_frag_source(mut self, source: &CString) -> Result< Shader, String> {
        match self.compile_source(source, GL::FRAGMENT_SHADER) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }

    fn compile_source(&mut self, source: &CString, kind: types::GLenum) -> Result<(), String> {
        let gl = self.gl.clone();
        let id = unsafe { gl.CreateShader(kind) };
        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), ptr::null());
            gl.CompileShader(id);
        }
        let mut success: types::GLint = 1;
        unsafe {
            gl.GetShaderiv(id, GL::COMPILE_STATUS, &mut success);
        }
        return if success != 0 {
            self.id = id;
            Ok(())
        } else {
            let mut len: types::GLint = 0;
            unsafe { gl.GetShaderiv(id, GL::INFO_LOG_LENGTH, &mut len); }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl.GetShaderInfoLog(
                    id,
                    len,
                    ptr::null_mut(),
                    error.as_ptr() as *mut types::GLchar,
                );
            }
            Err(error.to_string_lossy().into_owned())
        };
    }

    pub fn id(&self) -> types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}


fn create_whitespace_cstring_with_len(len: usize) -> CString {
    unsafe { CString::from_vec_unchecked(vec![0u8; len + 1]) }

//// allocate buffer of correct size
//    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
//// fill it with len spaces
//    buffer.extend([b' '].iter().cycle().take(len));
//// convert buffer to CString
//    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct Program {
    gl: RcGl,
    id: types::GLuint,
}

impl Program {
    pub fn new(gl: RcGl) -> Program { Program { gl, id: 0 } }

    pub fn link_shaders(mut self, shaders: &[Shader]) -> Result< Program, String> {
        let gl = self.gl.clone();
        self.id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe { gl.AttachShader(self.id, shader.id()); }
        }
        unsafe { gl.LinkProgram(self.id); }
        let mut success: types::GLint = 1;
        unsafe {
            gl.GetProgramiv(self.id, GL::LINK_STATUS, &mut success);
        }
        return if success != 0 {
            Ok(self)
        } else {
            let mut len: types::GLint = 0;
            unsafe {
                gl.GetProgramiv(self.id, GL::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl.GetProgramInfoLog(
                    self.id,
                    len,
                    ptr::null_mut(),
                    error.as_ptr() as *mut types::GLchar,
                );
            }
            Err(error.to_string_lossy().into_owned())
        };
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn get_attr_loc(&self, name: CString) -> types::GLint {
        unsafe {
            self.gl.GetAttribLocation(self.id, name.as_ptr())
        }
    }

    pub fn id(&self) -> types::GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}
