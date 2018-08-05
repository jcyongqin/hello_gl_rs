//mod bindings {
//    include!( concat! ( env! ( "OUT_DIR" ), "/bindings.rs" ));
//}
mod bindings;


pub use bindings::types;
pub use bindings::Gl;
pub use bindings::FnPtr;

pub mod constants {
    pub use bindings::*;
}
