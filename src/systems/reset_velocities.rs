use crate::components::Velocity;
use crate::resources::{ResetInterval, Ticks};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use specs::{prelude::*, ReadExpect, System, WriteStorage};
use std::f32::consts::PI;

pub struct ResetVelocities;

impl<'a> System<'a> for ResetVelocities {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadExpect<'a, Ticks>,
        ReadExpect<'a, ResetInterval>,
    );

    fn run(&mut self, (mut velocities, ticks, interval): Self::SystemData) {
        let interval = interval.0;
        if ticks.get() % interval != 0 {
            return;
        }

        let heading_range = Uniform::from(0.0..(2.0 * PI));
        let magnitude_range = Uniform::from(5.0..100.0);
        let mut rng = thread_rng();

        for v in (&mut velocities).join() {
            v.heading = heading_range.sample(&mut rng);
            v.magnitude = magnitude_range.sample(&mut rng);
        }
    }
}
