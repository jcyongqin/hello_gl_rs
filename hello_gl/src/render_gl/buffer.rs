use super::*;

pub struct Buffer {
    gl: Context,
    id: types::GLuint,
    target: Option<types::GLenum>,
    usage: Option<types::GLenum>,
}

impl Buffer {
    pub fn new(gl: Context) -> Self {
        let mut id: types::GLuint = 0;
        unsafe { gl.GenBuffers(1, &mut id); };
        Buffer { gl, id, target: None, usage: None }
    }

    pub fn data<T>(mut self, data: &[T], target: types::GLenum, usage: types::GLenum)
                   -> Result<Self, String> {
        self.target = match target {
            GL::ARRAY_BUFFER
            | GL::ELEMENT_ARRAY_BUFFER
            | GL::PIXEL_PACK_BUFFER
            | GL::PIXEL_UNPACK_BUFFER => Some(target),
            _ => None,
        };
        self.usage = match usage {
            GL::STREAM_DRAW
            | GL::STREAM_READ
            | GL::STREAM_COPY
            | GL::STATIC_DRAW
            | GL::STATIC_READ
            | GL::STATIC_COPY
            | GL::DYNAMIC_DRAW
            | GL::DYNAMIC_READ
            | GL::DYNAMIC_COPY => Some(usage),
            _ => None,
        };

        unsafe {
            self.gl.BindBuffer(target, self.id);
            self.gl.BufferData(
                self.target.unwrap(),
                (data.len() * mem::size_of::<T>()) as types::GLsizeiptr,
                data.as_ptr() as *const types::GLvoid,
                self.usage.unwrap(),
            );
        }
        Ok(self)
    }
}

