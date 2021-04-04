use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use specs::{
    prelude::*, Builder, Component, DispatcherBuilder, Entities, NullStorage, ReadExpect,
    ReadStorage, System, VecStorage, World, WorldExt, WriteStorage,
};
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Agent {
    score: i32,
}

impl Agent {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn inc(&mut self) {
        self.score += 1;
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Target;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    heading: f32,
    magnitude: f32,
}

#[derive(Default)]
struct DeltaTime(f32);

struct PrintWorld;

impl<'a> System<'a> for PrintWorld {
    type SystemData = (Entities<'a>, ReadStorage<'a, Position>);

    fn run(&mut self, (entities, position): Self::SystemData) {
        for (entity, pos) in (&entities, &position).join() {
            println!("{}: {:?}", entity.id(), pos);
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
        let delta = delta.0;
        for (pos, vel) in (&mut position, &velocity).join() {
            pos.x += vel.heading.cos() * vel.magnitude * delta;
            pos.y += vel.heading.sin() * vel.magnitude * delta;
        }
    }
}

struct DecayVelocity;

impl<'a> System<'a> for DecayVelocity {
    type SystemData = WriteStorage<'a, Velocity>;

    fn run(&mut self, mut velocity: Self::SystemData) {
        for vel in (&mut velocity).join() {
            vel.magnitude *= 0.95;
        }
    }
}

struct CollisionCheck;

impl<'a> System<'a> for CollisionCheck {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Agent>,
        ReadStorage<'a, Target>,
        Entities<'a>,
    );

    fn run(&mut self, (position, mut agent, target, entities): Self::SystemData) {
        for (pos, agent) in (&position, &mut agent).join() {
            for (target, _, e) in (&position, &target, &entities).join() {
                if (pos.x - target.x).abs() < 5.0 && (pos.y - target.y).abs() < 5.0 {
                    // It's possible for multiple agents to hit the same target in a single tick here
                    // I'm okay with this because it seems "confusing" for an agent to follow behavior that normally results in a hit and it suddenly get nothing
                    entities
                        .delete(e)
                        .expect("Unable to delete target on collision");
                    agent.inc();
                    println!("Score: {}", agent.score());
                }
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("genetic", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let black = Color::RGB(0, 0, 0);
    let white = Color::RGB(255, 255, 255);

    canvas.set_draw_color(black);
    canvas.clear();
    canvas.present();

    let mut world = World::new();
    world.insert(DeltaTime(1.0 / 60.0));
    world.register::<Agent>();
    world.register::<Target>();
    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Target)
        .with(Position { x: 150.0, y: 250.0 })
        .build();
    world
        .create_entity()
        .with(Target)
        .with(Position { x: 178.0, y: 273.0 })
        .build();
    world
        .create_entity()
        .with(Target)
        .with(Position { x: 198.0, y: 303.5 })
        .build();
    world
        .create_entity()
        .with(Agent::new())
        .with(Position { x: 100.0, y: 200.0 })
        .with(Velocity {
            heading: 1.0 / 4.0 * PI,
            magnitude: 30.0,
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(ApplyVelocity, "apply_velocity", &[])
        .with(CollisionCheck, "collision_check", &["apply_velocity"])
        .build();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(black);
        canvas.clear();
        canvas.set_draw_color(white);

        {
            let position = world.read_storage::<Position>();
            let velocity = world.read_storage::<Velocity>();
            for (p, v) in (&position, (&velocity).maybe()).join() {
                if let Some(v) = v {
                    let mut heading = v.heading;
                    let x1 = (p.x + heading.cos() * 8.0).round() as i16;
                    let y1 = (p.y + heading.sin() * 8.0).round() as i16;
                    heading -= 1.0 / 2.0 * PI;
                    let x2 = (p.x + heading.cos() * 4.0).round() as i16;
                    let y2 = (p.y + heading.sin() * 4.0).round() as i16;
                    heading += PI;
                    let x3 = (p.x + heading.cos() * 4.0).round() as i16;
                    let y3 = (p.y + heading.sin() * 4.0).round() as i16;
                    canvas.aa_trigon(x1, y1, x2, y2, x3, y3, canvas.draw_color())
                } else {
                    canvas.aa_circle(
                        p.x.round() as i16,
                        p.y.round() as i16,
                        3,
                        canvas.draw_color(),
                    )
                }
                .expect("Error drawing to buffer");
            }
        };

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        dispatcher.dispatch(&world);
        world.maintain();
        thread::sleep(Duration::from_secs_f32(
            world.read_resource::<DeltaTime>().0,
        ));
    }
}
