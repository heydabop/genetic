mod components;
mod neural;
mod resources;
mod systems;

use components::{Agent, Position, Target, Velocity};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use resources::{DeltaTime, HitTargets, MaxPos};
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use specs::{prelude::*, World, WorldExt};
use std::collections::HashSet;
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;
use systems::{
    apply_velocity::ApplyVelocity, collision_check::CollisionCheck,
    spawn_new_targets::SpawnNewTargets,
};

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
                    let mut point_dir = v.heading;
                    let (sin, cos) = point_dir.sin_cos();
                    let x1 = cos.mul_add(8.0, p.x).round() as i16;
                    let y1 = sin.mul_add(8.0, p.y).round() as i16;
                    point_dir -= 1.0 / 2.0 * PI;
                    let (sin, cos) = point_dir.sin_cos();
                    let x2 = cos.mul_add(4.0, p.x).round() as i16;
                    let y2 = sin.mul_add(4.0, p.y).round() as i16;
                    point_dir += PI;
                    let (sin, cos) = point_dir.sin_cos();
                    let x3 = cos.mul_add(4.0, p.x).round() as i16;
                    let y3 = sin.mul_add(4.0, p.y).round() as i16;
                    canvas.trigon(x1, y1, x2, y2, x3, y3, canvas.draw_color())
                } else {
                    canvas.circle(
                        p.x.round() as i16,
                        p.y.round() as i16,
                        4,
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
