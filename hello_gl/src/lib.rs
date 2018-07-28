/// #将主要逻辑写在mod中
extern crate sdl2;
extern crate libgl;

pub mod render_gl;

pub use sdl2::{EventPump, VideoSubsystem, video::Window};


pub fn init_video() -> Result<(sdl2::VideoSubsystem, EventPump), String> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::Core);
    video_subsystem.gl_attr().set_context_version(2, 1);
    let event_pump = sdl.event_pump().unwrap();
    Ok((video_subsystem, event_pump))
}

pub fn create_window(
    video_subsystem: &sdl2::VideoSubsystem,
    name: &str,
    width: i32,
    height: i32,
) -> std::result::Result<sdl2::video::Window, String> {
    let window = video_subsystem
        .window(name, width as u32, height as u32)
        .opengl() // add opengl flag
        .resizable()
        .build()
        .unwrap();
    Ok(window)
}