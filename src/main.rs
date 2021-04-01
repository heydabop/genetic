use specs::{
    Builder, Component, DispatcherBuilder, ReadExpect, ReadStorage, RunNow, System, VecStorage,
    World, WorldExt, WriteStorage,
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Default)]
struct DeltaTime(f32);

struct PrintWorld;

impl<'a> System<'a> for PrintWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for pos in position.join() {
            println!("{:?}", pos);
        }
    }
}

struct ApplyVelocity;

impl<'a> System<'a> for ApplyVelocity {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (delta, mut position, velocity): Self::SystemData) {
        use specs::Join;

        let delta = delta.0;
        for (pos, vel) in (&mut position, &velocity).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

struct DecayVelocity;

impl<'a> System<'a> for DecayVelocity {
    type SystemData = WriteStorage<'a, Velocity>;

    fn run(&mut self, mut velocity: Self::SystemData) {
        use specs::Join;

        for vel in (&mut velocity).join() {
            vel.x *= 0.95;
            vel.y *= 0.95;
        }
    }
}

fn main() {
    let mut world = World::new();
    world.insert(DeltaTime(1.0 / 60.0));
    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Position { x: 1.0, y: 2.0 })
        .build();
    world
        .create_entity()
        .with(Position { x: 100.0, y: 200.0 })
        .with(Velocity { x: 2.0, y: 1.0 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintWorld, "print_world", &[])
        .with(ApplyVelocity, "apply_velocity", &["print_world"])
        .with(PrintWorld, "print_world_updated", &["apply_velocity"])
        .build();

    dispatcher.dispatch(&mut world);
    world.maintain();
}
