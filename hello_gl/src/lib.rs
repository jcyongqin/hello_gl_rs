/// #将主要逻辑写在mod中
extern crate sdl2;
extern crate image;
extern crate libgl;

pub mod render_gl;
pub mod resources;

pub use sdl2::{EventPump, VideoSubsystem, video::Window, video::GLContext, video::SwapInterval};
pub use image::{open, DynamicImage, RgbImage,GenericImage};

pub fn init_sys() -> Option<(VideoSubsystem, EventPump)> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::Core);
    video_subsystem.gl_attr().set_context_version(2, 1);

    let event_pump = sdl.event_pump().unwrap();
    Some((video_subsystem, event_pump))
}

pub fn create_window(
    video_subsystem: &VideoSubsystem,
    name: &str,
    width: i32,
    height: i32,
) -> Option<Window> {
    let window = video_subsystem
        .window(name, width as u32, height as u32)
        .opengl() // add opengl flag
        .resizable()
        .build()
        .unwrap();
    Some(window)
}

