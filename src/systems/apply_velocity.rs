use crate::components::{Position, Velocity};
use crate::resources::{DeltaTime, MaxPos};
use specs::{prelude::*, ReadExpect, ReadStorage, System, WriteStorage};

pub struct ApplyVelocity;

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
            let (sin, cos) = vel.heading.sin_cos();
            pos.x = (cos.mul_add(vel.magnitude * delta, pos.x)).rem_euclid(max.x);
            pos.y = (sin.mul_add(vel.magnitude * delta, pos.y)).rem_euclid(max.y);
        }
    }
}
