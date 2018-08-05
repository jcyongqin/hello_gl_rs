/// #将主要逻辑写在mod中
extern crate sdl2;
extern crate image;
extern crate libgl;

pub use render_gl::Context;
pub use sdl2::{EventPump, video::Window, video::GLContext, video::SwapInterval};
pub use image::{open, DynamicImage, RgbImage, GenericImage};

pub mod render_gl;
pub mod resources;

pub struct Application {
    app: sdl2::Sdl,
}

impl Application {
    pub fn init() -> Self {
        Application { app: sdl2::init().unwrap() }
    }

    pub fn create_window(&self, name: &str, width: u32, height: u32) -> Option<Window> {
        let window = self.app.video()
            .unwrap()
            .window(name, width, height)
            .resizable()
            .opengl()
            .build()
            .unwrap();
        Some(window)
    }

    pub fn event_pump(&self) -> Option<EventPump> {
        let event_pump = self.app.event_pump().unwrap();
        Some(event_pump)
    }

    pub fn create_context() {}

    pub fn create_standalone_context(&self, vwindow: Window, major: u8, minor: u8)
                                     -> Context {
        let window = unsafe { Window::from_ref(vwindow.context().clone()) };
        let video = vwindow.subsystem();
        video.gl_attr().set_context_version(major, minor);
        let gl_context = vwindow.gl_create_context().unwrap();
        vwindow.gl_make_current(&gl_context);

        let load_gl_fn = move |s| {
            video.gl_get_proc_address(s) as *const std::os::raw::c_void
        };

        Context::load_gl(gl_context, window, load_gl_fn)
    }
}




