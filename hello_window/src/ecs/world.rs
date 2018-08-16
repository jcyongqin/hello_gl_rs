pub use specs::{Builder, Join, Component, Read, ReadStorage, WriteStorage, System, VecStorage, World, RunNow, DispatcherBuilder};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

// resources, data which is shared between systems.
#[derive(Default)]
pub struct DeltaTime(f32);

// syatem
pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, mut position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (Read<'a, DeltaTime>,
                       ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;

        // `Read` implements `Deref`, so it
        // coerces to `&DeltaTime`.
        let delta = delta.0;

        for (vel, mut pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
        println!("asd");
    }
}