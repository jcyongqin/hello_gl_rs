#[macro_use]
extern crate glium;
extern crate specs;




pub use glium::glutin::GlContext;
pub use glium::{glutin, Surface};

pub mod ecs;

pub use ecs::world;