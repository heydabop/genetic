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
            pos.x = (pos.x + vel.heading.cos() * vel.magnitude * delta).rem_euclid(max.x);
            pos.y = (pos.y + vel.heading.sin() * vel.magnitude * delta).rem_euclid(max.y);
        }
    }
}
