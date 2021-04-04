use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use specs::{
    prelude::*, Builder, Component, DispatcherBuilder, Entities, NullStorage, ReadExpect,
    ReadStorage, System, VecStorage, World, WorldExt, WriteStorage,
};
use std::collections::HashSet;
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

#[derive(Clone, Component, Copy, Debug)]
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

struct DeltaTime(f32);

struct MaxPos(Position);

struct HitTargets(HashSet<specs::world::Index>);

struct ApplyVelocity;

impl<'a> System<'a> for ApplyVelocity {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        ReadExpect<'a, MaxPos>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (delta, max, mut position, velocity): Self::SystemData) {
        let delta = delta.0;
        let max = max.0;
        for (pos, vel) in (&mut position, &velocity).join() {
            pos.x = (pos.x + vel.heading.cos() * vel.magnitude * delta).rem_euclid(max.x);
            pos.y = (pos.y + vel.heading.sin() * vel.magnitude * delta).rem_euclid(max.y);
        }
    }
}

struct CollisionCheck;

impl<'a> System<'a> for CollisionCheck {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Agent>,
        ReadStorage<'a, Target>,
        WriteExpect<'a, HitTargets>,
        Entities<'a>,
    );

    fn run(&mut self, (position, mut agent, target, mut hit_targets, entities): Self::SystemData) {
        let hit_targets = &mut (hit_targets.0);
        for (pos, agent) in (&position, &mut agent).join() {
            for (target, _, e) in (&position, &target, &entities).join() {
                if (pos.x - target.x).abs() < 5.0 && (pos.y - target.y).abs() < 5.0 {
                    // It's possible for multiple agents to hit the same target in a single tick here
                    // I'm okay with this because it seems "confusing" for an agent to follow behavior that normally results in a hit and it suddenly get nothing
                    hit_targets.insert(e.id());
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

struct SpawnNewTargets;

impl<'a> System<'a> for SpawnNewTargets {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Target>,
        WriteExpect<'a, HitTargets>,
        ReadExpect<'a, MaxPos>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut position, mut target, mut hit_targets, max, entities): Self::SystemData,
    ) {
        let max = max.0;
        hit_targets.0.drain().for_each(|_| {
            let t = entities.create();
            target
                .insert(t, Target)
                .expect("Unable to insert new Target");
            position
                .insert(
                    t,
                    Position {
                        x: thread_rng().gen_range(0.0..max.x),
                        y: thread_rng().gen_range(0.0..max.y),
                    },
                )
                .expect("Unable to insert new target Position");
        });
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 800;
    let window_height = 600;
    let num_targets = 10;
    let num_agents = 15;

    let window = video_subsystem
        .window("genetic", window_width, window_height)
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
    world.insert(MaxPos(Position {
        x: window_width as f32,
        y: window_height as f32,
    }));
    world.insert(HitTargets(HashSet::<specs::world::Index>::new()));
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

    let x_range = Uniform::from(0.0..window_width as f32);
    let y_range = Uniform::from(0.0..window_height as f32);
    let heading_range = Uniform::from(0.0..(2.0 * PI));
    let magnitude_range = Uniform::from(5.0..50.0);
    let mut rng = thread_rng();

    for _ in 0..num_targets {
        world
            .create_entity()
            .with(Target)
            .with(Position {
                x: x_range.sample(&mut rng),
                y: y_range.sample(&mut rng),
            })
            .build();
    }

    for _ in 0..num_agents {
        world
            .create_entity()
            .with(Agent::new())
            .with(Position {
                x: x_range.sample(&mut rng),
                y: y_range.sample(&mut rng),
            })
            .with(Velocity {
                heading: heading_range.sample(&mut rng),
                magnitude: magnitude_range.sample(&mut rng),
            })
            .build();
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(ApplyVelocity, "apply_velocity", &[])
        .with(CollisionCheck, "collision_check", &["apply_velocity"])
        .with(SpawnNewTargets, "spawn_new_targets", &["collision_check"])
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
                    canvas.trigon(x1, y1, x2, y2, x3, y3, canvas.draw_color())
                } else {
                    canvas.circle(
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
