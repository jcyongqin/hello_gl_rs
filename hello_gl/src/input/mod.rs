use sdl2::keyboard;
use sdl2::event::Event as Event;
use lazy_static;
use keyboard::Keycode;
use keyboard::Scancode;
use keyboard::Mod;
use sdl2::mouse::MouseButton;
use sdl2::mouse::MouseState;

lazy_static! {

static ref INPUT_STATE: InputState = InputState::Used;
}

#[warn(dead_code)]
enum InputState {
    Used,
    KeyDown {
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool,
    },
    KeyUp {
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool,
    },
    MouseMotion {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mousestate: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32
    },

    MouseButtonDown {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32
    },
    MouseButtonUp {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32
    },
}


pub fn event_handle(event: Event) -> bool {
    match event {
        Event::Quit { .. } => {}
        Event::AppTerminating { .. } => {}
        Event::AppLowMemory { .. } => {}
        Event::AppWillEnterBackground { .. } => {}
        Event::AppDidEnterBackground { .. } => {}
        Event::AppWillEnterForeground { .. } => {}
        Event::AppDidEnterForeground { .. } => {}
        Event::Window { .. } => {}
        Event::KeyDown { .. } => {}
        Event::KeyUp { .. } => {}
        Event::TextEditing { .. } => {}
        Event::TextInput { .. } => {}
        Event::MouseMotion { .. } => {}
        Event::MouseButtonDown { .. } => {}
        Event::MouseButtonUp { .. } => {}
        Event::MouseWheel { .. } => {}
        Event::JoyAxisMotion { .. } => {}
        Event::JoyBallMotion { .. } => {}
        Event::JoyHatMotion { .. } => {}
        Event::JoyButtonDown { .. } => {}
        Event::JoyButtonUp { .. } => {}
        Event::JoyDeviceAdded { .. } => {}
        Event::JoyDeviceRemoved { .. } => {}
        Event::ControllerAxisMotion { .. } => {}
        Event::ControllerButtonDown { .. } => {}
        Event::ControllerButtonUp { .. } => {}
        Event::ControllerDeviceAdded { .. } => {}
        Event::ControllerDeviceRemoved { .. } => {}
        Event::ControllerDeviceRemapped { .. } => {}
        Event::FingerDown { .. } => {}
        Event::FingerUp { .. } => {}
        Event::FingerMotion { .. } => {}
        Event::DollarGesture { .. } => {}
        Event::DollarRecord { .. } => {}
        Event::MultiGesture { .. } => {}
        Event::ClipboardUpdate { .. } => {}
        Event::DropFile { .. } => {}
        Event::User { .. } => {}
        Event::Unknown { .. } => {}
    }
    true
}