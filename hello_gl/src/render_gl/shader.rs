use super::*;



#[derive(Debug)]
pub struct Shader {
    gl: Context,
    id: types::GLuint,
    kind: types::GLenum,
}


impl Shader {
    pub fn new(gl: Context) -> Shader {
        Shader { gl, id: 0, kind: 0 }
    }
    pub fn end(self) -> Shader { self }
    pub fn comp_vert_source(mut self, source: &CString) -> Result<Shader, String> {
        match self.compile_source(source, GL::VERTEX_SHADER) {
            Ok(_) => Ok(self),
            Err(e) => Err(e)
        }
    }

    pub fn comp_frag_source(mut self, source: &CString) -> Result<Shader, String> {
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

