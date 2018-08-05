use ::image::DynamicImage;


use ::image::{
    open, load,
};
use super::{types, Context};
use ::std::{path::Path};



pub struct Texture {
    gl: Context,
    id: types::GLuint,
    kind: types::GLenum,
}

impl Texture {
    pub fn gen(gl: Context, kind: types::GLenum) -> Self {
        let mut id: types::GLenum = 0;
        unsafe { gl.GenTextures(1, &mut id) };
        Texture {
            gl,
            id,
            kind,
        }
    }

    pub fn bind(&self){
        unsafe { self.gl.BindTexture(self.kind, self.id) }
    }


}