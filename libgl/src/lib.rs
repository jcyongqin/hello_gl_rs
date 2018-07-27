mod bindings {
    include!( concat! ( env! ( "OUT_DIR" ), "/bindings.rs" ));
}

//pub use bindings::types as types;
//pub use bindings::Gl as InnerGl;
pub use bindings::*;


use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub
struct GlS {
    inner: Rc<bindings::Gl>,
}

impl GlS {
    pub fn load_with<F>(loadfn: F) -> Gl
        where F: FnMut(&'static str) -> *const types::GLvoid
    {
        GlS {
            inner: Rc::new(bindings::Gl::load_with(loadfn))
        }
    }
}

impl Deref for GlS {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}
