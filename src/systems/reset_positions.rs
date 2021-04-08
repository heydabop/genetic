use crate::components::Position;
use crate::resources::{MaxPos, ResetInterval, Ticks};
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use specs::{prelude::*, ReadExpect, System, WriteStorage};

pub struct ResetPositions;

impl<'a> System<'a> for ResetPositions {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadExpect<'a, Ticks>,
        ReadExpect<'a, ResetInterval>,
        ReadExpect<'a, MaxPos>,
    );

    fn run(&mut self, (mut positions, ticks, interval, max): Self::SystemData) {
        let interval = interval.0;
        if ticks.get() % interval != 0 {
            return;
        }
        let max = max.0;

        let x_range = Uniform::from(0.0..max.x as f32);
        let y_range = Uniform::from(0.0..max.y as f32);
        let mut rng = thread_rng();

        for p in (&mut positions).join() {
            p.x = x_range.sample(&mut rng);
            p.y = y_range.sample(&mut rng);
        }
    }
}
