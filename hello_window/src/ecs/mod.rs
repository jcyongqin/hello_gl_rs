pub mod world;

use std::ops::Drop;

/// Scene Manager 目前强制加载内置Scene
struct World {
    scenes: Vec<Scene>,
}

impl World {
    pub fn new() -> Self {
        World {
            scenes: vec![],
        }
    }
    pub fn load_scene() {}

    pub fn create_scene() {}
}

impl Drop for World {
    fn drop(&mut self) {
        println!("World Drop")
    }
}

struct Scene {
    root_object: Box<i32>,
    objects: Vec<Box<i32>>,
}

impl Scene {
    pub fn create_game_object(&mut self) {
        let new_entity = Box::<i32>::from(12);
        self.objects.push(new_entity);
    }
    /// When the state is added to the stack, this method is called.
    fn on_start() {}
    /// When it is removed from the stack, this method is called.
    fn on_stop() {}
    /// When a State is pushed over the current one, the current one is paused.
    fn on_pause() {}
    /// When the State that was pushed over the current State is popped, the current one resumes.
    fn on_resume() {}
    /// Allows easily handling events, like the window closing or a key being pressed.
    fn handle_event() {}
    /// This method is called at a fixed time interval (default 1/60th second).
    fn fixed_update() {}
    /// This method is called as often as possible by the engine.
    fn update() {}
}

impl Drop for Scene {
    fn drop(&mut self) {
        println!("Scene Drop")
    }
}
