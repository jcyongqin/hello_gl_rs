extern crate hello_gl;

use hello_gl::*;
use hello_gl::Behavior;
use hello_gl::game_obj::*;


fn main() {
    let app = Application::init();
    let window = app.create_window("Game 窗口", 900, 700).unwrap();
    let mut event_pump = app.event_pump().unwrap();
    let ctx = app.create_standalone_context(window, 2, 1);

    let mut game = GameObject::start(ctx.clone());
    'main: loop {
        for e in event_pump.poll_iter() {
            match e {
                event::Event::Quit { .. } =>
                    break 'main,
                event::Event::KeyDown { scancode, .. } =>
                    if scancode.unwrap() == keyboard::Scancode::Escape { break 'main; }

                _ => {}
            }
        }
        game.update(ctx.clone());
        game.render(ctx.clone());

        ctx.swap_window();
    }
}




