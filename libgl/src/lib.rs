//mod bindings {
//    include!( concat! ( env! ( "OUT_DIR" ), "/bindings.rs" ));
//}
pub mod bindings;

pub use bindings as GL;

use bindings::{types, Gl};

use std::ops::Deref;
use std::rc::Rc;
use std::ffi::CStr;
use std::fmt::{Result, Debug, Formatter};


#[derive(Clone, Debug)]
pub struct RcGl {
    inner: Rc<bindings::Gl>,
}

impl RcGl {
    pub fn load_with<F>(loadfn: F) -> RcGl
        where F: FnMut(&'static str) -> *const types::GLvoid {
        let gl = RcGl {
            inner: Rc::new(bindings::Gl::load_with(loadfn))
        };
        let version = unsafe {
            let data = CStr::from_ptr(gl.GetString(bindings::VERSION) as *const _).to_bytes().to_vec();
            String::from_utf8(data).unwrap()
        };
        println!("OpenGL version {}", version);
        return gl;
    }
}

impl Deref for RcGl {
    type Target = bindings::Gl;
    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}

impl Debug for Gl {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RcGl")
    }
}