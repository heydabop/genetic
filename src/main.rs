mod components;
mod neural;
mod resources;
mod systems;

use components::{Agent, Fitness, Force, Position, Score, Target, Velocity};
use neural::Network;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use resources::{DeltaTime, HitTargets, MaxPos, ResetInterval, Ticks};
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use specs::{prelude::*, World, WorldExt};
use std::f32::consts::PI;
use std::{collections::HashSet, time::Instant};
use systems::{
    apply_force::ApplyForce, apply_velocity::ApplyVelocity, collision_check::CollisionCheck,
    control::Control, crossover::Crossover, mutate::Mutate, print_stats::PrintStats,
    rank_selection::RankSelection, reset_positions::ResetPositions, reset_scores::ResetScores,
    reset_velocities::ResetVelocities, spawn_new_targets::SpawnNewTargets,
    tick_counter::TickCounter, vision::Vision,
};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 1200;
    let window_height = 1200;
    let tickrate = 120;
    let framerate_ratio = 1; // ratio of framerate to tickrate, render FPS will be tickrate/framerate_ratio
    let num_targets = 50;
    let num_agents = 30;
    let population_lifetime_seconds = 60;

    let window = video_subsystem
        .window("genetic", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let black = Color::RGBA(30, 30, 30, 255);
    let white = Color::RGBA(225, 225, 225, 255);

    canvas.set_draw_color(black);
    canvas.clear();
    canvas.present();

    let mut world = World::new();
    world.insert(DeltaTime(1.0 / tickrate as f32));
    world.insert(MaxPos(Position {
        x: window_width as f32,
        y: window_height as f32,
    }));
    world.insert(HitTargets(HashSet::<specs::world::Index>::new()));
    world.insert(Ticks::default());
    world.insert(ResetInterval(tickrate as u64 * population_lifetime_seconds));
    world.register::<Agent>();
    world.register::<Score>();
    world.register::<Fitness>();
    world.register::<Target>();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Force>();

    let x_range = Uniform::from(0.0..window_width as f32);
    let y_range = Uniform::from(0.0..window_height as f32);
    let heading_range = Uniform::from(0.0..(2.0 * PI));
    let magnitude_range = Uniform::from(5.0..150.0);
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
            .with(Agent {
                inputs: None,
                network: Network::random(&mut rng, &[9, 15, 2]),
            })
            .with(Score::new())
            .with(Position {
                x: x_range.sample(&mut rng),
                y: y_range.sample(&mut rng),
            })
            .with(Velocity {
                heading: heading_range.sample(&mut rng),
                magnitude: magnitude_range.sample(&mut rng),
            })
            .with(Force::default())
            .build();
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(TickCounter, "tick_counter", &[])
        .with(Vision, "vision", &[])
        .with(Control, "control", &["vision"])
        .with(ApplyForce, "apply_force", &["control"])
        .with(ApplyVelocity, "apply_velocity", &["apply_force"])
        .with(CollisionCheck, "collision_check", &["apply_velocity"])
        .with(PrintStats, "print_stats", &["collision_check"])
        .with(SpawnNewTargets, "spawn_new_targets", &["collision_check"])
        .with(ResetVelocities, "reset_velocities", &["collision_check"])
        .with(RankSelection, "selection", &["collision_check"])
        .with(ResetPositions, "reset_positions", &["spawn_new_targets"])
        .with(ResetScores, "reset_scores", &["selection"])
        .with(Crossover, "crossover", &["selection"])
        .with(Mutate, "mutate", &["crossover"])
        .build();

    let mut skip = false;
    let mut skip_tick = 0;
    let mut skip_start = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager
        .set_framerate(tickrate)
        .expect("Unable to set framerate");
    'running: loop {
        canvas.set_draw_color(black);
        canvas.clear();
        canvas.set_draw_color(white);

        let ticks = world.read_resource::<Ticks>().get();

        if skip && ticks % world.read_resource::<ResetInterval>().0 == 0 {
            skip = false;
            let elapsed = skip_start.elapsed().as_secs_f32();
            let elapsed_ticks = ticks - skip_tick;
            println!(
                "Seconds: {elapsed:.2} - Ticks: {elapsed_ticks} - Tickrate: {:.2}",
                elapsed_ticks as f32 / elapsed
            );
        }

        if !skip && (framerate_ratio == 1 || ticks % framerate_ratio == 0) {
            let position = world.read_storage::<Position>();
            let velocity = world.read_storage::<Velocity>();
            for (p, v) in (&position, (&velocity).maybe()).join() {
                if let Some(v) = v {
                    let mut point_dir = v.heading;
                    let (sin, cos) = point_dir.sin_cos();
                    let x1 = cos.mul_add(6.0, p.x).round() as i16;
                    let y1 = sin.mul_add(6.0, p.y).round() as i16;
                    point_dir += 2.0 / 3.0 * PI;
                    let (sin, cos) = point_dir.sin_cos();
                    let x2 = cos.mul_add(5.0, p.x).round() as i16;
                    let y2 = sin.mul_add(5.0, p.y).round() as i16;
                    point_dir += 1.0 / 3.0 * PI;
                    let x3 = cos.mul_add(1.0, p.x).round() as i16;
                    let y3 = sin.mul_add(1.0, p.y).round() as i16;
                    point_dir += 1.0 / 3.0 * PI;
                    let (sin, cos) = point_dir.sin_cos();
                    let x4 = cos.mul_add(5.0, p.x).round() as i16;
                    let y4 = sin.mul_add(5.0, p.y).round() as i16;
                    canvas.polygon(&[x1, x2, x3, x4], &[y1, y2, y3, y4], canvas.draw_color())
                } else {
                    canvas.filled_circle(
                        p.x.round() as i16,
                        p.y.round() as i16,
                        4,
                        canvas.draw_color(),
                    )
                }
                .expect("Error drawing to buffer");
            }

            canvas.present();

            skip = false;
        }

        dispatcher.dispatch(&world);
        world.maintain();

        if !skip {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::G),
                        ..
                    } => {
                        skip = true;
                        skip_start = Instant::now();
                        skip_tick = ticks;
                    }
                    _ => {}
                }
            }

            fps_manager.delay();
        }
    }
}
