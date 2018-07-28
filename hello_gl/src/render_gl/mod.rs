use ::sdl2;
use ::sdl2::*;
use ::libgl::Gl as gl;
use ::libgl;
use ::libgl::types;

use std;
use std::ffi::{CString, CStr};

pub fn init_video() -> Result<(VideoSubsystem, EventPump), String> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::Core);
    video_subsystem.gl_attr().set_context_version(2, 1);
    let event_pump = sdl.event_pump().unwrap();
    Ok((video_subsystem, event_pump))
}

pub fn create_window(video_subsystem: &VideoSubsystem, name: &str, width: i32, height: i32) -> Result<video::Window, String> {
    let window = video_subsystem
        .window(name, width as u32, height as u32)
        .opengl() // add opengl flag
        .resizable()
        .build()
        .unwrap();
    Ok(window)
}

pub struct Shader {
    id: libgl::types::GLuint,
}

impl Shader {
    pub fn from_source(
        source: &CStr,
        kind: types::GLenum,
    ) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, libgl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, libgl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(
    source: &CStr,
    kind: types::GLenum,
) -> Result<types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct Program {
    id: types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }
        unsafe { gl::LinkProgram(program_id); }
        // continue with error handling here
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }
        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }
        Ok(Program { id: program_id })
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn id(&self) -> types::GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}