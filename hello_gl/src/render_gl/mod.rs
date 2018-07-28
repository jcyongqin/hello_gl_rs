use libgl as gl;
pub use libgl::RcGl;
use libgl::types;


use std::ffi::{CString, CStr};
use std::ptr;

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

    pub fn comp_vert_source(&self, source: &CString) -> Result<(), String> {
        match self.compile_source(source, gl::VERTEX_SHADER) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    pub fn comp_frag_source(&self, source: &CString) -> Result<(), String> {
        match self.compile_source(source, gl::FRAGMENT_SHADER) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn compile_source(&self, source: &CString, kind: types::GLenum) -> Result<(), String> {
        let gl = self.gl;
        let id = unsafe { gl.CreateShader(kind) };
        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), ptr::null());
            gl.CompileShader(id);
        }
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        return if success != 0 {
            self.id = id;
            Ok(())
        } else {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::RcGl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::RcGl::GetShaderInfoLog(
                    id,
                    len,
                    ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
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
    gl: gl::RcGl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn new(gl: RcGl) -> Program { Program { gl, id: 0 } }

    pub fn link_shaders(&self, shaders: &[Shader]) -> Result<(), String> {
        let gl = self.gl;
        self.id = unsafe { gl.CreateProgram() };
        for shader in shaders {
            unsafe { gl.AttachShader(self.id, shader.id()); }
        }
        unsafe { gl.LinkProgram(self.id); }
        let mut success: gl::types::GLint = 1;
        unsafe {
            self.gl.GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        }
        return if success != 0 {
            Ok(())
        } else {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                self.gl.GetProgramInfoLog(
                    self.id,
                    len,
                    ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
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
