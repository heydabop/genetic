use crate::components::{Force, Velocity};
use crate::resources::DeltaTime;
use specs::{prelude::*, ReadExpect, ReadStorage, System, WriteStorage};
use std::f32::consts::PI;

pub struct ApplyForce;

impl<'a> System<'a> for ApplyForce {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Force>,
    );

    fn run(&mut self, (delta, mut velocities, forces): Self::SystemData) {
        let delta = delta.0;
        for (vel, force) in (&mut velocities, &forces).join() {
            vel.magnitude = delta.mul_add(force.translation, vel.magnitude).max(0.0);
            vel.heading = delta.mul_add(force.rotation, vel.heading);
            while vel.heading >= 2.0 * PI {
                vel.heading -= 2.0 * PI;
            }
            while vel.heading < 0.0 {
                vel.heading += 2.0 * PI;
            }
        }
    }
}
