use super::*;
use render_gl::shader::Shader;

pub struct Program {
    gl: Context,
    id: types::GLuint,
}

impl Program {
    pub fn new(gl: Context) -> Program { Program { gl, id: 0 } }

    pub fn link_shaders(mut self, shaders: &[Shader]) -> Result<Program, String> {
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

    // uniform工具函数
    pub fn set_int(&self, name: CString, value: i32) {
        let uniform_loc = unsafe {
            self.gl.GetUniformLocation(self.id, name.as_ptr())
        };
        self.set_used();
        unsafe { self.gl.Uniform1i(uniform_loc, value) }
    }

    pub fn set_float(&self, name: CString, value: f32) {
        let uniform_loc = unsafe {
            self.gl.GetUniformLocation(self.id, name.as_ptr())
        };
        self.set_used();
        unsafe { self.gl.Uniform1f(uniform_loc, value) }
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
